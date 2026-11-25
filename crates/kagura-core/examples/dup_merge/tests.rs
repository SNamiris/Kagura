use super::*;
use journal::Journal;
use kagura_core::{consts::MANIFEST, manifest};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::Path;

const HALL: &str = "Library/Stages/Artist/Hall";
const OLD: &str = "Library/Stages/Artist/Double";
const NEW: &str = "Library/Stages/Artist/Canonical";
const KEEP: &str = "00000000000000000000000002";
const DROP: &str = "00000000000000000000000006";
const SONG: &str = "Library/Motions/Beat(Live)_Composer";
const MOTION: &str = "00000000000000000000000004";
const SONG_ID: &str = "00000000000000000000000003";
const WALK: &str = "Library/Motions/Beat(Live)_Composer/motion_Animator_take";
const DUP_WALK: &str = "Library/Motions/Beat(Live)_Composer/motion_Animator_copy";

fn fixture(root: &Path, dir: &str, file: &str) {
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini_library");
    fs::create_dir_all(root.join(dir)).unwrap();
    fs::copy(source.join(dir).join(file), root.join(dir).join(file)).unwrap();
}

fn asset(root: &Path, dir: &str, file: &str, incoming: Value) -> Value {
    let raw = fs::read(root.join(dir).join(MANIFEST)).unwrap();
    let v = manifest::parse(&raw).unwrap();
    let payload = fs::read(root.join(dir).join(file)).unwrap();
    json!({
        "id": v["id"], "rel_dir":dir, "manifest":v,
        "manifest_sha256":format!("{:x}",Sha256::digest(&raw)),
        "manifest_blake3":journal::digest(&raw),
        "payload":[{"rel":file,"size":payload.len(),"blake3":journal::digest(&payload)}],
        "incoming":incoming,
    })
}

fn from_groups(groups: Value) -> plan::Report {
    serde_json::from_value(json!({"format":"kagura-duplicate-asset-review/v1","groups": groups})).unwrap()
}

#[test]
fn fixture_merge_rekey_and_undo_restore_exact_bytes() -> Result<()> {
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();
    for (dir, file) in [(HALL, MANIFEST), (HALL, "stage.txt")] {
        fixture(root, dir, file);
    }
    fs::create_dir(root.join(HALL).with_file_name("Double"))?;
    fs::create_dir(root.join("_Duplicates"))?;
    fs::copy(
        root.join(HALL).join("stage.txt"),
        root.join(OLD).join("stage.txt"),
    )?;
    let mut dup = manifest::read(&root.join(HALL).join(MANIFEST))?;
    dup["id"] = json!(DROP);
    dup["key"]["name"] = json!("Double");
    fs::write(
        root.join(OLD).join(MANIFEST),
        manifest::encode(&dup, Default::default())?.bytes,
    )?;
    let original = manifest::read(&root.join(HALL).join(MANIFEST))?;
    let format = manifest::DocumentFormat {
        bom: true,
        newline: manifest::NewlineStyle::Crlf,
    };
    let bytes = manifest::encode(&original, format)?.bytes;
    fs::write(root.join(HALL).join(MANIFEST), &bytes)?;
    let archived = fs::read(root.join(OLD).join(MANIFEST))?;

    let report = from_groups(json!([{
        "group":1,"assets":[asset(root,HALL,"stage.txt",json!([])),asset(root,OLD,"stage.txt",json!([]))],
        "review":{"automatic_draft":true,"category":"C","keep_id":KEEP,"canonical_dir":NEW,"candidate_archive_ids":[DROP]},
        "plan":{"actions":[
            {"step":1,"op":"merge","scope":"asset","keep_id":KEEP,"retire_ids":[DROP],"set_fields":[
                {"pointer":"/note","op":"add","before":null,"value":"原说明"}
            ],"redirects":[],"target_dir":NEW},
            {"step":2,"op":"quarantine","id":DROP,"from_dir":OLD},
            {"step":3,"op":"rekey","id":KEEP,"from_dir":HALL,"to_dir":NEW,"fields":[
                {"pointer":"/key","op":"replace","before":original["key"],"value":{"author":"Artist","name":"Canonical"}}
            ]}
        ]}
    },{
        "group":2,"assets":[],
        "review":{"automatic_draft":false,"category":"B","keep_id":null,"canonical_dir":null,"candidate_archive_ids":[]},
        "plan":{"actions":[{"step":1,"op":"add_alias"}]}
    }]));
    assert!(report.eligible(&BTreeSet::from([2])).is_err());
    let unread = Journal::read(root)?;
    let plans = plan::prepare(root, report.eligible(&BTreeSet::new())?, &unread.completed()?)?;
    assert_eq!(plans.len(), 1);
    assert!(!root.join(RUN).exists() && root.join(OLD).is_dir());
    let mut journal = Journal::open(root)?;
    for p in plans {
        journal.apply(root, p)?;
    }
    let completed = journal.completed()?;
    assert_eq!(completed, BTreeSet::from([1]));
    assert!(plan::prepare(root, report.eligible(&BTreeSet::new())?, &completed)?.is_empty());
    let after = manifest::read_document(&root.join(NEW).join(MANIFEST))?;
    assert_eq!(after.format, format);
    assert_eq!(after.value["updated"], DATE);
    assert_eq!(
        after.value["note"],
        format!("原说明\n{DATE} 合并同内容资产 {DROP}")
    );
    let retired = root.join(ARCHIVE).join("Stages/Artist/Double");
    assert_eq!(fs::read(retired.join(MANIFEST))?, archived);
    journal.undo(root, true)?;
    assert!(retired.is_dir() && !root.join(HALL).exists());
    journal.undo(root, false)?;
    assert!(journal.completed()?.is_empty());
    assert_eq!(fs::read(root.join(HALL).join(MANIFEST))?, bytes);
    assert_eq!(fs::read(root.join(OLD).join(MANIFEST))?, archived);
    assert!(!root.join(ARCHIVE).exists() && !root.join(NEW).exists());
    Ok(())
}

