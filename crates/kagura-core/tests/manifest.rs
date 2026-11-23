use kagura_core::consts::{FORMAT, MANIFEST};
use kagura_core::manifest::{
    dumps, encode, metadata_role, parse, parse_document, read_document, AssetView, DocumentFormat,
    MetadataRole, NewlineStyle,
};
use kagura_core::schema::Schemas;
use serde::Deserialize;
use serde_json::json;
use std::path::Path;

#[test]
fn runtime_configs_from_real_assets() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for fixture in ["runtime_character.json", "runtime_stage.json"] {
        let document = read_document(&dir.join(fixture)).unwrap();
        assert_eq!(metadata_role(&document.value), MetadataRole::RuntimeConfig);
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
fn preserves_crlf_lf_and_escaped_line_breaks() {
    let lf = "{\n  \"name\": \"a\\r\\nb\\nc\"\n}\n";
    let crlf = lf.replace('\n', "\r\n");
    for (text, style) in [(lf, NewlineStyle::Lf), (crlf.as_str(), NewlineStyle::Crlf)] {
        let document = parse_document(text.as_bytes()).unwrap();
        assert_eq!(document.format.newline, style);
        assert!(!document.format.bom);
        let encoded = encode(&document.value, document.format).unwrap();
        assert_eq!(encoded.bytes, text.as_bytes());
        assert!(!encoded.normalized_newlines);
    }
}

#[test]
fn no_line_breaks_and_new_files_use_lf() {
    let document = parse_document(b"{}").unwrap();
    assert_eq!(document.format.newline, NewlineStyle::Lf);
    let encoded = encode(&document.value, document.format).unwrap();
    assert_eq!(encoded.bytes, b"{}\n");
    assert!(!encoded.normalized_newlines);

    let encoded = encode(&document.value, DocumentFormat::default()).unwrap();
    assert_eq!(encoded.bytes, b"{}\n");
    assert!(!encoded.normalized_newlines);
}

#[test]
fn retains_bom_after_edit() {
    let original = b"\xef\xbb\xbf{\r\n  \"x\": 1\r\n}\r\n";
    let mut document = parse_document(original).unwrap();
    assert!(document.format.bom);
    let encoded = encode(&document.value, document.format).unwrap();
    assert_eq!(encoded.bytes, original);
    assert!(!encoded.normalized_newlines);

    document.value["x"] = json!(2);
    let encoded = encode(&document.value, document.format).unwrap();
    assert_eq!(encoded.bytes, b"\xef\xbb\xbf{\r\n  \"x\": 2\r\n}\r\n");
}

#[test]
fn mixed_line_breaks_use_majority_and_default_on_ties() {
    let lf = b"{\n  \"x\": 1\n}\n".as_slice();
    let crlf = b"{\r\n  \"x\": 1\r\n}\r\n".as_slice();
    for (raw, lf_count, crlf_count, expected) in [
        ("{\r\n  \"x\": 1\n}\n", 2, 1, lf),
        ("{\n  \"x\": 1\r\n}\r\n", 1, 2, crlf),
        ("{\n  \"x\": 1\r\n}", 1, 1, lf),
    ] {
        let document = parse_document(raw.as_bytes()).unwrap();
        assert_eq!(
            document.format.newline,
            NewlineStyle::Mixed {
                lf: lf_count,
                crlf: crlf_count
            }
        );
        let encoded = encode(&document.value, document.format).unwrap();
        assert_eq!(encoded.bytes, expected);
        assert!(encoded.normalized_newlines);
        let reread = parse_document(&encoded.bytes).unwrap();
        assert_eq!(reread.value, document.value);
        assert!(!encode(&reread.value, reread.format).unwrap().normalized_newlines);
    }
}

#[test]
#[ignore = "需要设置 KAGURA_LIBRARY 为包含 Library 的真实库根目录"]
fn real_library_manifest_roundtrip() {
    let root = std::env::var_os("KAGURA_LIBRARY").expect("请设置 KAGURA_LIBRARY");
    let root = Path::new(&root);
    let schemas = Schemas::load().unwrap();
    let mut exact = 0;
    let mut total = 0;
    let (mut crlf, mut lf, mut mixed, mut bom) = (0, 0, 0, 0);
    let mut examples = Vec::new();
    for row in include_str!("golden/destination.jsonl").lines() {
        let record: GoldenRecord = serde_json::from_str(row).unwrap();
        let relative = format!("{}/{MANIFEST}", record.dir);
        let raw = std::fs::read(root.join(&relative)).unwrap_or_else(|e| panic!("{relative}: {e}"));
        let document = parse_document(&raw).unwrap_or_else(|e| panic!("{relative}: {e}"));
        assert_eq!(metadata_role(&document.value), MetadataRole::Asset, "{relative}");
        let issues = schemas.validate_asset(&document.value, 8).unwrap();
        assert!(issues.is_empty(), "{relative}: {issues:?}");
        let view = AssetView::new(&document.value).unwrap();
        assert_eq!(
            (view.id, view.ty),
            (record.id.as_str(), record.ty.as_str()),
            "{relative}"
        );

        match document.format.newline {
            NewlineStyle::Crlf => crlf += 1,
            NewlineStyle::Lf => lf += 1,
            NewlineStyle::Mixed { .. } => mixed += 1,
        }
        if document.format.bom {
            bom += 1;
        }
        let written = encode(&document.value, document.format).unwrap();
        total += 1;
        if raw == written.bytes {
            exact += 1;
        } else if examples.len() < 10 {
            examples.push((relative, document.format));
        }
    }
    println!("manifest total={total}, byte_exact={exact}, CRLF={crlf}, LF={lf}, Mixed={mixed}, BOM={bom}, no_BOM={}", total - bom);
    assert_eq!(total, 8824);
    assert_eq!(exact, total, "前 10 个字节不一致的文件：{examples:?}");
    assert_eq!(lf, total, "迁移后的真实库应全部使用 LF");
}
