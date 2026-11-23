use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use kagura_core::check::{check_library, Check};
use kagura_core::consts::MANIFEST;
use kagura_core::naming::utf16_len;
use serde_json::{json, Value};
use tempfile::tempdir;

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini_library")
}

fn copy_tree(source: &Path, dest: &Path) {
    fs::create_dir_all(dest).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = dest.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

fn change_manifest(dir: &Path, change: impl FnOnce(&mut Value)) {
    let path = dir.join(MANIFEST);
    let mut value = kagura_core::manifest::read(&path).unwrap();
    change(&mut value);
    fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
}

#[test]
fn clean_mini_library() {
    let report = check_library(&fixture());
    assert!(report.findings.is_empty(), "{:#?}", report.findings);
    assert_eq!(report.counts.values().sum::<usize>(), 5);
}

#[test]
fn reports_all_seven_checks_and_each_reference_field() {
    let temp = tempdir().unwrap();
    let root = temp.path();
    copy_tree(&fixture(), root);
    let model = root.join("Library/Models/Props/Builder/Box");
    let stage = root.join("Library/Stages/Artist/Hall");
    let song = root.join("Library/Motions/Beat(Live)_Composer");
    let motion = song.join("motion_Animator_take");
    let missing_id = "00000000000000000000000009";
    let duplicate = kagura_core::manifest::read(&model.join(MANIFEST)).unwrap()["id"].clone();

    change_manifest(&model, |v| {
        v["key"]["name"] = json!("Wrong");
        v["derived_from"] = json!(missing_id);
        v["variants"][0]["file"] = json!("missing.pmx");
    });
    change_manifest(&stage, |v| {
        v["id"] = duplicate;
        v["requires"] = json!([missing_id]);
        v["parts"] = json!([{"file":"missing.x"}]);
    });
    change_manifest(&song, |v| {
        v["related"] = json!([missing_id]);
        v["sets"] = json!([{"label":"test", "motion":[missing_id], "face":missing_id, "lip":missing_id,
                            "camera":missing_id, "pack":missing_id}]);
        v["audio"] = json!([{"label":"test","file":"missing.wav"}]);
    });
    change_manifest(&motion, |v| {
        v["song"] = json!(missing_id);
        v["tested_with"] = json!([missing_id]);
        v["files"][0]["file"] = json!("missing.vmd");
    });

    // The two names are distinct even on ordinary case-insensitive NTFS.
    fs::create_dir_all(root.join("Library/Stages/Straße/Orphan")).unwrap();
    fs::create_dir_all(root.join("Library/Stages/STRASSE/Other")).unwrap();
    fs::create_dir(song.join("_AuDiO/Empty")).unwrap();

    // Cross the library's 240-unit limit while remaining below legacy MAX_PATH.
    let relative = "Library/Models/Props/Builder/Box/original";
    let parent = root.join(relative);
    let chars = 241usize
        .saturating_sub(utf16_len(&parent.to_string_lossy()) + 1)
        .max(1);
    let long_name = "x".repeat(chars);
    fs::write(parent.join(&long_name), b"x").unwrap();

    let report = check_library(root);
    let mut issues = std::collections::BTreeMap::new();
    for finding in &report.findings {
        *issues.entry(finding.check).or_insert(0) += 1;
    }
    println!("injected mini_library: {issues:?}");
    assert_eq!(
        issues.keys().copied().collect::<BTreeSet<_>>(),
        BTreeSet::from([
            Check::IdUnique,
            Check::PathMatchesKey,
            Check::RefsResolve,
            Check::FilesExist,
            Check::NoOrphanDirs,
            Check::AuthorCase,
            Check::PathLength,
        ]),
        "{:#?}",
        report.findings
    );
    let details = |check| {
        report
            .findings
            .iter()
            .filter(move |f| f.check == check)
            .map(|f| &f.detail)
    };
    for field in [
        "song",
        "derived_from",
        "related[0]",
        "requires[0]",
        "tested_with[0]",
        "sets[0].motion[0]",
        "sets[0].face",
        "sets[0].lip",
        "sets[0].camera",
        "sets[0].pack",
    ] {
        let expected = format!("{field}: unknown id {missing_id}");
        assert!(
            details(Check::RefsResolve).any(|message| message == &expected),
            "{expected}"
        );
    }
    for field in ["variants", "parts", "files", "audio"] {
        assert!(details(Check::FilesExist).any(|message| message.starts_with(&format!("{field}[0].file:"))));
    }
    let has = |check, path: &str, text: &str| {
        report
            .findings
            .iter()
            .any(|f| f.check == check && f.path == path && f.detail.contains(text))
    };
    assert!(has(
        Check::NoOrphanDirs,
        "Library/Stages/Straße/Orphan",
        "no valid asset manifest"
    ));
    assert!(has(
        Check::NoOrphanDirs,
        "Library/Motions/Beat(Live)_Composer/_AuDiO/Empty",
        "empty directory"
    ));
    assert!(has(Check::AuthorCase, "Library/Stages", "Straße"));
    assert!(has(
        Check::PathLength,
        &format!("{relative}/{long_name}"),
        "UTF-16"
    ));
}

#[test]
#[ignore = "set KAGURA_LIBRARY to the MMD root and run with --ignored"]
fn real_library_is_consistent() {
    let root = std::env::var_os("KAGURA_LIBRARY").expect("set KAGURA_LIBRARY");
    let report = check_library(Path::new(&root));
    assert_eq!(report.counts.values().sum::<usize>(), 8824);
    assert!(report.findings.is_empty(), "{:#?}", report.findings);
}
