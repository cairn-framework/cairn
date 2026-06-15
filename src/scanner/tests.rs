//! Tests for scanner orchestration: deduplication, divergence, blueprint-change gating, and provenance coverage.

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::{
    artefacts::registry::types::Decision,
    artefacts::registry::{ArtefactSet, DecisionStatus},
    blueprint::{NodeKind, ast::Span},
    map::graph::{Finding, FindingSeverity, Graph, NodeRecord, NodeState},
    scanner::{
        config::Config,
        state::{BlueprintSnapshot, NodeFingerprint},
    },
};

use super::*;

fn finding(
    code: &str,
    node: Option<&str>,
    path: Option<&str>,
    target: Option<&str>,
    message: &str,
) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Warning,
        message: message.to_owned(),
        node: node.map(str::to_owned),
        path: path.map(str::to_owned),
        target: target.map(str::to_owned),
    }
}

#[test]
fn test_dedup_drops_exact_duplicate() {
    let mut findings = vec![
        finding("CC001", Some("app.api"), None, None, "msg"),
        finding("CC001", Some("app.api"), None, None, "msg"),
    ];
    dedup_findings(&mut findings);
    assert_eq!(findings.len(), 1, "exact duplicate must be dropped");
}

#[test]
fn test_dedup_keeps_different_targets() {
    // Same code, node, path, message — but different dependency target.
    // Previously these were incorrectly collapsed because the key was
    // (code, node, message) and did not include `target`.
    let mut findings = vec![
        finding("CC002", Some("app.api"), None, Some("db"), "missing edge"),
        finding(
            "CC002",
            Some("app.api"),
            None,
            Some("cache"),
            "missing edge",
        ),
    ];
    dedup_findings(&mut findings);
    assert_eq!(
        findings.len(),
        2,
        "findings for different targets must both be kept"
    );
}

#[test]
fn test_dedup_keeps_different_paths() {
    let mut findings = vec![
        finding(
            "CAIRN_RECONCILE_ORPHANED_FILE",
            Some("app.api"),
            Some("src/a.rs"),
            None,
            "msg",
        ),
        finding(
            "CAIRN_RECONCILE_ORPHANED_FILE",
            Some("app.api"),
            Some("src/b.rs"),
            None,
            "msg",
        ),
    ];
    dedup_findings(&mut findings);
    assert_eq!(
        findings.len(),
        2,
        "findings for different file paths must both be kept"
    );
}

#[test]
fn test_dedup_merges_same_issue_different_message() {
    // Same issue (code + node + path + target) with a different message text
    // — the second is redundant; the first occurrence is preserved.
    let mut findings = vec![
        finding(
            "CC001",
            Some("app.api"),
            Some("src/lib.rs"),
            None,
            "first message",
        ),
        finding(
            "CC001",
            Some("app.api"),
            Some("src/lib.rs"),
            None,
            "second message",
        ),
    ];
    dedup_findings(&mut findings);
    assert_eq!(
        findings.len(),
        1,
        "same issue with different message texts must be deduplicated"
    );
    assert_eq!(
        findings[0].message, "first message",
        "first occurrence must be kept"
    );
}

#[test]
fn test_dedup_preserves_order_and_first_occurrence() {
    let mut findings = vec![
        finding("CC001", Some("app.api"), None, None, "alpha"),
        finding("CC002", Some("app.db"), None, None, "beta"),
        finding("CC001", Some("app.api"), None, None, "alpha"),
    ];
    dedup_findings(&mut findings);
    assert_eq!(findings.len(), 2);
    assert_eq!(findings[0].code, "CC001");
    assert_eq!(findings[1].code, "CC002");
}

#[test]
fn test_dedup_empty_is_noop() {
    let mut findings: Vec<Finding> = Vec::new();
    dedup_findings(&mut findings);
    assert!(findings.is_empty());
}

// ── helpers ───────────────────────────────────────────────────────────────

fn bare_node(id: &str) -> NodeRecord {
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
        contracts: Vec::new(),
        state: NodeState::Synced,
        files: Vec::new(),
        span: Span::point("test", 1, 1),
    }
}

fn graph_with_leaf(id: &str) -> Graph {
    let mut g = empty_graph();
    g.nodes.insert(id.to_owned(), bare_node(id));
    g
}

fn graph_with_parent(parent_id: &str, child_id: &str) -> Graph {
    let mut g = empty_graph();
    let mut parent = bare_node(parent_id);
    parent.children = vec![child_id.to_owned()];
    g.nodes.insert(parent_id.to_owned(), parent);
    g.nodes.insert(child_id.to_owned(), bare_node(child_id));
    g
}

fn empty_graph() -> Graph {
    Graph {
        nodes: BTreeMap::new(),
        names: BTreeMap::new(),
        outbound: BTreeMap::new(),
        inbound: BTreeMap::new(),
        findings: Vec::new(),
    }
}

