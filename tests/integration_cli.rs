use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

use agentmem::config::Config;
use agentmem::types::{Key, ProjectName, Value};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_agentmem")
}

fn init_project(root: &Path) {
    let config = Config::for_project_root(
        ProjectName::new("demo-project").expect("valid project name"),
        root,
    )
    .expect("config");

    let config_path = Config::project_config_path(root);
    config.save(&config_path).expect("save config");
}

fn run_cli(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .current_dir(root)
        .args(args)
        .output()
        .expect("run cli")
}

#[test]
fn info_fails_cleanly_when_project_is_not_initialized() {
    let dir = tempdir().expect("temp dir");
    let output = run_cli(dir.path(), &["info"]);

    assert!(!output.status.success());
}

#[test]
fn info_shows_project_and_store_metadata() {
    let dir = tempdir().expect("temp dir");
    init_project(dir.path());

    let output = run_cli(dir.path(), &["info"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Store Info"));
    assert!(stdout.contains("Project"));
    assert!(stdout.contains("Format"));
}

#[test]
fn get_missing_key_returns_success_with_human_message() {
    let dir = tempdir().expect("temp dir");
    init_project(dir.path());

    let output = run_cli(dir.path(), &["get", "agent/codex/missing"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("key not found"));
}

#[test]
fn set_creates_store_file_on_first_write() {
    let dir = tempdir().expect("temp dir");
    init_project(dir.path());
    let store_path = Config::project_store_path(dir.path());
    assert!(!store_path.exists());

    let output = run_cli(
        dir.path(),
        &["set", "agent/codex/current_task", "Review PR"],
    );
    assert!(output.status.success());
    assert!(store_path.exists());
}

#[test]
fn set_then_get_roundtrips_through_real_cli() {
    let dir = tempdir().expect("temp dir");
    init_project(dir.path());

    let set_output = run_cli(
        dir.path(),
        &["set", "agent/codex/current_task", "Review PR"],
    );
    assert!(set_output.status.success());

    let get_output = run_cli(dir.path(), &["get", "agent/codex/current_task"]);
    assert!(get_output.status.success());
    let stdout = String::from_utf8_lossy(&get_output.stdout);
    assert!(stdout.contains("Review PR"));
}

#[test]
fn delete_removes_existing_key() {
    let dir = tempdir().expect("temp dir");
    init_project(dir.path());

    let _ = run_cli(dir.path(), &["set", "agent/codex/current_task", "Review"]);
    let output = run_cli(dir.path(), &["delete", "agent/codex/current_task"]);
    assert!(output.status.success());

    let get_output = run_cli(dir.path(), &["get", "agent/codex/current_task"]);
    let stdout = String::from_utf8_lossy(&get_output.stdout);
    assert!(stdout.contains("key not found"));
}

#[test]
fn list_with_prefix_filters_results() {
    let dir = tempdir().expect("temp dir");
    init_project(dir.path());

    let _ = run_cli(dir.path(), &["set", "agent/codex/task", "A"]);
    let _ = run_cli(dir.path(), &["set", "agent/claude/task", "B"]);

    let output = run_cli(dir.path(), &["list", "--prefix", "agent/codex"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("agent/codex/task"));
    assert!(!stdout.contains("agent/claude/task"));
}

#[test]
fn list_prints_all_entries_in_sorted_order() {
    let dir = tempdir().expect("temp dir");
    init_project(dir.path());

    let _ = run_cli(dir.path(), &["set", "agent/codex/task", "C"]);
    let _ = run_cli(dir.path(), &["set", "agent/claude/task", "A"]);
    let _ = run_cli(dir.path(), &["set", "agent/claude/context", "B"]);

    let output = run_cli(dir.path(), &["list"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    let first = stdout.find("agent/claude/context").expect("first");
    let second = stdout.find("agent/claude/task").expect("second");
    let third = stdout.find("agent/codex/task").expect("third");
    assert!(first < second && second < third);
}

#[test]
fn typed_key_and_value_validation_examples() {
    let key = Key::new("agent/codex/current_task").expect("valid key");
    let value = Value::new("review docs").expect("valid value");
    assert_eq!(key.as_str(), "agent/codex/current_task");
    assert_eq!(value.as_str(), "review docs");
}
