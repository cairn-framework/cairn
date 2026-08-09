//! The coordination fact envelope: one shape, four fact families.
//!
//! `evidence_class` is required from format 1 and fixed per fact kind
//! (`dec.rung-three-coordination-substrate` clause 4); the appender derives
//! it, callers never choose. A reader that meets an unknown `format` or
//! `evidence_class` fails closed, mirroring the `read_versioned_json`
//! discipline in `persist`.

use super::time::validate_rfc3339_utc;
use serde::{Deserialize, Serialize};

/// The store format this build writes and accepts.
pub const STORE_FORMAT: u32 = 1;

/// Who recorded a fact.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Actor {
    /// Actor family: `maintainer`, `driver`, or `console`.
    pub kind: String,
    /// Actor identity within the family.
    pub id: String,
}

/// One immutable coordination fact.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Envelope {
    /// Store format the fact was written under.
    pub format: u32,
    /// First 12 hex of SHA-256 over the canonical body with this field empty.
    pub fact_id: String,
    /// Fact kind, e.g. `ruling.run`, `lease.grant`.
    pub kind: String,
    /// RFC 3339 UTC recording instant.
    pub recorded_at: String,
    /// The recording actor.
    pub recorded_by: Actor,
    /// 40-hex commit at recording.
    pub commit: String,
    /// `deterministic`, `attested`, or `observed`; fixed per kind.
    pub evidence_class: String,
    /// Fact id this record supersedes, when folding a chain.
    pub supersedes: Option<String>,
    /// Family-specific payload.
    pub payload: serde_json::Value,
}

/// The evidence class fixed for `kind`, or `None` for a kind outside the
/// four sanctioned families (which the appender refuses).
pub(crate) fn evidence_class_for(kind: &str) -> Option<&'static str> {
    if kind.starts_with("ruling.")
        || kind.starts_with("lease.")
        || kind.starts_with("driver.singleton.")
        || kind.starts_with("outcome.run_")
        || kind == "outcome.unit"
    {
        Some("attested")
    } else if kind == "outcome.touched_files" {
        Some("observed")
    } else {
        None
    }
}

/// Returns true when `class` is one of the three sanctioned classes.
pub(crate) fn known_evidence_class(class: &str) -> bool {
    matches!(class, "deterministic" | "attested" | "observed")
}

/// Validates the payload fields required by lease-chain projections.
pub(crate) fn validate_lease_payload(
    kind: &str,
    payload: &serde_json::Value,
) -> Result<(), String> {
    if !kind.starts_with("lease.") {
        return Ok(());
    }
    let Some(unit_id) = payload.get("unit_id").and_then(serde_json::Value::as_str) else {
        return Err(format!("lease fact `{kind}` is missing string `unit_id`"));
    };
    if unit_id.is_empty() {
        return Err(format!("lease fact `{kind}` has an empty `unit_id`"));
    }
    if matches!(kind, "lease.grant" | "lease.renew") {
        let Some(expires_at) = payload
            .get("expires_at")
            .and_then(serde_json::Value::as_str)
        else {
            return Err(format!(
                "lease fact `{kind}` is missing string `expires_at`"
            ));
        };
        validate_rfc3339_utc(expires_at)
            .map_err(|error| format!("lease fact `{kind}` has malformed `expires_at`: {error}"))?;
    }
    Ok(())
}

/// Computes the fact id: first 12 hex of SHA-256 over the canonical JSON
/// body serialised with `fact_id` empty.
pub(crate) fn fact_id_for(envelope: &Envelope) -> Result<String, String> {
    let mut body = envelope.clone();
    body.fact_id = String::new();
    let canonical = serde_json::to_string(&body)
        .map_err(|error| format!("fact body does not serialise: {error}"))?;
    let mut hex = crate::artefacts::registry::sha256::sha256_hex(canonical.as_bytes());
    hex.truncate(12);
    Ok(hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_class_is_fixed_per_kind() {
        assert_eq!(evidence_class_for("ruling.run"), Some("attested"));
        assert_eq!(evidence_class_for("ruling.park"), Some("attested"));
        assert_eq!(evidence_class_for("lease.grant"), Some("attested"));
        assert_eq!(
            evidence_class_for("driver.singleton.renew"),
            Some("attested")
        );
        assert_eq!(evidence_class_for("outcome.run_declined"), Some("attested"));
        assert_eq!(evidence_class_for("outcome.unit"), Some("attested"));
        assert_eq!(
            evidence_class_for("outcome.touched_files"),
            Some("observed")
        );
        assert_eq!(evidence_class_for("gossip.rumour"), None);
    }

    #[test]
    fn fact_id_is_stable_and_ignores_the_stored_id() {
        let mut fact = Envelope {
            format: STORE_FORMAT,
            fact_id: String::new(),
            kind: "ruling.run".to_owned(),
            recorded_at: "2026-08-07T03:45:12Z".to_owned(),
            recorded_by: Actor {
                kind: "maintainer".to_owned(),
                id: "m".to_owned(),
            },
            commit: "0".repeat(40),
            evidence_class: "attested".to_owned(),
            supersedes: None,
            payload: serde_json::json!({ "target": "plan-0123456789abcdef" }),
        };
        let first = fact_id_for(&fact).expect("hashes");
        assert_eq!(first.len(), 12);
        fact.fact_id = first.clone();
        assert_eq!(fact_id_for(&fact).expect("hashes"), first);
    }
}
