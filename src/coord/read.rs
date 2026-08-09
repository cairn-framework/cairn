//! The full fold: every read lists and parses `facts/` in full.
//!
//! Filenames are second-precision and renames land after listing starts, so
//! no reader may fold incrementally above a high-water mark: within one
//! second the sort order is decided by `<kind>-<fact_id>`, which has nothing
//! to do with creation order, and a dropped lease grant is exactly the
//! failure rung 3 exists to prevent. The facts directory is the ground truth.

use std::path::Path;

use super::envelope::{
    Envelope, STORE_FORMAT, evidence_class_for, fact_id_for, validate_kind, validate_lease_payload,
};
use super::store;
use super::time::{compact_rfc3339, compare_instants, parse_rfc3339, validate_rfc3339_utc};

/// The outcome of reading a family store.
pub enum StoreRead {
    /// No append has ever initialised the store; a read creates nothing.
    Uninitialised,
    /// Every fact, sorted by filename, fully parsed and validated.
    Ready(Vec<NamedFact>),
}

/// A fact together with the filename that keys it.
#[derive(Clone, Debug)]
pub struct NamedFact {
    /// Filename under `facts/`.
    pub name: String,
    /// The parsed envelope.
    pub fact: Envelope,
}
/// Validates one parsed fact; any failure fails the whole read.
pub(crate) fn validate(name: &str, fact: &Envelope) -> Result<(), String> {
    validate_format_and_evidence(name, fact)?;
    validate_actor_and_timestamp(name, fact)?;
    validate_payload_and_identity(name, fact)?;
    Ok(())
}

fn validate_format_and_evidence(name: &str, fact: &Envelope) -> Result<(), String> {
    if fact.format != STORE_FORMAT {
        return Err(format!(
            "fact `{name}` carries unknown format {}; failing closed",
            fact.format
        ));
    }
    validate_kind(&fact.kind)
        .map_err(|error| format!("fact `{name}` has invalid kind: {error}"))?;
    let Some(expected_evidence_class) = evidence_class_for(&fact.kind) else {
        return Err(format!(
            "fact `{name}` carries an unsanctioned kind `{}`; failing closed",
            fact.kind
        ));
    };
    if fact.evidence_class != expected_evidence_class {
        return Err(format!(
            "fact `{name}` carries evidence class `{}`; expected `{expected_evidence_class}`",
            fact.evidence_class
        ));
    }
    Ok(())
}

fn validate_actor_and_timestamp(name: &str, fact: &Envelope) -> Result<(), String> {
    if !matches!(
        fact.recorded_by.kind.as_str(),
        "maintainer" | "driver" | "console"
    ) {
        return Err(format!(
            "fact `{name}` carries unknown actor kind `{}`",
            fact.recorded_by.kind
        ));
    }
    if fact.commit.len() != 40 || !fact.commit.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!(
            "fact `{name}` carries a non-40-hex commit `{}`",
            fact.commit
        ));
    }
    validate_rfc3339_utc(&fact.recorded_at)
        .map_err(|error| format!("fact `{name}` has malformed `recorded_at`: {error}"))?;
    Ok(())
}

fn validate_payload_and_identity(name: &str, fact: &Envelope) -> Result<(), String> {
    validate_lease_payload(&fact.kind, &fact.payload)
        .map_err(|error| format!("fact `{name}` is malformed: {error}"))?;
    let expected = fact_id_for(fact)?;
    if fact.fact_id != expected {
        return Err(format!(
            "fact `{name}` has invalid identity `{}`; expected `{expected}`",
            fact.fact_id
        ));
    }
    let expected_name = format!(
        "{}-{}-{}.json",
        compact_rfc3339(&fact.recorded_at),
        fact.kind,
        fact.fact_id
    );
    if name != expected_name {
        return Err(format!(
            "fact `{name}` filename does not match its content; expected `{expected_name}`"
        ));
    }
    Ok(())
}

/// Reads every fact in the family store for `root`.
///
/// # Errors
///
/// Fails closed on any unreadable, unparseable, or unknown-format fact: the
/// driver must never dispatch against a partially read store.
pub fn read_facts(root: &Path) -> Result<StoreRead, String> {
    let store = store::store_root(root)?;
    read_store(&store)
}

