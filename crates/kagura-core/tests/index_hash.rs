use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use kagura_core::{index::Index, scan::scan_library};
use rusqlite::Connection;
use tempfile::{tempdir, TempDir};

const ID: &str = "00000000000000000000000001";
const DIR: &str = "Library/Models/Props/Builder/Box";
const MANIFEST: &[u8] =
    include_bytes!("fixtures/mini_library/Library/Models/Props/Builder/Box/_asset.manifest.json");

struct Library {
    root: PathBuf,
    db: PathBuf,
    index: Index,
    _temp: TempDir,
}

impl Library {
    fn new() -> Self {
        let temp = tempdir().unwrap();
        let root = temp.path().join("root");
        let dir = root.join(DIR);
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("_asset.manifest.json"), MANIFEST).unwrap();
        fs::write(dir.join("model.txt"), b"box").unwrap();
        fs::write(dir.join("sub/extra.pmx"), b"mesh").unwrap();
        let db = temp.path().join("test.sqlite");
        let mut index = Index::open(&db).unwrap();
        index.rebuild(&scan_library(&root)).unwrap();
        Self {
            _temp: temp,
            root,
            db,
            index,
        }
    }

    fn digest(&self, rel: &str) -> Option<String> {
        Connection::open(&self.db)
            .unwrap()
            .query_row(
                "SELECT blake3 FROM files WHERE asset_id = ?1 AND rel = ?2",
                [ID, rel],
                |row| row.get(0),
            )
            .unwrap()
    }

    fn file(&self, rel: &str) -> PathBuf {
        self.root.join(DIR).join(rel)
    }
}

fn replace_same_size(path: &Path, bytes: &[u8]) {
    let previous = fs::metadata(path).unwrap().modified().unwrap();
    fs::write(path, bytes).unwrap();
    fs::OpenOptions::new()
        .write(true)
        .open(path)
        .unwrap()
        .set_modified(previous + Duration::from_secs(20))
        .unwrap();
}

#[test]
fn hashes_are_opt_in_and_cached_by_size_and_mtime() {
    let mut lib = Library::new();
    assert_eq!(lib.digest("model.txt"), None);
    let first = lib.index.hash_asset(ID).unwrap();
    assert_eq!(
        (first.hashed_files, first.cached_files, first.bytes_hashed),
        (2, 0, 7)
    );
    assert_eq!(
        lib.digest("model.txt").unwrap(),
        blake3::hash(b"box").to_hex().as_str()
    );
    let second = lib.index.hash_asset(ID).unwrap();
    assert_eq!(
        (second.hashed_files, second.cached_files, second.bytes_hashed),
        (0, 2, 0)
    );

    replace_same_size(&lib.file("model.txt"), b"new");
    let refreshed = lib.index.hash_asset(ID).unwrap();
    assert_eq!(
        (
            refreshed.hashed_files,
            refreshed.cached_files,
            refreshed.bytes_hashed
        ),
        (1, 1, 3)
    );
    assert_eq!(
        lib.digest("model.txt").unwrap(),
        blake3::hash(b"new").to_hex().as_str()
    );

    fs::write(lib.file("sub/extra.pmx"), b"new mesh").unwrap();
    lib.index.update(&lib.root).unwrap();
    assert_eq!(lib.digest("sub/extra.pmx"), None);
    let changed = lib.index.hash_asset(ID).unwrap();
    assert_eq!(
        (changed.hashed_files, changed.cached_files, changed.bytes_hashed),
        (1, 1, 8)
    );
    assert!(lib.index.hash_asset("missing").is_err());
}

#[test]
fn hash_all_reports_each_file_and_detects_changes_before_commit() {
    let mut lib = Library::new();
    let model = lib.file("model.txt");
    assert!(lib
        .index
        .hash_all(|p| {
            if p.rel == "model.txt" {
                fs::write(&model, b"changed during progress callback").unwrap();
            }
        })
        .is_err());
    assert_eq!(lib.digest("model.txt"), None);
    assert_eq!(lib.digest("sub/extra.pmx"), None);

    let mut events = Vec::new();
    let result = lib
        .index
        .hash_all(|p| {
            assert_eq!(p.asset_id, ID);
            events.push((p.rel.to_owned(), p.completed_files, p.total_files));
        })
        .unwrap();
    assert_eq!(result.hashed_files, 2);
    assert_eq!(
        events,
        [("model.txt".to_owned(), 1, 2), ("sub/extra.pmx".to_owned(), 2, 2)]
    );
    assert_eq!(lib.index.hash_all(|_| {}).unwrap().cached_files, 2);

    fs::remove_file(model).unwrap();
    assert!(lib.index.hash_asset(ID).is_err());
    lib.index.update(&lib.root).unwrap();
    let reduced = lib
        .index
        .hash_all(|p| assert_eq!((p.rel, p.total_files), ("sub/extra.pmx", 1)))
        .unwrap();
    assert_eq!((reduced.hashed_files, reduced.cached_files), (0, 1));
}

