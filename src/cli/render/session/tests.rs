//! Tests for the session-continuity renderer.

use super::render_context;
use crate::artefacts::registry::DecisionStatus;
use crate::cli::render::project::tests::{decision, node_record, parsed, scan_with_nodes, system};
use crate::map::graph::NodeState;

#[test]
fn render_context_shows_system_name_and_counts() {
    let scan = scan_with_nodes(vec![
        system("sys", "MySystem", "A test system"),
        node_record("app"),
    ]);
    let rendered =
        render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan).unwrap();
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
    let rendered =
        render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan).unwrap();
    assert!(
        rendered.contains("Structure:\n"),
        "structure heading: {rendered}"
    );
    assert!(rendered.contains("  app\n"), "node line: {rendered}");
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
    let rendered =
        render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan).unwrap();
    assert!(
        rendered.contains("  app.a\n    -> app.b  # calls"),
        "missing labeled edge under source: {rendered}"
    );
}

#[test]
fn render_context_defaults_when_no_system() {
    let scan = scan_with_nodes(vec![node_record("app")]);
    let rendered =
        render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan).unwrap();
    assert!(rendered.contains("unknown (1 nodes, 0 edges)"));
}

#[test]
fn render_context_counts_pending_signatures() {
    let mut scan = scan_with_nodes(vec![system("sys", "Sys", "Mission headline")]);
    let rendered =
        render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan).unwrap();
    assert!(
        rendered.contains("Pending signatures: 0"),
        "zero proposed decisions: {rendered}"
    );

    scan.artefacts.decisions = vec![
        decision("dec.signed", DecisionStatus::Accepted),
        decision("dec.waiting", DecisionStatus::Proposed),
        decision("dec.also-waiting", DecisionStatus::Proposed),
    ];
    scan.artefacts.decisions[1].body =
        "# Waiting\n\n## Decision\n\nKeep the queue clear.\n\n## The rubric\n\n- **Tier**: `binding`.\n- **Unblocks**: the next step.\n"
            .to_owned();
    let rendered =
        render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan).unwrap();
    assert!(
        rendered.contains("Mission headline\n"),
        "system description is the mission headline: {rendered}"
    );
    assert!(
        rendered.contains("Pending decision: dec.waiting"),
        "{rendered}"
    );
    assert!(rendered.contains("Keep the queue clear."), "{rendered}");
    assert!(
        rendered.contains("Next action: Say your ruling in this session"),
        "{rendered}"
    );
    assert!(
        rendered.contains("Reopen this briefing with: cairn pending dec.waiting"),
        "{rendered}"
    );
    assert!(
        rendered.contains("Pending signatures: 2"),
        "proposed decisions only: {rendered}"
    );
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
    let rendered = render_context(&parsed(false), &dir, &scan).unwrap();
    assert!(rendered.contains("Backlog: 1 ready"));
    assert!(rendered.contains("cairn-aaa [P2] Do thing"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn render_context_shows_ghost_suffix_for_ghost_node() {
    // Empty scaffolding (and any Ghost) must be visible in context output.
    let mut empty = node_record("app.empty");
    empty.state = NodeState::Ghost;
    empty.paths = vec!["./src/empty".to_owned()];
    let mut real = node_record("app.real");
    real.state = NodeState::Synced;
    real.paths = vec!["./src/real".to_owned()];
    let scan = scan_with_nodes(vec![system("app", "App", "Smoke"), empty, real]);
    let rendered =
        render_context(&parsed(false), std::path::Path::new("/nonexistent"), &scan).unwrap();
    assert!(
        rendered.contains("empty [Ghost]"),
        "context must show [Ghost] suffix for empty scaffolding: {rendered}"
    );
    assert!(
        !rendered.contains("real [Ghost]") && !rendered.contains("real [Synced]"),
        "Synced nodes must not carry a state suffix: {rendered}"
    );
}
