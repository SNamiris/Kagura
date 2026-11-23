use std::{
    fs,
    path::{Path, PathBuf},
};

use kagura_core::{
    consts::MANIFEST,
    index::{Index, IndexChanges, SearchFilters},
    scan::{scan_library, scan_library_for_check},
};
use rusqlite::Connection;
use serde_json::{json, Value};

const MODEL: &str = "Library/Models/Props/Builder/Box";
const STAGE: &str = "Library/Stages/Artist/Hall";
const SONG: &str = "00000000000000000000000003";
const BOX_ID: &str = "00000000000000000000000001";

fn mini() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini_library")
}

fn copy_library(root: &Path) {
    let fixture = mini();
    let scan = scan_library_for_check(&fixture);
    assert!(scan.errors.is_empty());
    for file in scan.files {
        let dest = root.join(&file);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(fixture.join(file), dest).unwrap();
    }
}

fn write_model(directory: &Path, model: &Value) {
    fs::create_dir_all(directory).unwrap();
    fs::write(directory.join(MANIFEST), serde_json::to_vec(model).unwrap()).unwrap();
}

#[test]
fn inventory_keeps_song_attachments_and_part_files_with_their_owners() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("nested/cache.sqlite");
    let mut index = Index::open_for_library(&path, &mini()).unwrap();
    let changes = index.rebuild(&scan_library(&mini())).unwrap();
    assert_eq!(changes.inserted, 5);
    assert_eq!(index.update(&mini()).unwrap(), IndexChanges::default());
    let conn = Connection::open(path).unwrap();
    let mut files = conn
        .prepare("SELECT rel FROM files WHERE asset_id=? ORDER BY rel")
        .unwrap();
    let song_files: Vec<String> = files
        .query_map([SONG], |row| row.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(song_files, ["_AuDiO/_asset.manifest.json", "_AuDiO/track.txt"]);
    let model_files: Vec<String> = files
        .query_map([BOX_ID], |row| row.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert!(model_files.contains(&"original/_asset.manifest.json".to_owned()));
    assert!(!model_files.contains(&MANIFEST.to_owned()));
    assert_eq!(index.children(SONG).unwrap().len(), 1);
    assert!(conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM refs WHERE from_id=? AND field='sets[0].motion[0]')",
            [SONG],
            |row| row.get::<_, bool>(0),
        )
        .unwrap());
}

#[test]
fn update_applies_asset_and_file_deltas_and_rolls_back_failures() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("source");
    copy_library(&root);
    let db = temp.path().join("cache.sqlite");
    let mut index = Index::open(&db).unwrap();
    assert_eq!(index.update(&root).unwrap().inserted, 5);
    let mut model = index.get(BOX_ID).unwrap().unwrap();
    model["tags"] = json!(["wood"]);
    write_model(&root.join(MODEL), &model);
    fs::remove_file(root.join(STAGE).join(MANIFEST)).unwrap();
    let new_id = "00000000000000000000000006";
    let new_dir = root.join("Library/Models/Props/Builder/Second");
    model["id"] = json!(new_id);
    model["key"]["name"] = json!("Second");
    write_model(&new_dir, &model);
    fs::write(new_dir.join("model.txt"), "payload").unwrap();
    let changes = index.update(&root).unwrap();
    assert_eq!((changes.inserted, changes.updated, changes.deleted), (1, 1, 1));
    assert_eq!(changes.files_changed, 1);
    assert_eq!(index.stats().unwrap().counts.get("stage"), None);
    assert_eq!(index.get(new_id).unwrap().unwrap(), model);
    assert_eq!(
        index
            .search("wood", &SearchFilters::default(), 10, 0)
            .unwrap()
            .total,
        2
    );
    assert_eq!(index.update(&root).unwrap(), IndexChanges::default());

    // Conflicting ID, then malformed JSON: neither failure publishes partial changes.
    model["id"] = json!(BOX_ID);
    write_model(&new_dir, &model);
    // Force a distinct stamp even on filesystems with coarse-grained mtimes.
    fs::write(
        new_dir.join(MANIFEST),
        serde_json::to_string_pretty(&model).unwrap(),
    )
    .unwrap();
    assert!(index
        .update(&root)
        .unwrap_err()
        .to_string()
        .contains("duplicate asset id"));
    fs::write(root.join(MODEL).join(MANIFEST), "[").unwrap();
    assert!(index.update(&root).is_err());
    assert!(index.get(new_id).unwrap().is_some());
    assert_eq!(index.get(BOX_ID).unwrap().unwrap()["tags"], json!(["wood"]));

    let fresh = temp.path().join("other-root");
    copy_library(&fresh);
    assert_eq!(index.update(&fresh).unwrap().inserted, 5);
    assert!(index.get(new_id).unwrap().is_none());
    assert_eq!(index.get(BOX_ID).unwrap().unwrap().get("tags"), None);
}

