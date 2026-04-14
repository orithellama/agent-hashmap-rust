use tempfile::tempdir;

use agentmem::config::Config;
use agentmem::store::persist;
use agentmem::types::{ProjectName, StorePath};

#[test]
fn config_for_project_root_uses_expected_default_paths() {
    let dir = tempdir().expect("temp dir");
    let config =
        Config::for_project_root(ProjectName::new("demo-project").expect("valid"), dir.path())
            .expect("config");

    assert!(config
        .store_path()
        .as_path()
        .ends_with(".agentmem/store.json"));
}

#[test]
fn project_config_and_store_paths_live_under_hidden_state_dir() {
    let dir = tempdir().expect("temp dir");
    let config_path = Config::project_config_path(dir.path());
    let store_path = Config::project_store_path(dir.path());

    assert!(config_path.ends_with(".agentmem/agentmem.json"));
    assert!(store_path.ends_with(".agentmem/store.json"));
}

#[test]
fn store_path_wrapper_accepts_default_project_store_location() {
    let dir = tempdir().expect("temp dir");
    let store_path = Config::project_store_path(dir.path());

    let wrapped = StorePath::new(store_path).expect("valid store path");
    assert!(wrapped.as_path().ends_with(".agentmem/store.json"));
}

#[test]
fn config_save_creates_parent_directory() {
    let dir = tempdir().expect("temp dir");
    let config =
        Config::for_project_root(ProjectName::new("demo-project").expect("valid"), dir.path())
            .expect("config");

    let config_path = Config::project_config_path(dir.path());
    config.save(&config_path).expect("save");
    assert!(config_path.exists());
}

#[test]
fn config_save_then_load_roundtrips() {
    let dir = tempdir().expect("temp dir");
    let config =
        Config::for_project_root(ProjectName::new("demo-project").expect("valid"), dir.path())
            .expect("config");

    let config_path = Config::project_config_path(dir.path());
    config.save(&config_path).expect("save");
    let loaded = Config::load(&config_path).expect("load");

    assert_eq!(loaded.project_name(), config.project_name());
    assert_eq!(loaded.store_path(), config.store_path());
}

#[test]
fn initialize_if_missing_creates_empty_store_file() {
    let dir = tempdir().expect("temp dir");
    let store_path = Config::project_store_path(dir.path());

    persist::initialize_if_missing(&store_path).expect("init store");
    assert!(store_path.exists());
}

#[test]
fn initialize_if_missing_is_idempotent() {
    let dir = tempdir().expect("temp dir");
    let store_path = Config::project_store_path(dir.path());

    persist::initialize_if_missing(&store_path).expect("init first");
    let first = std::fs::read_to_string(&store_path).expect("read");

    persist::initialize_if_missing(&store_path).expect("init second");
    let second = std::fs::read_to_string(&store_path).expect("read");

    assert_eq!(first, second);
}

#[test]
fn config_from_json_rejects_invalid_project_name() {
    let raw = r#"{
        "version": 1,
        "project_name": "bad project name with spaces",
        "store_path": "/tmp/store.json"
    }"#;

    assert!(Config::from_json(raw).is_err());
}

#[test]
fn config_from_json_rejects_invalid_store_path() {
    let raw = r#"{
        "version": 1,
        "project_name": "demo-project",
        "store_path": ""
    }"#;

    assert!(Config::from_json(raw).is_err());
}
