// cairn:allow-large-module reason: registry validation test suite covering integrity, decision, provenance, source, and gap checks together; splitting by validator would duplicate the shared fixture helpers.
//! Tests for artefact registry validation.

#![allow(clippy::field_reassign_with_default)]
// Reason: two-step Default + field assignment is more readable in test fixtures
// than the single-expression struct-update alternative when multiple fields vary.
use std::{collections::BTreeSet, path::Path};

use super::filenames;
use super::sources::{
    validate_source_self_reference, validate_sources, validate_tracked_source,
    validate_verified_source,
};
use super::{sha256::sha256_hex, *};
use crate::map::FindingSeverity;

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
        defers: Vec::new(),
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
        subject_hash: None,
        lens_prompt_hash: None,
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
        ratification: crate::artefacts::registry::RatificationTier::Binding,
        affects: Vec::new(),
        ratified_by_machine: false,
        receipts: Vec::new(),
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

// ── validate_tracked_source ───────────────────────────────────────────────

#[test]
fn test_tracked_source_resolving_file_no_finding() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("notes.md"), b"live").unwrap();
    let mut set = ArtefactSet::default();
    let source = make_source("src1", SourceVerification::Tracked, "notes.md");
    validate_tracked_source(dir.path(), &source, &mut set);
    assert!(
        set.findings.is_empty(),
        "resolving tracked file must produce no findings; got: {:?}",
        set.findings
    );
}

#[test]
fn test_tracked_source_resolving_directory_no_finding() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("summariser")).unwrap();
    let mut set = ArtefactSet::default();
    // A directory path is what rules out fs::read as the probe.
    let source = make_source("src1", SourceVerification::Tracked, "summariser/");
    validate_tracked_source(dir.path(), &source, &mut set);
    assert!(
        set.findings.is_empty(),
        "resolving tracked directory must produce no findings; got: {:?}",
        set.findings
    );
}

#[test]
fn test_tracked_source_leading_curdir_accepted() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("notes.md"), b"live").unwrap();
    let mut set = ArtefactSet::default();
    let source = make_source("src1", SourceVerification::Tracked, "./notes.md");
    validate_tracked_source(dir.path(), &source, &mut set);
    assert!(
        set.findings.is_empty(),
        "leading ./ before a real path must be accepted; got: {:?}",
        set.findings
    );
}

#[test]
fn test_tracked_source_interior_curdir_normalised_accepted() {
    // The lexical rule is specified over `Path::components()`
    // (todo.source-tracked-verification-mode), which normalises an interior
    // `./` away: `notes/./sub.md` denotes `notes/sub.md`, cannot escape, and
    // still goes through resolution and containment.
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("notes")).unwrap();
    std::fs::write(dir.path().join("notes/sub.md"), b"live").unwrap();
    let mut set = ArtefactSet::default();
    let source = make_source("src1", SourceVerification::Tracked, "notes/./sub.md");
    validate_tracked_source(dir.path(), &source, &mut set);
    assert!(
        set.findings.is_empty(),
        "interior ./ normalises to the same contained path; got: {:?}",
        set.findings
    );
}

#[test]
fn test_tracked_source_missing_path_emits_read_failed() {
    let dir = tempfile::tempdir().unwrap();
    let mut set = ArtefactSet::default();
    let source = make_source("src1", SourceVerification::Tracked, "gone.md");
    validate_tracked_source(dir.path(), &source, &mut set);
    assert_eq!(finding_codes(&set), vec!["CAIRN_SOURCE_READ_FAILED"]);
    assert_eq!(set.findings[0].severity, FindingSeverity::Error);
}

#[test]
fn test_tracked_source_rejects_unsafe_paths_lexically() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("notes.md"), b"live").unwrap();
    // Absolute, parent traversal, and bare `./` are rejected before any
    // filesystem probe; the absolute form points INSIDE the root to prove
    // the rejection is lexical.
    let absolute = dir.path().join("notes.md").display().to_string();
    for file in [absolute.as_str(), "../notes.md", "./"] {
        let mut set = ArtefactSet::default();
        let source = make_source("src1", SourceVerification::Tracked, file);
        validate_tracked_source(dir.path(), &source, &mut set);
        assert_eq!(
            finding_codes(&set),
            vec!["CAIRN_SOURCE_READ_FAILED"],
            "`{file}` must be rejected"
        );
    }
}

