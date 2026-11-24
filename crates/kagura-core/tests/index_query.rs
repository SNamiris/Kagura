use std::{fs, path::Path};

use kagura_core::{
    consts::MANIFEST,
    index::{Index, Missing, SearchFilters, Sort},
    scan::{scan_library, scan_library_for_check},
};
use serde_json::json;

const MODEL: &str = "00000000000000000000000001";
const STAGE: &str = "00000000000000000000000002";
const SONG: &str = "00000000000000000000000003";
const MOTION: &str = "00000000000000000000000004";

/// Give the five mini-library manifests searchable text and independently sortable dates and ratings.
fn populate(root: &Path) -> Index {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini_library");
    let library_root = root.join("mmd");
    for record in scan_library(&fixture).records {
        let mut value = record.value;
        match value["type"].as_str().unwrap() {
            "model" => {
                value["name"] = json!("Shared");
                value["name_original"] = json!("鎌倉物語");
                value["author"] = json!(["Second Creator"]);
                value["tags"] = json!(["vocaloid"]);
                value["note"] = json!("初音ミク 𠮷野 小苹果 100% _% \\ \"ab\"");
                value["character"] = json!("紅色角色");
                value["series"] = json!("星空系列");
                value["license"] = json!({"commercial": false});
                value["added"] = json!("2025-11-22");
                value["updated"] = json!("2026-11-20");
                value["rating"] = json!(3);
            }
            "stage" => {
                value["key"]["author"] = json!("_Unknown");
                value["name"] = json!("Shared");
                value["license"] = json!({"credit_required": true, "modify": null});
                value["added"] = json!("2024-11-22");
                value["updated"] = json!("2026-11-21");
                value["rating"] = json!(5);
            }
            "song" => {
                value["key"]["composer"] = json!("_Unknown");
                value["title_alias"] = json!(["Violette", "薄紫の夜"]);
            }
            _ => {}
        }
        let dir = record
            .dir
            .replace("Beat(Live)_Composer", "Beat(Live)__Unknown")
            .replace("Stages/Artist", "Stages/_Unknown");
        let dir = library_root.join(dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(MANIFEST), serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    }
    let report = scan_library_for_check(&library_root);
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    let mut index = Index::open(root.join("mini.sqlite")).unwrap();
    index.rebuild(&report).unwrap();
    index
}

#[test]
fn literal_search_covers_fts_fields_short_unicode_and_metacharacters() {
    let temp = tempfile::tempdir().unwrap();
    let index = populate(temp.path());
    for (term, id, used_like) in [
        ("Box", MODEL, false),
        ("shared", MODEL, false),
        ("倉物語", MODEL, false),
        ("CREATOR", MODEL, false),
        ("Builder", MODEL, false),
        ("小苹果", MODEL, false),
        ("\"ab\"", MODEL, false),
        ("100%", MODEL, false),
        ("caloid", MODEL, false),
        ("紅色角", MODEL, false),
        ("空系列", MODEL, false),
        ("lette", SONG, false),
        ("薄紫の", SONG, false),
        ("Beat", SONG, false),
        ("初音", MODEL, true),
        ("𠮷野", MODEL, true),
        ("_%", MODEL, true),
        ("\\", MODEL, true),
        ("%", MODEL, true),
    ] {
        let page = index.search(term, &SearchFilters::default(), 10, 0).unwrap();
        assert_eq!(page.used_like, used_like, "{term}");
        assert_eq!(page.total, if term == "shared" { 2 } else { 1 }, "{term}");
        assert_eq!(page.items[0].id, id, "{term}");
    }

    let filtered = SearchFilters {
        asset_type: Some("model".into()),
        kind: Some("prop".into()),
        author: Some("Builder".into()),
        tag: Some("vocaloid".into()),
        ..Default::default()
    };
    for term in ["", "初音", "小苹果"] {
        let page = index.search(term, &filtered, 20, 0).unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.items[0].id, MODEL);
        assert_eq!(page.items[0].name.as_deref(), Some("Shared"));
    }
    let other_tag = SearchFilters {
        tag: Some("vocal".into()),
        ..filtered
    };
    assert_eq!(index.search("小苹果", &other_tag, 1, 0).unwrap().total, 0);
}

#[test]
fn missing_stats_children_stable_sort_and_pagination() {
    let temp = tempfile::tempdir().unwrap();
    let index = populate(temp.path());
    let stats = index.stats().unwrap();
    assert_eq!(stats.counts.values().sum::<usize>(), 5);
    assert_eq!(stats.counts["motion_part"], 2);
    assert_eq!(stats.unknown_author["stage"], 1);
    assert_eq!(stats.unknown_composer, 1);
    assert_eq!(stats.explicit_license["model"], 1);
    assert_eq!(stats.explicit_license["stage"], 0);
    for (missing, expected_count, expected_id) in [
        (Missing::Author, 1, Some(STAGE)),
        (Missing::Composer, 1, Some(SONG)),
        (Missing::License, 4, None),
        (Missing::Any, 4, None),
    ] {
        let filters = SearchFilters {
            missing: Some(missing),
            ..Default::default()
        };
        let page = index.search("", &filters, 10, 0).unwrap();
        assert!(!page.used_like);
        assert_eq!(page.total, expected_count);
        if let Some(id) = expected_id {
            assert_eq!(page.items[0].id, id);
        }
    }

    let doc = index.get(SONG).unwrap().unwrap();
    assert_eq!(doc["title_alias"], json!(["Violette", "薄紫の夜"]));
    assert!(index.get("absent").unwrap().is_none());
    let children = index.children(SONG).unwrap();
    assert_eq!(children.len(), 1);
    assert_eq!(children[0].id, MOTION);
    assert_eq!(children[0].suffix.as_deref(), Some("take"));
    assert!(index.children("absent").unwrap().is_empty());

    for (sort, descending, first) in [
        (Sort::Name, None, MODEL),
        (Sort::Added, None, MODEL),
        (Sort::Updated, None, STAGE),
        (Sort::Rating, None, STAGE),
        (Sort::Rating, Some(false), MODEL),
    ] {
        let filters = SearchFilters {
            sort,
            descending,
            ..Default::default()
        };
        let page = index.search("Shared", &filters, 1, 0).unwrap();
        assert_eq!(page.items[0].id, first);
        let next = index.search("Shared", &filters, 1, 1).unwrap();
        assert_eq!(next.total, 2);
        assert_ne!(next.items[0].id, first);
        assert!(index.search("Shared", &filters, 1, 2).unwrap().items.is_empty());
    }
    let empty_page = index.search("", &SearchFilters::default(), 0, 0).unwrap();
    assert_eq!(empty_page.total, 5);
    assert!(empty_page.items.is_empty());
}

