//! Unit tests for pending decision briefing extraction.

use super::*;

pub(super) fn decision(body: &str) -> Decision {
    Decision {
        id: "dec.test".to_owned(),
        path: "meta/decisions/test.md".to_owned(),
        nodes: vec!["app".to_owned()],
        status: crate::artefacts::registry::DecisionStatus::Proposed,
        ratification: crate::artefacts::registry::RatificationTier::Local,
        affects: Vec::new(),
        ratified_by_machine: false,
        receipts: Vec::new(),
        date: "2026-08-01".to_owned(),
        revisited: None,
        revisit_triggers: Vec::new(),
        informed_by: Vec::new(),
        supersedes: Vec::new(),
        refines: Vec::new(),
        refined_by: Vec::new(),
        superseded_by: Vec::new(),
        related: Vec::new(),
        orphaned: false,
        orphan_reason: None,
        gap: false,
        claims: None,
        body: body.to_owned(),
    }
}

fn review(body: &str, hash: Option<&str>) -> Review {
    Review {
        path: "meta/reviews/rev.one.md".to_owned(),
        node: "app".to_owned(),
        review_type: crate::artefacts::registry::ReviewType::Human,
        date: "2026-08-01".to_owned(),
        reviewer: "Ada".to_owned(),
        subject_hash: hash.map(str::to_owned),
        lens_prompt_hash: None,
        related_change: None,
        body: body.to_owned(),
    }
}

#[test]
fn parses_rubric_variants_and_summary() {
    let body = "# Test\n\n## Decision\n\nChoose the safer path. It keeps the queue clear.\n\n## The rubric, applied to this decision\n\n- **Tier**: `local`. One node.\n- **Unblocks**: the next step.\n  - the console\n- **Alignment**: against the mission.\n  - Goal 1: keep moving.\n- **Options considered**: (a) ship it; (b) defer it.\n";
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &decision(body),
        &[],
        None,
        true,
    );
    assert_eq!(
        parsed.ruling_summary.as_deref(),
        Some("Choose the safer path. It keeps the queue clear.")
    );
    let rubric = parsed.rubric.expect("rubric");
    assert_eq!(
        rubric.unblocks,
        Some(vec!["the next step.".to_owned(), "the console".to_owned()])
    );
    assert_eq!(
        rubric.alignment,
        Some(vec![
            "against the mission.".to_owned(),
            "Goal 1: keep moving.".to_owned()
        ])
    );
    assert_eq!(
        rubric.options,
        Some(vec!["(a) ship it".to_owned(), "(b) defer it".to_owned()])
    );
}

#[test]
fn missing_rubric_is_absent_without_panic() {
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &decision("# Test\n\n## Decision\n\nUse this."),
        &[],
        None,
        false,
    );
    assert!(parsed.rubric.is_none());
    assert_eq!(parsed.ruling_summary.as_deref(), Some("Use this."));
    assert!(parsed.evidence.is_none());
}

#[test]
fn parses_review_verdict_and_subject_match() {
    let mut item = decision("# Test\n\n## Ruling\n\nKeep it.");
    item.receipts = vec!["rev.one".to_owned()];
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &item,
        &[review(
            "# Review\n\n## Verdict\n\nAccepted.",
            Some("sha256:x"),
        )],
        Some("sha256:x"),
        true,
    );
    let receipt = &parsed.evidence.expect("evidence").receipts[0];
    assert_eq!(receipt.reviewer.as_deref(), Some("Ada"));
    assert_eq!(receipt.verdict.as_deref(), Some("Accepted."));
    assert_eq!(receipt.subject_hash_matches, Some(true));
    assert!(!parsed.changed_since_review);
}

#[test]
fn changed_since_review_when_no_receipt_matches() {
    let mut item = decision("# Test\n\n## Ruling\n\nKeep it.");
    item.receipts = vec!["rev.one".to_owned()];
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &item,
        &[review("## Verdict\n\nRejected.", Some("sha256:old"))],
        Some("sha256:new"),
        true,
    );
    assert!(parsed.changed_since_review);
}

#[test]
fn ruling_summary_skips_blank_heading_gap() {
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &decision("# Test\n\n## Ruling\n\n\nChoose this path."),
        &[],
        None,
        false,
    );
    assert_eq!(parsed.ruling_summary.as_deref(), Some("Choose this path."));
}