#[cfg(unix)]
#[test]
fn test_tracked_source_symlink_escaping_root_emits_read_failed() {
    let outside = tempfile::tempdir().unwrap();
    std::fs::write(outside.path().join("target.md"), b"outside").unwrap();
    let dir = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(outside.path().join("target.md"), dir.path().join("link.md"))
        .unwrap();
    let mut set = ArtefactSet::default();
    let source = make_source("src1", SourceVerification::Tracked, "link.md");
    validate_tracked_source(dir.path(), &source, &mut set);
    assert_eq!(finding_codes(&set), vec!["CAIRN_SOURCE_READ_FAILED"]);
}

#[cfg(unix)]
#[test]
fn test_tracked_source_symlink_inside_root_no_finding() {
    // Containment is canonical, not a symlink ban: a link whose target stays
    // under the root resolves and passes.
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("target.md"), b"inside").unwrap();
    std::os::unix::fs::symlink(dir.path().join("target.md"), dir.path().join("link.md")).unwrap();
    let mut set = ArtefactSet::default();
    let source = make_source("src1", SourceVerification::Tracked, "link.md");
    validate_tracked_source(dir.path(), &source, &mut set);
    assert!(
        set.findings.is_empty(),
        "symlink resolving inside the root must pass; got: {:?}",
        set.findings
    );
}

#[cfg(unix)]
#[test]
fn test_tracked_source_special_file_emits_read_failed() {
    // A socket canonicalises and stays contained, but is neither a file nor
    // a directory; dec.source-tracked-verification clause 1 cites files and
    // directories only.
    let dir = tempfile::tempdir().unwrap();
    let _listener = std::os::unix::net::UnixListener::bind(dir.path().join("live.sock")).unwrap();
    let mut set = ArtefactSet::default();
    let source = make_source("src1", SourceVerification::Tracked, "live.sock");
    validate_tracked_source(dir.path(), &source, &mut set);
    assert_eq!(finding_codes(&set), vec!["CAIRN_SOURCE_READ_FAILED"]);
}

#[test]
fn test_tracked_source_sha256_emits_unexpected_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("notes.md"), b"live").unwrap();
    let mut set = ArtefactSet::default();
    let mut source = make_source("src1", SourceVerification::Tracked, "notes.md");
    source.sha256 = Some(sha256_hex(b"live"));
    validate_tracked_source(dir.path(), &source, &mut set);
    assert_eq!(finding_codes(&set), vec!["CAIRN_SOURCE_SHA256_UNEXPECTED"]);
    assert_eq!(set.findings[0].severity, FindingSeverity::Error);
}

#[test]
fn test_validate_sources_routes_tracked_arm() {
    let dir = tempfile::tempdir().unwrap();
    let mut set = ArtefactSet::default();
    set.sources = vec![make_source("src1", SourceVerification::Tracked, "gone.md")];
    // Reference it so it doesn't also emit CAIRN_SOURCE_ORPHAN.
    set.research = vec![make_research("r1", &["app.real"], &["src1"])];
    validate_sources(dir.path(), &node_ids(&[]), &mut set);
    assert_eq!(finding_codes(&set), vec!["CAIRN_SOURCE_READ_FAILED"]);
}

// ── source self-reference (CA044, dec.source-file-never-self) ────────────

/// A source whose artefact file really exists under `root`, with `path` set
/// the way the loader sets it: the walked, root-embedded path.
fn make_source_on_disk(
    root: &Path,
    id: &str,
    verification: SourceVerification,
    file: &str,
) -> Source {
    let dir = root.join("meta/sources");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("{id}.md"));
    std::fs::write(&path, b"---\n---\n").unwrap();
    let mut source = make_source(id, verification, file);
    source.path = path.to_string_lossy().into_owned();
    source
}

