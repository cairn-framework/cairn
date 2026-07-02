// cairn:allow-large-module reason: registry validation test suite covering integrity, decision, provenance, source, and gap checks together; splitting by validator would duplicate the shared fixture helpers.
//! Tests for artefact registry validation.

#![allow(clippy::field_reassign_with_default)]
// Reason: two-step Default + field assignment is more readable in test fixtures
// than the single-expression struct-update alternative when multiple fields vary.
use std::{collections::BTreeSet, path::Path};

use super::{sha256::sha256_hex, *};

// ── fixtures ─────────────────────────────────────────────────────────────

fn node_ids(ids: &[&str]) -> BTreeSet<String> {
    ids.iter().map(ToString::to_string).collect()
}

fn make_todo(node: &str) -> Todo {
    Todo {
        path: "meta/todos/t.md".to_owned(),
        node: node.to_owned(),
        status: TodoStatus::Open,
        created: "2024-01-01".to_owned(),
        satisfies: None,
        body: String::new(),
    }
}

fn make_review(node: &str) -> Review {
    Review {
        path: "meta/reviews/r.md".to_owned(),
        node: node.to_owned(),
        review_type: ReviewType::Human,
        date: "2024-01-01".to_owned(),
        reviewer: "alice".to_owned(),
        related_change: None,
        body: String::new(),
    }
}

fn make_research(id: &str, nodes: &[&str], sources: &[&str]) -> Research {
    Research {
        id: id.to_owned(),
        path: format!("meta/research/{id}.md"),
        nodes: nodes.iter().map(ToString::to_string).collect(),
        date: "2024-01-01".to_owned(),
        sources: sources.iter().map(ToString::to_string).collect(),
        method: ResearchMethod::Secondary,
        tags: Vec::new(),
        body: String::new(),
    }
}

