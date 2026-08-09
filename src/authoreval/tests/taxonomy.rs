//! Failure-taxonomy classification.

use super::super::taxonomy::{FailureClass, FailureSubclass, classify};

#[test]
fn test_blueprint_codes_classify_as_syntax_blueprint() {
    for code in [
        "CAIRN_PARSE_UNTERMINATED_STRING",
        "CAIRN_INTEGRITY_DUPLICATE_ID",
        "CAIRN_TAG_UNREGISTERED",
    ] {
        assert_eq!(
            classify(code, false),
            (FailureClass::Syntax, FailureSubclass::Blueprint),
            "{code} must attribute to blueprint syntax"
        );
    }
}

#[test]
fn test_artefact_codes_classify_as_syntax_artefact() {
    for code in [
        "CAIRN_ARTEFACT_MISSING_FIELD",
        "CAIRN_DECISION_STATUS_INVALID",
        "CAIRN_CONTRACT_WRONG_NODE",
    ] {
        assert_eq!(
            classify(code, false),
            (FailureClass::Syntax, FailureSubclass::Artefact),
            "{code} must attribute to artefact syntax"
        );
    }
}

#[test]
fn test_reconcile_codes_classify_as_generated_guidance_graph() {
    assert_eq!(
        classify("CAIRN_RECONCILE_ORPHANED_FILE", false),
        (FailureClass::GeneratedGuidance, FailureSubclass::Graph)
    );
    assert_eq!(
        classify("CAIRN_PROVENANCE_NO_DECISION", false),
        (FailureClass::GeneratedGuidance, FailureSubclass::Graph)
    );
}

#[test]
fn test_unknown_code_stays_visible_as_generated_guidance_unknown() {
    assert_eq!(
        classify("CAIRN_SOMETHING_NOBODY_HAS_TABLED", false),
        (FailureClass::GeneratedGuidance, FailureSubclass::Unknown),
        "an untabled code must not mimic an unrelated class"
    );
}

#[test]
fn test_persistence_overrides_the_code_table_and_keeps_the_subclass() {
    assert_eq!(
        classify("CAIRN_ARTEFACT_MISSING_FIELD", true),
        (
            FailureClass::MissingRepairAffordance,
            FailureSubclass::Artefact
        ),
        "a code that survived repair feedback is a missing affordance, and its origin survives"
    );
    assert_eq!(
        classify("CAIRN_PARSE_UNTERMINATED_STRING", true),
        (
            FailureClass::MissingRepairAffordance,
            FailureSubclass::Blueprint
        )
    );
}