#[test]
fn test_source_self_reference_emits_warning_under_all_four_modes() {
    // Driven through validate_sources, not the helper, so the wiring is the
    // contract: the rule must not depend on the declared mode
    // (dec.source-file-never-self). A tracked self-pointer resolves and
    // stays inside the root, so the containment probe alone would pass it.
    for verification in [
        SourceVerification::Verified,
        SourceVerification::External,
        SourceVerification::Unverified,
        SourceVerification::Tracked,
    ] {
        let dir = tempfile::tempdir().unwrap();
        let mut set = ArtefactSet::default();
        set.sources = vec![make_source_on_disk(
            dir.path(),
            "src1",
            verification,
            "meta/sources/src1.md",
        )];
        // Reference it so the mode's own findings are the only extras.
        set.research = vec![make_research("r1", &["app.real"], &["src1"])];
        validate_sources(dir.path(), &node_ids(&[]), &mut set);
        let finding = set
            .findings
            .iter()
            .find(|f| f.code == "CAIRN_SOURCE_SELF_REFERENCE")
            .unwrap_or_else(|| panic!("{verification:?}: {:?}", set.findings));
        assert_eq!(
            finding.severity,
            FindingSeverity::Warning,
            "{verification:?}"
        );
    }
}

#[test]
fn test_source_self_reference_counts_respelled_paths() {
    // Both sides canonicalise, so the check is not dodged by respelling the
    // same path with `./` or a `..` detour.
    for file in ["./meta/sources/src1.md", "meta/sources/../sources/src1.md"] {
        let dir = tempfile::tempdir().unwrap();
        let mut set = ArtefactSet::default();
        let source = make_source_on_disk(dir.path(), "src1", SourceVerification::Unverified, file);
        validate_source_self_reference(dir.path(), &source, &mut set);
        assert_eq!(
            finding_codes(&set),
            vec!["CAIRN_SOURCE_SELF_REFERENCE"],
            "{file}"
        );
    }
}

#[cfg(unix)]
#[test]
fn test_source_self_reference_counts_symlink_alias() {
    // Canonicalisation also collapses a symlink alias back onto the record.
    let dir = tempfile::tempdir().unwrap();
    let mut set = ArtefactSet::default();
    let source = make_source_on_disk(
        dir.path(),
        "src1",
        SourceVerification::Unverified,
        "meta/sources/alias.md",
    );
    std::os::unix::fs::symlink(
        dir.path().join("meta/sources/src1.md"),
        dir.path().join("meta/sources/alias.md"),
    )
    .unwrap();
    validate_source_self_reference(dir.path(), &source, &mut set);
    assert_eq!(
        finding_codes(&set),
        vec!["CAIRN_SOURCE_SELF_REFERENCE"],
        "symlink alias"
    );
}

#[test]
fn test_source_self_reference_ignores_null_url_and_other_paths() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("paper.md"), b"evidence").unwrap();
    for file in ["null", "https://example.com/paper", "paper.md", "gone.md"] {
        let mut set = ArtefactSet::default();
        let source = make_source_on_disk(dir.path(), "src1", SourceVerification::Unverified, file);
        validate_source_self_reference(dir.path(), &source, &mut set);
        assert!(set.findings.is_empty(), "{file}: {:?}", set.findings);
    }
}

