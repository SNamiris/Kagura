use kagura_core::naming::safe;
use serde::Deserialize;

#[derive(Deserialize)]
struct Case {
    input: String,
    limit: usize,
    output: Option<String>,
    error: Option<String>,
}

#[test]
fn safe_matches_golden() {
    let mut total = 0;
    let mut mismatches = 0;
    let mut examples = Vec::new();

    for (index, line) in include_str!("golden/safe.jsonl").lines().enumerate() {
        let line_number = index + 1;
        let case: Case = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("safe.jsonl 第 {line_number} 行：{e}"));
        let expected = match (case.output, case.error) {
            (Some(output), None) => Ok(output),
            (None, Some(error)) => Err(error),
            _ => panic!("safe.jsonl 第 {line_number} 行应有且仅有 output 或 error"),
        };
        let actual = safe(&case.input, case.limit).map_err(|error| error.to_string());
        total += 1;
        if actual != expected {
            mismatches += 1;
            if examples.len() < 20 {
                examples.push(format!(
                    "第 {line_number} 行 {:?}, limit={}: Python {expected:?}, Rust {actual:?}",
                    case.input, case.limit
                ));
            }
        }
    }

    assert!(total > 0, "safe.jsonl 为空");
    assert_eq!(
        mismatches, 0,
        "{total} 条中有 {mismatches} 条差异\n{}",
        examples.join("\n")
    );
    println!("safe.jsonl: {total} 条与 Python 一致");
}
