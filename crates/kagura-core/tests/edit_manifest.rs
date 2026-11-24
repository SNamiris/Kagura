use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Barrier},
    thread,
};

use kagura_core::{
    index::{Index, SearchFilters},
    txn::{EditPatch, TxnError},
};
use serde_json::{json, Value};
use tempfile::{tempdir, TempDir};

const ID: &str = "00000000000000000000000002";
const REL: &str = "Library/Stages/Artist/Hall/_asset.manifest.json";

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let dest = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &dest);
        } else {
            fs::copy(entry.path(), dest).unwrap();
        }
    }
}

struct Fixture {
    _temp: TempDir,
    root: PathBuf,
    cache: PathBuf,
    journal: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let temp = tempdir().unwrap();
        let root = temp.path().join("MMD");
        copy_tree(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini_library"),
            &root,
        );
        let cache = temp.path().join("app/index.sqlite");
        let journal = temp.path().join("app/edits");
        let fixture = Self {
            _temp: temp,
            root,
            cache,
            journal,
        };
        fixture.index().update(&fixture.root).unwrap();
        fixture
    }
    fn index(&self) -> Index {
        let mut index = Index::open_for_library(&self.cache, &self.root).unwrap();
        index.set_edits_dir(&self.journal);
        index
    }
    fn raw(&self) -> Vec<u8> {
        fs::read(self.root.join(REL)).unwrap()
    }
    fn hash(&self) -> String {
        blake3::hash(&self.raw()).to_hex().to_string()
    }
    fn edit(&self, index: &mut Index, patch: Value) -> String {
        index
            .edit_manifest(
                &self.root,
                ID,
                serde_json::from_value(patch).unwrap(),
                &self.hash(),
            )
            .unwrap()
            .entry_id
    }
    fn log(&self) -> PathBuf {
        fs::read_dir(&self.journal)
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| p.extension().is_some_and(|ext| ext == "jsonl"))
            .unwrap()
    }
}