/// Reads every fact from an already-resolved store path.
///
/// # Errors
///
/// Fails closed on any unreadable, unparseable, or unknown-format fact.
pub(crate) fn read_store(store: &Path) -> Result<StoreRead, String> {
    if !store::is_initialised(store) {
        return Ok(StoreRead::Uninitialised);
    }
    store::check_format(store)?;

    let mut names = Vec::new();
    let entries = std::fs::read_dir(store.join("facts"))
        .map_err(|error| format!("cannot list coordination facts: {error}"))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("cannot read facts entry: {error}"))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if std::path::Path::new(&name)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
        {
            names.push(name);
        }
    }
    names.sort();

    let mut facts = Vec::with_capacity(names.len());
    for name in names {
        let path = store.join("facts").join(&name);
        let raw =
            std::fs::read(&path).map_err(|error| format!("cannot read fact `{name}`: {error}"))?;
        let fact: Envelope = serde_json::from_slice(&raw)
            .map_err(|error| format!("fact `{name}` does not parse: {error}"))?;
        validate(&name, &fact)?;
        facts.push(NamedFact { name, fact });
    }
    Ok(StoreRead::Ready(facts))
}
/// The rendered view of a lease chain head, for the stale projection.
#[derive(Clone, Debug)]
pub struct LeaseState<'a> {
    /// The chain-head fact.
    pub head: &'a NamedFact,
    /// The holder object from the lease payload.
    pub holder: Option<&'a serde_json::Value>,
    /// The lease's `expires_at`, when present.
    pub expires_at: Option<&'a str>,
    /// The recoverable residue (branch, worktree, pr).
    pub residue: Option<&'a serde_json::Value>,
}

/// Folds the `supersedes` chain for `unit` and returns its head: the latest
/// lease fact no other lease fact supersedes. `None` means no chain at all.
#[must_use]
pub fn lease_chain_head<'a>(facts: &'a [NamedFact], unit: &str) -> Option<&'a NamedFact> {
    let unit_facts: Vec<&NamedFact> = facts
        .iter()
        .filter(|named| {
            named.fact.kind.starts_with("lease.")
                && named
                    .fact
                    .payload
                    .get("unit_id")
                    .and_then(serde_json::Value::as_str)
                    == Some(unit)
        })
        .collect();
    let superseded: std::collections::BTreeSet<&str> = unit_facts
        .iter()
        .filter_map(|named| named.fact.supersedes.as_deref())
        .collect();
    unit_facts
        .into_iter()
        .filter(|named| !superseded.contains(named.fact.fact_id.as_str()))
        .max_by(|a, b| {
            (a.fact.recorded_at.as_str(), a.fact.fact_id.as_str())
                .cmp(&(b.fact.recorded_at.as_str(), b.fact.fact_id.as_str()))
        })
}

fn lease_state(head: &NamedFact) -> LeaseState<'_> {
    LeaseState {
        head,
        holder: head.fact.payload.get("holder"),
        expires_at: head
            .fact
            .payload
            .get("expires_at")
            .and_then(serde_json::Value::as_str),
        residue: head.fact.payload.get("residue"),
    }
}

fn compare_expiry_at(expiry: &str, at: &str) -> std::cmp::Ordering {
    match (parse_rfc3339(expiry), parse_rfc3339(at)) {
        (Some(expiry), Some(at)) => compare_instants(expiry, at),
        _ => expiry.cmp(at),
    }
}

fn expiry_is_after(expiry: &str, at: &str) -> bool {
    compare_expiry_at(expiry, at) == std::cmp::Ordering::Greater
}

/// True when `unit`'s folded chain has no release and `expires_at` is after
/// the caller's instant. Valid RFC 3339 spellings compare as instants; the
/// historical lexical fallback remains for an invalid caller-only `--at`.
#[must_use]
pub fn held(facts: &[NamedFact], unit: &str, at: &str) -> bool {
    lease_chain_head(facts, unit).is_some_and(|head| {
        head.fact.kind != "lease.release"
            && lease_state(head)
                .expires_at
                .is_some_and(|expiry| expiry_is_after(expiry, at))
    })
}

/// The same chain with `expires_at <= at`: a first-class state carrying the
/// holder, the expiry, and the recoverable residue.
#[must_use]
pub fn stale<'a>(facts: &'a [NamedFact], unit: &str, at: &str) -> Option<LeaseState<'a>> {
    let head = lease_chain_head(facts, unit)?;
    if head.fact.kind == "lease.release" {
        return None;
    }
    let state = lease_state(head);
    state
        .expires_at
        .is_some_and(|expiry| compare_expiry_at(expiry, at) != std::cmp::Ordering::Greater)
        .then_some(state)
}

