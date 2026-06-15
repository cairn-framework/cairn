use super::*;
use super::{
    apply::apply_blueprint_delta, delta::parse_blueprint_delta, validate::validate_change,
};
use std::time::{SystemTime, UNIX_EPOCH};

// ── helpers ───────────────────────────────────────────────────────────────

fn empty_change(id: &str) -> Change {
    Change {
        id: id.to_owned(),
        path: PathBuf::from(format!("meta/changes/{id}")),
        title: id.to_owned(),
        proposal: String::new(),
        design: None,
        delta: BlueprintDelta::default(),
        artefacts: Vec::new(),
        findings: Vec::new(),
    }
}

fn leaf_node(id: &str) -> Node {
    Node {
        kind: NodeKind::Module,
        id: id.to_owned(),
        name: id.to_owned(),
        description: String::new(),
        tags: Vec::new(),
        paths: Vec::new(),
        owns_files: false,
        contracts: Vec::new(),
        raw_fields: Vec::new(),
        children: Vec::new(),
        span: crate::blueprint::Span::point("test", 1, 1),
    }
}

fn edge(from: &str, to: &str) -> Edge {
    Edge {
        from: from.to_owned(),
        to: to.to_owned(),
        description: "dep".to_owned(),
        span: crate::blueprint::Span::point("test", 1, 1),
    }
}

fn nodes_set(ids: &[&str]) -> BTreeSet<String> {
    ids.iter().map(|s| (*s).to_owned()).collect()
}

// ── operation_summary ─────────────────────────────────────────────────────

#[test]
fn test_operation_summary_empty_returns_no_operations() {
    assert_eq!(operation_summary(&empty_change("c")), "no operations");
}

#[test]
fn test_operation_summary_single_category() {
    let mut c = empty_change("c");
    c.delta.added_nodes = vec![leaf_node("a"), leaf_node("b")];
    assert_eq!(operation_summary(&c), "2 added_nodes");
}

#[test]
fn test_operation_summary_multiple_categories_are_sorted_alphabetically() {
    let mut c = empty_change("c");
    c.delta.added_nodes = vec![leaf_node("a")];
    c.delta.removed_nodes = vec!["b".to_owned()];
    // BTreeMap sorts keys: "added_nodes" < "removed_nodes"
    assert_eq!(operation_summary(&c), "1 added_nodes, 1 removed_nodes");
}

#[test]
fn test_operation_summary_artefact_key_uses_lowercase_debug_variant() {
    let mut c = empty_change("c");
    c.artefacts.push(ArtefactOperation {
        operation: ChangeOperation::Added,
        change_path: PathBuf::from("meta/changes/c/foo.md"),
        target_path: PathBuf::from("meta/decisions/foo.md"),
        renamed_from: None,
        content: String::new(),
    });
    // format!("{:?}", Added) = "Added" → to_lowercase → "added" → "added_artefacts"
    assert_eq!(operation_summary(&c), "1 added_artefacts");
}

// ── active_changes_lines ──────────────────────────────────────────────────

#[test]
fn test_active_changes_lines_format() {
    let mut c = empty_change("phase-1");
    c.title = "Add API node".to_owned();
    c.delta.added_nodes = vec![leaf_node("a")];
    let lines = active_changes_lines(&[c]);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "phase-1 - Add API node (1 added_nodes)");
}

#[test]
fn test_active_changes_lines_empty_returns_empty_vec() {
    assert!(active_changes_lines(&[]).is_empty());
}

// ── operations_for_nodes ──────────────────────────────────────────────────

#[test]
fn test_operations_for_nodes_added_node_in_set() {
    let mut c = empty_change("c");
    c.delta.added_nodes = vec![leaf_node("app.api")];
    let lines = operations_for_nodes(&[c], &nodes_set(&["app.api"]));
    assert_eq!(lines.len(), 1);
    assert!(lines[0].contains("added node app.api"), "{lines:?}");
}

#[test]
fn test_operations_for_nodes_removed_node_in_set() {
    let mut c = empty_change("c");
    c.delta.removed_nodes = vec!["app.api".to_owned()];
    let lines = operations_for_nodes(&[c], &nodes_set(&["app.api"]));
    assert_eq!(lines.len(), 1);
    assert!(lines[0].contains("removed node app.api"), "{lines:?}");
}

#[test]
fn test_operations_for_nodes_renamed_node_matches_from_or_to() {
    let mk = || {
        let mut c = empty_change("c");
        c.delta.renamed_nodes = vec![Rename {
            from: "app.api".to_owned(),
            to: "app.http".to_owned(),
        }];
        c
    };
    // Matches via "from" side.
    let by_from = operations_for_nodes(&[mk()], &nodes_set(&["app.api"]));
    assert!(!by_from.is_empty(), "must match via from: {by_from:?}");
    // Matches via "to" side.
    let by_to = operations_for_nodes(&[mk()], &nodes_set(&["app.http"]));
    assert!(!by_to.is_empty(), "must match via to: {by_to:?}");
}

