//! Read-only consistency checks for a Kagura library.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Component, Path};

use serde::Serialize;
use serde_json::Value;
use unicase::UniCase;

use crate::consts::{MANIFEST, PATH_LIMIT};
use crate::layout::{destination, expected_type, song_dir, SongMap};
use crate::manifest::NewlineStyle;
use crate::naming::utf16_len;
use crate::scan::{is_reparse, Record, ScanReport};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Check {
    IdUnique,
    PathMatchesKey,
    RefsResolve,
    FilesExist,
    NoOrphanDirs,
    AuthorCase,
    PathLength,
    NewlineStyle,
    Scan,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Finding {
    pub check: Check,
    /// POSIX path relative to the supplied root.
    pub path: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
pub struct CheckReport {
    pub counts: BTreeMap<String, usize>,
    pub check_counts: BTreeMap<Check, usize>,
    pub skipped: usize,
    pub pruned: usize,
    pub mixed_newlines: Vec<String>,
    pub findings: Vec<Finding>,
}

impl Default for CheckReport {
    fn default() -> Self {
        use Check::*;
        let checks = [
            IdUnique,
            PathMatchesKey,
            RefsResolve,
            FilesExist,
            NoOrphanDirs,
            AuthorCase,
            PathLength,
            NewlineStyle,
            Scan,
        ];
        Self {
            counts: BTreeMap::new(),
            check_counts: checks.map(|check| (check, 0)).into(),
            skipped: 0,
            pruned: 0,
            mixed_newlines: Vec::new(),
            findings: Vec::new(),
        }
    }
}

impl CheckReport {
    fn add(&mut self, check: Check, path: &str, detail: impl Into<String>) {
        *self.check_counts.entry(check).or_default() += 1;
        self.findings.push(Finding {
            check,
            path: path.to_owned(),
            detail: detail.into(),
        });
    }
}

/// Audit the library under root/Library and return all independent findings.
pub fn check_library(root: &Path) -> CheckReport {
    let mut report = CheckReport::default();
    let root = match std::path::absolute(root) {
        Ok(root) => root,
        Err(error) => {
            report.add(Check::Scan, "Library", error.to_string());
            return report;
        }
    };
    let scan = crate::scan::scan_library_for_check(&root);
    report.skipped = scan.skipped.len();
    report.pruned = scan.pruned.len();
    for error in &scan.errors {
        report.add(Check::Scan, &error.path, &error.message);
    }

    let mut ids = HashMap::new();
    let mut songs = SongMap::new();
    for record in &scan.records {
        let value = &record.value;
        // scan.records contains only schema-validated assets.
        let id = value["id"].as_str().expect("validated id");
        let ty = value["type"].as_str().expect("validated type");
        *report.counts.entry(ty.to_owned()).or_default() += 1;
        if let Some(first) = ids.insert(id, &record.dir) {
            report.add(
                Check::IdUnique,
                &record.dir,
                format!("{id} also occurs in {first}"),
            );
        }
        // Build the map even when another song is misplaced so its error doesn't
        // prevent checking the rest of the library. destination reports bad keys below.
        if ty == "song" {
            if let Ok(directory) = song_dir(&value["key"]) {
                songs.entry(id.to_owned()).or_insert(directory);
            }
        }
    }

    for record in &scan.records {
        check_newline(record, &mut report);
        match destination(&record.value, &songs) {
            Ok(expected) if expected != record.dir => {
                report.add(Check::PathMatchesKey, &record.dir, expected)
            }
            Err(error) => report.add(Check::PathMatchesKey, &record.dir, error.to_string()),
            _ => {}
        }
        check_references(record, |field, id| {
            if !ids.contains_key(id) {
                report.add(
                    Check::RefsResolve,
                    &record.dir,
                    format!("{field}: unknown id {id}"),
                );
            }
        });
        check_files(&root, record, &mut report);
    }
    check_directories(&root, &scan, &mut report);
    report.mixed_newlines = scan.mixed_newlines;
    report.findings.sort();
    report
}

fn check_newline(record: &Record, report: &mut CheckReport) {
    let detail = match record.format.newline {
        NewlineStyle::Lf => return,
        NewlineStyle::Crlf => "CRLF".to_owned(),
        NewlineStyle::Mixed { crlf, lf } => format!("Mixed (CRLF={crlf}, LF={lf})"),
    };
    report.add(Check::NewlineStyle, &format!("{}/{MANIFEST}", record.dir), detail);
}

pub(crate) fn check_references(record: &Record, mut visit: impl FnMut(&str, &str)) {
    let value = &record.value;
    for field in ["song", "derived_from"] {
        if let Some(id) = value[field].as_str() {
            visit(field, id);
        }
    }
    for field in ["related", "requires", "tested_with"] {
        for (i, id) in array(&value[field]).iter().enumerate() {
            if let Some(id) = id.as_str() {
                visit(&format!("{field}[{i}]"), id);
            }
        }
    }
    for (i, set) in array(&value["sets"]).iter().enumerate() {
        for (j, id) in array(&set["motion"]).iter().enumerate() {
            if let Some(id) = id.as_str() {
                visit(&format!("sets[{i}].motion[{j}]"), id);
            }
        }
        for field in ["face", "lip", "camera", "pack"] {
            if let Some(id) = set[field].as_str() {
                visit(&format!("sets[{i}].{field}"), id);
            }
        }
    }
}

fn array(value: &Value) -> &[Value] {
    value.as_array().map(Vec::as_slice).unwrap_or_default()
}

fn check_files(root: &Path, record: &Record, report: &mut CheckReport) {
    let directory = root.join(&record.dir);
    for field in ["variants", "parts", "files", "audio"] {
        for (i, entry) in array(&record.value[field]).iter().enumerate() {
            let Some(file) = entry["file"].as_str() else {
                continue;
            };
            let reason = match referenced_file(&directory, file) {
                Ok(true) => continue,
                Ok(false) => "not a regular library file".to_owned(),
                Err(error) => error.to_string(),
            };
            report.add(
                Check::FilesExist,
                &record.dir,
                format!("{field}[{i}].file: {file}: {reason}"),
            );
        }
    }
}

fn referenced_file(directory: &Path, relative: &str) -> io::Result<bool> {
    // The source schema accepts both separator styles. Match the host filesystem's
    // spelling rules, but never follow a junction/symlink while checking a reference.
    let normalized = relative.replace('\\', "/");
    let mut path = directory.to_owned();
    let mut file = false;
    for component in Path::new(&normalized).components() {
        match component {
            Component::Normal(name) => path.push(name),
            Component::CurDir => continue,
            _ => return Ok(false),
        }
        let metadata = fs::symlink_metadata(&path)?;
        if is_reparse(&metadata) {
            return Ok(false);
        }
        file = metadata.is_file();
    }
    Ok(file)
}

fn check_directories(root: &Path, scan: &ScanReport, report: &mut CheckReport) {
    let assets: HashSet<_> = scan.records.iter().map(|record| record.dir.as_str()).collect();
    let mut authors = BTreeMap::<(&str, UniCase<&str>), BTreeSet<&str>>::new();
    for directory in &scan.directories {
        let path = directory.path.as_str();
        if directory.empty {
            report.add(Check::NoOrphanDirs, path, "empty directory");
        }
        if expected_type(path).is_some() && !assets.contains(path) {
            report.add(Check::NoOrphanDirs, path, "no valid asset manifest");
        }
        if let Some((parent, name)) = path.rsplit_once('/') {
            if matches!(
                parent,
                "Library/Stages" | "Library/Models/Characters" | "Library/Models/Props"
            ) {
                authors
                    .entry((parent, UniCase::unicode(name)))
                    .or_default()
                    .insert(name);
            }
        }
    }
    for ((parent, _), spellings) in authors {
        if spellings.len() > 1 {
            report.add(
                Check::AuthorCase,
                parent,
                spellings.into_iter().collect::<Vec<_>>().join(" / "),
            );
        }
    }
    for relative in scan.directories.iter().map(|d| &d.path).chain(&scan.files) {
        let full = root.join(relative);
        let length = utf16_len(&full.to_string_lossy());
        if length > PATH_LIMIT {
            report.add(
                Check::PathLength,
                relative,
                format!("{length} UTF-16 units (limit {PATH_LIMIT})"),
            );
        }
    }
}
