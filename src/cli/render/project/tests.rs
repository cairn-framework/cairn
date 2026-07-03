//! Tests for project-wide query renderers (context, status, dependencies, backlog).

use super::*;
use crate::{
    artefacts::registry::{Todo, TodoStatus},
    blueprint::{NodeKind, Span},
    map::{Graph, NodeRecord, NodeState},
    scanner::{ScanResult, state::TargetHashes},
};
use std::collections::BTreeMap;

fn node_record(id: &str) -> NodeRecord {
    NodeRecord {
        kind: NodeKind::Module,
        id: id.to_owned(),
        name: id.to_owned(),
        description: String::new(),
        tags: Vec::new(),
        parent: None,
        children: Vec::new(),
        paths: Vec::new(),
        owns_files: false,
        symbols: Vec::new(),
        contracts: Vec::new(),
        state: NodeState::Synced,
        files: Vec::new(),
        span: Span::point("test", 1, 1),
    }
}

fn system(id: &str, name: &str, desc: &str) -> NodeRecord {
    NodeRecord {
        kind: NodeKind::System,
        id: id.to_owned(),
        name: name.to_owned(),
        description: desc.to_owned(),
        tags: Vec::new(),
        parent: None,
        children: Vec::new(),
        paths: Vec::new(),
        owns_files: false,
        symbols: Vec::new(),
        contracts: Vec::new(),
        state: NodeState::Synced,
        files: Vec::new(),
        span: Span::point("test", 1, 1),
    }
}

fn scan_with_nodes(nodes: Vec<NodeRecord>) -> ScanResult {
    let graph_nodes: BTreeMap<String, NodeRecord> =
        nodes.into_iter().map(|n| (n.id.clone(), n)).collect();
    ScanResult {
        graph: Graph {
            nodes: graph_nodes,
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        },
        artefacts: crate::artefacts::registry::ArtefactSet::default(),
        contracts: crate::artefacts::contract::ContractSet::default(),
        interface_hash: String::new(),
        target_reports: Vec::new(),
        target_hashes: TargetHashes::default(),
        blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
    }
}

fn scan_with_todos(todos: Vec<Todo>) -> ScanResult {
    let mut nodes = BTreeMap::new();
    nodes.insert("app".to_owned(), node_record("app"));
    ScanResult {
        graph: Graph {
            nodes,
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        },
        artefacts: crate::artefacts::registry::ArtefactSet {
            todos,
            ..Default::default()
        },
        contracts: crate::artefacts::contract::ContractSet::default(),
        interface_hash: String::new(),
        target_reports: Vec::new(),
        target_hashes: TargetHashes::default(),
        blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
    }
}

fn parsed(json: bool) -> ParsedArgs {
    ParsedArgs {
        json,
        strict: false,
        file: std::path::PathBuf::from("cairn.blueprint"),
        changes_dir: std::path::PathBuf::from("meta/changes"),
        command: "status".to_owned(),
        command_args: vec!["status".to_owned()],
    }
}

fn todo(status: TodoStatus) -> Todo {
    Todo {
        path: "./todo.md".to_owned(),
        node: "app".to_owned(),
        status,
        created: "2026-01-01".to_owned(),
        satisfies: None,
        body: String::new(),
    }
}

#[test]
fn render_status_human_lists_open_and_in_progress_todos() {
    let scan = scan_with_todos(vec![todo(TodoStatus::Open), todo(TodoStatus::Done)]);
    let rendered = render_status(&parsed(false), &scan, std::path::Path::new("."));
    assert!(rendered.contains("Status:"));
    assert!(rendered.contains("[open]"));
    assert!(!rendered.contains("[done]"));
}

#[test]
fn render_status_human_empty_todos_renders_none() {
    let scan = scan_with_todos(Vec::new());
    let rendered = render_status(&parsed(false), &scan, std::path::Path::new("."));
    assert!(rendered.contains("Open todos:"));
    assert!(!rendered.contains("[open]"));
}