#[test]
fn old_schema_is_recreated_and_an_unrelated_database_is_preserved() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("index.sqlite");
    let mut index = Index::open(&path).unwrap();
    index.update(&mini()).unwrap();
    drop(index);
    Connection::open(&path)
        .unwrap()
        .execute_batch(
            "PRAGMA user_version=999;
             CREATE TABLE \"old\"\"notes\"(id INTEGER);
             CREATE VIEW old_view AS SELECT id FROM assets;
             CREATE VIRTUAL TABLE old_search USING fts5(text);
             INSERT INTO old_search(text) VALUES('obsolete');",
        )
        .unwrap();
    let mut index = Index::open(&path).unwrap();
    assert!(index.stats().unwrap().counts.is_empty());
    assert_eq!(
        Connection::open(&path)
            .unwrap()
            .query_row(
                "SELECT count(*) FROM sqlite_schema WHERE name LIKE 'old%'",
                [],
                |r| r.get::<_, i64>(0),
            )
            .unwrap(),
        0
    );
    assert_eq!(index.update(&mini()).unwrap().inserted, 5);

    let unrelated = temp.path().join("notes.sqlite");
    let conn = Connection::open(&unrelated).unwrap();
    conn.execute_batch("CREATE TABLE sqliteNotes(body TEXT); INSERT INTO sqliteNotes VALUES('keep');")
        .unwrap();
    assert!(Index::open(&unrelated).is_err());
    assert_eq!(
        conn.query_row("SELECT body FROM sqliteNotes", [], |row| row.get::<_, String>(0))
            .unwrap(),
        "keep"
    );
}

#[test]
fn refuses_source_output_before_creating_directories() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("source");
    copy_library(&root);
    let target = root.join("nested/cache/index.sqlite");
    assert!(Index::open_for_library(&target, &root).is_err());
    assert!(Index::open(&target).is_err());
    assert!(!root.join("nested").exists());
}

#[test]
fn cached_classifications_use_both_size_and_mtime() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("source");
    copy_library(&root);
    let dir = root.join("Library/Models/Props/Builder/Extra");
    fs::create_dir_all(&dir).unwrap();
    let foreign = dir.join(MANIFEST);
    fs::write(&foreign, r#"{"schema":1,"format":"other"}"#).unwrap();
    let mut index = Index::open(temp.path().join("cache.sqlite")).unwrap();
    index.rebuild(&scan_library(&root)).unwrap();
    let mut model = index.get(BOX_ID).unwrap().unwrap();
    for path in [&foreign, &root.join(MODEL).join(MANIFEST)] {
        // Simulate an external tool preserving BOTH freshness indicators. An attempted
        // reparse of these bytes would fail, for both assets and skipped foreign manifests.
        let meta = fs::metadata(path).unwrap();
        fs::write(path, vec![b'['; meta.len() as usize]).unwrap();
        fs::OpenOptions::new()
            .write(true)
            .open(path)
            .unwrap()
            .set_modified(meta.modified().unwrap())
            .unwrap();
    }
    assert_eq!(index.update(&root).unwrap(), IndexChanges::default());
    let model_path = root.join(MODEL).join(MANIFEST);
    let previous = fs::metadata(&model_path).unwrap().modified().unwrap();
    fs::OpenOptions::new()
        .write(true)
        .open(model_path)
        .unwrap()
        .set_modified(previous + std::time::Duration::from_secs(30))
        .unwrap();
    assert!(index.update(&root).is_err()); // mtime alone invalidates the classification.
    write_model(&root.join(MODEL), &model);
    model["id"] = json!("00000000000000000000000006");
    write_model(&dir, &model);
    assert_eq!(index.update(&root).unwrap().inserted, 1);
    fs::write(&foreign, "[]").unwrap();
    assert_eq!(index.update(&root).unwrap().deleted, 1);
    assert_eq!(index.update(&root).unwrap(), IndexChanges::default());
}