fn make_decision(id: &str, nodes: &[&str], status: DecisionStatus) -> Decision {
    Decision {
        id: id.to_owned(),
        path: format!("meta/decisions/{id}.md"),
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

fn make_source(id: &str, verification: SourceVerification, file: &str) -> Source {
    Source {
        id: id.to_owned(),
        path: format!("meta/sources/{id}.md"),
        file: file.to_owned(),
        sha256: None,
        verification,
        source_type: "paper".to_owned(),
        date: "2024-01-01".to_owned(),
        tags: Vec::new(),
        description: String::new(),
        body: String::new(),
    }
}

fn finding_codes(set: &ArtefactSet) -> Vec<&str> {
    set.findings.iter().map(|f| f.code.as_str()).collect()
}

// ── validate_nodes ────────────────────────────────────────────────────────

#[test]
fn test_todo_unknown_node_emits_orphan_warning() {
    let mut set = ArtefactSet::default();
    set.todos = vec![make_todo("app.ghost")];
    validate_nodes(&node_ids(&["app.real"]), &mut set);
    assert!(
        finding_codes(&set).contains(&"CAIRN_TODO_ORPHAN_NODE"),
        "todo referencing unknown node must emit CAIRN_TODO_ORPHAN_NODE; got: {:?}",
        finding_codes(&set)
    );
}

#[test]
fn test_review_unknown_node_emits_error() {
    let mut set = ArtefactSet::default();
    set.reviews = vec![make_review("app.ghost")];
    validate_nodes(&node_ids(&["app.real"]), &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_REVIEW_UNKNOWN_NODE"));
}

#[test]
fn test_research_empty_nodes_emits_missing_nodes_error() {
    let mut set = ArtefactSet::default();
    set.research = vec![make_research("r1", &[], &["src1"])];
    validate_nodes(&node_ids(&["app.real"]), &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_ARTEFACT_MISSING_NODES"));
}

#[test]
fn test_research_unknown_node_emits_unknown_node_error() {
    let mut set = ArtefactSet::default();
    set.research = vec![make_research("r1", &["app.ghost"], &["src1"])];
    validate_nodes(&node_ids(&["app.real"]), &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_ARTEFACT_UNKNOWN_NODE"));
}

#[test]
fn test_decision_empty_nodes_emits_missing_nodes_error() {
    let mut set = ArtefactSet::default();
    set.decisions = vec![make_decision("d1", &[], DecisionStatus::Accepted)];
    validate_nodes(&node_ids(&["app.real"]), &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_DECISION_MISSING_NODES"));
}

#[test]
fn test_decision_only_unknown_nodes_emits_orphaned_error() {
    let mut set = ArtefactSet::default();
    set.decisions = vec![make_decision(
        "d1",
        &["app.ghost"],
        DecisionStatus::Accepted,
    )];
    validate_nodes(&node_ids(&["app.real"]), &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_DECISION_ORPHANED"));
}

#[test]
fn test_decision_orphaned_flag_with_reason_suppresses_error() {
    let mut set = ArtefactSet::default();
    let mut d = make_decision("d1", &["app.ghost"], DecisionStatus::Accepted);
    d.orphaned = true;
    d.orphan_reason = Some("node was retired".to_owned());
    set.decisions = vec![d];
    validate_nodes(&node_ids(&["app.real"]), &mut set);
    assert!(
        !finding_codes(&set).contains(&"CAIRN_DECISION_ORPHANED"),
        "orphaned=true with reason must suppress CAIRN_DECISION_ORPHANED"
    );
}

#[test]
fn test_validate_nodes_happy_path_no_findings() {
    let mut set = ArtefactSet::default();
    set.todos = vec![make_todo("app.real")];
    set.reviews = vec![make_review("app.real")];
    set.research = vec![make_research("r1", &["app.real"], &[])];
    set.decisions = vec![make_decision("d1", &["app.real"], DecisionStatus::Accepted)];
    validate_nodes(&node_ids(&["app.real"]), &mut set);
    assert!(
        set.findings.is_empty(),
        "all known refs must produce no findings"
    );
}

// ── validate_decision_refs ────────────────────────────────────────────────

#[test]
fn test_decision_supersedes_unknown_decision_emits_warning() {
    let mut set = ArtefactSet::default();
    let mut d = make_decision("d1", &["app.real"], DecisionStatus::Accepted);
    d.supersedes = vec!["d.ghost".to_owned()];
    set.decisions = vec![d];
    let decisions = set
        .decisions
        .iter()
        .map(|d| (d.id.clone(), d.status))
        .collect();
    validate_decision_refs(&decisions, &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_DECISION_REFERENCE_UNKNOWN"));
}

#[test]
fn test_decision_supersedes_non_superseded_emits_status_warning() {
    let mut set = ArtefactSet::default();
    let target = make_decision("d.old", &["app.real"], DecisionStatus::Accepted);
    let mut d = make_decision("d.new", &["app.real"], DecisionStatus::Accepted);
    d.supersedes = vec!["d.old".to_owned()];
    set.decisions = vec![d, target];
    let decisions = set
        .decisions
        .iter()
        .map(|d| (d.id.clone(), d.status))
        .collect();
    validate_decision_refs(&decisions, &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_DECISION_SUPERSEDES_STATUS"));
}

#[test]
fn test_decision_supersedes_superseded_no_warning() {
    let mut set = ArtefactSet::default();
    let target = make_decision("d.old", &["app.real"], DecisionStatus::Superseded);
    let mut d = make_decision("d.new", &["app.real"], DecisionStatus::Accepted);
    d.supersedes = vec!["d.old".to_owned()];
    set.decisions = vec![d, target];
    let decisions = set
        .decisions
        .iter()
        .map(|d| (d.id.clone(), d.status))
        .collect();
    validate_decision_refs(&decisions, &mut set);
    assert!(
        !finding_codes(&set).contains(&"CAIRN_DECISION_SUPERSEDES_STATUS"),
        "superseding a Superseded decision must produce no warning"
    );
}

// ── validate_gaps ────────────────────────────────────────────────────────

#[test]
fn test_proposed_gap_emits_unresolved_warning() {
    let mut set = ArtefactSet::default();
    let mut d = make_decision(
        "dec.gap-app-real-what-now",
        &["app.real"],
        DecisionStatus::Proposed,
    );
    d.gap = true;
    set.decisions = vec![d];
    validate_gaps(&mut set);
    let finding = set
        .findings
        .iter()
        .find(|f| f.code == "CAIRN_GAP_UNRESOLVED")
        .expect("proposed gap must warn");
    assert_eq!(
        finding.severity,
        crate::map::graph::FindingSeverity::Warning
    );
    assert_eq!(finding.node.as_deref(), Some("app.real"));
}

#[test]
fn test_accepted_gap_emits_no_warning() {
    let mut set = ArtefactSet::default();
    let mut d = make_decision(
        "dec.gap-app-real-what-now",
        &["app.real"],
        DecisionStatus::Accepted,
    );
    d.gap = true;
    set.decisions = vec![d];
    validate_gaps(&mut set);
    assert!(
        !finding_codes(&set).contains(&"CAIRN_GAP_UNRESOLVED"),
        "accepting the gap decision must clear the warning"
    );
}

#[test]
fn test_non_gap_decision_emits_no_warning() {
    let mut set = ArtefactSet::default();
    set.decisions = vec![make_decision("d1", &["app.real"], DecisionStatus::Proposed)];
    validate_gaps(&mut set);
    assert!(!finding_codes(&set).contains(&"CAIRN_GAP_UNRESOLVED"));
}

// ── validate_provenance_refs ──────────────────────────────────────────────

#[test]
fn test_research_no_sources_emits_missing_sources_error() {
    let mut set = ArtefactSet::default();
    set.research = vec![make_research("r1", &["app.real"], &[])];
    let source_ids = BTreeSet::new();
    validate_provenance_refs(&BTreeSet::new(), &source_ids, &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_RESEARCH_MISSING_SOURCES"));
}

#[test]
fn test_research_primary_with_no_sources_is_clean() {
    let mut set = ArtefactSet::default();
    let mut research = make_research("r1", &["app.real"], &[]);
    research.method = ResearchMethod::Primary;
    set.research = vec![research];
    validate_provenance_refs(&BTreeSet::new(), &BTreeSet::new(), &mut set);
    assert!(!finding_codes(&set).contains(&"CAIRN_RESEARCH_MISSING_SOURCES"));
}

#[test]
fn test_research_unknown_source_emits_warning() {
    let mut set = ArtefactSet::default();
    set.research = vec![make_research("r1", &["app.real"], &["src.ghost"])];
    let source_ids = node_ids(&["src.real"]);
    validate_provenance_refs(&BTreeSet::new(), &source_ids, &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_RESEARCH_UNKNOWN_SOURCE"));
}

#[test]
fn test_decision_unknown_provenance_emits_warning() {
    let mut set = ArtefactSet::default();
    let mut d = make_decision("d1", &["app.real"], DecisionStatus::Accepted);
    d.informed_by = vec!["research.ghost".to_owned()];
    set.decisions = vec![d];
    validate_provenance_refs(&BTreeSet::new(), &BTreeSet::new(), &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_DECISION_UNKNOWN_PROVENANCE"));
}

#[test]
fn test_research_not_cited_by_decision_emits_info_orphan() {
    let mut set = ArtefactSet::default();
    set.research = vec![make_research("res.dangling", &["app.real"], &["src1"])];
    // A decision exists but does not cite the research.
    set.decisions = vec![make_decision("d1", &["app.real"], DecisionStatus::Accepted)];
    validate_provenance_refs(&BTreeSet::new(), &node_ids(&["src1"]), &mut set);
    let orphan = set
        .findings
        .iter()
        .find(|f| f.code == "CAIRN_RESEARCH_ORPHAN")
        .expect("uncited research must surface the orphan tension");
    // spec:441: orphan research surfaces at info-level, not warning/error.
    assert_eq!(orphan.severity, crate::map::graph::FindingSeverity::Info);
}

#[test]
fn test_research_cited_by_decision_is_not_orphan() {
    let mut set = ArtefactSet::default();
    set.research = vec![make_research("res.cited", &["app.real"], &["src1"])];
    let mut d = make_decision("d1", &["app.real"], DecisionStatus::Accepted);
    d.informed_by = vec!["res.cited".to_owned()];
    set.decisions = vec![d];
    validate_provenance_refs(&BTreeSet::new(), &node_ids(&["src1"]), &mut set);
    assert!(
        !finding_codes(&set).contains(&"CAIRN_RESEARCH_ORPHAN"),
        "research cited via informed_by must not be flagged"
    );
}

// ── validate_sources ──────────────────────────────────────────────────────

#[test]
fn test_unreferenced_source_emits_orphan_warning() {
    let mut set = ArtefactSet::default();
    set.sources = vec![make_source(
        "src1",
        SourceVerification::Unverified,
        "file.pdf",
    )];
    // No research or decisions reference src1.
    validate_sources(Path::new("/tmp"), &node_ids(&["src1"]), &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_ORPHAN"));
}

#[test]
fn test_external_source_non_url_emits_error() {
    let mut set = ArtefactSet::default();
    set.sources = vec![make_source(
        "src1",
        SourceVerification::External,
        "./local.pdf",
    )];
    // Reference it so it doesn't also emit CAIRN_SOURCE_ORPHAN.
    set.research = vec![make_research("r1", &["app.real"], &["src1"])];
    validate_sources(Path::new("/tmp"), &node_ids(&["src1"]), &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_EXTERNAL_URL"));
}

#[test]
fn test_external_source_valid_url_no_error() {
    let mut set = ArtefactSet::default();
    set.sources = vec![make_source(
        "src1",
        SourceVerification::External,
        "https://example.com/paper.pdf",
    )];
    set.research = vec![make_research("r1", &["app.real"], &["src1"])];
    validate_sources(Path::new("/tmp"), &node_ids(&["src1"]), &mut set);
    assert!(
        !finding_codes(&set).contains(&"CAIRN_SOURCE_EXTERNAL_URL"),
        "external source with https URL must not error"
    );
}

#[test]
fn test_unverified_source_emits_info() {
    let mut set = ArtefactSet::default();
    set.sources = vec![make_source(
        "src1",
        SourceVerification::Unverified,
        "file.pdf",
    )];
    set.research = vec![make_research("r1", &["app.real"], &["src1"])];
    validate_sources(Path::new("/tmp"), &node_ids(&["src1"]), &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_UNVERIFIED"));
}

#[test]
fn test_indexed_but_missing_source_emits_index_gap_warning() {
    let mut set = ArtefactSet::default();
    // source_ids contains "src.missing" but set.sources does not.
    validate_sources(Path::new("/tmp"), &node_ids(&["src.missing"]), &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_INDEX_GAP"));
}

// ── validate_verified_source ──────────────────────────────────────────────

#[test]
fn test_verified_source_without_sha256_emits_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("paper.pdf"), b"content").unwrap();
    let mut set = ArtefactSet::default();
    let source = make_source("src1", SourceVerification::Verified, "paper.pdf");
    // sha256 field is None — validated source must require it.
    validate_verified_source(dir.path(), &source, &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_SHA256_MISSING"));
}

#[test]
fn test_verified_source_wrong_sha256_emits_mismatch_error() {
    let dir = tempfile::tempdir().unwrap();
    let content = b"hello world";
    std::fs::write(dir.path().join("paper.pdf"), content).unwrap();
    let mut set = ArtefactSet::default();
    let mut source = make_source("src1", SourceVerification::Verified, "paper.pdf");
    source.sha256 =
        Some("0000000000000000000000000000000000000000000000000000000000000000".to_owned());
    validate_verified_source(dir.path(), &source, &mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_SOURCE_SHA256_MISMATCH"));
}

#[test]
fn test_verified_source_correct_sha256_no_finding() {
    let dir = tempfile::tempdir().unwrap();
    let content = b"hello world";
    std::fs::write(dir.path().join("paper.pdf"), content).unwrap();
    let mut set = ArtefactSet::default();
    let mut source = make_source("src1", SourceVerification::Verified, "paper.pdf");
    source.sha256 = Some(sha256_hex(content));
    validate_verified_source(dir.path(), &source, &mut set);
    assert!(
        set.findings.is_empty(),
        "correct sha256 must produce no findings; got: {:?}",
        set.findings
    );
}

// ── decision claim cross-check (CA004) ───────────────────────────────────

fn write_registry(dir: &Path, rows: &str) {
    let registry = dir.join("docs/registries");
    std::fs::create_dir_all(&registry).unwrap();
    let table = format!(
        "# Declared Items\n\n| ID | Item | Source | Status | Phase | Notes |\n|----|------|--------|--------|-------|-------|\n{rows}",
    );
    std::fs::write(registry.join("declared-items.md"), table).unwrap();
}

#[test]
fn test_claimed_closed_questions_requires_a_close_verb() {
    let with_verb = claimed_closed_questions("This decision closes Q-03 for good.");
    assert!(with_verb.contains("Q-03"), "close verb + Q-NN must match");

    let no_verb = claimed_closed_questions("See Q-04 for related context.");
    assert!(no_verb.is_empty(), "a bare Q-NN reference must not match");

    let multi = claimed_closed_questions("Resolves Q-01 and Q-02 together.");
    assert!(
        multi.contains("Q-01") && multi.contains("Q-02"),
        "all ids on a verb line match"
    );
}

#[test]
fn test_question_statuses_parses_status_column() {
    let registry = "| Q-01 | thing | 16.1 | open | Phase 2 | note |\n| Q-03 | other | 16.3 | resolved | mid | closed by dec.x |\n";
    let statuses = question_statuses(registry);
    assert_eq!(statuses.get("Q-01").map(String::as_str), Some("open"));
    assert_eq!(statuses.get("Q-03").map(String::as_str), Some("resolved"));
}

#[test]
fn test_decision_claim_unresolved_emits_finding() {
    let dir = tempfile::tempdir().unwrap();
    write_registry(
        dir.path(),
        "| Q-01 | thing | 16.1 | open | Phase 2 | note |\n",
    );
    let mut set = ArtefactSet::default();
    let mut decision = make_decision("d1", &[], DecisionStatus::Accepted);
    decision.body = "This decision closes Q-01.".to_owned();
    set.decisions = vec![decision];
    validate_decision_claims(dir.path(), &mut set);
    assert!(
        finding_codes(&set).contains(&"CAIRN_DECISION_CLAIM_UNRESOLVED"),
        "claiming to close an open question must warn; got: {:?}",
        set.findings
    );
}

#[test]
fn test_decision_claim_resolved_no_finding() {
    let dir = tempfile::tempdir().unwrap();
    write_registry(
        dir.path(),
        "| Q-01 | thing | 16.1 | resolved | mid | closed by dec.d1 |\n",
    );
    let mut set = ArtefactSet::default();
    let mut decision = make_decision("d1", &[], DecisionStatus::Accepted);
    decision.body = "This decision closes Q-01.".to_owned();
    set.decisions = vec![decision];
    validate_decision_claims(dir.path(), &mut set);
    assert!(
        !finding_codes(&set).contains(&"CAIRN_DECISION_CLAIM_UNRESOLVED"),
        "a registry-resolved question must not warn; got: {:?}",
        set.findings
    );
}

#[test]
fn test_decision_claim_absent_question_emits_finding() {
    let dir = tempfile::tempdir().unwrap();
    write_registry(
        dir.path(),
        "| Q-01 | thing | 16.1 | open | Phase 2 | note |\n",
    );
    let mut set = ArtefactSet::default();
    let mut decision = make_decision("d1", &[], DecisionStatus::Accepted);
    decision.body = "This decision resolves Q-99.".to_owned();
    set.decisions = vec![decision];
    validate_decision_claims(dir.path(), &mut set);
    assert!(
        finding_codes(&set).contains(&"CAIRN_DECISION_CLAIM_UNRESOLVED"),
        "claiming to close a question absent from the registry must warn; got: {:?}",
        set.findings
    );
}
