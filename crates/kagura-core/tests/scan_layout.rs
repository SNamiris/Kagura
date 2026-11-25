#[path = "common/live_baseline.rs"]
mod live_baseline;

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use kagura_core::consts::MANIFEST;
use kagura_core::layout::{destination, LayoutError};
use kagura_core::manifest::NewlineStyle;
use kagura_core::scan::{scan_library, scan_library_for_check, song_map, SkipReason};
use serde_json::{json, Value};

fn mini() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini_library")
}

#[test]
fn mini_records_skip_original_and_reserved() {
    let report = scan_library(&mini());
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    assert_eq!(report.records.len(), 5);
    assert_eq!(report.skipped.len(), 1);
    assert_eq!(report.skipped[0].reason, SkipReason::UnexpectedDepth);
    assert!(report.skipped[0].path.ends_with("/original/_asset.manifest.json"));
    assert_eq!(report.pruned.len(), 1);
    assert_eq!(report.pruned[0].reason, SkipReason::Reserved);
    assert!(report.pruned[0].path.ends_with("/_AuDiO"));
    assert!(report.files.iter().any(|p| p.ends_with("original/info.json")));
    assert!(!report.files.iter().any(|p| p.contains("/_AuDiO/")));
    let songs = song_map(&report.records).unwrap();
    for record in &report.records {
        assert_eq!(destination(&record.value, &songs).unwrap(), record.dir);
    }
    let inventory = scan_library_for_check(&mini());
    assert_eq!(inventory.records.len(), report.records.len());
    assert_eq!(inventory.skipped.len(), report.skipped.len());
    assert!(inventory.files.iter().any(|p| p.ends_with("/_AuDiO/track.txt")));
}

#[test]
fn song_index_rejects_duplicate_id_and_wrong_directory() {
    let report = scan_library(&mini());
    let song = report
        .records
        .into_iter()
        .find(|r| r.value["type"] == "song")
        .unwrap();
    let mut other = song.clone();
    other.dir = "Library/Motions/wrong".into();
    assert!(matches!(
        song_map(&[other]),
        Err(LayoutError::SongDirectory { .. })
    ));
    assert!(matches!(
        song_map(&[song.clone(), song]),
        Err(LayoutError::DuplicateSong(_))
    ));
}

