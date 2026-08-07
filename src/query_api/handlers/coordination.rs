//! Coordination read surface: raw facts from the family store.
//!
//! The wire contract (`dec.rung-three-coordination-substrate` clause 2):
//! responses stamp `schema_version`, carry `store_state`, `cursor`,
//! `truncated`, and `conflicts`, and echo `observed_at` exactly as supplied
//! (`null` when absent, no clock consulted). There is no `active`,
//! `expired`, `stale`, or `status` field anywhere: the core evaluates no
//! expiry, and the filter lives in the renderer. A partially resolvable
//! store fails closed rather than returning a short list.

use std::path::Path;

use serde_json::{Value, json};

use crate::coord::read::{NamedFact, StoreRead, read_facts};

use super::super::{QueryError, QueryRequest, SCHEMA_VERSION};

/// The kinds `ruling list|show` returns: rulings and the run lifecycle
/// outcomes the reader predicates join them against.
fn is_ruling_kind(kind: &str) -> bool {
    kind.starts_with("ruling.") || kind.starts_with("outcome.run_")
}

/// The kinds `lease list` returns: leases and the driver singleton chain.
fn is_lease_kind(kind: &str) -> bool {
    kind.starts_with("lease.") || kind.starts_with("driver.singleton.")
}

fn fact_json(named: &NamedFact) -> Value {
    let mut value = serde_json::to_value(&named.fact).unwrap_or(Value::Null);
    if let Some(object) = value.as_object_mut() {
        object.insert("name".to_owned(), json!(named.name));
    }
    value
}

fn envelope(
    request: &QueryRequest,
    store_state: &str,
    key: &str,
    facts: &[Value],
    cursor: Option<&str>,
) -> Value {
    json!({
        "schema_version": SCHEMA_VERSION,
        "store_state": store_state,
        "observed_at": request.at,
        "cursor": cursor,
        "truncated": false,
        "conflicts": [],
        key: facts,
    })
}

fn read_filtered(
    root: &Path,
    request: &QueryRequest,
    key: &str,
    keep: fn(&str) -> bool,
) -> Result<Value, QueryError> {
    let facts = match read_facts(root).map_err(read_error)? {
        StoreRead::Uninitialised => {
            return Ok(envelope(request, "uninitialised", key, &[], None));
        }
        StoreRead::Ready(facts) => facts,
    };
    let selected: Vec<&NamedFact> = facts
        .iter()
        .filter(|named| keep(&named.fact.kind))
        .filter(|named| {
            request
                .since
                .as_deref()
                .is_none_or(|since| named.name.as_str() > since)
        })
        .collect();
    let cursor = selected.last().map(|named| named.name.clone());
    let rendered: Vec<Value> = selected.iter().map(|named| fact_json(named)).collect();
    Ok(envelope(
        request,
        "ready",
        key,
        &rendered,
        cursor.as_deref(),
    ))
}

fn read_error(message: String) -> QueryError {
    QueryError {
        code: "CAIRN_COORD_READ_FAILED".to_owned(),
        message,
        source_span: None,
        remediation: None,
    }
}

/// `ruling list` and `ruling show <fact-id>`: raw ruling and run-outcome
/// facts.
///
/// # Errors
///
/// Fails closed on a partially resolvable store; `show` errors when the
/// fact id matches nothing.
pub(in crate::query_api) fn coordination_rulings_json(
    root: &Path,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    if request.tool.ends_with("show") {
        let Some(fact_id) = request.node.as_deref() else {
            return Err(QueryError {
                code: "CAIRN_QUERY_MISSING_NODE".to_owned(),
                message: "ruling show requires a fact id".to_owned(),
                source_span: None,
                remediation: None,
            });
        };
        let facts = match read_facts(root).map_err(read_error)? {
            StoreRead::Uninitialised => Vec::new(),
            StoreRead::Ready(facts) => facts,
        };
        let hit = facts
            .iter()
            .find(|named| named.fact.fact_id == fact_id && is_ruling_kind(&named.fact.kind));
        return hit.map_or_else(
            || {
                Err(QueryError {
                    code: "CAIRN_COORD_FACT_NOT_FOUND".to_owned(),
                    message: format!("no ruling fact `{fact_id}` in the coordination store"),
                    source_span: None,
                    remediation: None,
                })
            },
            |named| {
                Ok(json!({
                    "schema_version": SCHEMA_VERSION,
                    "store_state": "ready",
                    "observed_at": request.at,
                    "conflicts": [],
                    "ruling": fact_json(named),
                }))
            },
        );
    }
    read_filtered(root, request, "rulings", is_ruling_kind)
}

