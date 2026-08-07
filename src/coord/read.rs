//! The full fold: every read lists `facts/` in full, with a derived,
//! disposable parse cache.
//!
//! Filenames are second-precision and renames land after listing starts, so
//! no reader may fold incrementally above a high-water mark: within one
//! second the sort order is decided by `<kind>-<fact_id>`, which has nothing
//! to do with creation order, and a dropped lease grant is exactly the
//! failure rung 3 exists to prevent. The cache saves parses, never listings:
//! filename is an exact content key because facts are immutable, so parse
//! cost is O(new facts) while a full listing stays the ground truth.

use std::collections::BTreeMap;
use std::path::Path;

use crate::persist;

use super::envelope::{Envelope, STORE_FORMAT, known_evidence_class};
use super::store;

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
fn validate(name: &str, fact: &Envelope) -> Result<(), String> {
    if fact.format != STORE_FORMAT {
        return Err(format!(
            "fact `{name}` carries unknown format {}; failing closed",
            fact.format
        ));
    }
    if !known_evidence_class(&fact.evidence_class) {
        return Err(format!(
            "fact `{name}` carries unknown evidence class `{}`; failing closed",
            fact.evidence_class
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

    let cache_path = store.join("cache/parsed.json");
    let mut cache: BTreeMap<String, Envelope> = persist::read_json(&cache_path).unwrap_or_default();

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

    let mut parsed_new = false;
    let mut facts = Vec::with_capacity(names.len());
    for name in names {
        let fact = if let Some(hit) = cache.get(&name) {
            hit.clone()
        } else {
            let raw = std::fs::read_to_string(store.join("facts").join(&name))
                .map_err(|error| format!("cannot read fact `{name}`: {error}"))?;
            let fact: Envelope = serde_json::from_str(&raw)
                .map_err(|error| format!("fact `{name}` does not parse: {error}"))?;
            cache.insert(name.clone(), fact.clone());
            parsed_new = true;
            fact
        };
        validate(&name, &fact)?;
        facts.push(NamedFact { name, fact });
    }

    if parsed_new {
        // Derived and disposable: a failed cache write never fails the read.
        let _ = persist::write_json(&cache_path, &cache);
    }
    Ok(StoreRead::Ready(facts))
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
                serde_json::json!({ "unit_id": "todo.example" }),
            ),
        )
        .expect("lease appends");

        let StoreRead::Ready(facts) = read_facts(dir.path()).expect("reads") else {
            panic!("store is initialised");
        };
        assert_eq!(facts.len(), 2, "both same-second facts fold");
        assert!(facts[0].name < facts[1].name, "sorted by filename");
        // A second read hits the cache and still returns both.
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
}
