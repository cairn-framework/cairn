//! Regression coverage for canonical artefact paths across status and todos wires.

use std::path::{Path, PathBuf};

use cairn::{query_api, scanner};

fn write_fixture(name: &str) -> (PathBuf, PathBuf) {
    let fixture = PathBuf::from(format!(
        "target/cairn-status-todos-wire-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&fixture);
    std::fs::create_dir_all(fixture.join("meta/todos")).expect("create todo fixture");
    let pointer = format!("./{}/meta/todos", fixture.display());
    let blueprint = fixture.join("cairn.blueprint");
    std::fs::write(
        &blueprint,
        format!(
            "System Test \"T\" id \"t\" {{\n    Module Todo \"Todo\" id \"t.todo\" {{\n        todos \"{pointer}\"\n    }}\n}}\n"
        ),
    )
    .expect("write blueprint fixture");
    std::fs::write(
        fixture.join("meta/todos/todo.path.md"),
        "---\nnode: t.todo\nstatus: open\ncreated: 2026-01-01\n---\n# Path todo\n",
    )
    .expect("write todo fixture");
    (fixture, blueprint)
}

fn query_paths(root: &Path, blueprint: &Path, changes: &Path) -> (String, String) {
    let scan = scanner::load_project(root, blueprint).expect("load fixture scan");
    let request = |tool: &str| query_api::QueryRequest {
        tool: tool.to_owned(),
        ..query_api::QueryRequest::default()
    };
    let status = query_api::execute_with_scan(root, blueprint, changes, &request("status"), &scan)
        .expect("status query must succeed")
        .data;
    let todos = query_api::execute_with_scan(root, blueprint, changes, &request("todos"), &scan)
        .expect("todos query must succeed")
        .data;

    let status_path = status["open_todos"][0]["path"]
        .as_str()
        .expect("status path must be a string")
        .to_owned();
    let todos_path = todos["todos"][0]["path"]
        .as_str()
        .expect("todos path must be a string")
        .to_owned();
    (status_path, todos_path)
}

#[test]
fn status_and_todos_json_share_registry_path_spelling() {
    let (fixture, blueprint) = write_fixture("relative");
    let (status_path, todos_path) =
        query_paths(Path::new("."), &blueprint, Path::new("meta/changes"));

    assert_eq!(status_path, todos_path);
    assert_eq!(
        status_path,
        format!("{}/meta/todos/todo.path.md", fixture.display())
    );
    let _ = std::fs::remove_dir_all(&fixture);
}

#[test]
fn status_and_todos_json_share_registry_path_spelling_for_absolute_root() {
    let (fixture, blueprint) = write_fixture("absolute");
    let root = std::fs::canonicalize(".").expect("canonicalize project root");
    let blueprint = std::fs::canonicalize(blueprint).expect("canonicalize blueprint");
    let (status_path, todos_path) = query_paths(&root, &blueprint, &root.join("meta/changes"));

    assert_eq!(status_path, todos_path);
    assert_eq!(
        status_path,
        format!("{}/meta/todos/todo.path.md", fixture.display())
    );
    let _ = std::fs::remove_dir_all(&fixture);
}