/// No chain at all: a different state and a different sentence.
#[must_use]
pub fn no_lease(facts: &[NamedFact], unit: &str) -> bool {
    lease_chain_head(facts, unit).is_none()
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

    fn fact(kind: &str, recorded_at: &str, payload: serde_json::Value) -> NewFact {
        NewFact {
            kind: kind.to_owned(),
            recorded_at: recorded_at.to_owned(),
            recorded_by: Actor {
                kind: "driver".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: None,
            payload,
        }
    }

    #[test]
    fn missing_store_reads_as_uninitialised_and_creates_nothing() {
        let dir = repo();
        assert!(matches!(
            read_facts(dir.path()).expect("reads"),
            StoreRead::Uninitialised
        ));
        assert!(
            !dir.path().join(".git/cairn").exists(),
            "a read never creates the store"
        );
    }

    #[test]
    fn same_second_facts_are_both_returned_by_the_full_listing() {
        let dir = repo();
        // Two facts in the same second whose names sort opposite to their
        // write order: the ruling lands first but sorts after the lease
        // fact (r > l), so a high-water reader marked at the ruling would
        // never fold the lease grant.
        append_fact(
            dir.path(),
            fact(
                "ruling.run",
                "2026-08-07T03:45:12Z",
                serde_json::json!({ "target": "plan-0123456789abcdef" }),
            ),
        )
        .expect("ruling appends");
        append_fact(
            dir.path(),
            fact(
                "lease.grant",
                "2026-08-07T03:45:12Z",
                serde_json::json!({
                    "unit_id": "todo.example",
                    "expires_at": "2026-08-07T04:00:00Z",
                }),
            ),
        )
        .expect("lease appends");

        let StoreRead::Ready(facts) = read_facts(dir.path()).expect("reads") else {
            panic!("store is initialised");
        };
        assert_eq!(facts.len(), 2, "both same-second facts fold");
        assert!(facts[0].name < facts[1].name, "sorted by filename");
        // A second read re-lists immutable facts and still returns both.
        let StoreRead::Ready(again) = read_facts(dir.path()).expect("re-reads") else {
            panic!("store is initialised");
        };
        assert_eq!(again.len(), 2);
    }

    #[test]
    fn an_unreadable_fact_fails_the_whole_read() {
        let dir = repo();
        append_fact(
            dir.path(),
            fact(
                "ruling.run",
                "2026-08-07T03:45:12Z",
                serde_json::json!({ "target": "plan-0123456789abcdef" }),
            ),
        )
        .expect("appends");
        let store = dir.path().join(".git/cairn/coord");
        std::fs::write(
            store.join("facts/20260807T034513Z-ruling.run-badbadbadbad.json"),
            "not json",
        )
        .expect("garbage lands");
        assert!(
            read_facts(dir.path()).is_err(),
            "a partially resolvable store is an error, not a short list"
        );
    }

    #[test]
    fn an_unknown_format_fact_fails_the_whole_read() {
        let dir = repo();
        let path = append_fact(
            dir.path(),
            fact(
                "ruling.run",
                "2026-08-07T03:45:12Z",
                serde_json::json!({ "target": "plan-0123456789abcdef" }),
            ),
        )
        .expect("appends");
        let raw = std::fs::read_to_string(&path).expect("bytes");
        std::fs::write(&path, raw.replace("\"format\": 1", "\"format\": 9")).expect("mutates");
        assert!(read_facts(dir.path()).is_err());
    }

    #[test]
    fn predicates_fold_the_supersedes_chain() {
        let dir = repo();
        let grant_path = append_fact(
            dir.path(),
            NewFact {
                kind: "lease.grant".to_owned(),
                recorded_at: "2026-08-07T03:00:00Z".to_owned(),
                recorded_by: Actor {
                    kind: "driver".to_owned(),
                    id: "t".to_owned(),
                },
                commit: "a".repeat(40),
                supersedes: None,
                payload: serde_json::json!({
                    "unit_id": "todo.x",
                    "holder": { "harness_kind": "omp", "session": "s1" },
                    "expires_at": "2026-08-07T04:00:00Z",
                    "residue": { "branch": "loop/x", "worktree": "../wt/x", "pr": null },
                }),
            },
        )
        .expect("grant");
        let grant: Envelope =
            serde_json::from_str(&std::fs::read_to_string(&grant_path).expect("grant bytes"))
                .expect("grant parses");
        append_fact(
            dir.path(),
            NewFact {
                kind: "lease.renew".to_owned(),
                recorded_at: "2026-08-07T03:30:00Z".to_owned(),
                recorded_by: Actor {
                    kind: "driver".to_owned(),
                    id: "t".to_owned(),
                },
                commit: "a".repeat(40),
                supersedes: Some(grant.fact_id),
                payload: serde_json::json!({
                    "unit_id": "todo.x",
                    "holder": { "harness_kind": "omp", "session": "s1" },
                    "expires_at": "2026-08-07T05:00:00Z",
                    "residue": { "branch": "loop/x", "worktree": "../wt/x", "pr": 42 },
                }),
            },
        )
        .expect("renew");

        let StoreRead::Ready(facts) = read_facts(dir.path()).expect("reads") else {
            panic!("store is initialised");
        };
        // The chain folds to the renewal: held before its expiry, stale after.
        assert!(held(&facts, "todo.x", "2026-08-07T04:30:00Z"));
        assert!(stale(&facts, "todo.x", "2026-08-07T04:30:00Z").is_none());
        let gone = stale(&facts, "todo.x", "2026-08-07T05:00:00Z").expect("stale at expiry");
        assert_eq!(gone.expires_at, Some("2026-08-07T05:00:00Z"));
        assert_eq!(
            gone.residue.and_then(|residue| residue.get("pr")),
            Some(&serde_json::json!(42)),
            "the folded chain renders the renewed residue"
        );
        assert!(!held(&facts, "todo.x", "2026-08-07T05:00:00Z"));
        // A unit with no chain at all is a different state.
        assert!(no_lease(&facts, "todo.other"));
        assert!(!no_lease(&facts, "todo.x"));
    }
}
#[cfg(test)]
#[path = "read_regressions.rs"]
mod regressions;