#[test]
fn moved_library_requires_an_index_at_the_current_root() {
    let mut lib = Library::new();
    lib.index.hash_asset(ID).unwrap();
    let moved = lib.root.with_file_name("moved");
    fs::rename(&lib.root, &moved).unwrap();
    assert!(lib.index.hash_asset(ID).is_err());
    lib.index.rebuild(&scan_library(&moved)).unwrap();
    assert_eq!(lib.index.hash_asset(ID).unwrap().hashed_files, 2);
}

#[test]
fn parallel_hashes_resume_from_durable_files_and_refresh_stale_sizes() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let mut lib = Library::new();
    for i in 0..200_u16 {
        fs::write(lib.file(&format!("sub/{i:03}.bin")), i.to_le_bytes()).unwrap();
    }
    lib.index.update(&lib.root).unwrap();
    let keep = AtomicBool::new(true);
    let partial = lib
        .index
        .hash_all_parallel(
            &lib.root,
            2,
            || keep.load(Ordering::Relaxed),
            |p| {
                if p.completed_files >= 64 {
                    keep.store(false, Ordering::Relaxed);
                }
            },
        )
        .unwrap();
    assert!(partial.stopped);
    assert_eq!((partial.total_files, partial.total_bytes), (202, 407));
    assert!((64..202).contains(&partial.completed_files));
    let first = lib.index.hash_coverage().unwrap();
    assert_eq!(
        (first.cached_files, first.cached_bytes),
        (partial.completed_files, partial.completed_bytes)
    );
    drop(lib.index);
    lib.index = Index::open(&lib.db).unwrap();
    let resumed = lib
        .index
        .hash_all_parallel(&lib.root, 4, || true, |_| {})
        .unwrap();
    assert_eq!(resumed.cached_files, partial.completed_files);
    assert_eq!(resumed.cached_bytes, partial.completed_bytes);
    assert_eq!((resumed.completed_files, resumed.completed_bytes), (202, 407));
    assert!(!resumed.stopped);
    assert!(resumed.bytes_per_second > 0.0);
    assert_eq!(resumed.eta_seconds, Some(0.0));
    assert_eq!(
        lib.digest("sub/198.bin").unwrap(),
        blake3::hash(&198_u16.to_le_bytes()).to_hex().as_str()
    );
    fs::write(lib.file("model.txt"), b"new box").unwrap();
    let changed = lib
        .index
        .hash_all_parallel(&lib.root, 1, || true, |_| {})
        .unwrap();
    assert_eq!(
        (changed.cached_files, changed.total_bytes, changed.completed_bytes),
        (201, 411, 411)
    );
    assert_eq!(
        lib.digest("model.txt").unwrap(),
        blake3::hash(b"new box").to_hex().as_str()
    );
    assert!(lib
        .index
        .hash_all_parallel(&lib.root, 0, || true, |_| {})
        .is_err());
    assert!(lib
        .index
        .hash_all_parallel(&lib.root, 5, || true, |_| {})
        .is_err());
    assert!(lib
        .index
        .hash_all_parallel(&lib.root.parent().unwrap().join("wrong"), 1, || true, |_| {})
        .is_err());
}

#[test]
fn parallel_job_reports_a_concurrent_inventory_addition() {
    let mut lib = Library::new();
    let extra = lib.file("later.bin");
    let mut other = Index::open(&lib.db).unwrap();
    let mut added = false;
    let result = lib.index.hash_all_parallel(
        &lib.root,
        2,
        || true,
        |_| {
            if !added {
                fs::write(&extra, b"later").unwrap();
                other.update(&lib.root).unwrap();
                added = true;
            }
        },
    );
    assert!(result.unwrap_err().to_string().contains("文件清单"));
    let status = other.hash_coverage().unwrap();
    assert_eq!((status.total_files, status.completed_files), (3, 2));
    let resumed = lib
        .index
        .hash_all_parallel(&lib.root, 2, || true, |_| {})
        .unwrap();
    assert_eq!((resumed.completed_files, resumed.cached_files), (3, 2));
}

#[test]
fn interrupted_large_file_is_not_recorded_as_a_complete_digest() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let mut lib = Library::new();
    let large = vec![0x66; 4 * 1024 * 1024];
    fs::write(lib.file("000-large.bin"), &large).unwrap();
    lib.index.update(&lib.root).unwrap();
    let checks = AtomicUsize::new(0);
    let interrupted = lib
        .index
        .hash_all_parallel(
            &lib.root,
            1,
            || checks.fetch_add(1, Ordering::Relaxed) < 5,
            |_| {},
        )
        .unwrap();
    assert!(interrupted.stopped);
    assert_eq!(interrupted.completed_bytes, 0);
    assert_eq!(lib.digest("000-large.bin"), None);
    let done = lib
        .index
        .hash_all_parallel(&lib.root, 1, || true, |_| {})
        .unwrap();
    assert_eq!(done.completed_bytes, large.len() as u64 + 7);
    assert_eq!(
        lib.digest("000-large.bin").unwrap(),
        blake3::hash(&large).to_hex().as_str()
    );
}
