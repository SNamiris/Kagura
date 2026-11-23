//! Normalize only scanned asset manifests. Dry-run and apply calculate the same SHA-256 plan.
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

use kagura_core::consts::MANIFEST;
use kagura_core::manifest::{parse, DocumentFormat, NewlineStyle};
use kagura_core::scan::scan_library;
use serde::Serialize;
use sha2::{Digest, Sha256};
use tempfile::NamedTempFile;

const UNDO: &[u8] = include_bytes!("normalize_newlines/undo.py");

#[derive(Serialize)]
struct Entry {
    // Relative to Library, exactly as in the backup tree.
    path: String,
    old_sha256: String,
    new_sha256: String,
}

struct Plan {
    entries: Vec<Entry>,
    log: Vec<u8>,
    digest: String,
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn normalize(bytes: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(bytes.len());
    let mut rest = bytes;
    while let Some(next) = rest.iter().position(|&byte| byte == b'\n') {
        let line = &rest[..next];
        result.extend_from_slice(line.strip_suffix(b"\r").unwrap_or(line));
        result.push(b'\n');
        rest = &rest[next + 1..];
    }
    result.extend_from_slice(rest);
    result
}

fn refuse(message: impl Into<String>) -> io::Error {
    io::Error::other(message.into())
}

#[cfg(windows)]
fn reparse(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    metadata.file_attributes() & 0x400 != 0
}

#[cfg(not(windows))]
fn reparse(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

// Recheck from Library downwards, including the manifest itself, before each access.
fn asset_path(library: &Path, relative: &str) -> io::Result<PathBuf> {
    let mut path = library.to_owned();
    if reparse(&fs::symlink_metadata(&path)?) {
        return Err(refuse(format!("reparse point: {}", path.display())));
    }
    let relative = Path::new(relative);
    if relative.file_name() != Some(MANIFEST.as_ref()) {
        return Err(refuse(format!("not a manifest: {}", relative.display())));
    }
    for component in relative.components() {
        let Component::Normal(name) = component else {
            return Err(refuse(format!("invalid manifest path: {}", relative.display())));
        };
        path.push(name);
        if reparse(&fs::symlink_metadata(&path)?) {
            return Err(refuse(format!("reparse point: {}", path.display())));
        }
    }
    Ok(path)
}

fn plan(root: &Path) -> io::Result<Plan> {
    let scan = scan_library(root);
    for item in &scan.skipped {
        println!("skipped {:?}: {}", item.reason, item.path);
    }
    for item in &scan.errors {
        eprintln!("scan error: {}: {}", item.path, item.message);
    }
    if !scan.errors.is_empty() {
        return Err(refuse(format!("scan failed for {} paths", scan.errors.len())));
    }

    let library = root.join("Library");
    let mut entries = Vec::with_capacity(scan.records.len());
    let mut log = Vec::new();
    let (mut crlf, mut lf, mut mixed, mut bom, mut changes) = (0, 0, 0, 0, 0);
    for record in scan.records {
        let dir = record
            .dir
            .strip_prefix("Library/")
            .ok_or_else(|| refuse(&record.dir))?;
        let relative = format!("{dir}/{MANIFEST}");
        let path = asset_path(&library, &relative)?;
        let before = fs::read(&path)?;
        let after = normalize(&before);
        let decode = |bytes: &[u8]| parse(bytes).map_err(|e| refuse(format!("{relative}: {e}")));
        let original = decode(&before)?;
        if original != record.value || decode(&after)? != original {
            return Err(refuse(format!(
                "{relative}: JSON differs from scanned asset or after conversion"
            )));
        }

        let format = DocumentFormat::detect(&before);
        bom += usize::from(format.bom);
        match format.newline {
            NewlineStyle::Crlf => crlf += 1,
            NewlineStyle::Lf => lf += 1,
            NewlineStyle::Mixed { .. } => {
                mixed += 1;
                println!("mixed: {relative}");
            }
        }
        changes += usize::from(before != after);
        let entry = Entry {
            path: relative,
            old_sha256: sha256(&before),
            new_sha256: sha256(&after),
        };
        serde_json::to_writer(&mut log, &entry)?;
        log.push(b'\n');
        entries.push(entry);
    }
    let digest = sha256(&log);
    println!(
        "assets={} change={changes} unchanged={} CRLF={crlf} LF={lf} Mixed={mixed} BOM={bom} skipped={} pruned={} errors=0\nplan_sha256={digest}",
        entries.len(), entries.len() - changes, scan.skipped.len(), scan.pruned.len()
    );
    Ok(Plan { entries, log, digest })
}

fn write_new(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut file = File::create_new(path)?;
    file.write_all(bytes)?;
    file.sync_all()
}

fn read_original(library: &Path, entry: &Entry) -> io::Result<(PathBuf, Vec<u8>)> {
    let path = asset_path(library, &entry.path)?;
    let bytes = fs::read(&path)?;
    if sha256(&bytes) != entry.old_sha256 {
        return Err(refuse(format!("{}: file changed since scan", entry.path)));
    }
    Ok((path, bytes))
}

fn backup(library: &Path, run: &Path, plan: &Plan) -> io::Result<()> {
    // Creating the directory exclusively protects previous recovery bundles.
    let parent = run
        .parent()
        .ok_or_else(|| refuse("missing run directory parent"))?;
    let parent = fs::canonicalize(if parent.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent
    })?;
    let name = run.file_name().ok_or_else(|| refuse("invalid run directory"))?;
    let run = parent.join(name);
    if run.starts_with(fs::canonicalize(library)?) {
        return Err(refuse("the backup must be outside Library"));
    }
    fs::create_dir(&run)?;
    let backup = run.join("backup");
    fs::create_dir(&backup)?;
    write_new(&run.join("undo.py"), UNDO)?;
    for entry in &plan.entries {
        let (_, data) = read_original(library, entry)?;
        let file = backup.join(&entry.path);
        let parent = file.parent().expect("manifest has a backup parent");
        fs::create_dir_all(parent)?;
        write_new(&file, &data).map_err(|e| refuse(format!("backup {}: {e}", entry.path)))?;
    }
    write_new(&run.join("log.jsonl"), &plan.log)?;
    println!(
        "backups={} log_sha256={} run_dir={}",
        plan.entries.len(),
        plan.digest,
        run.display()
    );
    Ok(())
}

fn replace(library: &Path, entry: &Entry) -> io::Result<()> {
    let (path, original) = read_original(library, entry)?;
    let after = normalize(&original);
    if sha256(&after) != entry.new_sha256 {
        return Err(refuse("normalized SHA-256 differs from plan"));
    }
    let parent = path.parent().expect("manifest has a parent");
    let mut temp = NamedTempFile::new_in(parent)?;
    temp.write_all(&after)?;
    temp.as_file()
        .set_permissions(fs::metadata(&path)?.permissions())?;
    temp.as_file().sync_all()?;
    // Refuse to overwrite an edit made while preparing the replacement.
    read_original(library, entry)?;
    temp.persist(&path)?;
    Ok(())
}

fn apply(root: &Path, run: &Path, plan: &Plan) -> io::Result<()> {
    let library = root.join("Library");
    backup(&library, run, plan)?;
    let mut applied = 0;
    for entry in plan.entries.iter().filter(|e| e.old_sha256 != e.new_sha256) {
        replace(&library, entry).map_err(|e| {
            refuse(format!(
                "stopped after {applied} replacements at {}: {e}; recovery bundle: {}",
                entry.path,
                run.display()
            ))
        })?;
        applied += 1;
    }
    println!(
        "converted={applied} unchanged={} failures=0",
        plan.entries.len() - applied
    );
    Ok(())
}

fn run(args: Vec<OsString>) -> io::Result<()> {
    let usage = "usage: normalize_newlines <MMD root> --dry-run | <MMD root> --run-dir <new directory> [--expect-plan <SHA-256>]";
    let (root_arg, args) = args.split_first().ok_or_else(|| refuse(usage))?;
    let (run_dir, expected) = match args {
        [flag] if flag == "--dry-run" => (None, None),
        [flag, dir] if flag == "--run-dir" => (Some(Path::new(dir)), None),
        [flag, dir, check, digest] if flag == "--run-dir" && check == "--expect-plan" => (
            Some(Path::new(dir)),
            Some(digest.to_str().ok_or_else(|| refuse(usage))?),
        ),
        _ => return Err(refuse(usage)),
    };
    let root = fs::canonicalize(root_arg)?;
    let plan = plan(&root)?;
    if expected.is_some_and(|hash| hash != plan.digest) {
        return Err(refuse("plan changed since dry-run"));
    }
    if let Some(dir) = run_dir {
        apply(&root, dir, &plan)?;
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(std::env::args_os().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STAGE: &str =
        include_str!("../tests/fixtures/mini_library/Library/Stages/Artist/Hall/_asset.manifest.json");
    const REL: &str = "Stages/Artist/Hall/_asset.manifest.json";

    fn fixture(root: &Path) -> (PathBuf, Vec<u8>) {
        let file = root.join("Library").join(REL);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        let mut original = vec![0xef, 0xbb, 0xbf];
        original.extend_from_slice(STAGE.replace("\r\n", "\n").replacen('\n', "\r\n", 2).as_bytes());
        fs::write(&file, &original).unwrap();
        (file, original)
    }

    #[test]
    fn conversion_keeps_every_other_byte_and_excludes_non_assets() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let (file, original) = fixture(root);
        let nested = file.parent().unwrap().join("original");
        fs::create_dir(&nested).unwrap();
        for name in [MANIFEST, "info.json"] {
            fs::write(nested.join(name), &original).unwrap();
        }
        let foreign = root.join("Library/Stages/Artist/Other");
        fs::create_dir(&foreign).unwrap();
        let foreign_bytes = b"{\"schema\":1,\"format\":\"foreign\"}\r\n";
        fs::write(foreign.join(MANIFEST), foreign_bytes).unwrap();
        let reserve = root.join("Library/Motions/Song/_audio");
        fs::create_dir_all(&reserve).unwrap();
        fs::write(reserve.join(MANIFEST), &original).unwrap();

        run(vec![root.into(), "--dry-run".into()]).unwrap();
        assert_eq!(fs::read(&file).unwrap(), original);
        let run_dir = root.join("run");
        let plan = plan(root).unwrap();
        assert_eq!(plan.entries.len(), 1);
        assert_eq!(plan.entries[0].path, REL);
        assert!(run(vec![
            root.into(),
            "--run-dir".into(),
            run_dir.clone().into(),
            "--expect-plan".into(),
            "wrong".into()
        ])
        .is_err());
        assert!(!run_dir.exists());
        apply(root, &run_dir, &plan).unwrap();
        assert_eq!(fs::read(run_dir.join("log.jsonl")).unwrap(), plan.log);
        assert_eq!(fs::read(run_dir.join("backup").join(REL)).unwrap(), original);
        assert_eq!(fs::read(run_dir.join("undo.py")).unwrap(), UNDO);
        let new = fs::read(&file).unwrap();
        assert_eq!(new, normalize(&original));
        assert!(new.starts_with(b"\xef\xbb\xbf"));
        assert_eq!(parse(&new).unwrap(), parse(&original).unwrap());
        assert_eq!(fs::read(nested.join(MANIFEST)).unwrap(), original);
        assert_eq!(fs::read(nested.join("info.json")).unwrap(), original);
        assert_eq!(fs::read(reserve.join(MANIFEST)).unwrap(), original);
        assert_eq!(fs::read(foreign.join(MANIFEST)).unwrap(), foreign_bytes);
        assert!(apply(root, &run_dir, &plan).is_err());
        assert_eq!(fs::read(run_dir.join("backup").join(REL)).unwrap(), original);
    }

    #[test]
    fn stale_source_prevents_overwriting_the_edit() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let (file, _) = fixture(root);
        let plan = plan(root).unwrap();
        let run_dir = root.join("run");
        backup(&root.join("Library"), &run_dir, &plan).unwrap();
        let edited = STAGE.replace("stage.txt", "edited.txt");
        fs::write(&file, &edited).unwrap();
        assert!(replace(&root.join("Library"), &plan.entries[0]).is_err());
        assert_eq!(fs::read_to_string(file).unwrap(), edited);
    }
}