#[test]
fn desktop_filters_combine_multiselects_and_escape_author_input() {
    let temp = tempfile::tempdir().unwrap();
    let index = populate(temp.path());
    let filters = SearchFilters {
        types: vec!["model".into(), "stage".into()],
        kinds: vec!["prop".into(), "stage".into()],
        ..Default::default()
    };
    assert_eq!(index.search("", &filters, 2, 0).unwrap().total, 2);
    let author = SearchFilters {
        author_contains: Some("uil".into()),
        ..filters.clone()
    };
    assert_eq!(index.search("shared", &author, 10, 0).unwrap().items[0].id, MODEL);
    let unknown = SearchFilters {
        author_contains: Some("_".into()),
        ..filters.clone()
    };
    assert_eq!(index.search("", &unknown, 10, 0).unwrap().total, 1);
    let literal = SearchFilters {
        author_contains: Some("%".into()),
        ..filters
    };
    assert_eq!(index.search("", &literal, 10, 0).unwrap().total, 0);
    let missing = SearchFilters {
        missing_any: vec![Missing::Author, Missing::Composer],
        ..Default::default()
    };
    assert_eq!(index.search("", &missing, 10, 0).unwrap().total, 2);
}

#[test]
fn desktop_detail_and_previews_use_only_the_active_assets_files() {
    let temp = tempfile::tempdir().unwrap();
    let mut index = populate(temp.path());
    let root = temp.path().join("mmd");
    let model = root.join("Library/Models/Props/Builder/Box");
    let song = root.join("Library/Motions/Beat(Live)__Unknown");
    fs::create_dir_all(song.join("_AuDiO")).unwrap();
    fs::write(song.join("_AuDiO/take.wav"), b"sound").unwrap();
    fs::create_dir_all(model.join("media")).unwrap();
    for rel in ["preview.png", "media/view.webp", "media/manual.txt"] {
        fs::write(model.join(rel), rel).unwrap();
    }
    let mut manifest = index.get(MODEL).unwrap().unwrap();
    manifest["preview"] = json!(["media/manual.txt", "media/missing.jpg", "media/view.webp"]);
    fs::write(model.join(MANIFEST), serde_json::to_vec(&manifest).unwrap()).unwrap();
    index.update(&root).unwrap();
    let song_detail = index.asset_detail(&root, SONG).unwrap().unwrap();
    assert_eq!((song_detail.file_count, song_detail.total_bytes), (1, 5));
    assert_eq!(song_detail.audio_files, ["_AuDiO/take.wav"]);
    assert_eq!(song_detail.outgoing[0].id, MOTION);
    assert_eq!(song_detail.outgoing[0].field, "sets[0].motion[0]");
    assert_eq!(song_detail.incoming[0].id, MOTION);
    assert_eq!(song_detail.incoming[0].field, "song");
    assert!(index.asset_directory(&root, "../../outside").unwrap().is_none());
    assert!(index.asset_directory(temp.path(), MODEL).is_err());
    let folder = index.asset_directory(&root, MODEL).unwrap().unwrap();
    assert_eq!(folder.canonicalize().unwrap(), model.canonicalize().unwrap());

    let mut items = index
        .search("Box", &SearchFilters::default(), 20, 0)
        .unwrap()
        .items;
    index.load_previews(&root, &mut items).unwrap();
    assert!(Path::new(items[0].preview_path.as_ref().unwrap()).ends_with("media/view.webp"));
    fs::remove_file(model.join("media/view.webp")).unwrap();
    index.load_previews(&root, &mut items).unwrap();
    assert!(Path::new(items[0].preview_path.as_ref().unwrap()).ends_with("preview.png"));
    fs::remove_file(model.join("preview.png")).unwrap();
    index.load_previews(&root, &mut items).unwrap();
    assert!(items[0].preview_path.is_none());

    // An asset directory can be replaced between scanning and clicking a card.
    // Neither opening the folder nor its previously indexed preview may follow the link.
    let outside = temp.path().join("outside");
    fs::rename(&model, &outside).unwrap();
    fs::write(outside.join("preview.png"), b"outside").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, &model).unwrap();
    #[cfg(windows)]
    {
        let output = std::process::Command::new("cmd.exe")
            .args(["/d", "/c", "mklink", "/J"])
            .arg(&folder)
            .arg(&outside)
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
    }
    assert!(index.asset_directory(&root, MODEL).is_err());
    index.load_previews(&root, &mut items).unwrap();
    assert!(items[0].preview_path.is_none());
}
