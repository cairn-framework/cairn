//! Failure-taxonomy classification.

use super::super::scorer::Finding;
use super::super::taxonomy::{FailureClass, FailureSubclass, classify};

#[test]
fn test_blueprint_codes_classify_as_syntax_blueprint() {
    for code in [
        "CAIRN_PARSE_UNTERMINATED_STRING",
        "CAIRN_INTEGRITY_DUPLICATE_ID",
        "CAIRN_TAG_UNREGISTERED",
    ] {
        assert_eq!(
            classify(code, None, false),
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
            classify(code, None, false),
            (FailureClass::Syntax, FailureSubclass::Artefact),
            "{code} must attribute to artefact syntax"
        );
    }
}

#[test]
fn test_reconcile_codes_classify_as_generated_guidance_graph() {
    assert_eq!(
        classify("CAIRN_RECONCILE_ORPHANED_FILE", None, false),
        (FailureClass::GeneratedGuidance, FailureSubclass::Graph)
    );
    assert_eq!(
        classify("CAIRN_PROVENANCE_NO_DECISION", None, false),
        (FailureClass::GeneratedGuidance, FailureSubclass::Graph)
    );
}

#[test]
fn test_unknown_code_stays_visible_as_generated_guidance_unknown() {
    assert_eq!(
        classify("CAIRN_SOMETHING_NOBODY_HAS_TABLED", None, false),
        (FailureClass::GeneratedGuidance, FailureSubclass::Unknown),
        "an untabled code must not mimic an unrelated class"
    );
}

#[test]
fn test_persistence_overrides_the_code_table_and_keeps_the_subclass() {
    assert_eq!(
        classify("CAIRN_ARTEFACT_MISSING_FIELD", None, true),
        (
            FailureClass::MissingRepairAffordance,
            FailureSubclass::Artefact
        ),
        "a code that survived repair feedback is a missing affordance, and its origin survives"
    );
    assert_eq!(
        classify("CAIRN_PARSE_UNTERMINATED_STRING", None, true),
        (
            FailureClass::MissingRepairAffordance,
            FailureSubclass::Blueprint
        )
    );
}

#[test]
fn test_an_envelope_span_naming_the_blueprint_attributes_to_blueprint_syntax() {
    assert_eq!(
        classify("CAIRN_COMMAND_FAILED", Some("cairn.blueprint"), false),
        (FailureClass::Syntax, FailureSubclass::Blueprint),
        "an unparseable blueprint arrives under a generic code; its span is the only attribution"
    );
}

#[test]
fn test_an_envelope_span_elsewhere_stays_unknown() {
    assert_eq!(
        classify("CAIRN_COMMAND_FAILED", Some("cairn.config.yaml"), false),
        (FailureClass::GeneratedGuidance, FailureSubclass::Unknown),
        "only a blueprint span attributes to blueprint syntax; the rest keep the gap visible"
    );
}

#[test]
fn test_the_envelope_span_never_overrides_the_code_table() {
    assert_eq!(
        classify(
            "CAIRN_RECONCILE_ORPHANED_FILE",
            Some("cairn.blueprint"),
            false
        ),
        (FailureClass::GeneratedGuidance, FailureSubclass::Graph),
        "a tabled code keeps its subclass whatever span accompanies it"
    );
}

#[test]
fn test_a_wire_finding_offers_no_span_however_it_is_pathed() {
    let wire = Finding {
        path: Some("cairn.blueprint".to_owned()),
        message: "cairn.blueprint:1:1: something".to_owned(),
        ..Finding::default()
    };
    assert_eq!(
        wire.envelope_parse_span(),
        None,
        "a path the wire published is not an attribution; the table's gap must stay visible"
    );
    assert_eq!(
        classify(
            "CAIRN_SOMETHING_NOBODY_HAS_TABLED",
            wire.envelope_parse_span(),
            false
        ),
        (FailureClass::GeneratedGuidance, FailureSubclass::Unknown)
    );
}

#[test]
fn test_an_envelope_attributes_only_when_it_reports_a_position() {
    let positioned = Finding {
        from_envelope: true,
        path: Some("cairn.blueprint".to_owned()),
        message: "cairn.blueprint:57:101: expected `{`, encountered word `path`".to_owned(),
        ..Finding::default()
    };
    assert_eq!(
        positioned.envelope_parse_span(),
        Some("cairn.blueprint"),
        "a parse failure reports `<span>:line:col`"
    );

    // `lint` labels every project load failure with the blueprint path,
    // including a state snapshot the blueprint had no part in. Attributing
    // that to blueprint syntax would be a fabricated measurement.
    let unpositioned = Finding {
        from_envelope: true,
        path: Some("cairn.blueprint".to_owned()),
        message: "failed to read the blueprint snapshot: invalid json".to_owned(),
        ..Finding::default()
    };
    assert_eq!(unpositioned.envelope_parse_span(), None);
    assert_eq!(
        classify(
            "CAIRN_COMMAND_FAILED",
            unpositioned.envelope_parse_span(),
            false
        ),
        (FailureClass::GeneratedGuidance, FailureSubclass::Unknown),
        "an unpositioned load failure keeps the gap visible rather than claiming blueprint syntax"
    );
}

#[test]
fn test_a_message_prefixed_by_the_span_still_needs_a_numeric_position() {
    // The prefix alone is not the parse shape: a load failure that happens to
    // lead with the blueprint path would otherwise claim blueprint syntax.
    for message in [
        "cairn.blueprint: could not be read",
        "cairn.blueprint:snapshot:invalid: state is corrupt",
        "cairn.blueprint:57: truncated position",
    ] {
        let envelope = Finding {
            from_envelope: true,
            path: Some("cairn.blueprint".to_owned()),
            message: message.to_owned(),
            ..Finding::default()
        };
        assert_eq!(
            envelope.envelope_parse_span(),
            None,
            "`{message}` reports no decimal position and must not attribute"
        );
    }
}
