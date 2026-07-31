//! Tests for ratification-tier scanner checks.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    artefacts::registry::{
        ArtefactSet, Decision, DecisionStatus, RatificationTier, Review, ReviewType,
        manifest::{compute_subject_hash, contained_file_sha256},
    },
    blueprint::{NodeKind, Span},
    map::graph::{FindingSeverity, Graph, NodeRecord, NodeState},
};

use super::convergence::{clean_verdict, convergence_leg, machine_debate_record};
use super::{check_ratification, load_allowlist};

mod checks;

#[test]
fn test_clean_verdict_pass_is_clean() {
    assert!(clean_verdict("Context\n## Verdict\n\nPASS: reviewed"));
}

#[test]
fn test_clean_verdict_blocking_is_not_clean() {
    assert!(!clean_verdict("## Verdict\nBLOCKING: revise"));
}

#[test]
fn test_clean_verdict_lowercase_token_is_not_clean() {
    assert!(!clean_verdict("## Verdict\npass"));
}

#[test]
fn test_clean_verdict_missing_heading_is_not_clean() {
    assert!(!clean_verdict("PASS"));
}

#[test]
fn test_clean_verdict_empty_section_is_not_clean() {
    assert!(!clean_verdict("## Verdict\n\n"));
}

#[test]
fn test_clean_verdict_trailing_prose_after_pass_is_clean() {
    assert!(clean_verdict(
        "## Verdict\nPASS\nThe reviewer found no blockers."
    ));
}

#[test]
fn test_clean_verdict_first_of_multiple_headings_controls_result() {
    assert!(!clean_verdict("## Verdict\nBLOCKING\n## Verdict\nPASS"));
}

#[test]
fn test_machine_debate_record_ordered_non_empty_sections_pass() {
    assert!(
        machine_debate_record(
            "## For\nSupports the decision.\n## Against\nCosts remain.\n## Verdict\nProceed."
        )
        .is_ok()
    );
}

#[test]
fn test_machine_debate_record_missing_section_fails() {
    assert_eq!(
        machine_debate_record("## For\nSupports the decision.\n## Verdict\nProceed."),
        Err("machine debate record missing ## Against section")
    );
}

#[test]
fn test_machine_debate_record_empty_section_fails() {
    assert_eq!(
        machine_debate_record("## For\n\n## Against\nCosts remain.\n## Verdict\nProceed."),
        Err("machine debate record has empty ## For section")
    );
}

#[test]
fn test_machine_debate_record_wrong_order_fails() {
    assert_eq!(
        machine_debate_record(
            "## Against\nCosts remain.\n## For\nSupports the decision.\n## Verdict\nProceed."
        ),
        Err("machine debate record sections are not in required order")
    );
}

#[test]
fn test_load_allowlist_missing_file_fails_closed() {
    let root = temp_root("missing-allowlist");
    assert!(load_allowlist(&root, root.canonicalize().ok().as_deref()).is_err());
}

#[test]
fn test_load_allowlist_zero_rows_fails_closed() {
    let root = temp_root("empty-allowlist");
    fs::create_dir_all(root.join("docs/registries")).expect("create registries dir");
    fs::write(root.join("docs/registries/binding-surface.md"), "# Empty\n")
        .expect("write allowlist");
    assert!(load_allowlist(&root, root.canonicalize().ok().as_deref()).is_err());
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn test_convergence_leg_two_clean_independent_receipts_passes() {
    let root = PathBuf::from("/tmp/ratification-happy");
    let mut decision = decision();
    decision.affects = vec!["meta/reviews/".to_owned()];
    let artefacts = reviews(
        review(&root, "rev.one", "model-a/correctness"),
        review(&root, "rev.two", "model-b/security"),
    );
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:subject"),
        None
    );
}

#[test]
fn test_convergence_leg_one_receipt_reports_shortfall() {
    let root = PathBuf::from("/tmp/ratification-shortfall");
    let mut decision = decision();
    decision.receipts.pop();
    let artefacts = reviews(
        review(&root, "rev.one", "model-a/correctness"),
        review(&root, "rev.two", "model-b/security"),
    );
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:subject"),
        Some("fewer than two resolved receipts".to_owned())
    );
}

#[test]
fn test_convergence_leg_missing_subject_hash_reports_receipt_grade() {
    let root = PathBuf::from("/tmp/ratification-grade");
    let decision = decision();
    let mut first = review(&root, "rev.one", "model-a/correctness");
    first.subject_hash = None;
    let artefacts = reviews(first, review(&root, "rev.two", "model-b/security"));
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:subject"),
        Some("receipt is not receipt-grade".to_owned())
    );
}

