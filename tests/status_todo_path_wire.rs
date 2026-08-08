//! Regression coverage for canonical artefact paths across status and todos wires.

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use cairn::{query_api, scanner};

fn write_cli_fixture(name: &str) -> (PathBuf, PathBuf) {
    let fixture = PathBuf::from(format!(
        "target/cairn-status-todos-cli-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&fixture);
    std::fs::create_dir_all(fixture.join("meta/todos")).expect("create CLI todo fixture");
    let blueprint = fixture.join("cairn.blueprint");
    std::fs::write(
        &blueprint,
        "System Test \"T\" id \"t\" {\n    Module Todo \"Todo\" id \"t.todo\" {\n        todos \"./meta/todos\"\n    }\n}\n",
    )
    .expect("write CLI blueprint fixture");
    std::fs::write(
        fixture.join("meta/todos/todo.path.md"),
        "---\nnode: t.todo\nstatus: open\ncreated: 2026-01-01\n---\n# Path todo\n",
    )
    .expect("write CLI todo fixture");
    (
        std::fs::canonicalize(&fixture).expect("canonicalize CLI fixture"),
        std::fs::canonicalize(blueprint).expect("canonicalize CLI blueprint"),
    )
}

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
#[test]
fn cli_status_and_todos_paths_match_for_absolute_and_relative_files_from_both_cwds() {
    let (fixture, blueprint) = write_cli_fixture("matrix");
    let outside = tempfile::tempdir_in(fixture.parent().expect("fixture parent"))
        .expect("create outside cwd");
    let relative_outside = PathBuf::from("..")
        .join(fixture.file_name().expect("fixture name"))
        .join("cairn.blueprint");
    let cases = vec![
        (fixture.clone(), blueprint.clone()),
        (fixture.clone(), PathBuf::from("cairn.blueprint")),
        (outside.path().to_path_buf(), blueprint),
        (outside.path().to_path_buf(), relative_outside),
    ];

    for (cwd, file) in cases {
        let file = file.to_string_lossy().into_owned();
        let status_human = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(&cwd)
            .args(["status", "--file", &file])
            .output()
            .expect("status human command runs");
        assert!(
            status_human.status.success(),
            "status human failed: {}",
            String::from_utf8_lossy(&status_human.stderr)
        );
        assert!(
            String::from_utf8_lossy(&status_human.stdout)
                .contains("- t.todo [open] meta/todos/todo.path.md"),
            "status human path must be root-relative: {}",
            String::from_utf8_lossy(&status_human.stdout)
        );

        let status = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(&cwd)
            .args(["status", "--json", "--file", &file])
            .output()
            .expect("status JSON command runs");
        assert!(
            status.status.success(),
            "status JSON failed: {}",
            String::from_utf8_lossy(&status.stderr)
        );
        let status: serde_json::Value =
            serde_json::from_slice(&status.stdout).expect("status JSON parses");

        let todos = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(&cwd)
            .args(["todos", "t.todo", "--json", "--file", &file])
            .output()
            .expect("todos JSON command runs");
        assert!(
            todos.status.success(),
            "todos JSON failed: {}",
            String::from_utf8_lossy(&todos.stderr)
        );
        let todos: serde_json::Value =
            serde_json::from_slice(&todos.stdout).expect("todos JSON parses");

        assert_eq!(
            status["open_todos"][0]["path"],
            todos["todos"][0]["path"],
            "status and todos must agree from cwd {}",
            cwd.display()
        );
        assert_eq!(
            status["open_todos"][0]["path"],
            "meta/todos/todo.path.md",
            "status path must be root-relative from cwd {}",
            cwd.display()
        );
    }

    let _ = std::fs::remove_dir_all(&fixture);
}
