//! The Python fixture is the 2026-11-24 snapshot. Apply the audited 2026-11-26
//! library migration to its expectations, or opt in to checking an older copy.
#![allow(dead_code)] // Each integration-test crate uses a different part of the shared baseline.

use std::collections::BTreeMap;

use serde_json::Value;

fn legacy() -> bool {
    std::env::var("KAGURA_LIBRARY_SNAPSHOT").as_deref() == Ok("20261124")
}

fn review() -> Value {
    serde_json::from_str(include_str!("../../../../docs/reviews/dup-assets-20261124.json"))
        .expect("committed duplicate review")
}

pub fn destinations() -> BTreeMap<String, (String, String)> {
    let mut rows: BTreeMap<_, _> = include_str!("../golden/destination.jsonl")
        .lines()
        .map(|line| {
            let val: Value = serde_json::from_str(line).expect("Python destination row");
            (text(&val["id"]), (text(&val["type"]), text(&val["dir"])))
        })
        .collect();
    if !legacy() {
        for g in automatic(&review()) {
            for id in g["review"]["candidate_archive_ids"].as_array().unwrap() {
                assert!(rows.remove(id.as_str().unwrap()).is_some());
            }
            rows.get_mut(g["review"]["keep_id"].as_str().unwrap()).unwrap().1 =
                text(&g["review"]["canonical_dir"]);
        }
    }
    rows
}

pub fn counts(rows: &BTreeMap<String, (String, String)>) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for (ty, _) in rows.values() {
        *counts.entry(ty.clone()).or_default() += 1;
    }
    counts
}

pub fn author_license_counts() -> (BTreeMap<String, usize>, BTreeMap<String, usize>) {
    let map = |pairs: &[(&str, usize)]| {
        pairs
            .iter()
            .map(|&(k, n)| (k.to_owned(), n))
            .collect::<BTreeMap<_, _>>()
    };
    let mut unknown = map(&[("model", 1289), ("stage", 725), ("motion_part", 2898)]);
    let mut license = map(&[("model", 433), ("stage", 65)]);
    if !legacy() {
        for g in automatic(&review()) {
            let keep = g["review"]["keep_id"].as_str().unwrap();
            for item in g["assets"].as_array().unwrap() {
                let orig = &item["manifest"];
                let t = orig["type"].as_str().unwrap();
                let mut edited = orig.clone();
                if item["id"] == keep {
                    for action in g["plan"]["actions"].as_array().unwrap() {
                        if let Some(fields) = action["set_fields"]
                            .as_array()
                            .or_else(|| action["fields"].as_array())
                        {
                            for edit in fields {
                                let k = edit["pointer"].as_str().unwrap().strip_prefix('/').unwrap();
                                edited[k] = edit["value"].clone();
                            }
                        }
                    }
                }
                let survives = item["id"] == keep;
                let entries = [
                    (
                        &mut unknown,
                        orig["key"]["author"] == "_Unknown",
                        survives && edited["key"]["author"] == "_Unknown",
                    ),
                    (&mut license, explicit(orig), survives && explicit(&edited)),
                ];
                for (i, (counts, before, after)) in entries.into_iter().enumerate() {
                    if i == 1 && !matches!(t, "model" | "stage") {
                        continue;
                    }
                    match (before, after) {
                        (true, false) => *counts.entry(t.to_owned()).or_default() -= 1,
                        (false, true) => *counts.entry(t.to_owned()).or_default() += 1,
                        _ => (),
                    }
                }
            }
        }
    }
    license.retain(|key, _| matches!(key.as_str(), "model" | "stage"));
    (unknown, license)
}

fn explicit(v: &Value) -> bool {
    ["commercial", "modify", "redistribute"]
        .into_iter()
        .any(|k| v["license"][k].is_boolean())
}

fn automatic(review: &Value) -> impl Iterator<Item = &Value> {
    review["groups"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|g| g["review"]["automatic_draft"] == true)
}

fn text(v: &Value) -> String {
    v.as_str().unwrap().to_owned()
}