#[test]
fn test_convergence_leg_wrong_review_type_reports_type() {
    let root = PathBuf::from("/tmp/ratification-type");
    let decision = decision();
    let mut first = review(&root, "rev.one", "model-a/correctness");
    first.review_type = ReviewType::Human;
    let artefacts = reviews(first, review(&root, "rev.two", "model-b/security"));
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:subject"),
        Some("receipt review_type is not agent_cross_model".to_owned())
    );
}

#[test]
fn test_convergence_leg_blocking_verdict_reports_verdict() {
    let root = PathBuf::from("/tmp/ratification-verdict");
    let decision = decision();
    let mut first = review(&root, "rev.one", "model-a/correctness");
    first.body = "## Verdict\nBLOCKING".to_owned();
    let artefacts = reviews(first, review(&root, "rev.two", "model-b/security"));
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:subject"),
        Some("receipt verdict is not a clean PASS".to_owned())
    );
}

#[test]
fn test_convergence_leg_duplicate_identity_reports_duplicate() {
    let root = PathBuf::from("/tmp/ratification-identity");
    let decision = decision();
    let first = review(&root, "rev.one", "model-a/correctness");
    let mut second = review(&root, "rev.two", "model-a/correctness");
    second.lens_prompt_hash = Some(format!("sha256:{}", "f".repeat(64)));
    let artefacts = reviews(first, second);
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:subject"),
        Some("fewer than two independent reviewer identities".to_owned())
    );
}

#[test]
fn test_convergence_leg_missing_lens_prompt_reports_missing_file() {
    let root = temp_root("missing-lens-prompt");
    let decision = decision();
    let first = review(&root, "rev.one", "model-a/correctness");
    fs::remove_file(root.join("docs/agent/lenses/correctness.md")).expect("remove lens prompt");
    let artefacts = reviews(first, review(&root, "rev.two", "model-b/security"));
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:subject"),
        Some(
            "lens prompt file `docs/agent/lenses/correctness.md` is missing for reviewer `model-a/correctness`"
                .to_owned()
        )
    );
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn test_convergence_leg_lens_prompt_hash_mismatch_reports_mismatch() {
    let root = temp_root("lens-prompt-mismatch");
    let decision = decision();
    let mut first = review(&root, "rev.one", "model-a/correctness");
    first.lens_prompt_hash = Some(format!("sha256:{}", "f".repeat(64)));
    let artefacts = reviews(first, review(&root, "rev.two", "model-b/security"));
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:subject"),
        Some(
            "lens prompt hash mismatch for reviewer `model-a/correctness` at `docs/agent/lenses/correctness.md`"
                .to_owned()
        )
    );
    fs::remove_dir_all(root).expect("cleanup");
}
#[test]
fn test_convergence_leg_different_hashes_reports_mismatch() {
    let root = PathBuf::from("/tmp/ratification-hashes");
    let decision = decision();
    let first = review(&root, "rev.one", "model-a/correctness");
    let mut second = review(&root, "rev.two", "model-b/security");
    second.subject_hash = Some("sha256:other".to_owned());
    let artefacts = reviews(first, second);
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:subject"),
        Some("receipt subject_hash values are not all equal".to_owned())
    );
}

#[test]
fn test_convergence_leg_old_manifest_reports_recomputed_mismatch() {
    let root = PathBuf::from("/tmp/ratification-manifest");
    let decision = decision();
    let artefacts = reviews(
        review(&root, "rev.one", "model-a/correctness"),
        review(&root, "rev.two", "model-b/security"),
    );
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:new-subject"),
        Some("receipt subject_hash does not equal the recomputed manifest".to_owned())
    );
}

#[test]
fn test_convergence_leg_uncovered_receipt_reports_path() {
    let root = PathBuf::from("/tmp/ratification-coverage");
    let mut decision = decision();
    decision.affects = vec!["src/".to_owned()];
    let artefacts = reviews(
        review(&root, "rev.one", "model-a/correctness"),
        review(&root, "rev.two", "model-b/security"),
    );
    assert_eq!(
        convergence_leg(&root, &decision, &artefacts, "sha256:subject"),
        Some("receipt path is not covered by affects".to_owned())
    );
}