#[test]
fn planned_parent_reference_updates_and_new_references_are_rejected() -> Result<()> {
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();
    for (dir, file) in [(SONG, MANIFEST), (WALK, MANIFEST), (WALK, "walk.txt")] {
        fixture(root, dir, file)
    }
    fs::create_dir_all(root.join(SONG).join("_AuDiO"))?;
    fixture(root, &format!("{SONG}/_AuDiO"), "track.txt");
    fs::create_dir(root.join(DUP_WALK))?;
    fs::create_dir(root.join("_Duplicates"))?;
    fs::copy(
        root.join(WALK).join("walk.txt"),
        root.join(DUP_WALK).join("walk.txt"),
    )?;
    let mut duplicate = manifest::read(&root.join(WALK).join(MANIFEST))?;
    duplicate["id"] = json!(DROP);
    duplicate["key"]["suffix"] = json!("copy");
    fs::write(
        root.join(DUP_WALK).join(MANIFEST),
        manifest::encode(&duplicate, Default::default())?.bytes,
    )?;
    let songpath = root.join(SONG).join(MANIFEST);
    let mut song = manifest::read(&songpath)?;
    song["sets"][0]["motion"][0] = json!(DROP);
    let before = manifest::encode(&song, Default::default())?.bytes;
    fs::write(&songpath, &before)?;
    let pointer = "/sets/0/motion/0";
    let group = json!({
        "group":4,"assets":[asset(root,WALK,"walk.txt",json!([])),asset(root,DUP_WALK,"walk.txt",json!([{"from_id":SONG_ID,"pointer":pointer}]))],
        "review":{"automatic_draft":true,"category":"A","keep_id":MOTION,"canonical_dir":WALK,"candidate_archive_ids":[DROP]},
        "plan":{"actions":[
            {"step":1,"op":"merge","scope":"asset","keep_id":MOTION,"retire_ids":[DROP],"set_fields":[],
                "redirects":[{"from_id":SONG_ID,"pointer":pointer,"before":DROP,"after":MOTION}],"target_dir":WALK},
            {"step":2,"op":"quarantine","id":DROP,"from_dir":DUP_WALK}
        ]}
    });
    let report = from_groups(json!([group]));
    song["related"] = json!([DROP]);
    fs::write(&songpath, manifest::encode(&song, Default::default())?.bytes)?;
    assert!(plan::prepare(root, report.eligible(&BTreeSet::new())?, &BTreeSet::new())?.is_empty());
    fs::write(&songpath, &before)?;
    let raw = fs::read(root.join(DUP_WALK).join("walk.txt"))?;
    fs::write(root.join(DUP_WALK).join("walk.txt"), vec![b'!'; raw.len()])?;
    assert!(plan::prepare(root, report.eligible(&BTreeSet::new())?, &BTreeSet::new())?.is_empty());
    fs::write(root.join(DUP_WALK).join("walk.txt"), raw)?;
    let plans = plan::prepare(root, report.eligible(&BTreeSet::new())?, &BTreeSet::new())?;
    let mut journal = Journal::open(root)?;
    for p in plans {
        journal.apply(root, p)?;
    }
    assert_eq!(manifest::read(&songpath)?["sets"][0]["motion"][0], MOTION);
    journal.undo(root, true)?;
    journal.undo(root, false)?;
    assert_eq!(fs::read(songpath)?, before);
    assert!(root.join(DUP_WALK).is_dir());
    Ok(())
}