fn snap(items: &[(&str, &str)]) -> BlueprintSnapshot {
    let mut s = BlueprintSnapshot::new();
    for (id, kind) in items {
        s.nodes.insert(
            id.to_string(),
            NodeFingerprint {
                kind: kind.to_string(),
                parent: None,
                paths: Vec::new(),
            },
        );
    }
    s
}

fn decision(id: &str, nodes: &[&str], status: DecisionStatus) -> Decision {
    Decision {
        id: id.to_owned(),
        path: "meta/decisions/test.md".to_owned(),
        nodes: nodes.iter().map(ToString::to_string).collect(),
        status,
        date: "2024-01-01".to_owned(),
        revisited: None,
        revisit_triggers: Vec::new(),
        informed_by: Vec::new(),
        supersedes: Vec::new(),
        refines: Vec::new(),
        related: Vec::new(),
        orphaned: false,
        orphan_reason: None,
        claims: None,
        body: String::new(),
    }
}

fn artefacts_with(decisions: Vec<Decision>) -> ArtefactSet {
    ArtefactSet {
        decisions,
        ..Default::default()
    }
}

fn report_and_target(
    node_id: &str,
    path: &str,
    role: &str,
    hash: &str,
) -> (super::TargetReport, crate::reconcile::target::Target) {
    use crate::reconcile::{
        ReconcilerId,
        target::{Language, Target, TargetId},
    };
    let path_buf = PathBuf::from(path);
    let report = super::TargetReport {
        target_id: TargetId {
            node_id: node_id.to_owned(),
            path: path_buf.clone(),
        },
        language: Language::Rust,
        reconciler_id: ReconcilerId("rust-code".to_owned()),
        claimed_files: Vec::new(),
        symbols: std::sync::Arc::new(Vec::new()),
        hash: hash.to_owned(),
    };
    let target = Target::new(node_id.to_owned(), path_buf, Language::Rust)
        .with_contract_role(role.to_owned());
    (report, target)
}

// ── check_blueprint_change_decisions ──────────────────────────────────────