/// `lease list`: raw lease and driver-singleton facts.
///
/// # Errors
///
/// Fails closed on a partially resolvable store.
pub(in crate::query_api) fn coordination_leases_json(
    root: &Path,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    read_filtered(root, request, "leases", is_lease_kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::append::{NewFact, append_fact};
    use crate::coord::envelope::Actor;

    fn repo() -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(dir.path())
            .args(["init", "-q"])
            .output()
            .expect("git runs");
        assert!(output.status.success());
        dir
    }

    fn request(tool: &str, node: Option<&str>, at: Option<&str>) -> QueryRequest {
        QueryRequest {
            tool: tool.to_owned(),
            node: node.map(ToOwned::to_owned),
            at: at.map(ToOwned::to_owned),
            ..QueryRequest::default()
        }
    }

    fn record(root: &Path, kind: &str, recorded_at: &str, payload: serde_json::Value) {
        append_fact(
            root,
            NewFact {
                kind: kind.to_owned(),
                recorded_at: recorded_at.to_owned(),
                recorded_by: Actor {
                    kind: "maintainer".to_owned(),
                    id: "t".to_owned(),
                },
                commit: "a".repeat(40),
                supersedes: None,
                payload,
            },
        )
        .expect("appends");
    }

    #[test]
    fn missing_store_reads_uninitialised_and_creates_nothing() {
        let dir = repo();
        let data = coordination_rulings_json(dir.path(), &request("ruling list", None, None))
            .expect("reads");
        assert_eq!(data["store_state"], "uninitialised");
        assert_eq!(data["observed_at"], serde_json::Value::Null);
        assert_eq!(data["rulings"], serde_json::json!([]));
        assert!(
            !dir.path().join(".git/cairn").exists(),
            "a read never creates the store"
        );
    }

    #[test]
    fn same_second_facts_both_return_and_no_verdict_fields_appear() {
        let dir = repo();
        // Names sort opposite to write order within the second (z before a).
        record(
            dir.path(),
            "ruling.run",
            "2026-08-07T03:45:12Z",
            serde_json::json!({ "target": "plan-zzzzzzzzzzzzzzzz" }),
        );
        record(
            dir.path(),
            "ruling.park",
            "2026-08-07T03:45:12Z",
            serde_json::json!({ "target": "todo.a" }),
        );
        let data = coordination_rulings_json(
            dir.path(),
            &request("ruling list", None, Some("2026-08-07T04:00:00Z")),
        )
        .expect("reads");
        assert_eq!(data["store_state"], "ready");
        assert_eq!(
            data["observed_at"], "2026-08-07T04:00:00Z",
            "echoed verbatim"
        );
        assert_eq!(data["conflicts"], serde_json::json!([]));
        let rulings = data["rulings"].as_array().expect("array");
        assert_eq!(rulings.len(), 2, "both same-second facts fold");
        for fact in rulings {
            let object = fact.as_object().expect("object");
            for banned in ["active", "expired", "stale", "status"] {
                assert!(
                    !object.contains_key(banned),
                    "no derived verdict field `{banned}` on the wire"
                );
            }
        }
    }

    #[test]
    fn show_finds_a_fact_by_id_and_errors_on_unknown() {
        let dir = repo();
        record(
            dir.path(),
            "ruling.run",
            "2026-08-07T03:45:12Z",
            serde_json::json!({ "target": "plan-0123456789abcdef" }),
        );
        let listing = coordination_rulings_json(dir.path(), &request("ruling list", None, None))
            .expect("reads");
        let fact_id = listing["rulings"][0]["fact_id"].as_str().expect("id");
        let shown =
            coordination_rulings_json(dir.path(), &request("ruling show", Some(fact_id), None))
                .expect("shows");
        assert_eq!(shown["ruling"]["fact_id"], fact_id);
        let missing =
            coordination_rulings_json(dir.path(), &request("ruling show", Some("nope"), None))
                .expect_err("unknown fact errors");
        assert_eq!(missing.code, "CAIRN_COORD_FACT_NOT_FOUND");
    }

    #[test]
    fn leases_tool_returns_lease_and_singleton_kinds_only() {
        let dir = repo();
        record(
            dir.path(),
            "ruling.run",
            "2026-08-07T03:45:12Z",
            serde_json::json!({ "target": "plan-0123456789abcdef" }),
        );
        let mut lease = NewFact {
            kind: "lease.grant".to_owned(),
            recorded_at: "2026-08-07T03:45:13Z".to_owned(),
            recorded_by: Actor {
                kind: "driver".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: None,
            payload: serde_json::json!({ "unit_id": "todo.x" }),
        };
        append_fact(dir.path(), lease).expect("lease appends");
        lease = NewFact {
            kind: "driver.singleton.grant".to_owned(),
            recorded_at: "2026-08-07T03:45:14Z".to_owned(),
            recorded_by: Actor {
                kind: "driver".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: None,
            payload: serde_json::json!({ "session": "s1" }),
        };
        append_fact(dir.path(), lease).expect("singleton appends");
        let data = coordination_leases_json(dir.path(), &request("lease list", None, None))
            .expect("reads");
        let leases = data["leases"].as_array().expect("array");
        assert_eq!(leases.len(), 2, "the ruling is not a lease fact");
        assert_eq!(data["cursor"].as_str().map(str::is_empty), Some(false));
    }

    #[test]
    fn a_partially_resolvable_store_fails_closed_on_the_wire() {
        let dir = repo();
        record(
            dir.path(),
            "ruling.run",
            "2026-08-07T03:45:12Z",
            serde_json::json!({ "target": "plan-0123456789abcdef" }),
        );
        std::fs::write(
            dir.path()
                .join(".git/cairn/coord/facts/20260807T034513Z-ruling.run-badbadbadbad.json"),
            "not json",
        )
        .expect("garbage lands");
        let error = coordination_rulings_json(dir.path(), &request("ruling list", None, None))
            .expect_err("fails closed");
        assert_eq!(error.code, "CAIRN_COORD_READ_FAILED");
    }
}
