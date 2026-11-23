//! 依赖验证：Rust `jsonschema` 与 Python 版（jsonschema + FormatChecker）的结论必须一致。
//!
//! fixtures 由 Python 从 G:\MMD\Library 抽样生成：
//! - valid.json：12 份真实 manifest（每类 3 份，含内容最多的一份），都应通过
//! - cases.json：在真实 manifest 上做的 12 种改动，`valid` 是 Python 的结论
//!
//! 跑法：`cargo test -p kagura-core --test schema_parity -- --nocapture`

use kagura_core::schema::Schemas;
use serde_json::Value;

fn load(name: &str) -> Value {
    let p = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    serde_json::from_str(&std::fs::read_to_string(&p).expect(&p)).expect(&p)
}

#[test]
fn schemas_compile() {
    Schemas::load().expect("全部 schema 应能编译（含跨文件 $ref、unevaluatedProperties）");
}

#[test]
fn real_manifests_pass() {
    let s = Schemas::load().unwrap();
    let data = load("valid.json");
    for case in data["valid"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let issues = s.validate_asset(&case["manifest"], 8).unwrap();
        assert!(issues.is_empty(), "{name} 应通过，实际：{issues:#?}");
    }
}

#[test]
fn matches_python_verdicts() {
    let s = Schemas::load().unwrap();
    let data = load("cases.json");
    let mut mismatch = Vec::new();
    for case in data["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let ty = case["type"].as_str().unwrap();
        let expect = case["valid"].as_bool().unwrap();
        let issues = s.validate_as(ty, &case["manifest"], 8).unwrap();
        let got = issues.is_empty();
        println!(
            "{name:32} python={expect:<5} rust={got:<5} {}",
            issues.first().map(|i| i.message.as_str()).unwrap_or("")
        );
        if got != expect {
            mismatch.push((name.to_owned(), expect, issues));
        }
    }
    assert!(mismatch.is_empty(), "与 Python 结论不一致：{mismatch:#?}");
}

#[test]
fn validate_does_not_mutate() {
    let s = Schemas::load().unwrap();
    let data = load("valid.json");
    let m = data["valid"][0]["manifest"].clone();
    let before = serde_json::to_string(&m).unwrap();
    let _ = s.validate_asset(&m, 8).unwrap();
    assert_eq!(before, serde_json::to_string(&m).unwrap());
}
