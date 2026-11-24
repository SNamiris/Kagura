use kagura_core::index::{
    dedup::{DedupOptions, DedupSection, DuplicateReport},
    Index,
};
use rusqlite::Connection;
use serde_json::{json, Value};
use std::{fs, path::Path};

const MODEL: &str =
    include_str!("fixtures/mini_library/Library/Models/Props/Builder/Box/_asset.manifest.json");
const STAGE: &str = include_str!("fixtures/mini_library/Library/Stages/Artist/Hall/_asset.manifest.json");

fn asset(root: &Path, model: bool, n: u8, files: &[(&str, u8, usize)]) -> String {
    let asset_id = format!("{n:026}");
    let name = format!("Unit{n}");
    let mut m: Value = serde_json::from_str(if model { MODEL } else { STAGE }).unwrap();
    m["id"] = json!(&asset_id);
    m["key"]["name"] = json!(&name);
    let dir = root
        .join(if model {
            "Library/Models/Props/Builder"
        } else {
            "Library/Stages/Artist"
        })
        .join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("_asset.manifest.json"), serde_json::to_vec(&m).unwrap()).unwrap();
    for &(rel, byte, size) in files {
        let path = dir.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, vec![byte; size]).unwrap();
    }
    asset_id
}

#[test]
fn complete_sets_proper_subsets_and_mixed_extension_reclaim() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("root");
    let a = asset(
        &root,
        true,
        1,
        &[("a.png", 1, 5000), ("b.vmd", 2, 7000), ("sub/extra.pmx", 1, 5000)],
    );
    let b = asset(&root, true, 2, &[("other.vmd", 1, 5000), ("b.vmd", 2, 7000)]);
    let c = asset(&root, true, 3, &[("different.PnG", 1, 5000)]);
    asset(&root, true, 4, &[("a.txt", 3, 6000), ("b.txt", 3, 6000)]);
    asset(&root, true, 5, &[]);
    asset(&root, true, 6, &[("f", 1, 5000), ("incomplete", 4, 8000)]);
    let db = tmp.path().join("index.sqlite");
    let mut idx = Index::open_for_library(&db, &root).unwrap();
    idx.update(&root).unwrap();
    let before = idx.dedup_revision().unwrap();
    idx.hash_all(|_| {}).unwrap();
    assert_ne!(before, idx.dedup_revision().unwrap());
    let before = idx.dedup_revision().unwrap();
    Connection::open(&db)
        .unwrap()
        .execute("UPDATE files SET blake3=NULL WHERE rel='incomplete'", [])
        .unwrap();
    assert_ne!(before, idx.dedup_revision().unwrap());

    let report = idx.duplicate_report(Default::default()).unwrap();
    let s = &report.summary;
    assert_eq!(
        (s.total_assets, s.complete_assets, s.total_files, s.hashed_files),
        (6, 4, 10, 9)
    );
    assert_eq!(
        (s.cross.groups, s.cross.files, s.cross.reclaim_bytes),
        (2, 7, 27000)
    );
    assert_eq!(
        (s.within.groups, s.within.files, s.within.reclaim_bytes),
        (1, 2, 6000)
    );
    assert_eq!((s.all.groups, s.all.files, s.all.reclaim_bytes), (3, 9, 33000));
    assert_eq!(s.extensions.iter().map(|e| e.reclaim_bytes).sum::<u64>(), 33000);
    assert_eq!(s.extensions.iter().map(|e| e.files).sum::<usize>(), 9);
    let cross = report.page(DedupSection::Cross, 0, 1);
    assert_eq!(cross.total, 2);
    assert_eq!(
        (
            cross.file_groups[0].reclaim_bytes,
            cross.file_groups[0].copies.len()
        ),
        (20000, 5)
    );
    assert_eq!(cross.file_groups[0].copies[0].asset.id, a);
    assert!(report.page(DedupSection::Cross, 0, 0).file_groups.is_empty());

    let equal = report.page(DedupSection::Identical, 0, 10);
    assert_eq!((equal.total, s.identical_assets), (1, 2));
    assert_eq!(
        (
            equal.asset_groups[0].unique_files,
            equal.asset_groups[0].unique_bytes
        ),
        (2, 12000)
    );
    assert_eq!(equal.asset_groups[0].assets[1].id, b);
    // One asset with just one distinct hash is a strict subset of each equivalent superset.
    assert_eq!(s.subset_pairs, 2);
    let first = report.page(DedupSection::Subsets, 0, 1);
    let next = report.page(DedupSection::Subsets, 1, 1);
    assert_eq!(first.subsets[0].smaller.id, c);
    assert_eq!(first.subsets[0].larger.id, a);
    assert_eq!(next.subsets[0].larger.id, b);
    assert!(report
        .page(DedupSection::Subsets, usize::MAX, 2)
        .subsets
        .is_empty());

    let mut output = Vec::new();
    DuplicateReport::read_cache(&db, Default::default())
        .unwrap()
        .write_json(&mut output)
        .unwrap();
    let exported: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(exported["summary"]["complete_assets"], 4);
    assert_eq!(exported["subsets"].as_array().unwrap().len(), 2);
    assert_eq!(exported["cross"][0]["copies"].as_array().unwrap().len(), 5);
}

#[test]
fn exclusions_use_exact_boundaries_and_do_not_affect_relationships() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("root");
    let files = [
        ("small.sph", 1, 4095),
        ("tex/ToOn01.BMP", 2, 5000),
        ("MME Controller.PMX", 3, 65536),
        ("reg.pmx", 4, 4096),
        ("large controller.pmd", 5, 65537),
        ("toon11.bmp", 6, 5000),
    ];
    asset(&root, false, 1, &files);
    asset(&root, false, 2, &files);
    // A character or prop controller remains visible under the accessory filter.
    asset(
        &root,
        true,
        3,
        &[("MME Controller.PMX", 3, 65536), ("new", 7, 4250)],
    );
    let db = tmp.path().join("index.sqlite");
    let mut idx = Index::open_for_library(&db, &root).unwrap();
    idx.update(&root).unwrap();
    idx.hash_all(|_| {}).unwrap();
    let report = idx.duplicate_report(Default::default()).unwrap();
    assert_eq!((report.summary.all.groups, report.summary.excluded_files), (3, 6));
    assert_eq!(report.summary.identical_assets, 2);
    for opt in [
        DedupOptions {
            include_toon: true,
            ..Default::default()
        },
        DedupOptions {
            include_small: true,
            ..Default::default()
        },
        DedupOptions {
            include_accessories: true,
            ..Default::default()
        },
    ] {
        assert_eq!(idx.duplicate_report(opt).unwrap().summary.all.groups, 4);
    }
    let all = idx
        .duplicate_report(DedupOptions {
            include_small: true,
            include_toon: true,
            include_accessories: true,
        })
        .unwrap();
    assert_eq!(
        (
            all.summary.all.groups,
            all.summary.excluded_files,
            all.summary.identical_assets
        ),
        (6, 0, 2)
    );
    let page = all.page(DedupSection::Cross, 0, 20);
    let group = page.file_groups.iter().find(|g| g.size == 65536).unwrap();
    assert_eq!(group.copies.len(), 3);
    assert_eq!(
        group
            .copies
            .iter()
            .filter(|f| f.exclusions.iter().any(|v| v == "accessory"))
            .count(),
        2
    );
}
