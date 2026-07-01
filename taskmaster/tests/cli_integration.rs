use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

#[derive(Debug)]
struct TaskSnapshot {
    title: String,
    done: bool,
}

fn bin_path() -> String {
    std::env::var("CARGO_BIN_EXE_taskmaster")
        .expect("CARGO_BIN_EXE_taskmaster should be set for integration tests")
}

fn unique_test_file(suffix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!(
        "taskmaster_integration_{suffix}_{}_{}.json",
        std::process::id(),
        nanos
    ))
}

fn run_taskmaster(file: &Path, args: &[&str]) -> std::process::Output {
    let file_arg = file.to_string_lossy().to_string();
    Command::new(bin_path())
        .args(["--filename", &file_arg])
        .args(args)
        .output()
        .expect("taskmaster binary should execute")
}

fn read_tasks_json(file: &Path) -> Value {
    let content = fs::read_to_string(file).expect("task file should be readable");
    serde_json::from_str(&content).expect("task file should contain valid json")
}

fn persisted_tasks(json: &Value) -> Vec<TaskSnapshot> {
    let tasks = &json["tasks"];

    if let Some(array) = tasks.as_array() {
        return array
            .iter()
            .map(task_snapshot_from_value)
            .collect::<Vec<_>>();
    }

    if let Some(map) = tasks.as_object() {
        return map
            .values()
            .map(task_snapshot_from_value)
            .collect::<Vec<_>>();
    }

    panic!("tasks should be an array or object in persisted json");
}

fn task_snapshot_from_value(value: &Value) -> TaskSnapshot {
    let title = value["title"]
        .as_str()
        .expect("task title should be a string")
        .to_string();
    let done = value["done"]
        .as_bool()
        .expect("task done should be a boolean");

    TaskSnapshot { title, done }
}

fn task_by_title<'a>(tasks: &'a [TaskSnapshot], title: &str) -> &'a TaskSnapshot {
    tasks
        .iter()
        .find(|task| task.title == title)
        .unwrap_or_else(|| panic!("task with title '{title}' should exist"))
}

#[test]
fn create_mark_done_and_list_flow() {
    let file = unique_test_file("flow");

    let create_one = run_taskmaster(&file, &["create", "Buy milk"]);
    assert!(create_one.status.success(), "first create should succeed");

    let create_two = run_taskmaster(&file, &["create", "Write report"]);
    assert!(create_two.status.success(), "second create should succeed");

    let mark_done = run_taskmaster(&file, &["mark-done", "1"]);
    assert!(mark_done.status.success(), "mark-done should succeed");

    let listed = run_taskmaster(&file, &["list"]);
    assert!(listed.status.success(), "list should succeed");

    let listed_stdout = String::from_utf8_lossy(&listed.stdout);
    assert!(listed_stdout.contains("Tasks"));
    assert!(listed_stdout.contains("Buy milk"));
    assert!(listed_stdout.contains("Write report"));
    assert!(listed_stdout.contains("done: true"));

    let json = read_tasks_json(&file);
    let tasks = persisted_tasks(&json);
    assert_eq!(tasks.len(), 2);
    assert!(!task_by_title(&tasks, "Buy milk").done);
    assert!(task_by_title(&tasks, "Write report").done);

    let _ = fs::remove_file(&file);
}

#[test]
fn mark_done_with_unknown_id_does_not_change_tasks() {
    let file = unique_test_file("unknown_id");

    let create = run_taskmaster(&file, &["create", "Only task"]);
    assert!(create.status.success(), "create should succeed");

    let unknown_mark = run_taskmaster(&file, &["mark-done", "99"]);
    assert!(
        unknown_mark.status.success(),
        "mark-done with unknown id should still succeed"
    );

    let json = read_tasks_json(&file);
    let tasks = persisted_tasks(&json);
    assert_eq!(tasks.len(), 1);
    assert!(!task_by_title(&tasks, "Only task").done);

    let stderr = String::from_utf8_lossy(&unknown_mark.stderr);
    assert!(
        stderr.is_empty(),
        "command should not fail noisily when id is missing"
    );

    let _ = fs::remove_file(&file);
}

#[test]
fn create_fails_on_invalid_existing_json() {
    let file = unique_test_file("invalid_json");
    fs::write(&file, "{not-valid-json").expect("should write invalid json fixture");

    let result = run_taskmaster(&file, &["create", "Will fail"]);
    assert!(
        !result.status.success(),
        "create should fail when storage file contains invalid json"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("Error"),
        "stderr should explain that command failed"
    );

    let _ = fs::remove_file(&file);
}