#[test]
fn skipped_foreign_and_errors_remain_visible() {
    let root = tempfile::tempdir().unwrap();
    let base = root.path().join("Library/Stages/Author");
    let misplaced =
        include_str!("fixtures/mini_library/Library/Models/Props/Builder/Box/_asset.manifest.json");
    for (sub, raw) in [
        ("foreign", r#"{"schema":1,"format":"another"}"#),
        ("runtime", r#"{"name":"stage:test","stage":{}}"#),
        ("other", r#"[1]"#),
        (
            "invalid_schema",
            r#"{"format":"kagura-asset-manifest","schema":1,"type":"stage"}"#,
        ),
        ("invalid_json", "{"),
        ("misplaced", misplaced),
        ("wrong/depth", "{"),
    ] {
        let dir = base.join(sub);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(MANIFEST), raw).unwrap();
    }
    let report = scan_library(root.path());
    assert!(report.records.is_empty());
    assert_eq!(report.errors.len(), 3, "{:?}", report.errors);
    assert_eq!(report.skipped.len(), 4, "{:?}", report.skipped);
    for reason in [
        SkipReason::ForeignManifest,
        SkipReason::RuntimeConfig,
        SkipReason::SourceJson,
        SkipReason::UnexpectedDepth,
    ] {
        assert!(report.skipped.iter().any(|s| s.reason == reason));
    }
}

#[test]
fn mixed_newlines_are_reported_before_validation() {
    let root = tempfile::tempdir().unwrap();
    let base = root.path().join("Library/Stages/Artist");
    let stage = include_str!("fixtures/mini_library/Library/Stages/Artist/Hall/_asset.manifest.json");
    let lf = stage.replace("\r\n", "\n");
    let mixed = format!("\u{feff}{}", lf.replacen('\n', "\r\n", 1));
    for (dir, raw) in [
        ("Hall", mixed.as_str()),
        ("Foreign", "{\r\n\"schema\": 1\n}"),
        ("Broken", "[\r\n!\n]"),
        ("Other", "{}\r\n"),
        ("Other/deeper", "[\r\n!\n]"),
    ] {
        let dir = base.join(dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(MANIFEST), raw).unwrap();
    }

    let report = scan_library(root.path());
    let expected =
        ["Broken", "Foreign", "Hall"].map(|name| format!("Library/Stages/Artist/{name}/{MANIFEST}"));
    assert_eq!(report.mixed_newlines, expected);
    assert_eq!(report.records.len(), 1);
    let format = report.records[0].format;
    assert!(format.bom);
    assert_eq!(
        format.newline,
        NewlineStyle::Mixed {
            lf: lf.matches('\n').count() - 1,
            crlf: 1
        }
    );
    assert_eq!(report.errors.len(), 1);
    assert_eq!(report.skipped.len(), 3);
    assert_eq!(
        kagura_core::check::check_library(root.path()).mixed_newlines,
        expected
    );
}

#[cfg(any(windows, unix))]
#[test]
fn reparse_subtrees_are_not_followed() {
    let root = tempfile::tempdir().unwrap();
    let lib = root.path().join("Library");
    let target = root.path().join("outside");
    let link = lib.join("link");
    fs::create_dir(&lib).unwrap();
    fs::create_dir(&target).unwrap();
    fs::write(target.join(MANIFEST), b"{").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&target, &link).unwrap();
    #[cfg(windows)]
    {
        // A junction does not require Developer Mode or symlink privileges.
        let output = std::process::Command::new("cmd.exe")
            .args(["/d", "/c", "mklink", "/J"])
            .arg(&link)
            .arg(&target)
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
    }
    let report = scan_library(root.path());
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    assert!(report.files.is_empty());
    assert!(report.skipped.is_empty());
    assert_eq!(report.pruned.len(), 1);
    assert_eq!(report.pruned[0].path, "Library/link");
    assert_eq!(report.pruned[0].reason, SkipReason::Reparse);
}

#[test]
fn real_scan_and_destinations_match_python() {
    let Some(root) = env::var_os("KAGURA_LIBRARY") else {
        return;
    };
    let report = scan_library(&PathBuf::from(root));
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    let songs = song_map(&report.records).unwrap();
    let mut actual = BTreeMap::new();
    let mut counts = BTreeMap::new();
    for record in &report.records {
        let ty = record.value["type"].as_str().unwrap();
        let id = record.value["id"].as_str().unwrap();
        *counts.entry(ty.to_owned()).or_insert(0usize) += 1;
        assert!(
            actual
                .insert(id.to_owned(), (ty.to_owned(), record.dir.clone()))
                .is_none(),
            "duplicate ID {id}"
        );
        assert_eq!(destination(&record.value, &songs).unwrap(), record.dir, "{id}");
    }
    let summary: Value = serde_json::from_str(include_str!("golden/scan_summary.json")).unwrap();
    let expected = live_baseline::destinations();
    assert_eq!(counts, live_baseline::counts(&expected));
    let skipped: Vec<_> = report
        .skipped
        .iter()
        .map(|s| {
            let reason = match s.reason {
                SkipReason::SourceJson => "source_json",
                SkipReason::ForeignManifest => "foreign_manifest",
                SkipReason::RuntimeConfig => "source_runtime_config",
                other => panic!("额外跳过 {:?}：{}", other, s.path),
            };
            json!({"reason": reason, "path":s.path})
        })
        .collect();
    assert_eq!(json!(skipped), summary["skipped"]);
    let verified = expected.len();
    assert_eq!(actual, expected);
    let reserved = report
        .pruned
        .iter()
        .filter(|r| r.reason == SkipReason::Reserved)
        .count();
    let reparse = report
        .pruned
        .iter()
        .filter(|r| r.reason == SkipReason::Reparse)
        .count();
    println!("scan: {counts:?}; destinations={verified}, songs={}, skipped={}, reserved={reserved}, reparse={reparse}, errors={}", songs.len(), report.skipped.len(), report.errors.len());
}