#[test]
fn test_validate_sources_routes_self_reference_beside_mode_dispatch() {
    // The self-reference warning is additive: the declared mode's own
    // finding (here the unverified Info) still stands beside it.
    let dir = tempfile::tempdir().unwrap();
    let mut set = ArtefactSet::default();
    set.sources = vec![make_source_on_disk(
        dir.path(),
        "src1",
        SourceVerification::Unverified,
        "meta/sources/src1.md",
    )];
    // Reference it so it doesn't also emit CAIRN_SOURCE_ORPHAN.
    set.research = vec![make_research("r1", &["app.real"], &["src1"])];
    validate_sources(dir.path(), &node_ids(&[]), &mut set);
    assert_eq!(
        finding_codes(&set),
        vec!["CAIRN_SOURCE_SELF_REFERENCE", "CAIRN_SOURCE_UNVERIFIED"]
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

// ── change tasks completion (CC004) ───────────────────────────────────────

fn make_change(id: &str, change_dir: &Path) -> ChangeRecord {
    ChangeRecord {
        id: id.to_owned(),
        path: change_dir.to_string_lossy().to_string(),
        title: id.to_owned(),
        proposal: String::new(),
        design: None,
    }
}

#[test]
fn test_change_all_tasks_complete_emits_info() {
    let dir = tempfile::tempdir().unwrap();
    let change_dir = dir.path().join("meta/changes/complete-change");
    std::fs::create_dir_all(&change_dir).unwrap();
    std::fs::write(
        change_dir.join("tasks.md"),
        "# Tasks\n\n- [x] design\n- [X] implement\n- [x] test\n",
    )
    .unwrap();

    let mut set = ArtefactSet::default();
    set.changes = vec![make_change("complete-change", &change_dir)];
    validate_changes(&mut set);

    let finding = set
        .findings
        .iter()
        .find(|f| f.code == "CAIRN_CHANGE_TASKS_COMPLETE")
        .expect("all tasks complete must emit CAIRN_CHANGE_TASKS_COMPLETE");
    assert_eq!(finding.severity, crate::map::graph::FindingSeverity::Info);
    assert_eq!(
        finding.path.as_deref(),
        Some(change_dir.join("tasks.md").to_string_lossy().as_ref())
    );
    assert!(
        finding.message.contains("complete-change"),
        "message must include change id: {}",
        finding.message
    );
    assert!(
        finding
            .message
            .contains("cairn change apply complete-change"),
        "message must suggest apply command: {}",
        finding.message
    );
}

#[test]
fn test_change_unchecked_task_emits_no_finding() {
    let dir = tempfile::tempdir().unwrap();
    let change_dir = dir.path().join("meta/changes/partial-change");
    std::fs::create_dir_all(&change_dir).unwrap();
    std::fs::write(
        change_dir.join("tasks.md"),
        "# Tasks\n\n- [x] design\n- [ ] implement\n",
    )
    .unwrap();

    let mut set = ArtefactSet::default();
    set.changes = vec![make_change("partial-change", &change_dir)];
    validate_changes(&mut set);

    assert!(
        !finding_codes(&set).contains(&"CAIRN_CHANGE_TASKS_COMPLETE"),
        "unchecked tasks must not emit finding; got: {:?}",
        set.findings
    );
}

#[test]
fn test_change_no_checkboxes_emits_no_finding() {
    let dir = tempfile::tempdir().unwrap();
    let change_dir = dir.path().join("meta/changes/empty-change");
    std::fs::create_dir_all(&change_dir).unwrap();
    std::fs::write(
        change_dir.join("tasks.md"),
        "# Tasks\n\nNo tasks recorded yet.\n",
    )
    .unwrap();

    let mut set = ArtefactSet::default();
    set.changes = vec![make_change("empty-change", &change_dir)];
    validate_changes(&mut set);

    assert!(
        !finding_codes(&set).contains(&"CAIRN_CHANGE_TASKS_COMPLETE"),
        "tasks.md with no checkboxes must not emit finding; got: {:?}",
        set.findings
    );
}

#[test]
fn test_change_fenced_unchecked_example_does_not_suppress_finding() {
    let dir = tempfile::tempdir().unwrap();
    let change_dir = dir.path().join("meta/changes/fenced-change");
    std::fs::create_dir_all(&change_dir).unwrap();
    std::fs::write(
        change_dir.join("tasks.md"),
        "# Tasks\n\n- [x] design\n\n```markdown\n- [ ] example only\n```\n",
    )
    .unwrap();

    let mut set = ArtefactSet::default();
    set.changes = vec![make_change("fenced-change", &change_dir)];
    validate_changes(&mut set);

    assert!(
        finding_codes(&set).contains(&"CAIRN_CHANGE_TASKS_COMPLETE"),
        "unchecked checkbox inside a code fence must not suppress the finding; got: {:?}",
        set.findings
    );
}

#[test]
fn test_change_only_fenced_checked_example_emits_no_finding() {
    let dir = tempfile::tempdir().unwrap();
    let change_dir = dir.path().join("meta/changes/fenced-only-change");
    std::fs::create_dir_all(&change_dir).unwrap();
    std::fs::write(
        change_dir.join("tasks.md"),
        "# Tasks\n\n```\n- [x] example only\n```\n",
    )
    .unwrap();

    let mut set = ArtefactSet::default();
    set.changes = vec![make_change("fenced-only-change", &change_dir)];
    validate_changes(&mut set);

    assert!(
        !finding_codes(&set).contains(&"CAIRN_CHANGE_TASKS_COMPLETE"),
        "checked checkbox only inside a code fence must not emit finding; got: {:?}",
        set.findings
    );
}

#[test]
fn test_change_unchecked_star_bullet_suppresses_finding() {
    let dir = tempfile::tempdir().unwrap();
    let change_dir = dir.path().join("meta/changes/mixed-bullet-change");
    std::fs::create_dir_all(&change_dir).unwrap();
    std::fs::write(
        change_dir.join("tasks.md"),
        "# Tasks\n\n- [x] finished\n* [ ] remaining\n",
    )
    .unwrap();

    let mut set = ArtefactSet::default();
    set.changes = vec![make_change("mixed-bullet-change", &change_dir)];
    validate_changes(&mut set);

    assert!(
        !finding_codes(&set).contains(&"CAIRN_CHANGE_TASKS_COMPLETE"),
        "unchecked star-bullet task must suppress the finding; got: {:?}",
        set.findings
    );
}

#[test]
fn test_change_missing_tasks_md_emits_no_finding() {
    let dir = tempfile::tempdir().unwrap();
    let change_dir = dir.path().join("meta/changes/no-tasks");
    std::fs::create_dir_all(&change_dir).unwrap();

    let mut set = ArtefactSet::default();
    set.changes = vec![make_change("no-tasks", &change_dir)];
    validate_changes(&mut set);

    assert!(
        !finding_codes(&set).contains(&"CAIRN_CHANGE_TASKS_COMPLETE"),
        "missing tasks.md must not emit finding; got: {:?}",
        set.findings
    );
}

// ── validate_filenames ────────────────────────────────────────────────────

/// `make_decision` derives its path from the id, so a `dec.`-prefixed id
/// produces exactly the legacy `meta/decisions/dec.<slug>.md` shape.
#[test]
fn test_typed_filename_prefix_emits_drift_warning() {
    let mut set = ArtefactSet::default();
    set.decisions = vec![make_decision(
        "dec.no-orchestrator",
        &[],
        DecisionStatus::Accepted,
    )];
    filenames::validate_filenames(&mut set);

    assert_eq!(set.findings.len(), 1, "got: {:?}", set.findings);
    let finding = &set.findings[0];
    assert_eq!(finding.code, "CAIRN_ARTEFACT_FILENAME_DRIFT");
    assert_eq!(finding.severity, FindingSeverity::Warning);
    assert_eq!(
        finding.path.as_deref(),
        Some("meta/decisions/dec.no-orchestrator.md")
    );
    assert!(
        finding.message.contains("`no-orchestrator.md`"),
        "message must name the expected filename; got: {}",
        finding.message
    );
}

#[test]
fn test_research_and_source_prefixes_each_emit_drift_warning() {
    let mut set = ArtefactSet::default();
    set.research = vec![make_research("res.probe", &["app.real"], &[])];
    set.sources = vec![make_source("src.gh-cli", SourceVerification::Verified, "u")];
    filenames::validate_filenames(&mut set);

    assert_eq!(
        finding_codes(&set),
        vec![
            "CAIRN_ARTEFACT_FILENAME_DRIFT",
            "CAIRN_ARTEFACT_FILENAME_DRIFT"
        ],
        "got: {:?}",
        set.findings
    );
}

/// Only the final extension is stripped, so a namespaced slug such as
/// `res.gas-city.analysis` conforms as `gas-city.analysis.md`.
#[test]
fn test_slug_only_filenames_including_namespaced_slugs_are_clean() {
    let mut set = ArtefactSet::default();
    let mut decision = make_decision("dec.no-orchestrator", &[], DecisionStatus::Accepted);
    decision.path = "meta/decisions/no-orchestrator.md".to_owned();
    let mut research = make_research("res.gas-city.analysis", &["app.real"], &[]);
    research.path = "meta/research/gas-city.analysis.md".to_owned();
    let mut source = make_source("src.gh-cli", SourceVerification::Verified, "u");
    source.path = "meta/sources/gh-cli.md".to_owned();
    set.decisions = vec![decision];
    set.research = vec![research];
    set.sources = vec![source];
    filenames::validate_filenames(&mut set);

    assert!(set.findings.is_empty(), "got: {:?}", set.findings);
}

/// The id-to-filename comparison catches a file named after the wrong slug,
/// which carries no typed prefix and so a prefix-only rule would miss.
#[test]
fn test_filename_naming_a_different_slug_emits_drift_warning() {
    let mut set = ArtefactSet::default();
    let mut decision = make_decision("dec.no-orchestrator", &[], DecisionStatus::Accepted);
    decision.path = "meta/decisions/orchestrator-policy.md".to_owned();
    set.decisions = vec![decision];
    filenames::validate_filenames(&mut set);

    assert_eq!(
        finding_codes(&set),
        vec!["CAIRN_ARTEFACT_FILENAME_DRIFT"],
        "got: {:?}",
        set.findings
    );
}

/// Nothing else checks that an id carries the prefix its kind requires, so a
/// malformed id must not be turned into rename advice: `dec.` alone would ask
/// for `.md`, and a decision declaring `id: res.foo` would be told to adopt a
/// typed filename. Both report the id instead.
#[test]
fn test_malformed_id_reports_the_id_and_never_suggests_a_filename() {
    for id in ["dec.", "res.foo", ""] {
        let mut set = ArtefactSet::default();
        let mut decision = make_decision(id, &[], DecisionStatus::Accepted);
        decision.path = "meta/decisions/foo.md".to_owned();
        set.decisions = vec![decision];
        filenames::validate_filenames(&mut set);

        assert_eq!(
            finding_codes(&set),
            vec!["CAIRN_ARTEFACT_FILENAME_DRIFT"],
            "id `{id}` must drift; got: {:?}",
            set.findings
        );
        assert!(
            set.findings[0]
                .message
                .contains("which is not `dec.` plus a slug"),
            "id `{id}` must be reported as malformed, not renamed; got: {}",
            set.findings[0].message
        );
    }
}

/// Todos invert the rule: `cairn todo new`/`todo set` resolve slugs through
/// `meta/todos/todo.<slug>.md`, so a bare slug is the drift.
#[test]
fn test_todo_without_prefix_emits_drift_warning_and_prefixed_todo_is_clean() {
    let mut set = ArtefactSet::default();
    set.todos = vec![make_todo("app.real")];
    filenames::validate_filenames(&mut set);
    assert_eq!(
        finding_codes(&set),
        vec!["CAIRN_ARTEFACT_FILENAME_DRIFT"],
        "bare `meta/todos/t.md` must drift; got: {:?}",
        set.findings
    );

    let mut clean = ArtefactSet::default();
    let mut todo = make_todo("app.real");
    todo.path = "meta/todos/todo.wire-format-schemas.md".to_owned();
    clean.todos = vec![todo];
    filenames::validate_filenames(&mut clean);
    assert!(clean.findings.is_empty(), "got: {:?}", clean.findings);
}

#[test]
fn test_receipt_link_unknown_receipt_emits_ca055() {
    let mut set = ArtefactSet::default();
    let mut decision = make_decision("dec.test", &["app"], DecisionStatus::Proposed);
    decision.receipts = vec!["rev.missing".to_owned()];
    set.decisions.push(decision);
    validate_receipt_links(&mut set);
    assert!(finding_codes(&set).contains(&"CAIRN_DECISION_RECEIPT_UNKNOWN"));
}