#[test]
fn edit_preserves_unrelated_lexemes_bom_crlf_and_updates_search_then_undo_restores_exact_bytes() {
    let fixture = Fixture::new();
    let mut index = fixture.index();
    let raw = String::from_utf8(fixture.raw()).unwrap().replace("\r\n", "\n");
    let extra = r#"  "scale": 1e-05,
  "name_original": "\u5929\u7a7a",
  "source": {"platform" : "example", "url":"https://old.example"}
"#;
    let original = format!(
        "\u{feff}{},\n{extra}}}\n",
        raw.trim_end().strip_suffix('}').unwrap().trim_end()
    )
    .replace('\n', "\r\n");
    fs::write(fixture.root.join(REL), &original).unwrap();
    let entry = fixture.edit(
        &mut index,
        json!({"tags":["kagura-test"],"rating":4,"note":"甲\n乙","source":{"url":"https://new.example/a"}}),
    );
    let current = String::from_utf8(fixture.raw()).unwrap();
    assert!(current.starts_with('\u{feff}'));
    assert!(!current.replace("\r\n", "").contains('\n'));
    assert!(current.contains(r#""scale": 1e-05"#));
    assert!(current.contains(r#""name_original": "\u5929\u7a7a""#));
    assert!(current.contains(r#""source": {"platform" : "example", "url":"https://new.example/a"}"#));
    let value = kagura_core::manifest::parse(current.as_bytes()).unwrap();
    let expected_tail = format!(
        ",\r\n  \"tags\": [\r\n    \"kagura-test\"\r\n  ],\r\n  \"rating\": 4,\r\n  \"note\": \"甲\\n乙\",\r\n  \"updated\": \"{}\"\r\n}}\r\n",
        value["updated"].as_str().unwrap()
    );
    let rewritten = original.replace("https://old.example", "https://new.example/a");
    assert_eq!(
        current,
        format!("{}{expected_tail}", rewritten.strip_suffix("\r\n}\r\n").unwrap())
    );
    assert_eq!(value["rating"], 4);
    assert_eq!(value["note"], "甲\n乙");
    assert_eq!(value["tags"], json!(["kagura-test"]));
    assert_eq!(index.get(ID).unwrap().unwrap(), value);
    assert_eq!(
        index
            .search("kagura-test", &SearchFilters::default(), 20, 0)
            .unwrap()
            .total,
        1
    );
    let history = index.list_edits(&fixture.root, 5).unwrap();
    assert_eq!(history[0].entry_id, entry);
    assert_eq!(history[0].relative_path, REL);
    assert_eq!(history[0].new_hash, fixture.hash());
    assert_eq!(history[0].timestamp.get(..10), value["updated"].as_str());
    assert!(history[0].timestamp.ends_with("+08:00"));
    assert_eq!(
        history[0].changed_fields,
        ["tags", "rating", "note", "source.url", "updated"]
    );
    let before: Value = serde_json::from_str(fs::read_to_string(fixture.log()).unwrap().trim()).unwrap();
    assert_eq!(before["old_bytes"].as_str().unwrap(), original);
    let undo = index.undo_edit(&fixture.root, &entry).unwrap();
    assert_eq!(undo.new_hash, blake3::hash(original.as_bytes()).to_hex().as_str());
    assert_eq!(fixture.raw(), original.as_bytes());
    assert_eq!(
        index
            .search("kagura-test", &SearchFilters::default(), 20, 0)
            .unwrap()
            .total,
        0
    );
    assert!(index.list_edits(&fixture.root, 1).unwrap()[0].undone);
    assert_eq!(
        index.undo_edit(&fixture.root, &entry).unwrap_err().code(),
        "conflict"
    );
    assert_eq!(fs::read_to_string(fixture.log()).unwrap().lines().count(), 2);
}

#[test]
fn rejects_unknown_patch_fields_schema_errors_and_external_changes() {
    let fixture = Fixture::new();
    let mut index = fixture.index();
    for f in [
        "id", "schema", "format", "type", "key", "added", "updated", "license",
    ] {
        assert!(
            serde_json::from_value::<EditPatch>(json!({f:"value"})).is_err(),
            "{f}"
        );
    }
    for bad in [
        json!({"source":{"platform":"x"}}),
        json!({"source":{}}),
        json!({"source":null}),
        json!({"note":null}),
        json!({"tags":null}),
        json!({"rating":null}),
    ] {
        assert!(serde_json::from_value::<EditPatch>(bad).is_err());
    }
    let first = fixture.hash();
    for invalid in [
        json!({"rating":6}),
        json!({"tags":["duplicate","duplicate"]}),
        json!({"source":{"url":"not a URI"}}),
    ] {
        let err = index
            .edit_manifest(
                &fixture.root,
                ID,
                serde_json::from_value(invalid).unwrap(),
                &first,
            )
            .unwrap_err();
        assert_eq!(err.code(), "schema_invalid");
    }
    assert_eq!(fixture.hash(), first);
    let base = fixture.hash();
    let to_save: EditPatch = serde_json::from_value(json!({"note":"later"})).unwrap();
    let entry = fixture.edit(&mut index, json!({"rating":0}));
    assert_eq!(
        index
            .edit_manifest(&fixture.root, ID, to_save, &base)
            .unwrap_err()
            .code(),
        "conflict"
    );
    let mut external = fixture.raw();
    external.extend(b" \n");
    fs::write(fixture.root.join(REL), &external).unwrap();
    assert!(matches!(
        index.undo_edit(&fixture.root, &entry),
        Err(TxnError::Conflict(_))
    ));
    assert_eq!(fixture.raw(), external);
    assert_eq!(index.list_edits(&fixture.root, 3).unwrap().len(), 1);
}

#[test]
fn nested_url_removal_preserves_neighbors_in_compact_and_mixed_json() {
    let fixture = Fixture::new();
    let mut index = fixture.index();
    let original = String::from_utf8(fixture.raw()).unwrap();
    for (members, expected) in [
        (
            r#""platform":"p","url" : "https://example.org","archive":"z""#,
            r#""platform":"p","archive":"z""#,
        ),
        (
            r#""platform":"p", "url":"https://example.org""#,
            r#""platform":"p""#,
        ),
        (r#""url":"https://example.org""#, ""),
    ] {
        let text = original.trim_end().strip_suffix('}').unwrap().trim_end();
        let text = format!("{text},\r\n \"source\":{{{members}}}\n}}");
        fs::write(fixture.root.join(REL), &text).unwrap();
        let entry = fixture.edit(&mut index, json!({"source":{"url":null}}));
        let new = String::from_utf8(fixture.raw()).unwrap();
        assert!(new.contains(&format!("\"source\":{{{expected}}}")));
        assert!(new.contains(",\r\n \"source\""));
        assert!(!new.ends_with('\n'));
        index.undo_edit(&fixture.root, &entry).unwrap();
        assert_eq!(fixture.raw(), text.as_bytes());
    }
}

#[test]
fn separate_connections_cannot_commit_the_same_base_twice() {
    let fixture = Fixture::new();
    let hash = fixture.hash();
    let barrier = Arc::new(Barrier::new(2));
    let mut workers = Vec::new();
    for rating in [1, 2] {
        let mut index = fixture.index();
        let root = fixture.root.clone();
        let base = hash.clone();
        let barrier = Arc::clone(&barrier);
        workers.push(thread::spawn(move || {
            barrier.wait();
            index.edit_manifest(
                &root,
                ID,
                EditPatch {
                    rating: Some(rating),
                    ..Default::default()
                },
                &base,
            )
        }));
    }
    let results = workers.into_iter().map(|w| w.join().unwrap()).collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|r| matches!(r, Err(TxnError::Conflict(_))))
            .count(),
        1
    );
    assert_eq!(fixture.index().list_edits(&fixture.root, 5).unwrap().len(), 1);
}

#[test]
fn pending_recovery_updates_only_bookkeeping_and_preserves_a_third_version() {
    let fixture = Fixture::new();
    let mut index = fixture.index();
    let original = fixture.raw();
    let entry = fixture.edit(&mut index, json!({"tags":["recover"]}));
    let log = fixture.log();
    let record = fs::read(&log).unwrap();
    let row: Value = serde_json::from_slice(&record).unwrap();
    let digest = blake3::hash(row["root"].as_str().unwrap().as_bytes()).to_hex();
    let pending = fixture.journal.join(format!(".pending-{digest}.json"));
    fs::rename(&log, &pending).unwrap(); // source committed; publishing the log was interrupted
    let current = fixture.raw();
    index.recover_edits(&fixture.root).unwrap();
    assert_eq!(fixture.raw(), current);
    assert_eq!(index.list_edits(&fixture.root, 1).unwrap()[0].entry_id, entry);
    assert_eq!(fs::read(&log).unwrap(), record);
    fs::write(&pending, &record).unwrap(); // log published, pending cleanup was interrupted
    index.recover_edits(&fixture.root).unwrap();
    assert_eq!(fs::read(&log).unwrap(), record);
    fs::remove_file(&log).unwrap();
    fs::write(&pending, &record).unwrap();
    fs::write(fixture.root.join(REL), &original).unwrap(); // source replace never happened
    index.recover_edits(&fixture.root).unwrap();
    assert!(!pending.exists() && !log.exists());
    fs::write(&pending, &record).unwrap();
    let mut other = original;
    other.push(b' ');
    fs::write(fixture.root.join(REL), &other).unwrap();
    assert_eq!(index.recover_edits(&fixture.root).unwrap_err().code(), "conflict");
    assert!(!pending.exists() && !log.exists());
    assert_eq!(fixture.raw(), other);
    index.recover_edits(&fixture.root).unwrap();
    assert!(fs::read_dir(&fixture.journal).unwrap().any(|e| e
        .unwrap()
        .file_name()
        .to_string_lossy()
        .starts_with(".conflict-")));
}
