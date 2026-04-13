use std::fs;

use tempfile::tempdir;

use agent_hashmap::config::Config;
use agent_hashmap::index::{build_index, query_index, read_index_stats};
use agent_hashmap::store::Store;
use agent_hashmap::types::ProjectName;

#[test]
fn build_query_and_stats_roundtrip() {
    let temp = tempdir().expect("failed to create temp dir");

    let src_dir = temp.path().join("src");
    fs::create_dir_all(&src_dir).expect("failed to create src dir");

    fs::write(
        src_dir.join("auth.rs"),
        r#"
pub fn auth_middleware(token: &str) -> bool {
    !token.trim().is_empty()
}

pub fn authorize_admin(role: &str) -> bool {
    role == "admin"
}
"#,
    )
    .expect("failed to write auth file");

    let config = Config::for_project_root(
        ProjectName::new("demo-project").expect("valid project name"),
        temp.path(),
    )
    .expect("failed to build config");

    let mut store = Store::open_locked(config).expect("failed to open locked store");

    let report = build_index(&mut store, temp.path()).expect("index build should succeed");
    assert_eq!(report.file_count, 1);
    assert!(report.chunk_count >= 1);
    assert!(report.token_count >= 1);

    let stats = read_index_stats(&store);
    assert!(stats.built);
    assert_eq!(stats.file_count, 1);

    let result =
        query_index(&store, "where is auth middleware", 5, 2000).expect("query should succeed");

    assert!(!result.chunks.is_empty());
    assert!(result
        .chunks
        .iter()
        .any(|chunk| chunk.path.ends_with("src/auth.rs")));
    assert!(result.used_tokens > 0);
}