#[test]
fn ruling_summary_keeps_ids_and_paths_out_of_the_sentence_count() {
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &decision(
            "# Test\n\n## Decision\n\nAdopt dec.foo and src/lib.rs as the base. Keep dec.bar active. A third sentence falls off.\n",
        ),
        &[],
        None,
        false,
    );
    assert_eq!(
        parsed.ruling_summary.as_deref(),
        Some("Adopt dec.foo and src/lib.rs as the base. Keep dec.bar active.")
    );
}

#[test]
fn ruling_summary_counts_terminators_before_closing_quotes() {
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &decision(
            "# Test\n\n## Decision\n\nAdopt the rule (\"fix it away.\") Keep dec.bar active. A third sentence falls off.\n",
        ),
        &[],
        None,
        false,
    );
    assert_eq!(
        parsed.ruling_summary.as_deref(),
        Some("Adopt the rule (\"fix it away.\") Keep dec.bar active.")
    );
}

#[test]
fn ruling_summary_skips_an_h1_title_naming_decision() {
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &decision(
            "# Decision record for the queue\n\nLead prose that is not the ruling.\n\n## Decision\n\nAdopt the split.\n",
        ),
        &[],
        None,
        false,
    );
    assert_eq!(parsed.ruling_summary.as_deref(), Some("Adopt the split."));
}

#[test]
fn ruling_summary_reads_decision_not_a_leading_context_section() {
    // Live decisions open with `## Context` prose before `## Decision`; the
    // summary must quote the ruling, never the scene-setting.
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &decision(
            "# Test\n\n## Context\n\nThe repair landed earlier and work continued.\n\n## Decision\n\nAdopt the split corpora. Evidence stays unclaimed.\n",
        ),
        &[],
        None,
        false,
    );
    assert_eq!(
        parsed.ruling_summary.as_deref(),
        Some("Adopt the split corpora. Evidence stays unclaimed.")
    );
}

#[test]
fn options_keep_parenthetical_text_inside_recommendation() {
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &decision(
            "# Test\n\n## The rubric\n\n- **Options**: (a) keep it; (b) change it, which is the recommendation. The cost of no is option (b) rejected.\n",
        ),
        &[],
        None,
        false,
    );
    assert_eq!(
        parsed.rubric.expect("rubric").options,
        Some(vec![
            "(a) keep it".to_owned(),
            "(b) change it, which is the recommendation. The cost of no is option (b) rejected"
                .to_owned()
        ])
    );
}

#[test]
fn missing_review_receipt_renders_unknown_and_never_marks_changed() {
    // A named receipt whose review artefact is absent cannot be compared:
    // the row carries the unknown label instead of a false changed claim.
    let mut item = decision("# Test\n\n## Ruling\n\nKeep it.");
    item.receipts = vec!["rev.missing".to_owned()];
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &item,
        &[],
        Some("sha256:new"),
        true,
    );
    assert!(!parsed.changed_since_review);
    let evidence = parsed.evidence.expect("local tier keeps evidence");
    assert_eq!(evidence.receipts[0].subject_hash_matches, None);
}

#[test]
fn joins_wrapped_rubric_lines_and_keeps_nested_items() {
    let parsed = parse_pending_brief(
        std::path::Path::new("/repo"),
        &decision(
            "# Test\n\n## The rubric, applied to this decision\n\n- **Tier**: `local`. Mechanical\n  facts continue on the next line.\n- **Unblocks**: the first todo,\n  with more detail on the next line.\n  - the nested todo\n- **Alignment**: against the mission,\n  with a wrapped continuation.\n- **Options**: (a) keep it, with a wrapped\n  recommendation; (b) defer it.\n",
        ),
        &[],
        None,
        false,
    );
    let rubric = parsed.rubric.expect("rubric");
    assert_eq!(
        rubric.tier.as_deref(),
        Some("local. Mechanical facts continue on the next line.")
    );
    assert_eq!(
        rubric.unblocks,
        Some(vec![
            "the first todo, with more detail on the next line.".to_owned(),
            "the nested todo".to_owned()
        ])
    );
    assert_eq!(
        rubric.alignment,
        Some(vec![
            "against the mission, with a wrapped continuation.".to_owned()
        ])
    );
    assert_eq!(
        rubric.options,
        Some(vec![
            "(a) keep it, with a wrapped recommendation".to_owned(),
            "(b) defer it".to_owned()
        ])
    );
}