fn decision() -> Decision {
    Decision {
        id: "dec.local".to_owned(),
        path: "meta/decisions/dec.local.md".to_owned(),
        nodes: vec!["app.module".to_owned()],
        status: DecisionStatus::Accepted,
        ratification: RatificationTier::Local,
        affects: Vec::new(),
        ratified_by_machine: false,
        receipts: vec!["rev.one".to_owned(), "rev.two".to_owned()],
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
        body: String::new(),
    }
}

fn reviews(first: Review, second: Review) -> crate::artefacts::registry::ArtefactSet {
    crate::artefacts::registry::ArtefactSet {
        reviews: vec![first, second],
        ..Default::default()
    }
}

fn review(root: &Path, stem: &str, reviewer: &str) -> Review {
    let lens_id = reviewer.rsplit('/').next().expect("lens identifier");
    let lens_path = root.join("docs/agent/lenses").join(format!("{lens_id}.md"));
    fs::create_dir_all(lens_path.parent().expect("lens directory")).expect("create lens directory");
    fs::write(&lens_path, format!("# {lens_id}\n")).expect("write lens prompt");
    let lens_prompt_hash = contained_file_sha256(root, &format!("docs/agent/lenses/{lens_id}.md"))
        .expect("hash lens prompt");
    Review {
        path: root
            .join("meta/reviews")
            .join(format!("{stem}.md"))
            .to_string_lossy()
            .into_owned(),
        node: "app.module".to_owned(),
        review_type: ReviewType::AgentCrossModel,
        date: "2026-01-01".to_owned(),
        reviewer: reviewer.to_owned(),
        subject_hash: Some("sha256:subject".to_owned()),
        lens_prompt_hash: Some(lens_prompt_hash),
        related_change: None,
        body: "## Verdict\nPASS".to_owned(),
    }
}

fn temp_root(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("cairn-ratification-{name}-{nonce}"))
}

#[test]
fn test_load_allowlist_mixed_valid_and_invalid_row_fails_closed() {
    let root = allowlist_root("bad-allowlist");
    fs::write(root.join("docs/registries/binding-surface.md"), "- docs/spec.md\n- docs/registries/\n- tools/agent-pack/content/\n- src/artefacts/registry/\n- ../escape\n").expect("malformed allowlist");
    let error = load_allowlist(&root, root.canonicalize().ok().as_deref())
        .expect_err("malformed row must fail closed");
    assert!(error.contains("../escape"));
    fs::remove_dir_all(root).expect("cleanup");
}

#[cfg(unix)]
#[test]
fn test_load_allowlist_escaping_symlink_rule_fails_closed() {
    use std::os::unix::fs::symlink;

    let root = temp_root("escaping-allowlist");
    fs::create_dir_all(root.join("docs/registries")).expect("registries directory");
    let outside = tempfile::tempdir().expect("outside directory");
    symlink(outside.path(), root.join("outside")).expect("escaping symlink");
    fs::write(
        root.join("docs/registries/binding-surface.md"),
        "- outside/\n",
    )
    .expect("allowlist");
    let error = load_allowlist(&root, root.canonicalize().ok().as_deref())
        .expect_err("escaping rule must fail closed");
    assert!(error.contains("outside/"));
    fs::remove_dir_all(root).expect("cleanup");
}

fn allowlist_root(name: &str) -> PathBuf {
    let root = temp_root(name);
    fs::create_dir_all(root.join("docs/registries")).expect("registries directory");
    fs::write(root.join("docs/registries/binding-surface.md"), "- docs/spec.md\n- docs/registries/\n- tools/agent-pack/content/\n- src/artefacts/registry/\n- cairn.blueprint\n").expect("allowlist");
    root
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

fn graph_with_containers() -> Graph {
    let mut graph = empty_graph();
    for (id, kind) in [
        ("one", NodeKind::Container),
        ("two", NodeKind::Container),
        ("one.child", NodeKind::Module),
        ("two.child", NodeKind::Module),
    ] {
        graph.nodes.insert(
            id.to_owned(),
            NodeRecord {
                kind,
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
            },
        );
    }
    graph
}

fn receipt(root: &Path, stem: &str, reviewer: &str, hash: &str) -> Review {
    let mut item = review(root, stem, reviewer);
    item.subject_hash = Some(hash.to_owned());
    item
}

fn findings(graph: &Graph, code: &str) -> usize {
    graph
        .findings
        .iter()
        .filter(|finding| finding.code == code && finding.severity == FindingSeverity::Error)
        .count()
}