#[test]
fn render_status_json_includes_open_todos() {
    let scan = scan_with_todos(vec![todo(TodoStatus::InProgress)]);
    let rendered = render_status(&parsed(true), &scan, std::path::Path::new("."));
    assert!(rendered.contains("\"open_todos\""));
    assert!(rendered.contains("\"active_changes\""));
    assert!(rendered.contains("\"recent_log_entries\""));
}

#[test]
fn render_status_json_omits_done_todos() {
    let scan = scan_with_todos(vec![todo(TodoStatus::Done)]);
    let rendered = render_status(&parsed(true), &scan, std::path::Path::new("."));
    assert!(!rendered.contains("\"node\":\"app\""));
    assert!(!rendered.contains("in-progress"));
    assert!(!rendered.contains("\"status\":\"open\""));
}

#[test]
fn render_status_human_prefers_native_todo_in_next_recommended() {
    let scan = scan_with_todos(vec![Todo {
        path: "./todo.md".to_owned(),
        node: "app".to_owned(),
        status: TodoStatus::Open,
        created: "2026-01-01".to_owned(),
        satisfies: None,
        body: "# Wire the thing".to_owned(),
    }]);
    let rendered = render_status(&parsed(false), &scan, std::path::Path::new("."));
    assert!(
        rendered.contains("Next recommended:\nWire the thing (native todo, node: app)\n"),
        "human status must surface the open native todo, not fall through \
         to the beads backlog: {rendered}"
    );
}

#[test]
fn render_status_json_includes_next_recommended_for_native_todo() {
    let scan = scan_with_todos(vec![Todo {
        path: "./todo.md".to_owned(),
        node: "app".to_owned(),
        status: TodoStatus::Open,
        created: "2026-01-01".to_owned(),
        satisfies: None,
        body: "# Wire the thing".to_owned(),
    }]);
    let rendered = render_status(&parsed(true), &scan, std::path::Path::new("."));
    let value: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    assert_eq!(
        value["next_recommended"],
        "Wire the thing (native todo, node: app)"
    );
}

#[test]
fn render_status_json_next_recommended_null_when_clean() {
    // Use an isolated empty root, not "." (the real repo checkout): with
    // no beads and no todos this must be null regardless of what this
    // repo's own .beads/issues.jsonl happens to contain when the test
    // suite runs.
    let dir = std::env::temp_dir().join(format!("cairn-status-clean-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let scan = scan_with_todos(Vec::new());
    let rendered = render_status(&parsed(true), &scan, &dir);
    let value: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    assert!(value["next_recommended"].is_null());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn render_context_shows_system_name_and_counts() {
    let scan = scan_with_nodes(vec![
        system("sys", "MySystem", "A test system"),
        node_record("app"),
    ]);
    let rendered = render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan);
    assert!(rendered.contains("MySystem (2 nodes, 0 edges)"));
    assert!(rendered.contains("A test system"));
    assert!(rendered.contains("Findings: 0 errors, 0 warnings, 0 info"));
    assert!(rendered.contains(
        "Artefacts: 0 contracts, 0 decisions, 0 todos, 0 research, 0 reviews, 0 sources"
    ));
}

#[test]
fn render_context_node_line_omits_path_and_synced_state() {
    let mut app = node_record("app");
    app.paths = vec!["./src".to_owned()];
    let scan = scan_with_nodes(vec![system("sys", "Sys", ""), app]);
    let rendered = render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan);
    assert!(
        rendered.contains("Structure:\n  app\n"),
        "node line: {rendered}"
    );
    assert!(
        !rendered.contains("./src"),
        "path must be dropped: {rendered}"
    );
}

#[test]
fn render_context_lists_labeled_dependencies() {
    let mut scan = scan_with_nodes(vec![
        system("sys", "Sys", ""),
        node_record("app.a"),
        node_record("app.b"),
    ]);
    scan.graph.outbound.insert(
        "app.a".to_owned(),
        vec![crate::map::graph::EdgeRef {
            from: "app.a".to_owned(),
            to: "app.b".to_owned(),
            description: "calls".to_owned(),
        }],
    );
    let rendered = render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan);
    assert!(
        rendered.contains("  app.a\n    -> app.b  # calls"),
        "missing labeled edge under source: {rendered}"
    );
}

