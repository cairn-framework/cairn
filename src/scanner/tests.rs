// cairn:allow-large-module reason: extracted test module for scanner orchestration (convention: tests.rs split from parent when parent exceeds the line limit)
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
        deferred_by: None,
        parked_by: None,
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
        symbols: Vec::new(),
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
                edges: Vec::new(),
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
        gap: false,
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
        symbol_records: std::sync::Arc::new(Vec::new()),
        symbols: std::sync::Arc::new(Vec::new()),
        hash: Some(hash.to_owned()),
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
            edges: Vec::new(),
        },
    );
    let mut current = BlueprintSnapshot::new();
    current.nodes.insert(
        "app.api".to_owned(),
        NodeFingerprint {
            kind: "Module".to_owned(),
            parent: None,
            paths: vec!["src/new".to_owned()], // different path, same kind/parent
            edges: Vec::new(),
        },
    );
    let artefacts = artefacts_with(vec![decision("d1", &["x"], DecisionStatus::Accepted)]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert!(g.findings.is_empty(), "path-only change must not be gated");
}

#[test]
fn test_blueprint_change_superseded_decision_covers_added_node() {
    // Regression guard: Superseded decisions must still count as coverage.
    // The coverage filter intentionally includes Proposed, Accepted, and
    // Superseded so a later decision that supersedes an earlier one does not
    // leave its nodes unprotected by the change gate.
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

fn snap_e(items: &[(&str, &str, &[&str])]) -> BlueprintSnapshot {
    let mut s = BlueprintSnapshot::new();
    for (id, kind, edges) in items {
        s.nodes.insert(
            (*id).to_string(),
            NodeFingerprint {
                kind: (*kind).to_string(),
                parent: None,
                paths: Vec::new(),
                edges: edges.iter().map(ToString::to_string).collect(),
            },
        );
    }
    s
}

#[test]
fn test_blueprint_change_edge_added_uncovered_emits_finding() {
    let mut g = empty_graph();
    let previous = snap_e(&[("app.api", "Module", &[])]);
    let current = snap_e(&[("app.api", "Module", &["app.db"])]);
    let artefacts = artefacts_with(vec![decision("d1", &["x"], DecisionStatus::Accepted)]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert_eq!(g.findings.len(), 1);
    assert_eq!(g.findings[0].code, "CAIRN_BLUEPRINT_CHANGE_NO_DECISION");
    assert_eq!(g.findings[0].node.as_deref(), Some("app.api"));
}

#[test]
fn test_blueprint_change_edge_removed_uncovered_emits_finding() {
    let mut g = empty_graph();
    let previous = snap_e(&[("app.api", "Module", &["app.db"])]);
    let current = snap_e(&[("app.api", "Module", &[])]);
    let artefacts = artefacts_with(vec![decision("d1", &["x"], DecisionStatus::Accepted)]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert_eq!(g.findings.len(), 1);
    assert_eq!(g.findings[0].node.as_deref(), Some("app.api"));
}

#[test]
fn test_blueprint_change_edge_covered_no_finding() {
    let mut g = empty_graph();
    let previous = snap_e(&[("app.api", "Module", &[])]);
    let current = snap_e(&[("app.api", "Module", &["app.db"])]);
    let artefacts = artefacts_with(vec![decision("d1", &["app.api"], DecisionStatus::Accepted)]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert!(
        g.findings.is_empty(),
        "covered edge change must not be gated"
    );
}

#[test]
fn test_blueprint_change_edge_unchanged_no_finding() {
    let mut g = empty_graph();
    let previous = snap_e(&[("app.api", "Module", &["app.db"])]);
    let current = snap_e(&[("app.api", "Module", &["app.db"])]);
    let artefacts = artefacts_with(vec![decision("d1", &["x"], DecisionStatus::Accepted)]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert!(g.findings.is_empty(), "unchanged edges must not be gated");
}

#[test]
fn test_blueprint_change_edge_skipped_when_previous_pre_v2() {
    let mut g = empty_graph();
    let mut previous = snap_e(&[("app.api", "Module", &[])]);
    previous.version = 1; // v1 baseline predates edge tracking
    let current = snap_e(&[("app.api", "Module", &["app.db"])]);
    let artefacts = artefacts_with(vec![decision("d1", &["x"], DecisionStatus::Accepted)]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert!(
        g.findings.is_empty(),
        "edge drift must not fire against a pre-v2 baseline"
    );
}

#[test]
fn test_blueprint_change_shape_and_edge_change_emits_single_finding() {
    let mut g = empty_graph();
    let previous = snap_e(&[("app.api", "Module", &[])]);
    let current = snap_e(&[("app.api", "Container", &["app.db"])]); // kind AND edges changed
    let artefacts = artefacts_with(vec![decision("d1", &["x"], DecisionStatus::Accepted)]);
    checks::check_blueprint_change_decisions(&mut g, &artefacts, &current, &previous);
    assert_eq!(
        g.findings.len(),
        1,
        "a single node must yield at most one finding"
    );
    assert_eq!(g.findings[0].node.as_deref(), Some("app.api"));
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

// ── check_decision_accumulation ───────────────────────────────────────────

fn accepted_decisions(node: &str, count: usize) -> Vec<Decision> {
    (0..count)
        .map(|i| decision(&format!("d{i}"), &[node], DecisionStatus::Accepted))
        .collect()
}

#[test]
fn test_decision_accumulation_at_threshold_emits_nothing() {
    let mut g = graph_with_leaf("app.api");
    let artefacts = artefacts_with(accepted_decisions("app.api", 3));
    checks::check_decision_accumulation(&mut g, &artefacts, 3);
    assert!(g.findings.is_empty(), "count == threshold is not over it");
}

#[test]
fn test_decision_accumulation_counts_a_repeated_node_once() {
    let mut g = graph_with_leaf("app.api");
    let artefacts = artefacts_with(vec![decision(
        "d0",
        &["app.api", "app.api"],
        DecisionStatus::Accepted,
    )]);
    checks::check_decision_accumulation(&mut g, &artefacts, 1);
    assert!(
        g.findings.is_empty(),
        "one decision naming a node twice is still one decision"
    );
}

#[test]
fn test_decision_accumulation_over_threshold_emits_info() {
    let mut g = graph_with_leaf("app.api");
    let artefacts = artefacts_with(accepted_decisions("app.api", 4));
    checks::check_decision_accumulation(&mut g, &artefacts, 3);
    assert_eq!(g.findings.len(), 1);
    assert_eq!(g.findings[0].code, "CAIRN_DECISION_ACCUMULATION");
    assert_eq!(g.findings[0].severity, FindingSeverity::Info);
    assert_eq!(g.findings[0].node.as_deref(), Some("app.api"));
    assert!(
        g.findings[0].message.contains('4') && g.findings[0].message.contains("threshold 3"),
        "message reports the count and the threshold: {}",
        g.findings[0].message
    );
}

#[test]
fn test_decision_accumulation_counts_accepted_only() {
    let mut g = graph_with_leaf("app.api");
    let mut decisions = accepted_decisions("app.api", 2);
    decisions.push(decision("p1", &["app.api"], DecisionStatus::Proposed));
    decisions.push(decision("s1", &["app.api"], DecisionStatus::Superseded));
    let artefacts = artefacts_with(decisions);
    checks::check_decision_accumulation(&mut g, &artefacts, 2);
    assert!(
        g.findings.is_empty(),
        "proposed and superseded decisions do not accumulate"
    );
}

#[test]
fn test_decision_accumulation_skips_nodes_absent_from_graph() {
    let mut g = graph_with_leaf("app.api");
    let artefacts = artefacts_with(accepted_decisions("app.other", 4));
    checks::check_decision_accumulation(&mut g, &artefacts, 3);
    assert!(
        g.findings.is_empty(),
        "an unknown node is CAIRN_DECISION_ORPHANED, not accumulation"
    );
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

// ── check_orphan_beads ────────────────────────────────────────────────────

fn write_beads(dir: &std::path::Path, lines: &[&str]) {
    let beads = dir.join(".beads");
    std::fs::create_dir_all(&beads).unwrap();
    std::fs::write(beads.join("issues.jsonl"), lines.join("\n")).unwrap();
}

#[test]
fn test_orphan_beads_warns_on_unknown_node_label() {
    let dir = std::env::temp_dir().join(format!("cairn-orphanbeads-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    write_beads(
        &dir,
        &[
            r#"{"id":"cairn-real","status":"open","labels":["cairn-node:app.api"]}"#,
            r#"{"id":"cairn-orphan","status":"open","labels":["cairn-node:app.gone"]}"#,
            r#"{"id":"cairn-unlinked","status":"open","labels":["misc"]}"#,
        ],
    );
    let mut g = graph_with_leaf("app.api");
    checks::check_orphan_beads(&mut g, &dir);
    assert_eq!(g.findings.len(), 1, "only the orphan-labelled bead warns");
    assert_eq!(g.findings[0].code, "CAIRN_BACKLOG_ORPHAN_NODE");
    assert_eq!(g.findings[0].severity, FindingSeverity::Warning);
    assert_eq!(g.findings[0].node.as_deref(), Some("app.gone"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_orphan_beads_no_export_no_findings() {
    let dir = std::env::temp_dir().join(format!("cairn-orphanbeads-empty-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let mut g = graph_with_leaf("app.api");
    checks::check_orphan_beads(&mut g, &dir);
    assert!(g.findings.is_empty(), "no export -> no findings");
    let _ = std::fs::remove_dir_all(&dir);
}
// ── TargetReport.hash semantics ───────────────────────────────────────────
// The scanner must assign `hash: None` to Unknown-language targets (no
// reconciler runs) and `hash: Some(..)` to reconciled targets that own files.
use crate::blueprint::Ast;
use crate::blueprint::ast::Node;
use crate::reconcile::target::{Language, Target};
use std::fs;

fn target_report_ast(node_id: &str, node_path: &str) -> Ast {
    Ast {
        nodes: vec![Node {
            kind: NodeKind::Module,
            name: node_id.to_owned(),
            description: String::new(),
            id: node_id.to_owned(),
            tags: Vec::new(),
            paths: vec![node_path.to_owned()],
            owns_files: false,
            contracts: Vec::new(),
            raw_fields: Vec::new(),
            children: Vec::new(),
            span: Span::point("cairn.blueprint", 1, 1),
        }],
        edges: Vec::new(),
    }
}

#[test]
fn unknown_language_target_hash_is_none() {
    let root = std::env::temp_dir().join(format!("cairn-unknown-hash-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let ast = target_report_ast("app.api", ".");
    let target = Target::new("app.api".to_owned(), root.clone(), Language::Unknown);
    let (reports, findings) =
        super::reconcile_targets(&[target], &root, &[], &ast, &Config::default());
    assert_eq!(reports.len(), 1, "one target -> one report");
    assert_eq!(reports[0].language, Language::Unknown);
    assert!(
        reports[0].hash.is_none(),
        "Unknown-language targets must carry hash=None (no reconciler runs)"
    );
    assert!(
        findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_LANGUAGE_UNKNOWN"),
        "Unknown target must warn CAIRN_RECONCILE_LANGUAGE_UNKNOWN"
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn known_language_target_hash_is_some() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/reconcile_baseline/python");
    let root = std::env::temp_dir().join(format!("cairn-known-hash-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    // Copy the fixture so the reconciler cache write does not pollute the repo.
    for f in ["api.py", "orphan.py"] {
        fs::copy(fixture.join(f), root.join(f)).unwrap();
    }
    let ast = target_report_ast("app.api", ".");
    let target = Target::new("app.api".to_owned(), root.clone(), Language::Python);
    let (reports, _findings) =
        super::reconcile_targets(&[target], &root, &[], &ast, &Config::default());
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].language, Language::Python);
    assert!(
        reports[0].hash.is_some(),
        "a reconciled target that owns files must carry hash=Some"
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn test_build_targets_path_scoped_override() {
    use crate::blueprint::{Ast, Node, NodeKind, Span};
    use crate::scanner::config::{Config, TargetConfig};

    let ast = Ast {
        nodes: vec![Node {
            kind: NodeKind::Module,
            name: "UI".to_owned(),
            description: String::new(),
            id: "cairn.ui".to_owned(),
            tags: Vec::new(),
            paths: vec!["./src/ui".to_owned(), "./src/ui_assets".to_owned()],
            owns_files: true,
            contracts: Vec::new(),
            raw_fields: Vec::new(),
            children: Vec::new(),
            span: Span::point("test", 1, 1),
        }],
        edges: Vec::new(),
    };

    let mut config = Config::default();
    config.targets.push(TargetConfig {
        node_id: "cairn.ui".to_owned(),
        path: std::path::PathBuf::from("./src/ui_assets/"),
        language: "assets".to_owned(),
        contract_role: "public_api".to_owned(),
    });

    let temp_root =
        std::env::temp_dir().join(format!("cairn-test-build-targets-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&temp_root);
    std::fs::create_dir_all(&temp_root).unwrap();

    let targets = super::build_targets(&ast, &config, &temp_root, &[]);
    assert_eq!(targets.len(), 2);

    // The target matching ./src/ui should retain its default inferred language (Unknown or Rust if inferred)
    // The target matching ./src/ui_assets should override its language to Assets
    let ui_target = targets
        .iter()
        .find(|t| t.id.path == std::path::Path::new("src/ui"))
        .unwrap();
    let assets_target = targets
        .iter()
        .find(|t| t.id.path == std::path::Path::new("src/ui_assets"))
        .unwrap();

    assert_eq!(ui_target.language, Language::Unknown);
    assert_eq!(assets_target.language, Language::Assets);
    let _ = std::fs::remove_dir_all(&temp_root);
}

// ── check_todo_defers ─────────────────────────────────────────────────────

fn info_finding(code: &str, node: Option<&str>, path: Option<&str>) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Info,
        message: "standing info".to_owned(),
        node: node.map(str::to_owned),
        target: None,
        path: path.map(str::to_owned),
        deferred_by: None,
        parked_by: None,
    }
}

fn defers_todo(
    slug: &str,
    status: crate::artefacts::registry::TodoStatus,
    refs: &[(&str, &str)],
) -> crate::artefacts::registry::Todo {
    crate::artefacts::registry::Todo {
        path: format!("meta/todos/todo.{slug}.md"),
        node: "app.api".to_owned(),
        status,
        created: "2026-07-29".to_owned(),
        satisfies: None,
        defers: refs
            .iter()
            .map(|(code, location)| crate::artefacts::registry::DefersRef {
                code: (*code).to_owned(),
                location: (*location).to_owned(),
            })
            .collect(),
        body: String::new(),
    }
}

fn todos_set(todos: Vec<crate::artefacts::registry::Todo>) -> ArtefactSet {
    ArtefactSet {
        todos,
        ..ArtefactSet::default()
    }
}

use crate::artefacts::registry::TodoStatus;

#[test]
fn test_todo_defers_blocked_todo_parks_the_standing_info_pair() {
    // The acceptance shape: exactly the two CAIRN_SOURCE_UNVERIFIED Info
    // findings, parked by one blocked todo declaring both references.
    let mut g = empty_graph();
    g.findings.push(info_finding(
        "CAIRN_SOURCE_UNVERIFIED",
        None,
        Some("./meta/sources/a.md"),
    ));
    g.findings.push(info_finding(
        "CAIRN_SOURCE_UNVERIFIED",
        None,
        Some("./meta/sources/b.md"),
    ));
    let artefacts = todos_set(vec![defers_todo(
        "park-sources",
        TodoStatus::Blocked,
        &[
            ("CAIRN_SOURCE_UNVERIFIED", "meta/sources/a.md"),
            ("CAIRN_SOURCE_UNVERIFIED", "meta/sources/b.md"),
        ],
    )]);
    todo_defers::check_todo_defers(&mut g, &artefacts, std::path::Path::new("."));
    assert_eq!(
        g.findings.len(),
        2,
        "parking must never add or drop findings"
    );
    for finding in &g.findings {
        assert_eq!(finding.parked_by.as_deref(), Some("todo.park-sources"));
        assert_eq!(
            finding.message, "standing info",
            "parked is a field, never a message mutation"
        );
        assert_eq!(finding.severity, FindingSeverity::Info);
    }
}

#[test]
fn test_todo_defers_open_todo_parks_nothing_and_raises_nothing() {
    let mut g = empty_graph();
    g.findings.push(info_finding(
        "CAIRN_SOURCE_UNVERIFIED",
        None,
        Some("meta/sources/a.md"),
    ));
    let artefacts = todos_set(vec![defers_todo(
        "park-sources",
        TodoStatus::Open,
        &[("CAIRN_SOURCE_UNVERIFIED", "meta/sources/a.md")],
    )]);
    todo_defers::check_todo_defers(&mut g, &artefacts, std::path::Path::new("."));
    assert_eq!(g.findings.len(), 1);
    assert_eq!(
        g.findings[0].parked_by, None,
        "only a blocked todo parks; a matching open todo is inert"
    );
}

#[test]
fn test_todo_defers_stale_reference_raises_unmatched_for_any_status() {
    for status in [TodoStatus::Blocked, TodoStatus::Done, TodoStatus::Open] {
        let mut g = empty_graph();
        let artefacts = todos_set(vec![defers_todo(
            "park-sources",
            status,
            &[("CAIRN_SOURCE_UNVERIFIED", "meta/sources/gone.md")],
        )]);
        todo_defers::check_todo_defers(&mut g, &artefacts, std::path::Path::new("."));
        assert_eq!(g.findings.len(), 1, "{status:?}");
        let finding = &g.findings[0];
        assert_eq!(finding.code, "CAIRN_TODO_DEFERS_UNMATCHED");
        assert_eq!(finding.severity, FindingSeverity::Warning);
        assert_eq!(
            finding.path.as_deref(),
            Some("meta/todos/todo.park-sources.md")
        );
        assert!(
            finding.message.contains("todo.park-sources")
                && finding.message.contains("meta/sources/gone.md"),
            "message names the todo and the reference: {}",
            finding.message
        );
    }
}

#[test]
fn test_todo_defers_blocking_target_raises_and_never_parks() {
    for severity in [FindingSeverity::Warning, FindingSeverity::Error] {
        let mut g = empty_graph();
        let mut blocking = info_finding("CAIRN_TEST", None, Some("src/lib.rs"));
        blocking.severity = severity;
        g.findings.push(blocking);
        let artefacts = todos_set(vec![defers_todo(
            "park-blocking",
            TodoStatus::Blocked,
            &[("CAIRN_TEST", "src/lib.rs")],
        )]);
        todo_defers::check_todo_defers(&mut g, &artefacts, std::path::Path::new("."));
        assert_eq!(g.findings.len(), 2, "{severity:?}");
        assert_eq!(
            g.findings[0].parked_by, None,
            "an Error or Warning stays selecting whatever any artefact declares"
        );
        let raised = &g.findings[1];
        assert_eq!(raised.code, "CAIRN_TODO_DEFERS_BLOCKING");
        assert_eq!(raised.severity, FindingSeverity::Warning);
        assert!(
            raised.message.contains(severity.name()),
            "message names the target severity: {}",
            raised.message
        );
    }
}

#[test]
fn test_todo_defers_mixed_match_blocking_voids_the_park() {
    // The Info sibling comes FIRST, so a single-pass implementation would
    // park it before discovering the blocking match; the contract is that a
    // reference hitting any Error or Warning parks nothing at all.
    let mut g = empty_graph();
    g.findings
        .push(info_finding("CAIRN_TEST", None, Some("src/lib.rs")));
    let mut warning = info_finding("CAIRN_TEST", None, Some("src/lib.rs"));
    warning.severity = FindingSeverity::Warning;
    g.findings.push(warning);
    let artefacts = todos_set(vec![defers_todo(
        "park-mixed",
        TodoStatus::Blocked,
        &[("CAIRN_TEST", "src/lib.rs")],
    )]);
    todo_defers::check_todo_defers(&mut g, &artefacts, std::path::Path::new("."));
    assert_eq!(g.findings.len(), 3);
    assert!(
        g.findings[..2].iter().all(|f| f.parked_by.is_none()),
        "a reference hitting a blocking finding must not park its Info sibling"
    );
    assert_eq!(g.findings[2].code, "CAIRN_TODO_DEFERS_BLOCKING");
}

#[test]
fn test_todo_defers_first_blocked_parker_wins() {
    let mut g = empty_graph();
    g.findings.push(info_finding(
        "CAIRN_SOURCE_UNVERIFIED",
        None,
        Some("meta/sources/a.md"),
    ));
    let artefacts = todos_set(vec![
        defers_todo(
            "park-first",
            TodoStatus::Blocked,
            &[("CAIRN_SOURCE_UNVERIFIED", "meta/sources/a.md")],
        ),
        defers_todo(
            "park-second",
            TodoStatus::Blocked,
            &[("CAIRN_SOURCE_UNVERIFIED", "meta/sources/a.md")],
        ),
    ]);
    todo_defers::check_todo_defers(&mut g, &artefacts, std::path::Path::new("."));
    assert_eq!(
        g.findings[0].parked_by.as_deref(),
        Some("todo.park-first"),
        "the first blocked parker in registry order wins deterministically"
    );
    assert_eq!(
        g.findings.len(),
        1,
        "the second todo's reference still matches, so it is not stale"
    );
}

#[test]
fn test_todo_defers_one_node_reference_parks_every_match() {
    // One reference, two Info findings on the referenced node: both park,
    // and nothing is raised, because the reference matched.
    let mut g = empty_graph();
    g.findings
        .push(info_finding("CAIRN_TEST", Some("app.api"), None));
    g.findings
        .push(info_finding("CAIRN_TEST", Some("app.api"), None));
    let artefacts = todos_set(vec![defers_todo(
        "park-node-pair",
        TodoStatus::Blocked,
        &[("CAIRN_TEST", "app.api")],
    )]);
    todo_defers::check_todo_defers(&mut g, &artefacts, std::path::Path::new("."));
    assert_eq!(g.findings.len(), 2);
    assert!(
        g.findings
            .iter()
            .all(|f| f.parked_by.as_deref() == Some("todo.park-node-pair")),
        "every Info match of one reference parks"
    );
}

#[test]
fn test_todo_defers_node_reference_parks_by_node_id() {
    let mut g = empty_graph();
    g.findings.push(info_finding(
        "CAIRN_DECISION_ACCUMULATION",
        Some("app.api"),
        None,
    ));
    let artefacts = todos_set(vec![defers_todo(
        "park-node",
        TodoStatus::Blocked,
        &[("CAIRN_DECISION_ACCUMULATION", "app.api")],
    )]);
    todo_defers::check_todo_defers(&mut g, &artefacts, std::path::Path::new("."));
    assert_eq!(g.findings.len(), 1);
    assert_eq!(g.findings[0].parked_by.as_deref(), Some("todo.park-node"));
}

#[test]
fn test_todo_defers_deferred_finding_stays_deferred_not_parked() {
    let mut g = empty_graph();
    let mut deferred = info_finding(
        "CAIRN_SPEC_RULE_UNIMPLEMENTED",
        None,
        Some("docs/registries/spec-rules.md"),
    );
    deferred.deferred_by = Some("dec.x".to_owned());
    g.findings.push(deferred);
    let artefacts = todos_set(vec![defers_todo(
        "park-deferred",
        TodoStatus::Blocked,
        &[(
            "CAIRN_SPEC_RULE_UNIMPLEMENTED",
            "docs/registries/spec-rules.md",
        )],
    )]);
    todo_defers::check_todo_defers(&mut g, &artefacts, std::path::Path::new("."));
    assert_eq!(
        g.findings.len(),
        1,
        "a matching reference to a deferred finding is neither stale nor blocking"
    );
    assert_eq!(
        g.findings[0].parked_by, None,
        "a decision-deferred finding stays under the deferral regime"
    );
    assert_eq!(g.findings[0].deferred_by.as_deref(), Some("dec.x"));
}

#[test]
fn test_todo_defers_prose_mention_parks_nothing() {
    let mut g = empty_graph();
    g.findings.push(info_finding(
        "CAIRN_SOURCE_UNVERIFIED",
        None,
        Some("meta/sources/a.md"),
    ));
    let mut todo = defers_todo("prose-only", TodoStatus::Blocked, &[]);
    todo.body = "This mentions CAIRN_SOURCE_UNVERIFIED meta/sources/a.md in prose.".to_owned();
    let artefacts = todos_set(vec![todo]);
    todo_defers::check_todo_defers(&mut g, &artefacts, std::path::Path::new("."));
    assert_eq!(g.findings.len(), 1);
    assert_eq!(
        g.findings[0].parked_by, None,
        "a fold rests on a typed reference, never on prose"
    );
}

#[test]
fn test_todo_defers_root_relative_normalisation_under_absolute_root() {
    let root = std::path::Path::new("/tmp/cairn-proj");
    let mut g = empty_graph();
    g.findings.push(info_finding(
        "CAIRN_SOURCE_UNVERIFIED",
        None,
        Some("/tmp/cairn-proj/meta/sources/a.md"),
    ));
    // A sibling of the root must not be treated as inside it.
    g.findings.push(info_finding(
        "CAIRN_SOURCE_UNVERIFIED",
        None,
        Some("/tmp/cairn-proj-other/meta/sources/b.md"),
    ));
    let artefacts = todos_set(vec![defers_todo(
        "park-sources",
        TodoStatus::Blocked,
        &[
            ("CAIRN_SOURCE_UNVERIFIED", "meta/sources/a.md"),
            ("CAIRN_SOURCE_UNVERIFIED", "meta/sources/b.md"),
        ],
    )]);
    todo_defers::check_todo_defers(&mut g, &artefacts, root);
    assert_eq!(
        g.findings[0].parked_by.as_deref(),
        Some("todo.park-sources"),
        "a root-joined finding path must match a root-relative reference"
    );
    assert_eq!(
        g.findings[1].parked_by, None,
        "a sibling directory of the root is outside the project"
    );
    assert_eq!(
        g.findings
            .iter()
            .filter(|finding| finding.code == "CAIRN_TODO_DEFERS_UNMATCHED")
            .count(),
        1,
        "the sibling reference matches nothing and must surface as stale"
    );
}
