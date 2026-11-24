#![cfg(feature = "bindings")]

use kagura_core::txn::{EditPatch, SourcePatch};
use serde_json::json;
use ts_rs::{Config, TS};

#[test]
fn generated_patch_fields_match_serde_presence_and_null_rules() {
    let config = Config::new();
    assert_eq!(
        EditPatch::decl(&config),
        "type EditPatch = { tags?: Array<string>, rating?: number, note?: string, source?: SourcePatch, };"
    );
    assert_eq!(
        SourcePatch::decl(&config),
        "type SourcePatch = { url: string | null, };"
    );

    let empty: EditPatch = serde_json::from_value(json!({})).unwrap();
    assert!(empty.tags.is_none() && empty.rating.is_none() && empty.note.is_none() && empty.source.is_none());
    for field in ["tags", "rating", "note", "source"] {
        assert!(
            serde_json::from_value::<EditPatch>(json!({(field): null})).is_err(),
            "{field}"
        );
    }
    assert!(serde_json::from_value::<EditPatch>(json!({"source": {}})).is_err());
    let clear: EditPatch = serde_json::from_value(json!({"source": {"url": null}})).unwrap();
    assert_eq!(clear.source.unwrap().url, None);
}
