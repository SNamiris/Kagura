use kagura_core::consts::{FORMAT, MANIFEST};
use kagura_core::manifest::{dumps, metadata_role, parse, read, AssetView, MetadataRole};
use kagura_core::schema::Schemas;
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::path::Path;

#[test]
fn runtime_configs_from_real_assets() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for fixture in ["runtime_character.json", "runtime_stage.json"] {
        let value = read(&dir.join(fixture)).unwrap();
        assert_eq!(metadata_role(&value), MetadataRole::RuntimeConfig);
    }
}

#[test]
fn content_roles_and_candidate_precedence() {
    use MetadataRole::{Asset, ForeignManifest, SourceJson};
    for value in [
        json!({"schema": null}),
        json!({"classified_by": false}),
        json!({"id": "7000000000000000000000000Z"}),
        json!({"type": "stage", "id": null}),
        json!({"type": "song", "key": null}),
        json!({"key": null, "variants": null}),
        json!({"name": "char:x", "character": {}, "schema": 1, "format": "other"}),
    ] {
        assert_eq!(metadata_role(&value), ForeignManifest, "{value}");
    }
    for value in [
        json!([{"schema": 1}]),
        json!({"id": "80000000000000000000000000"}),
        json!({"id": "0000000000000000000000000I"}),
        json!({"id": "0000000000000000000000000a"}),
        json!({"id": "short"}),
        json!({"format": FORMAT}),
        json!({"type": "model"}),
        json!({"name": "char:x", "character": []}),
        json!({"name": "stage:x", "character": {}}),
    ] {
        assert_eq!(metadata_role(&value), SourceJson, "{value}");
    }
    let incomplete = json!({"schema": 1, "type": "model", "format": FORMAT});
    assert_eq!(metadata_role(&incomplete), Asset);
    assert!(!Schemas::load()
        .unwrap()
        .validate_asset(&incomplete, 8)
        .unwrap()
        .is_empty());
}

#[test]
fn bom_order_unicode_and_numbers() {
    let text = "{\n  \"z\": 1.0,\n  \"a\": \"日本語😀\",\n  \"other\": []\n}\n";
    let bytes = [b"\xef\xbb\xbf".as_slice(), text.as_bytes()].concat();
    assert_eq!(dumps(&parse(&bytes).unwrap()).unwrap(), text);
    assert!(parse(&[0xff]).is_err());
    let exponent = parse(b"1e-05").unwrap();
    let written = dumps(&exponent).unwrap();
    println!(
        "1e-05 => {}; 1.0 => {}",
        written.trim_end(),
        dumps(&json!(1.0)).unwrap().trim_end()
    );
    assert_eq!(parse(written.as_bytes()).unwrap(), exponent);
}

#[derive(Deserialize)]
struct GoldenRecord {
    id: String,
    #[serde(rename = "type")]
    ty: String,
    dir: String,
}

#[test]
#[ignore = "需要设置 KAGURA_LIBRARY 为包含 Library 的真实库根目录"]
fn real_library_manifest_roundtrip() {
    let root = std::env::var_os("KAGURA_LIBRARY").expect("请设置 KAGURA_LIBRARY");
    let root = Path::new(&root);
    let schemas = Schemas::load().unwrap();
    let mut exact = 0;
    let mut total = 0;
    let mut differences = BTreeMap::new();
    let mut examples = Vec::new();
    for row in include_str!("golden/destination.jsonl").lines() {
        let record: GoldenRecord = serde_json::from_str(row).unwrap();
        let relative = format!("{}/{MANIFEST}", record.dir);
        let raw = std::fs::read(root.join(&relative)).unwrap_or_else(|e| panic!("{relative}: {e}"));
        let value = parse(&raw).unwrap_or_else(|e| panic!("{relative}: {e}"));
        assert_eq!(metadata_role(&value), MetadataRole::Asset, "{relative}");
        let issues = schemas.validate_asset(&value, 8).unwrap();
        assert!(issues.is_empty(), "{relative}: {issues:?}");
        let view = AssetView::new(&value).unwrap();
        assert_eq!(
            (view.id, view.ty),
            (record.id.as_str(), record.ty.as_str()),
            "{relative}"
        );

        let written = dumps(&value).unwrap();
        total += 1;
        if raw == written.as_bytes() {
            exact += 1;
        } else {
            let original = std::str::from_utf8(&raw).unwrap();
            let no_bom = original.trim_start_matches('\u{feff}');
            let lf = no_bom.replace("\r\n", "\n");
            let mut reasons = Vec::new();
            if original.len() != no_bom.len() {
                reasons.push("BOM");
            }
            if no_bom.len() != lf.len() {
                reasons.push("CRLF");
            }
            if lf != written {
                if lf.trim_end_matches('\n') == written.trim_end_matches('\n') {
                    reasons.push("terminal newline");
                } else {
                    reasons.push("JSON spelling/format");
                }
            }
            let reason = reasons.join(" + ");
            *differences.entry(reason.clone()).or_insert(0usize) += 1;
            if examples.len() < 10 {
                examples.push((relative, reason));
            }
            assert_eq!(parse(written.as_bytes()).unwrap(), value, "写出改变了 JSON 值");
        }
    }
    assert!(total > 0);
    println!("manifest total={total}, byte_exact={exact}, differences={differences:?}");
    for (path, reason) in examples {
        println!("{reason}: {path}");
    }
}
