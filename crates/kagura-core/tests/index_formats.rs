use std::{
    fs,
    path::{Path, PathBuf},
};

use kagura_core::{
    consts::MANIFEST,
    index::{Index, SearchFilters},
};
use rusqlite::Connection;
use serde_json::{json, Value};

const ID: &str = "00000000000000000000000001";
const DIR: &str = "Library/Models/Props/Builder/Box";

fn fixture(root: &Path, variant: &str) -> PathBuf {
    let dir = root.join(DIR);
    fs::create_dir_all(dir.join("variant")).unwrap();
    let mut model: Value = serde_json::from_str(include_str!(
        "fixtures/mini_library/Library/Models/Props/Builder/Box/_asset.manifest.json"
    ))
    .unwrap();
    model["variants"][0]["file"] = json!(variant);
    fs::write(dir.join(MANIFEST), serde_json::to_vec(&model).unwrap()).unwrap();
    dir
}

fn pmd(texture: &str) -> Vec<u8> {
    let mut data = b"Pmd".to_vec();
    data.extend(1f32.to_le_bytes());
    data.extend(b"Box");
    data.resize(3 + 4 + 20 + 256, 0);
    data.extend([0; 8]); // no vertices or faces
    data.extend(1u32.to_le_bytes()); // one material
    data.extend([0; 50]);
    let mut field = [0; 20];
    field[..texture.len()].copy_from_slice(texture.as_bytes());
    data.extend(field);
    data.extend(1u16.to_le_bytes()); // one bone
    data.extend(b"Root");
    data.extend([0; 35]);
    data.extend([0; 10]); // IK, morph and display sections
    data
}

fn motion(frame: u32) -> Vec<u8> {
    let mut data = b"Vocaloid Motion Data 0002".to_vec();
    data.resize(30, 0);
    data.extend(b"Example");
    data.resize(50, 0);
    data.extend([0; 4]); // zero bone keys
    data.extend(1u32.to_le_bytes());
    data.extend(b"smile");
    data.extend([0; 10]);
    data.extend(frame.to_le_bytes());
    data.extend([0; 4]);
    data
}

fn totals(index: &Index) -> (usize, usize) {
    let find = |missing| {
        index
            .search(
                "",
                &SearchFilters {
                    missing_textures: missing,
                    nonstandard_bones: !missing,
                    ..Default::default()
                },
                10,
                0,
            )
            .unwrap()
            .total
    };
    (find(true), find(false))
}

#[test]
fn cache_tracks_variant_textures_file_stamps_errors_and_deletions() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("library_root");
    let dir = fixture(&root, "variant/main.pmd");
    let manifest = fs::read(dir.join(MANIFEST)).unwrap();
    fs::write(dir.join("variant/main.pmd"), pmd(r"..\shared.png")).unwrap();
    fs::write(dir.join("extra.pmd"), pmd("absent.png")).unwrap();
    let vmd = dir.join("smile.VMD");
    fs::write(&vmd, motion(90)).unwrap();
    fs::write(dir.join("broken.PMX"), b"bad").unwrap();
    let database = temp.path().join("index.sqlite");
    let mut index = Index::open_for_library(&database, &root).unwrap();
    index.update(&root).unwrap();
    let initial = index.parse_files(&root, |_| true).unwrap();
    assert_eq!((initial.total, initial.processed, initial.failed), (4, 4, 1));
    let warm = index.parse_files(&root, |_| true).unwrap();
    assert_eq!((warm.cached, warm.processed), (4, 0));
    assert_eq!(totals(&index), (1, 1));
    let detail = index.asset_detail(&root, ID).unwrap().unwrap();
    assert_eq!(detail.model_info.len(), 3);
    let model = detail
        .model_info
        .iter()
        .find(|m| m.file == "variant/main.pmd")
        .unwrap();
    assert_eq!(
        (
            model.name.as_deref(),
            model.bones,
            model.textures,
            model.missing_textures
        ),
        (Some("Box"), Some(1), Some(1), Some(1))
    );
    let corrupt = detail.model_info.iter().find(|m| m.file == "broken.PMX").unwrap();
    assert!(corrupt.parse_error.is_some() && corrupt.standard_bones.is_none() && corrupt.textures.is_none());
    let track = &detail.motion_info[0];
    assert_eq!(
        (track.target_model.as_deref(), track.max_frame, track.seconds),
        (Some("Example"), Some(90), Some(3.0))
    );
    let db = Connection::open(&database).unwrap();
    let morph: String = db
        .query_row("SELECT name FROM motion_morphs", [], |r| r.get(0))
        .unwrap();
    assert_eq!(morph, "smile");
    fs::write(dir.join("shared.png"), "texture now available").unwrap();
    index.update(&root).unwrap();
    assert_eq!(index.parse_files(&root, |_| true).unwrap().processed, 0);
    assert_eq!(totals(&index), (0, 1)); // undeclared extra.pmd also lacks a texture
    fs::remove_file(dir.join("shared.png")).unwrap();
    fs::write(&vmd, [motion(150), vec![0; 4]].concat()).unwrap();
    fs::remove_file(dir.join("broken.PMX")).unwrap();
    index.update(&root).unwrap();
    let resume = index.parse_files(&root, |_| true).unwrap();
    assert_eq!((resume.total, resume.cached, resume.processed), (3, 2, 1));
    let detail = index.asset_detail(&root, ID).unwrap().unwrap();
    assert_eq!(detail.motion_info[0].max_frame, Some(150));
    assert_eq!(detail.model_info.len(), 2);
    assert_eq!(totals(&index), (1, 1));
    assert_eq!(fs::read(dir.join(MANIFEST)).unwrap(), manifest);
}

