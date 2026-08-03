//! Tests for the pending queue handler.

use super::*;
use crate::artefacts::registry::dates::days_from_civil;

fn decision(id: &str, status: DecisionStatus, date: &str, nodes: &[&str]) -> Decision {
    Decision {
        id: id.to_owned(),
        path: format!("meta/decisions/{id}.md"),
        nodes: nodes.iter().map(|node| (*node).to_owned()).collect(),
        status,
        date: date.to_owned(),
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
        body: String::new(),
        ratification: crate::artefacts::registry::RatificationTier::Binding,
        affects: Vec::new(),
        ratified_by_machine: false,
        receipts: Vec::new(),
    }
}

#[test]
fn test_pending_lists_only_proposed_oldest_first() {
    let today = days_from_civil(2026, 7, 30);
    let decisions = [
        decision("dec.newer", DecisionStatus::Proposed, "2026-07-20", &["a"]),
        decision(
            "dec.accepted",
            DecisionStatus::Accepted,
            "2026-01-01",
            &["a"],
        ),
        decision(
            "dec.older",
            DecisionStatus::Proposed,
            "2026-07-01",
            &["a", "b"],
        ),
        decision(
            "dec.superseded",
            DecisionStatus::Superseded,
            "2026-01-01",
            &["a"],
        ),
        decision(
            "dec.deprecated",
            DecisionStatus::Deprecated,
            "2026-01-01",
            &["a"],
        ),
    ];
    let response = pending_response(std::path::Path::new("/"), &decisions, &[], today).unwrap();
    let ids: Vec<&str> = response.pending.iter().map(|row| row.id.as_str()).collect();
    assert_eq!(ids, ["dec.older", "dec.newer"]);
    assert_eq!(response.pending[0].age_days, 29);
    assert_eq!(response.pending[0].nodes, ["a", "b"]);
    assert_eq!(response.pending[1].age_days, 10);
    for row in &response.pending {
        assert_eq!(row.ratification, PendingTier::Binding);
        assert_eq!(row.subject_hash, None);
        assert_eq!(row.subject_hash_error, None);
    }
    let wire = serde_json::to_value(&response).unwrap();
    assert_eq!(wire["pending"][0]["subject_hash"], serde_json::Value::Null);
    assert!(wire["pending"][0].get("subject_hash_error").is_none());
}

#[test]
fn test_pending_age_ties_break_by_id_ascending() {
    let today = days_from_civil(2026, 7, 30);
    let decisions = [
        decision("dec.zeta", DecisionStatus::Proposed, "2026-07-10", &["a"]),
        decision("dec.alpha", DecisionStatus::Proposed, "2026-07-10", &["a"]),
    ];
    let response = pending_response(std::path::Path::new("/"), &decisions, &[], today).unwrap();
    let ids: Vec<&str> = response.pending.iter().map(|row| row.id.as_str()).collect();
    assert_eq!(ids, ["dec.alpha", "dec.zeta"]);
}

#[test]
fn test_pending_future_date_yields_negative_age_and_sorts_last() {
    let today = days_from_civil(2026, 7, 30);
    let decisions = [
        decision("dec.future", DecisionStatus::Proposed, "2026-08-04", &["a"]),
        decision("dec.past", DecisionStatus::Proposed, "2026-07-25", &["a"]),
    ];
    let response = pending_response(std::path::Path::new("/"), &decisions, &[], today).unwrap();
    let ids: Vec<&str> = response.pending.iter().map(|row| row.id.as_str()).collect();
    assert_eq!(ids, ["dec.past", "dec.future"]);
    assert_eq!(response.pending[1].age_days, -5);
}

#[test]
fn test_pending_invalid_date_is_a_deterministic_error() {
    let decisions = [decision(
        "dec.bad",
        DecisionStatus::Proposed,
        "not-a-date!",
        &["a"],
    )];
    let error = pending_response(std::path::Path::new("/"), &decisions, &[], 0).unwrap_err();
    assert_eq!(error.code, "CAIRN_PENDING_INVALID_DATE");
    assert!(error.message.contains("dec.bad"), "{}", error.message);
    assert!(error.message.contains("not-a-date!"), "{}", error.message);
    assert!(error.remediation.is_some());
}

#[test]
fn test_pending_ignores_invalid_dates_on_non_proposed_decisions() {
    let decisions = [
        decision("dec.done", DecisionStatus::Accepted, "garbage-date", &["a"]),
        decision("dec.live", DecisionStatus::Proposed, "2026-07-01", &["a"]),
    ];
    let response = pending_response(
        std::path::Path::new("/"),
        &decisions,
        &[],
        days_from_civil(2026, 7, 30),
    )
    .unwrap();
    assert_eq!(response.pending.len(), 1);
    assert_eq!(response.pending[0].id, "dec.live");
}

#[test]
fn test_pending_local_tier_includes_subject_hash() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(directory.path().join("meta/decisions")).unwrap();
    let raw = "---\nid: dec.local\nstatus: proposed\nratification: local\ndate: 2026-07-01\n---\n# Local\n";
    std::fs::write(directory.path().join("meta/decisions/dec.local.md"), raw).unwrap();
    let mut local = decision("dec.local", DecisionStatus::Proposed, "2026-07-01", &["a"]);
    local.ratification = crate::artefacts::registry::RatificationTier::Local;
    let response = pending_response(
        directory.path(),
        &[local],
        &[],
        days_from_civil(2026, 7, 30),
    )
    .unwrap();
    assert_eq!(response.pending[0].ratification, PendingTier::Local);
    assert_eq!(
        response.pending[0].subject_hash,
        crate::artefacts::registry::manifest::compute_subject_hash(
            directory.path(),
            "meta/decisions/dec.local.md",
            raw,
            &[],
        )
        .ok()
    );
    assert_eq!(response.pending[0].subject_hash_error, None);
}

#[test]
fn test_pending_local_tier_manifest_error_includes_message() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(directory.path().join("meta/decisions")).unwrap();
    std::fs::write(
            directory.path().join("meta/decisions/dec.local.md"),
            "---\nid: dec.local\nstatus: proposed\nratification: local\ndate: 2026-07-01\n---\n# Local\n",
        )
        .unwrap();
    let mut local = decision("dec.local", DecisionStatus::Proposed, "2026-07-01", &["a"]);
    local.ratification = crate::artefacts::registry::RatificationTier::Local;
    local.affects = vec!["src/missing.rs".to_owned()];
    let response = pending_response(
        directory.path(),
        &[local],
        &[],
        days_from_civil(2026, 7, 30),
    )
    .unwrap();
    assert_eq!(response.pending[0].subject_hash, None);
    assert!(
        response.pending[0]
            .subject_hash_error
            .as_deref()
            .is_some_and(|message| message.contains("src/missing.rs"))
    );
    let wire = serde_json::to_value(&response).unwrap();
    assert_eq!(wire["pending"][0]["subject_hash"], serde_json::Value::Null);
    assert!(
        wire["pending"][0]
            .get("subject_hash_error")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|message| message.contains("src/missing.rs"))
    );
}
