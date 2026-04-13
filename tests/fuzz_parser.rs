use agent_hashmap::store::migration::STORE_FORMAT_VERSION;
use agent_hashmap::store::persist::{
    empty_store_file, map_to_file, parse_store_file, serialize_store_file, StoreFile,
};
use agent_hashmap::types::Entry;

#[test]
fn malformed_json_is_rejected() {
    let raw = "{not-json";
    assert!(parse_store_file(raw).is_err());
}

#[test]
fn truncated_json_is_rejected() {
    let raw = r#"{"version":1,"records":["#;
    assert!(parse_store_file(raw).is_err());
}

#[test]
fn trailing_garbage_after_json_is_rejected() {
    let raw = r#"{"version":1,"records":[]} trailing"#;
    assert!(parse_store_file(raw).is_err());
}

#[test]
fn missing_version_field_is_rejected() {
    let raw = r#"{"records":[]}"#;
    assert!(parse_store_file(raw).is_err());
}

#[test]
fn missing_records_field_is_rejected() {
    let raw = r#"{"version":1}"#;
    assert!(parse_store_file(raw).is_err());
}

#[test]
fn unsupported_version_is_rejected() {
    let raw = r#"{"version":9999,"records":[]}"#;
    assert!(parse_store_file(raw).is_err());
}

#[test]
fn invalid_key_is_rejected() {
    let raw = format!(
        r#"{{"version":{},"records":[{{"key":"bad key with space","value":"ok"}}]}}"#,
        STORE_FORMAT_VERSION
    );
    assert!(parse_store_file(&raw).is_err());
}

#[test]
fn invalid_value_with_nul_byte_is_rejected() {
    let raw = format!(
        "{{\"version\":{},\"records\":[{{\"key\":\"agent/codex/task\",\"value\":\"bad\\u0000value\"}}]}}",
        STORE_FORMAT_VERSION
    );
    assert!(parse_store_file(&raw).is_err());
}

#[test]
fn oversized_but_structurally_valid_value_is_rejected() {
    let huge = "a".repeat(70_000);
    let raw = format!(
        r#"{{"version":{},"records":[{{"key":"agent/codex/task","value":"{}"}}]}}"#,
        STORE_FORMAT_VERSION, huge
    );
    assert!(parse_store_file(&raw).is_err());
}

#[test]
fn empty_records_array_is_accepted() {
    let raw = format!(r#"{{"version":{},"records":[]}}"#, STORE_FORMAT_VERSION);
    let parsed = parse_store_file(&raw).expect("empty records should parse");
    assert_eq!(parsed.records.len(), 0);
}

#[test]
fn valid_store_with_multiple_records_roundtrips() {
    let mut file = empty_store_file();
    file.records.push(
        map_to_file(
            &vec![
                Entry::try_new("agent/codex/task", "build index").expect("entry"),
                Entry::try_new("agent/claude/task", "review docs").expect("entry"),
            ]
            .into_iter()
            .collect(),
        )
        .records
        .remove(0),
    );
    file.records.push(
        map_to_file(
            &vec![Entry::try_new("project/demo/root", "/tmp/demo").expect("entry")]
                .into_iter()
                .collect(),
        )
        .records
        .remove(0),
    );

    let raw = serialize_store_file(&file).expect("serialize");
    let reparsed = parse_store_file(&raw).expect("parse");
    assert_eq!(reparsed.version, STORE_FORMAT_VERSION);
    assert_eq!(reparsed.records.len(), 2);
}

#[test]
fn duplicate_keys_parse_but_remain_visible_at_file_level() {
    let raw = format!(
        r#"{{"version":{},"records":[{{"key":"agent/codex/task","value":"one"}},{{"key":"agent/codex/task","value":"two"}}]}}"#,
        STORE_FORMAT_VERSION
    );
    let parsed = parse_store_file(&raw).expect("parse should succeed");
    assert_eq!(parsed.records.len(), 2);
}

#[test]
fn weird_but_valid_unicode_value_is_accepted() {
    let raw = format!(
        r#"{{"version":{},"records":[{{"key":"agent/codex/task","value":"zażółć gęślą jaźń"}}]}}"#,
        STORE_FORMAT_VERSION
    );
    let parsed = parse_store_file(&raw).expect("parse");
    assert_eq!(parsed.records.len(), 1);
}

#[test]
fn empty_store_file_roundtrips() {
    let file = StoreFile {
        version: STORE_FORMAT_VERSION,
        records: Vec::new(),
    };

    let raw = serialize_store_file(&file).expect("serialize");
    let parsed = parse_store_file(&raw).expect("parse");
    assert_eq!(parsed, file);
}
