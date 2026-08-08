//! Coordination read surface: raw facts from the family store.
//!
//! The wire contract (`dec.rung-three-coordination-substrate` clause 2):
//! responses stamp `schema_version`, carry `store_state`, `cursor`,
//! `truncated`, and `conflicts`, and echo `observed_at` exactly as supplied
//! (`null` when absent, no clock consulted). There is no `active`,
//! `expired`, `stale`, or `status` field anywhere: the core evaluates no
//! expiry, and the filter lives in the renderer. A partially resolvable
//! store fails closed rather than returning a short list. Because fact names
//! have only second precision, a cursor replays the rest of its second so a
//! fact appended later in that second cannot be lost to filename ordering.

use std::path::Path;

use serde_json::{Value, json};

use crate::coord::read::{NamedFact, StoreRead, read_facts};

use super::super::{QueryError, QueryRequest, QuerySince, SCHEMA_VERSION};

/// The kinds `ruling list|show` returns: rulings and the run lifecycle
/// outcomes the reader predicates join them against.
fn is_ruling_kind(kind: &str) -> bool {
    kind.starts_with("ruling.") || kind.starts_with("outcome.run_")
}

/// The kinds `lease list` returns: leases and the driver singleton chain.
fn is_lease_kind(kind: &str) -> bool {
    kind.starts_with("lease.") || kind.starts_with("driver.singleton.")
}

fn coordination_cursor(request: &QueryRequest) -> Option<&str> {
    match request.since.as_ref() {
        Some(QuerySince::CoordinationCursor(cursor)) => Some(cursor.as_str()),
        _ => None,
    }
}

fn fact_second(name: &str) -> Option<&str> {
    let second = name.get(..15)?;
    (second.as_bytes().get(8) == Some(&b'T')).then_some(second)
}

fn after_cursor(name: &str, cursor: &str) -> bool {
    match (fact_second(name), fact_second(cursor)) {
        (Some(name_second), Some(cursor_second)) => {
            name_second > cursor_second || (name_second == cursor_second && name != cursor)
        }
        _ => name > cursor,
    }
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
    let cursor = coordination_cursor(request);
    let facts = match read_facts(root).map_err(read_error)? {
        StoreRead::Uninitialised => {
            return Ok(envelope(request, "uninitialised", key, &[], None));
        }
        StoreRead::Ready(facts) => facts,
    };
    let selected: Vec<&NamedFact> = facts
        .iter()
        .filter(|named| keep(&named.fact.kind))
        .filter(|named| cursor.is_none_or(|cursor| after_cursor(&named.name, cursor)))
        .collect();
    let next_cursor = match (cursor, selected.last()) {
        (Some(previous), Some(last)) if last.name.as_str() > previous => Some(last.name.clone()),
        (Some(previous), _) => Some(previous.to_owned()),
        (None, Some(last)) => Some(last.name.clone()),
        (None, None) => None,
    };
    let rendered: Vec<Value> = selected.iter().map(|named| fact_json(named)).collect();
    Ok(envelope(
        request,
        "ready",
        key,
        &rendered,
        next_cursor.as_deref(),
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

    fn record(root: &Path, kind: &str, recorded_at: &str, payload: serde_json::Value) -> String {
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
        .expect("appends")
        .file_name()
        .expect("fact filename")
        .to_string_lossy()
        .into_owned()
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
    fn same_second_cursor_replays_a_late_lower_filename() {
        let dir = repo();
        let recorded_at = "2026-08-07T03:45:12Z";
        let first_name = record(
            dir.path(),
            "ruling.run",
            recorded_at,
            serde_json::json!({ "target": "cursor-first" }),
        );

        // Fact IDs are content hashes, so find a deterministic same-second
        // payload whose filename sorts below the first cursor.
        let scratch = repo();
        let (index, late_name) = (0..256)
            .find_map(|index| {
                let name = record(
                    scratch.path(),
                    "ruling.run",
                    recorded_at,
                    serde_json::json!({ "target": format!("cursor-late-{index}") }),
                );
                (name < first_name).then_some((index, name))
            })
            .expect("a same-second fact should sort below the cursor");
        let actual_late_name = record(
            dir.path(),
            "ruling.run",
            recorded_at,
            serde_json::json!({ "target": format!("cursor-late-{index}") }),
        );
        assert_eq!(actual_late_name, late_name);

        let mut request = request("ruling list", None, None);
        request.since = Some(QuerySince::CoordinationCursor(first_name.clone()));
        let data = coordination_rulings_json(dir.path(), &request).expect("reads");
        assert_eq!(
            data["cursor"].as_str(),
            Some(first_name.as_str()),
            "cursor must not move backwards within a second"
        );
        let names: Vec<&str> = data["rulings"]
            .as_array()
            .expect("rulings")
            .iter()
            .filter_map(|fact| fact["name"].as_str())
            .collect();
        assert!(
            names.contains(&late_name.as_str()),
            "late same-second fact {late_name} was lost behind the cursor"
        );
    }
    #[test]
    fn same_second_cursor_replays_fractional_second_facts() {
        let dir = repo();
        let first_name = record(
            dir.path(),
            "ruling.run",
            "2026-08-07T03:45:12Z",
            serde_json::json!({ "target": "cursor-whole-second" }),
        );
        let late_name = record(
            dir.path(),
            "ruling.run",
            "2026-08-07T03:45:12.500Z",
            serde_json::json!({ "target": "cursor-fractional-second" }),
        );
        assert!(
            late_name < first_name,
            "fractional filename should sort lower"
        );

        let mut request = request("ruling list", None, None);
        request.since = Some(QuerySince::CoordinationCursor(first_name));
        let data = coordination_rulings_json(dir.path(), &request).expect("reads");
        let names: Vec<&str> = data["rulings"]
            .as_array()
            .expect("rulings")
            .iter()
            .filter_map(|fact| fact["name"].as_str())
            .collect();
        assert!(
            names.contains(&late_name.as_str()),
            "fractional same-second fact was lost behind the cursor"
        );
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