#[test]
fn test_blueprint_change_no_finding_when_previous_is_empty() {
    let mut g = empty_graph();
    let current = snap(&[("app.new", "Module")]);
    let previous = BlueprintSnapshot::new(); // empty
    let artefacts = artefacts_with(vec![]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert!(g.findings.is_empty(), "empty previous must skip all checks");
}

#[test]
fn test_blueprint_change_no_finding_when_no_decisions() {
    let mut g = empty_graph();
    let previous = snap(&[("app.existing", "Module")]);
    let current = snap(&[("app.existing", "Module"), ("app.new", "Module")]);
    let artefacts = artefacts_with(vec![]); // no decisions
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert!(g.findings.is_empty(), "no decisions must skip all checks");
}

#[test]
fn test_blueprint_change_added_uncovered_node_emits_finding() {
    let mut g = empty_graph();
    let previous = snap(&[("app.existing", "Module")]);
    let current = snap(&[("app.existing", "Module"), ("app.new", "Module")]);
    // decision covers only "app.existing", not "app.new"
    let artefacts = artefacts_with(vec![decision(
        "d1",
        &["app.existing"],
        DecisionStatus::Accepted,
    )]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert_eq!(g.findings.len(), 1);
    assert_eq!(g.findings[0].code, "CAIRN_BLUEPRINT_CHANGE_NO_DECISION");
    assert_eq!(g.findings[0].node.as_deref(), Some("app.new"));
}

#[test]
fn test_blueprint_change_covered_added_node_no_finding() {
    let mut g = empty_graph();
    let previous = snap(&[("app.existing", "Module")]);
    let current = snap(&[("app.existing", "Module"), ("app.new", "Module")]);
    let artefacts = artefacts_with(vec![decision("d1", &["app.new"], DecisionStatus::Accepted)]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert!(g.findings.is_empty());
}

#[test]
fn test_blueprint_change_removed_node_uncovered_emits_finding() {
    let mut g = empty_graph();
    let previous = snap(&[("app.existing", "Module"), ("app.removed", "Module")]);
    let current = snap(&[("app.existing", "Module")]);
    let artefacts = artefacts_with(vec![decision(
        "d1",
        &["app.existing"],
        DecisionStatus::Accepted,
    )]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert_eq!(g.findings.len(), 1);
    assert_eq!(g.findings[0].node.as_deref(), Some("app.removed"));
}

#[test]
fn test_blueprint_change_path_only_no_finding() {
    // Path-only changes are explicitly not gated (comment in implementation).
    let mut g = empty_graph();
    let mut previous = BlueprintSnapshot::new();
    previous.nodes.insert(
        "app.api".to_owned(),
        NodeFingerprint {
            kind: "Module".to_owned(),
            parent: None,
            paths: vec!["src/old".to_owned()],
        },
    );
    let mut current = BlueprintSnapshot::new();
    current.nodes.insert(
        "app.api".to_owned(),
        NodeFingerprint {
            kind: "Module".to_owned(),
            parent: None,
            paths: vec!["src/new".to_owned()], // different path, same kind/parent
        },
    );
    let artefacts = artefacts_with(vec![decision("d1", &["x"], DecisionStatus::Accepted)]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert!(g.findings.is_empty(), "path-only change must not be gated");
}

#[test]
fn test_blueprint_change_superseded_decision_covers_added_node() {
    // The function message says "no decision artefact covers it" — a
    // Superseded decision IS still a decision artefact.  The filter
    // `Proposed | Accepted` wrongly excludes Superseded, causing the
    // gate to fire even though the node was legitimately decided.
    let mut g = empty_graph();
    let previous = snap(&[("app.existing", "Module")]);
    let current = snap(&[("app.existing", "Module"), ("app.new", "Module")]);
    let artefacts = artefacts_with(vec![decision(
        "d1",
        &["app.new"],
        DecisionStatus::Superseded, // the only covering decision is Superseded
    )]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert!(
        g.findings.is_empty(),
        "superseded decision must count as coverage; got: {:?}",
        g.findings
    );
}

// ── check_provenance_coverage ─────────────────────────────────────────────

#[test]
fn test_provenance_coverage_no_decisions_no_findings() {
    let mut g = graph_with_leaf("app.api");
    let artefacts = artefacts_with(vec![]);
    checks::check_provenance_coverage(&mut g, &artefacts);
    assert!(
        g.findings.is_empty(),
        "no decisions → early return, no warnings"
    );
}

#[test]
fn test_provenance_coverage_uncovered_leaf_emits_warning() {
    let mut g = graph_with_leaf("app.api");
    let artefacts = artefacts_with(vec![decision(
        "d1",
        &["app.other"],
        DecisionStatus::Accepted,
    )]);
    checks::check_provenance_coverage(&mut g, &artefacts);
    assert_eq!(g.findings.len(), 1);
    assert_eq!(g.findings[0].code, "CAIRN_PROVENANCE_NO_DECISION");
    assert_eq!(g.findings[0].severity, FindingSeverity::Warning);
    assert_eq!(g.findings[0].node.as_deref(), Some("app.api"));
}

#[test]
fn test_provenance_coverage_covered_leaf_no_warning() {
    let mut g = graph_with_leaf("app.api");
    let artefacts = artefacts_with(vec![decision("d1", &["app.api"], DecisionStatus::Accepted)]);
    checks::check_provenance_coverage(&mut g, &artefacts);
    assert!(g.findings.is_empty());
}

#[test]
fn test_provenance_coverage_parent_node_exempt_from_warning() {
    // Only leaf nodes (children.is_empty()) are checked for provenance.
    let mut g = graph_with_parent("app.system", "app.api");
    let artefacts = artefacts_with(vec![decision("d1", &["app.api"], DecisionStatus::Accepted)]);
    checks::check_provenance_coverage(&mut g, &artefacts);
    // app.system has children → exempt. app.api is covered → no warning.
    assert!(g.findings.is_empty());
}

// ── detect_divergence ─────────────────────────────────────────────────────

#[test]
fn test_divergence_single_report_no_finding() {
    let (r, t) = report_and_target("app.api", "src/api.rs", "public_api", "abc");
    let findings = detect_divergence(&[r], &[t], &Config::default());
    assert!(findings.is_empty(), "one report cannot diverge");
}

#[test]
fn test_divergence_two_reports_same_hash_no_finding() {
    let (r1, t1) = report_and_target("app.api", "src/v1.rs", "public_api", "abc");
    let (r2, t2) = report_and_target("app.api", "src/v2.rs", "public_api", "abc");
    let findings = detect_divergence(&[r1, r2], &[t1, t2], &Config::default());
    assert!(findings.is_empty(), "identical hashes must not diverge");
}

#[test]
fn test_divergence_two_reports_different_hash_emits_ct001() {
    let (r1, t1) = report_and_target("app.api", "src/v1.rs", "public_api", "abc");
    let (r2, t2) = report_and_target("app.api", "src/v2.rs", "public_api", "xyz");
    let findings = detect_divergence(&[r1, r2], &[t1, t2], &Config::default());
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].code, "CT001");
    assert_eq!(findings[0].severity, FindingSeverity::Error);
    assert_eq!(findings[0].node.as_deref(), Some("app.api"));
}

#[test]
fn test_divergence_different_roles_no_finding() {
    // Each role has only one target → len < 2 per role → no divergence.
    let (r1, t1) = report_and_target("app.api", "src/public.rs", "public_api", "abc");
    let (r2, t2) = report_and_target("app.api", "src/internal.rs", "internal", "xyz");
    let findings = detect_divergence(&[r1, r2], &[t1, t2], &Config::default());
    assert!(findings.is_empty(), "different roles must not be compared");
}
