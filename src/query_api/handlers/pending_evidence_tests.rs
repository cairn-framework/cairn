//! Tests for pending review-evidence assembly.

use super::*;

fn decision(receipts: &[&str], affects: &[&str]) -> Decision {
    Decision {
        id: "dec.subject".to_owned(),
        path: "meta/decisions/subject.md".to_owned(),
        nodes: vec!["app".to_owned()],
        status: crate::artefacts::registry::DecisionStatus::Proposed,
        date: "2026-08-01".to_owned(),
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
        ratification: crate::artefacts::registry::RatificationTier::Local,
        affects: affects.iter().map(|path| (*path).to_owned()).collect(),
        ratified_by_machine: false,
        receipts: receipts.iter().map(|stem| (*stem).to_owned()).collect(),
    }
}

fn review(stem: &str, hash: Option<&str>) -> Review {
    Review {
        path: format!("meta/reviews/{stem}.md"),
        node: "app".to_owned(),
        review_type: crate::artefacts::registry::ReviewType::Human,
        date: "2026-08-01".to_owned(),
        reviewer: "Ada".to_owned(),
        subject_hash: hash.map(str::to_owned),
        lens_prompt_hash: None,
        related_change: None,
        body: String::new(),
    }
}

#[test]
fn mixed_current_and_stale_receipts_keep_the_changed_marker() {
    let reviews = [
        review("rev.current", Some("sha256:aaa")),
        review("rev.stale", Some("sha256:old")),
    ];
    let (evidence, changed) = assemble(
        std::path::Path::new("/repo"),
        &decision(&["rev.current", "rev.stale"], &[]),
        &reviews,
        Some("sha256:aaa"),
    );
    assert!(changed, "one stale receipt must keep the marker");
    assert_eq!(evidence.receipts[0].subject_hash_matches, Some(true));
    assert_eq!(evidence.receipts[1].subject_hash_matches, Some(false));
}

#[test]
fn unknown_hashes_render_unknown_and_never_mark_changed() {
    let reviews = [review("rev.nohash", None)];
    let (evidence, changed) = assemble(
        std::path::Path::new("/repo"),
        &decision(&["rev.nohash"], &[]),
        &reviews,
        Some("sha256:aaa"),
    );
    assert!(
        !changed,
        "an uncomparable receipt is not evidence of change"
    );
    assert_eq!(evidence.receipts[0].subject_hash_matches, None);

    let reviews = [review("rev.hashed", Some("sha256:old"))];
    let (evidence, changed) = assemble(
        std::path::Path::new("/repo"),
        &decision(&["rev.hashed"], &[]),
        &reviews,
        None,
    );
    assert!(!changed, "no current hash means no comparison");
    assert_eq!(evidence.receipts[0].subject_hash_matches, None);
}

#[test]
fn affects_linked_stale_review_marks_changed_without_receipts() {
    // Review.path is root-joined (absolute) while affects entries are
    // authored repo-relative: the canonical matcher must join them.
    let mut linked = review("rev.linked", Some("sha256:old"));
    linked.path = "/repo/meta/reviews/rev.linked.md".to_owned();
    let reviews = [linked];
    let (evidence, changed) = assemble(
        std::path::Path::new("/repo"),
        &decision(&[], &["meta/reviews/rev.linked.md"]),
        &reviews,
        Some("sha256:aaa"),
    );
    assert_eq!(evidence.receipts.len(), 1, "affects-linked review joins");
    assert!(changed, "affects-only stale review must keep the marker");
    assert_eq!(evidence.receipts[0].subject_hash_matches, Some(false));
}

#[test]
fn dangling_exact_review_pointer_seeds_an_unverified_row() {
    // The decision names a review file that does not exist: the queue
    // shows an unverified receipt instead of silently dropping it.
    let (evidence, changed) = assemble(
        std::path::Path::new("/repo"),
        &decision(&[], &["meta/reviews/rev.gone.md"]),
        &[],
        Some("sha256:aaa"),
    );
    assert_eq!(evidence.receipts.len(), 1, "dangling pointer seeds a row");
    assert_eq!(evidence.receipts[0].stem, "rev.gone");
    assert_eq!(evidence.receipts[0].reviewer, None);
    assert_eq!(evidence.receipts[0].verdict, None);
    assert_eq!(evidence.receipts[0].subject_hash_matches, None);
    assert!(!changed, "an absent review is unknown, not stale");
}

#[test]
fn affects_directory_rule_covers_reviews_beneath_it() {
    let mut linked = review("rev.dir", Some("sha256:aaa"));
    linked.path = "/repo/meta/reviews/rev.dir.md".to_owned();
    let reviews = [linked];
    let (evidence, changed) = assemble(
        std::path::Path::new("/repo"),
        &decision(&[], &["meta/reviews/"]),
        &reviews,
        Some("sha256:aaa"),
    );
    assert_eq!(
        evidence.receipts.len(),
        1,
        "directory rule joins the review"
    );
    assert!(
        !changed,
        "current hash under a directory rule stays unchanged"
    );
    assert_eq!(evidence.receipts[0].subject_hash_matches, Some(true));
}
