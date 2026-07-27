//! Tests for artefact query renderers (todos, decisions, research, sources, rationale).

use super::*;
use crate::{
    artefacts::registry::{Decision, DecisionStatus},
    map::{Graph, NodeRecord, NodeState},
    scanner::{ScanResult, state::TargetHashes},
};
use std::collections::BTreeMap;

fn node_record(id: &str) -> NodeRecord {
    NodeRecord {
        kind: crate::blueprint::NodeKind::Module,
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
        span: crate::blueprint::Span::point("test", 1, 1),
    }
}

fn decision(id: &str, nodes: &[&str], body: &str, status: DecisionStatus) -> Decision {
    Decision {
        id: id.to_owned(),
        path: format!("meta/decisions/{id}.md"),
        nodes: nodes.iter().map(|node| (*node).to_owned()).collect(),
        status,
        date: "2026-01-01".to_owned(),
        revisited: None,
        revisit_triggers: Vec::new(),
        informed_by: Vec::new(),
        supersedes: Vec::new(),
        refines: Vec::new(),
        related: Vec::new(),
        orphaned: false,
        orphan_reason: None,
        gap: false,
        claims: None,
        body: body.to_owned(),
    }
}

fn scan_with_decisions(decisions: Vec<Decision>) -> ScanResult {
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
            decisions,
            ..Default::default()
        },
        contracts: crate::artefacts::contract::ContractSet::default(),
        interface_hash: String::new(),
        target_reports: Vec::new(),
        target_hashes: TargetHashes::default(),
        blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
    }
}

fn decisions_parsed(args: &[&str], json: bool) -> ParsedArgs {
    ParsedArgs {
        json,
        strict: false,
        file: std::path::PathBuf::from("cairn.blueprint"),
        changes_dir: std::path::PathBuf::from("meta/changes"),
        command: "decisions".to_owned(),
        command_args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        verbose: false,
        brief: false,
    }
}

#[test]
fn render_decisions_grep_matches_body_keyword() {
    let scan = scan_with_decisions(vec![
        decision(
            "dec.beads-loader",
            &["app"],
            "Read beads jsonl export",
            DecisionStatus::Accepted,
        ),
        decision(
            "dec.unrelated",
            &["app"],
            "Something else entirely",
            DecisionStatus::Accepted,
        ),
    ]);
    let p = decisions_parsed(&["decisions", "--grep", "beads"], false);
    let rendered = render_decisions(&p, &scan).unwrap();
    assert!(rendered.contains("Decisions matching \"beads\":"));
    assert!(rendered.contains("dec.beads-loader"));
    assert!(!rendered.contains("dec.unrelated"));
}

#[test]
fn render_decisions_grep_searches_without_node_arg() {
    let scan = scan_with_decisions(vec![decision(
        "dec.feedback",
        &["cairn.kernel.cli"],
        "feedback loop records friction",
        DecisionStatus::Accepted,
    )]);
    let p = decisions_parsed(&["decisions", "--grep", "friction"], false);
    let rendered = render_decisions(&p, &scan).unwrap();
    assert!(rendered.contains("dec.feedback"));
    assert!(!rendered.contains("Error"));
    assert!(!rendered.contains("NOT_FOUND"));
}

#[test]
fn render_decisions_grep_json_mode() {
    let scan = scan_with_decisions(vec![decision(
        "dec.beads-loader",
        &["app"],
        "beads",
        DecisionStatus::Accepted,
    )]);
    let p = decisions_parsed(&["decisions", "--grep", "beads"], true);
    let rendered = render_decisions(&p, &scan).unwrap();
    assert!(rendered.contains("\"query\":\"beads\""));
    assert!(rendered.contains("\"decisions\""));
    assert!(rendered.contains("dec.beads-loader"));
}

#[test]
fn render_decisions_grep_respects_status_filter() {
    let scan = scan_with_decisions(vec![
        decision("dec.live", &["app"], "beads", DecisionStatus::Accepted),
        decision("dec.old", &["app"], "beads", DecisionStatus::Superseded),
    ]);
    let p = decisions_parsed(
        &["decisions", "--grep", "beads", "--status", "accepted"],
        false,
    );
    let rendered = render_decisions(&p, &scan).unwrap();
    assert!(rendered.contains("dec.live"));
    assert!(!rendered.contains("dec.old"));
}

#[test]
fn rationale_text_labels_transitive_decisions_with_via() {
    let data = serde_json::json!({
        "node": "app",
        "decisions": [
            {
                "id": "dec.direct",
                "status": "accepted",
                "path": "meta/decisions/direct.md",
            },
            {
                "id": "dec.neighbour",
                "status": "accepted",
                "path": "meta/decisions/neighbour.md",
                "via": ["app.db"],
            },
        ],
        "research": [],
        "sources": [],
    });
    let rendered = rationale_text(&data);
    assert!(
        rendered.contains("dec.neighbour [accepted] meta/decisions/neighbour.md (via app.db)"),
        "transitive decision must carry a via label: {rendered}"
    );
    assert!(
        rendered.contains("dec.direct [accepted] meta/decisions/direct.md\n"),
        "direct decision must render without a via label: {rendered}"
    );
}

#[test]
fn todos_text_lists_matching_todos() {
    let data = serde_json::json!({
        "node": "app",
        "todos": [{
            "node": "app",
            "status": "open",
            "path": "meta/todos/todo.api.md",
        }],
    });
    let rendered = todos_text(&data);
    assert!(rendered.contains("Todos for app:"));
    assert!(rendered.contains("[open]"));
}

#[test]
fn todos_text_null_node_renders_project_wide_heading() {
    let data = serde_json::json!({
        "node": null,
        "todos": [
            {
                "node": "app.api",
                "status": "open",
                "path": "meta/todos/todo.api.md",
            },
            {
                "node": "app.db",
                "status": "done",
                "path": "meta/todos/todo.db.md",
            },
        ],
    });
    let rendered = todos_text(&data);
    assert!(rendered.contains("Todos (project-wide):"));
    assert!(rendered.contains("app.api"));
    assert!(rendered.contains("app.db"));
}

#[test]
fn todos_text_renders_filtered_todos() {
    // The status filter lives in the query_api handler; the renderer only
    // transforms whatever the canonical JSON carries. Feed the already
    // filtered payload and confirm the transform renders it correctly.
    let data = serde_json::json!({
        "node": "app",
        "todos": [{
            "node": "app",
            "status": "done",
            "path": "meta/todos/todo.done.md",
        }],
    });
    let rendered = todos_text(&data);
    assert!(rendered.contains("[done]"));
    assert!(!rendered.contains("[open]"));
}
