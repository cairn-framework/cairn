//! Record outcomes, driven through the real scorer.

mod authoreval_support;

use authoreval_support::*;
use cairn::authoreval::{FailureClass, FailureSubclass, Outcome, RECORD_SCHEMA_VERSION};
use serde_json::json;

#[test]
fn test_first_shot_clean_run_records_no_hotspots() {
    let record = run(3, "first-shot", &[write_turn(VALID_DECISION)]);

    assert_eq!(record.schema_version, RECORD_SCHEMA_VERSION);
    assert_eq!(record.prompt_id, "first-shot");
    assert_eq!(record.outcome, Outcome::CleanFirstShot);
    assert_eq!(record.backend.kind, "replay");
    assert_eq!(record.backend.model, MODEL);
    assert_eq!(record.iterations, 1);
    assert!(record.first_shot_valid);
    assert!(
        record.hotspots.is_empty(),
        "a first-shot-clean run carries no hotspots"
    );
    assert!(record.error.is_none());
    assert_eq!(record.tokens.prompt, 100);
    assert_eq!(record.tokens.completion, 20);
    assert_eq!(record.tokens.total, 120);
}

#[test]
fn test_clean_after_repair_carries_the_last_failed_scan() {
    let record = run(
        3,
        "after-repair",
        &[write_turn(BROKEN_DECISION), write_turn(VALID_DECISION)],
    );

    assert_eq!(record.outcome, Outcome::CleanAfterRepair);
    assert_eq!(record.iterations, 2);
    assert!(!record.first_shot_valid);
    assert!(record.error.is_none());
    assert_eq!(record.tokens.total, 240, "both responses are paid for");

    let hotspot = record
        .hotspots
        .iter()
        .find(|hotspot| hotspot.code == "CAIRN_ARTEFACT_MISSING_FIELD")
        .expect("the failed attempt's finding must survive into the record");
    assert_eq!(hotspot.class, FailureClass::Syntax);
    assert_eq!(hotspot.subclass, FailureSubclass::Artefact);
    assert_eq!(hotspot.severity, "error");
    assert_eq!(hotspot.count, 1);
}

#[test]
fn test_exhausted_run_attributes_a_surviving_code_to_a_missing_affordance() {
    let record = run(
        1,
        "exhausted",
        &[write_turn(BROKEN_DECISION), write_turn(BROKEN_DECISION)],
    );

    assert_eq!(record.outcome, Outcome::RepairBoundExhausted);
    assert_eq!(record.iterations, 2, "one first shot plus one repair");
    assert!(!record.first_shot_valid);
    assert!(record.error.is_none());
    assert_eq!(
        record.tokens.total, 240,
        "an exhausted run reports the full cost it spent"
    );

    let hotspot = record
        .hotspots
        .iter()
        .find(|hotspot| hotspot.code == "CAIRN_ARTEFACT_MISSING_FIELD")
        .expect("the last failed scan's hotspots must be carried");
    assert_eq!(
        hotspot.class,
        FailureClass::MissingRepairAffordance,
        "a code that survived the feedback is a missing repair affordance"
    );
    assert_eq!(
        hotspot.subclass,
        FailureSubclass::Artefact,
        "the origin of the failure survives the reclassification"
    );
}

#[test]
fn test_a_first_shot_failure_is_never_a_missing_affordance() {
    let record = run(0, "no-repairs", &[write_turn(BROKEN_DECISION)]);

    assert_eq!(record.outcome, Outcome::RepairBoundExhausted);
    assert_eq!(record.iterations, 1);

    let hotspot = record
        .hotspots
        .iter()
        .find(|hotspot| hotspot.code == "CAIRN_ARTEFACT_MISSING_FIELD")
        .expect("the failed scan's hotspots must be carried");
    assert_eq!(
        hotspot.class,
        FailureClass::Syntax,
        "with no preceding scan there is nothing the feedback failed to clear"
    );
}

#[test]
fn test_backend_failure_records_the_classified_error_and_no_hotspots() {
    let record = run(
        3,
        "backend-timeout",
        &[json!({"kind": "failure", "class": "timeout", "detail": "no answer"})],
    );

    assert_eq!(record.outcome, Outcome::BackendFailure);
    assert_eq!(record.iterations, 1);
    assert!(!record.first_shot_valid);
    assert_eq!(record.tokens.total, 0);
    assert!(record.hotspots.is_empty());

    let error = record.error.expect("a backend failure must carry an error");
    assert_eq!(error.class, cairn::authoreval::BackendErrorClass::Timeout);
    assert!(error.detail.contains("timeout"));
}

#[test]
fn test_backend_failure_after_a_dirty_scan_still_carries_no_hotspots() {
    let record = run(
        3,
        "backend-late-failure",
        &[
            write_turn(BROKEN_DECISION),
            json!({"kind": "failure", "class": "invocation", "detail": "backend died"}),
        ],
    );

    assert_eq!(record.outcome, Outcome::BackendFailure);
    assert_eq!(record.iterations, 2);
    assert!(!record.first_shot_valid);
    assert_eq!(
        record.tokens.total, 120,
        "the one response that did arrive is still paid for"
    );
    assert!(
        record.hotspots.is_empty(),
        "an infrastructure failure must not be attributed to authoring quality"
    );
    let error = record.error.expect("error");
    assert_eq!(
        error.class,
        cairn::authoreval::BackendErrorClass::Invocation
    );
}

#[test]
fn test_persistence_looks_only_at_the_immediately_preceding_scan() {
    // A appears, is replaced by B, then comes back. A did not survive the
    // feedback that preceded it, so it must stay `syntax`; reading persistence
    // cumulatively would wrongly call it a missing repair affordance.
    let record = run(
        2,
        "reappearing-code",
        &[
            write_turn(BROKEN_DECISION),
            write_turn(ORPHANED_DECISION),
            write_turn(BROKEN_DECISION),
        ],
    );

    assert_eq!(record.outcome, Outcome::RepairBoundExhausted);
    assert_eq!(record.iterations, 3);

    let hotspot = record
        .hotspots
        .iter()
        .find(|hotspot| hotspot.code == "CAIRN_ARTEFACT_MISSING_FIELD")
        .expect("the last failed scan's hotspots must be carried");
    assert_eq!(
        hotspot.class,
        FailureClass::Syntax,
        "a code absent from the immediately preceding scan did not survive any feedback"
    );
    assert!(
        !record
            .hotspots
            .iter()
            .any(|hotspot| hotspot.code == "CAIRN_DECISION_ORPHANED"),
        "only the last failed scan is reported, not the union of every scan"
    );
}