#[test]
fn concurrent_library_switch_cannot_publish_under_reused_asset_id() {
    let temp = tempfile::tempdir().unwrap();
    let a = temp.path().join("a");
    let b = temp.path().join("b");
    let first = fixture(&a, "model.pmd");
    let second = fixture(&b, "model.pmd");
    fs::write(first.join("model.pmd"), pmd("first.png")).unwrap();
    fs::copy(first.join("model.pmd"), second.join("model.pmd")).unwrap();
    let db = temp.path().join("index.sqlite");
    let mut worker = Index::open_for_library(&db, &a).unwrap();
    worker.update(&a).unwrap();
    let mut foreground = Index::open_for_library(&db, &b).unwrap();
    let mut switched = false;
    let err = worker
        .parse_files(&a, |_| {
            if !switched {
                foreground.update(&b).unwrap();
                switched = true;
            }
            true
        })
        .unwrap_err();
    assert!(err.to_string().contains("已变化"));
    assert!(foreground
        .asset_detail(&b, ID)
        .unwrap()
        .unwrap()
        .model_info
        .is_empty());
    assert_eq!(foreground.parse_files(&b, |_| false).unwrap().processed, 0);
    assert_eq!(foreground.parse_files(&b, |_| true).unwrap().processed, 1);
}

#[test]
fn shared_and_escaping_texture_paths_respect_library_boundary_and_filesystem_case() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("mmd");
    let dir = fixture(&root, if cfg!(windows) { "V.PMD" } else { "v.pmd" });
    let shared = dir.parent().unwrap().join("shared");
    fs::create_dir_all(&shared).unwrap();
    let actual = shared.join("tex.png");
    fs::write(&actual, "inside Library but outside this asset").unwrap();
    fs::write(root.join("x"), "outside Library").unwrap();
    let reference = if cfg!(windows) {
        "../SHARED/TEX.PNG"
    } else {
        "../shared/tex.png"
    };
    fs::write(dir.join("v.pmd"), pmd(reference)).unwrap();
    fs::write(dir.join("escape.pmd"), pmd("../../../../../x")).unwrap();
    let db = temp.path().join("index.sqlite");
    let mut index = Index::open_for_library(&db, &root).unwrap();
    index.update(&root).unwrap();
    index.parse_files(&root, |_| true).unwrap();
    let files = index.asset_detail(&root, ID).unwrap().unwrap().model_info;
    assert_eq!(files[0].file, "escape.pmd");
    assert_eq!(files[0].missing_textures, Some(1));
    assert_eq!(files[1].missing_textures, Some(0));
    assert_eq!(totals(&index), (0, 1));
    // The cross-directory texture belongs to no asset in files. Inventory must still notice
    // its disappearance and restoration; neither pass needs to reread the model bytes.
    fs::remove_file(&actual).unwrap();
    index.update(&root).unwrap();
    assert_eq!(index.parse_files(&root, |_| true).unwrap().processed, 0);
    assert_eq!(totals(&index), (1, 1));
    fs::write(actual, "").unwrap();
    index.update(&root).unwrap();
    assert_eq!(index.parse_files(&root, |_| true).unwrap().processed, 0);
    assert_eq!(totals(&index), (0, 1));
}