#[test]
fn render_context_defaults_when_no_system() {
    let scan = scan_with_nodes(vec![node_record("app")]);
    let rendered = render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan);
    assert!(rendered.contains("unknown (1 nodes, 0 edges)"));
}

#[test]
fn render_context_includes_backlog_section() {
    let dir = std::env::temp_dir().join(format!("cairn-ctx-backlog-{}", std::process::id()));
    let beads = dir.join(".beads");
    std::fs::create_dir_all(&beads).unwrap();
    std::fs::write(
        beads.join("issues.jsonl"),
        r#"{"id":"cairn-aaa","title":"Do thing","status":"open","priority":2,"issue_type":"task"}"#,
    )
    .unwrap();
    let scan = scan_with_nodes(vec![node_record("app")]);
    let rendered = render_context(&parsed(false), &dir, &scan);
    assert!(rendered.contains("Backlog: 1 ready"));
    assert!(rendered.contains("cairn-aaa [P2] Do thing"));
    let _ = std::fs::remove_dir_all(&dir);
}

fn backlog_args(node: &str, json: bool) -> ParsedArgs {
    let mut p = parsed(json);
    p.command = "backlog".to_owned();
    p.command_args = vec!["backlog".to_owned(), node.to_owned()];
    p
}

fn with_beads(tag: &str, lines: &str) -> std::path::PathBuf {
    let dir =
        std::env::temp_dir().join(format!("cairn-backlog-render-{tag}-{}", std::process::id()));
    let beads = dir.join(".beads");
    std::fs::create_dir_all(&beads).unwrap();
    std::fs::write(beads.join("issues.jsonl"), lines).unwrap();
    dir
}

#[test]
fn render_backlog_lists_node_linked_beads() {
    let dir = with_beads(
        "human",
        r#"{"id":"cairn-z","title":"Wire it","status":"open","priority":1,"issue_type":"task","labels":["cairn-node:app"]}"#,
    );
    let scan = scan_with_nodes(vec![node_record("app")]);
    let rendered = render_backlog(&backlog_args("app", false), &dir, &scan).unwrap();
    assert!(rendered.contains("Beads for app:"), "{rendered}");
    assert!(
        rendered.contains("cairn-z [P1] [open] Wire it"),
        "{rendered}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn render_backlog_json_emits_beads_array() {
    let dir = with_beads(
        "json",
        r#"{"id":"cairn-z","title":"Wire it","status":"open","priority":1,"issue_type":"task","labels":["cairn-node:app"]}"#,
    );
    let scan = scan_with_nodes(vec![node_record("app")]);
    let rendered = render_backlog(&backlog_args("app", true), &dir, &scan).unwrap();
    let value: serde_json::Value = serde_json::from_str(&rendered).expect("valid JSON");
    assert_eq!(value["node"], "app");
    let beads = value["beads"].as_array().expect("beads array");
    assert_eq!(beads.len(), 1);
    assert_eq!(beads[0]["id"], "cairn-z");
    assert_eq!(beads[0]["title"], "Wire it");
    assert_eq!(beads[0]["status"], "open");
    assert_eq!(beads[0]["priority"], 1);
    assert_eq!(beads[0]["issue_type"], "task");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn render_backlog_empty_node_uses_copy() {
    let dir = with_beads(
        "empty",
        r#"{"id":"cairn-other","title":"X","status":"open","priority":2,"issue_type":"task","labels":["cairn-node:other"]}"#,
    );
    let scan = scan_with_nodes(vec![node_record("app")]);
    let rendered = render_backlog(&backlog_args("app", false), &dir, &scan).unwrap();
    let expected = crate::copy::lookup("empty-states.node-no-beads.body");
    assert!(rendered.contains(expected), "{rendered}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn render_backlog_unknown_node_errs() {
    let dir = with_beads("unknown", "");
    let scan = scan_with_nodes(vec![node_record("app")]);
    assert!(render_backlog(&backlog_args("missing", false), &dir, &scan).is_err());
    let _ = std::fs::remove_dir_all(&dir);
}
