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
