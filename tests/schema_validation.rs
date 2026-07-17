//! Validation of committed public JSON schemas against representative wires.
use cairn::map::graph::{Finding, FindingSeverity};
use serde_json::Value;

fn schema(name: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(format!("schemas/{name}.schema.json")).unwrap())
        .unwrap()
}
fn assert_valid(name: &str, value: &Value) {
    let compiled = jsonschema::JSONSchema::compile(&schema(name)).unwrap();
    if let Err(error) = compiled.validate(value) {
        let messages: Vec<String> = error.map(|item| item.to_string()).collect();
        panic!("{name} rejected value: {messages:?}");
    }
}

#[test]
fn map_snapshot_schema_accepts_dogfood_shape() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let scan = cairn::scanner::load_project(root, &root.join("cairn.blueprint")).unwrap();
    let snapshot = cairn::scanner::snapshot::build(&scan.graph, &scan.interface_hash);
    assert_valid("map", &serde_json::to_value(snapshot).unwrap());
    let committed: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("map.json")).unwrap()).unwrap();
    assert_valid("map", &committed);
}

#[test]
fn finding_schema_accepts_wire_finding() {
    let finding = Finding {
        code: "CAIRN_TEST_SAMPLE".to_owned(),
        severity: FindingSeverity::Warning,
        message: "sample".to_owned(),
        node: Some("app".to_owned()),
        target: None,
        path: None,
        deferred_by: None,
    };
    assert_valid("finding", &serde_json::to_value(finding).unwrap());
}

#[test]
fn work_item_schema_accepts_status_json_projection() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("meta/todos")).unwrap();
    std::fs::write(
        dir.path().join("cairn.blueprint"),
        "System Test \"T\" id \"t\" {\n    todos \"./meta/todos\"\n}\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("meta/todos/todo.next.md"),
        "---\nnode: t\nstatus: open\ncreated: 2026-01-01\n---\n# Next thing\n",
    )
    .unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(dir.path())
        .args(["status", "--json"])
        .output()
        .expect("status command runs");
    assert!(
        output.status.success(),
        "status failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        !value["next_recommended"].is_null(),
        "fixture must produce a recommendation: {value}"
    );
    assert_valid("work-item", &value["next_recommended"]);
}

#[test]
fn committed_schemas_match_rust_types() {
    let generated = [
        (
            "map",
            serde_json::to_value(schemars::schema_for!(cairn::scanner::snapshot::MapSnapshot))
                .unwrap(),
        ),
        (
            "finding",
            serde_json::to_value(schemars::schema_for!(cairn::map::graph::Finding)).unwrap(),
        ),
        (
            "work-item",
            serde_json::to_value(schemars::schema_for!(cairn::query_api::WorkItem)).unwrap(),
        ),
    ];
    for (name, value) in generated {
        let path = format!("schemas/{name}.schema.json");
        if std::env::var_os("UPDATE_SCHEMAS").is_some() {
            std::fs::write(&path, serde_json::to_string_pretty(&value).unwrap() + "\n").unwrap();
        } else {
            let committed: Value =
                serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
            assert_eq!(
                committed, value,
                "{path} drifted; rerun with UPDATE_SCHEMAS=1"
            );
        }
    }
}