#[test]
fn test_operations_for_nodes_edge_endpoint_in_set() {
    let mut c = empty_change("c");
    c.delta.added_edges = vec![edge("app.api", "app.db")];
    let lines = operations_for_nodes(&[c], &nodes_set(&["app.api"]));
    assert!(
        !lines.is_empty(),
        "must match edge from endpoint: {lines:?}"
    );
}

#[test]
fn test_operations_for_nodes_renamed_edge_is_included() {
    // renamed_edges must appear in operations_for_nodes when an endpoint
    // is in the queried node set. This is the RED test: the current
    // implementation chains added/modified/removed edges but silently
    // skips renamed_edges entirely.
    let mut c = empty_change("c");
    c.delta.renamed_edges = vec![EdgeRename {
        from: edge("app.api", "app.db"),
        to: edge("app.http", "app.db"),
    }];
    let lines = operations_for_nodes(&[c], &nodes_set(&["app.api"]));
    assert!(
        !lines.is_empty(),
        "renamed edge affecting app.api must appear in operations_for_nodes output"
    );
}

#[test]
fn test_operations_for_nodes_no_match_returns_empty() {
    let mut c = empty_change("c");
    c.delta.added_nodes = vec![leaf_node("other.node")];
    let lines = operations_for_nodes(&[c], &nodes_set(&["app.api"]));
    assert!(lines.is_empty());
}

#[test]
fn test_parse_blueprint_delta_added_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let delta = parse_blueprint_delta(
        "change.delta",
        r#"## ADDED Nodes
System App "desc" id "app" {
Module Api "api" id "app.api" {}
}

## ADDED Edges
app.api -> app "reports"
"#,
    )?;

    let rendered = apply_blueprint_delta("", &delta)?;

    assert!(rendered.contains("System App"));
    assert!(rendered.contains("app.api -> app \"reports\""));
    Ok(())
}

#[test]
fn test_parse_blueprint_delta_modified_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let base = r#"System App "desc" id "app" {
Module Api "old" id "app.api" {}
}
"#;
    let delta = parse_blueprint_delta(
        "change.delta",
        r#"## MODIFIED Nodes
Module Api "new" id "app.api" {}
"#,
    )?;

    let rendered = apply_blueprint_delta(base, &delta)?;

    assert!(rendered.contains("Module Api \"new\" id \"app.api\""));
    Ok(())
}

#[test]
fn test_parse_blueprint_delta_removed_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let base = r#"System App "desc" id "app" {
Module Api "old" id "app.api" {}
}
app.api -> app "reports"
"#;
    let delta = parse_blueprint_delta(
        "change.delta",
        r#"## REMOVED Nodes
- app.api

## REMOVED Edges
app.api -> app "reports"
"#,
    )?;

    let rendered = apply_blueprint_delta(base, &delta)?;

    assert!(!rendered.contains("app.api"));
    Ok(())
}

#[test]
fn test_parse_blueprint_delta_renamed_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let base = r#"System App "desc" id "app" {
Module Api "old" id "app.api" {}
}
app.api -> app "reports"
"#;
    let delta = parse_blueprint_delta(
        "change.delta",
        r"## RENAMED Nodes
- app.api -> app.http
",
    )?;

    let rendered = apply_blueprint_delta(base, &delta)?;

    assert!(rendered.contains("id \"app.http\""));
    assert!(rendered.contains("app.http -> app \"reports\""));
    Ok(())
}

#[test]
fn test_validate_change_detects_conflicting_operations() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("conflict")?;
    write_project(&root)?;
    let graph = scanner::load_project(&root, &root.join("cairn.blueprint"))?.graph;
    let change = Change {
        id: "phase-7.5a-test-fortification".to_owned(),
        path: root.join("meta/changes/phase-7.5a-test-fortification"),
        title: "test".to_owned(),
        proposal: String::new(),
        design: None,
        delta: BlueprintDelta {
            modified_nodes: vec![Node {
                kind: NodeKind::Module,
                name: "Api".to_owned(),
                description: "desc".to_owned(),
                id: "app.api".to_owned(),
                tags: Vec::new(),
                paths: Vec::new(),
                owns_files: false,
                contracts: Vec::new(),
                raw_fields: Vec::new(),
                children: Vec::new(),
                span: crate::blueprint::Span::point("cairn.blueprint", 1, 1),
            }],
            removed_nodes: vec!["app.api".to_owned()],
            ..BlueprintDelta::default()
        },
        artefacts: Vec::new(),
        findings: Vec::new(),
    };

    let errors = validate_change(&change, &graph);

    assert!(
        errors
            .iter()
            .any(|error| error.contains("conflicting operations"))
    );
    Ok(())
}

fn write_project(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(root.join("meta/changes"))?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
Module Api "desc" id "app.api" {}
}
"#,
    )?;
    fs::write(
        root.join("cairn.config.yaml"),
        "reconcilers:\n  - id: rust-code\n    version: phase-1\n    config:\n      ignore:\n        - target\ncontext: \"\"\nrules: {}\n",
    )?;
    Ok(())
}

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-changes-tests-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
