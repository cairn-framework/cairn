//! Store integrity: `verify` checks the append-only discipline held, and
//! `compact` moves old facts to `archive/` without ever deleting.
//!
//! Two facts are never compactable: a `ruling.park` with no matching
//! `unpark` (the ready-set projection honours it indefinitely), and any fact
//! that is the antecedent of a live `supersedes` chain.

use std::collections::BTreeSet;
use std::path::Path;

use crate::persist;

use super::envelope::Envelope;
use super::read::{NamedFact, StoreRead, read_store};
use super::store;

/// Lists archived fact names and their parsed envelopes.
fn read_archive(store: &Path) -> Result<Vec<NamedFact>, String> {
    let mut out = Vec::new();
    let archive = store.join("archive");
    let months = match std::fs::read_dir(&archive) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(out),
        Err(error) => return Err(format!("cannot list archive: {error}")),
    };
    for month in months {
        let month = month.map_err(|error| format!("cannot read archive entry: {error}"))?;
        if !month.path().is_dir() {
            continue;
        }
        for entry in std::fs::read_dir(month.path())
            .map_err(|error| format!("cannot list archive month: {error}"))?
        {
            let entry = entry.map_err(|error| format!("cannot read archive entry: {error}"))?;
            let name = entry.file_name().to_string_lossy().to_string();
            if !std::path::Path::new(&name)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
            {
                continue;
            }
            let raw = std::fs::read_to_string(entry.path())
                .map_err(|error| format!("cannot read archived fact `{name}`: {error}"))?;
            let fact: Envelope = serde_json::from_str(&raw)
                .map_err(|error| format!("archived fact `{name}` does not parse: {error}"))?;
            out.push(NamedFact { name, fact });
        }
    }
    Ok(out)
}

/// True when `park` has a matching `ruling.unpark` anywhere in the store.
fn park_is_matched(park: &Envelope, all: &[&NamedFact]) -> bool {
    all.iter().any(|candidate| {
        candidate.fact.kind == "ruling.unpark"
            && candidate.fact.payload.get("target") == park.payload.get("target")
            && candidate.fact.recorded_at >= park.recorded_at
    })
}

/// Verifies store integrity for the family containing `root`.
///
/// Checks that the live fact set is a superset of every prior observation,
/// that no `supersedes` chain has a missing antecedent, that no live chain's
/// antecedent was compacted, and that no unmatched `ruling.park` sits in the
/// archive. Records the current observation snapshot on success.
///
/// # Errors
///
/// Returns the first violation found, or any read failure (fail closed).
pub fn verify(root: &Path) -> Result<(), String> {
    let store = store::store_root(root)?;
    let live = match read_store(&store)? {
        StoreRead::Uninitialised => return Ok(()),
        StoreRead::Ready(facts) => facts,
    };
    let archived = read_archive(&store)?;
    let live_ids: BTreeSet<&str> = live.iter().map(|f| f.fact.fact_id.as_str()).collect();
    let archived_ids: BTreeSet<&str> = archived.iter().map(|f| f.fact.fact_id.as_str()).collect();
    let everything: Vec<&NamedFact> = live.iter().chain(archived.iter()).collect();

    // Append-only: every previously observed fact still exists somewhere.
    let observed_path = store.join("cache/observed.json");
    let mut observed: BTreeSet<String> = persist::read_json(&observed_path).unwrap_or_default();
    let present: BTreeSet<String> = everything.iter().map(|f| f.name.clone()).collect();
    if let Some(missing) = observed.difference(&present).next() {
        return Err(format!(
            "previously observed fact `{missing}` has disappeared; the store is not append-only"
        ));
    }

    for named in &everything {
        if let Some(antecedent) = &named.fact.supersedes {
            let in_live = live_ids.contains(antecedent.as_str());
            let in_archive = archived_ids.contains(antecedent.as_str());
            if !in_live && !in_archive {
                return Err(format!(
                    "fact `{}` supersedes `{antecedent}`, which exists nowhere",
                    named.name
                ));
            }
            if live_ids.contains(named.fact.fact_id.as_str()) && !in_live {
                return Err(format!(
                    "live fact `{}` supersedes `{antecedent}`, which was compacted",
                    named.name
                ));
            }
        }
    }

    for named in &archived {
        if named.fact.kind == "ruling.park" && !park_is_matched(&named.fact, &everything) {
            return Err(format!(
                "unmatched `ruling.park` `{}` was compacted; readiness would silently change",
                named.name
            ));
        }
    }

    observed.extend(present);
    let _ = persist::write_json(&observed_path, &observed);
    Ok(())
}

/// Moves facts recorded strictly before `before` (a `YYYY-MM-DD` date) into
/// `archive/<yyyy-mm>/`. Never moves an unmatched `ruling.park` or the
/// antecedent of a live chain; deletes nothing. Returns the moved names.
///
/// # Errors
///
/// Returns a malformed date, a read failure, or a rename failure.
pub fn compact(root: &Path, before: &str) -> Result<Vec<String>, String> {
    let cutoff: String = before.chars().filter(char::is_ascii_digit).collect();
    if before.len() != 10 || cutoff.len() != 8 {
        return Err(format!("`{before}` is not a YYYY-MM-DD date"));
    }
    let store = store::store_root(root)?;
    let live = match read_store(&store)? {
        StoreRead::Uninitialised => return Ok(Vec::new()),
        StoreRead::Ready(facts) => facts,
    };
    let archived = read_archive(&store)?;

    let mut moving: BTreeSet<String> = live
        .iter()
        .filter(|f| f.name.get(..8).is_some_and(|day| day < cutoff.as_str()))
        .map(|f| f.name.clone())
        .collect();

    // Keep unmatched parks live.
    let everything: Vec<&NamedFact> = live.iter().chain(archived.iter()).collect();
    for named in &live {
        if named.fact.kind == "ruling.park" && !park_is_matched(&named.fact, &everything) {
            moving.remove(&named.name);
        }
    }
    // Keep the antecedent of any chain that stays live; chains may be long,
    // so iterate to a fixpoint.
    loop {
        let staying: Vec<&NamedFact> = live.iter().filter(|f| !moving.contains(&f.name)).collect();
        let staying_antecedents: BTreeSet<&str> = staying
            .iter()
            .filter_map(|f| f.fact.supersedes.as_deref())
            .collect();
        let retained: Vec<String> = moving
            .iter()
            .filter(|name| {
                live.iter().any(|f| {
                    &f.name == *name && staying_antecedents.contains(f.fact.fact_id.as_str())
                })
            })
            .cloned()
            .collect();
        if retained.is_empty() {
            break;
        }
        for name in retained {
            moving.remove(&name);
        }
    }

    let mut moved = Vec::new();
    for name in moving {
        let month = format!("{}-{}", &name[..4], &name[4..6]);
        let target_dir = store.join("archive").join(month);
        std::fs::create_dir_all(&target_dir)
            .map_err(|error| format!("cannot create archive month: {error}"))?;
        std::fs::rename(store.join("facts").join(&name), target_dir.join(&name))
            .map_err(|error| format!("cannot archive `{name}`: {error}"))?;
        moved.push(name);
    }
    // The parse cache may now hold archived names; it is derived and
    // disposable, so drop it rather than editing it.
    let _ = std::fs::remove_file(store.join("cache/parsed.json"));
    Ok(moved)
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

    fn record(
        root: &Path,
        kind: &str,
        recorded_at: &str,
        supersedes: Option<String>,
        payload: serde_json::Value,
    ) -> String {
        let path = append_fact(
            root,
            NewFact {
                kind: kind.to_owned(),
                recorded_at: recorded_at.to_owned(),
                recorded_by: Actor {
                    kind: "driver".to_owned(),
                    id: "t".to_owned(),
                },
                commit: "a".repeat(40),
                supersedes,
                payload,
            },
        )
        .expect("appends");
        let raw = std::fs::read_to_string(&path).expect("bytes");
        let fact: Envelope = serde_json::from_str(&raw).expect("parses");
        fact.fact_id
    }

    #[test]
    fn verify_passes_a_clean_store_and_fails_a_removed_antecedent() {
        let dir = repo();
        let grant = record(
            dir.path(),
            "lease.grant",
            "2026-08-01T00:00:00Z",
            None,
            serde_json::json!({ "unit_id": "todo.x" }),
        );
        record(
            dir.path(),
            "lease.renew",
            "2026-08-02T00:00:00Z",
            Some(grant.clone()),
            serde_json::json!({ "unit_id": "todo.x" }),
        );
        verify(dir.path()).expect("clean store verifies");

        let store = dir.path().join(".git/cairn/coord");
        let grant_file = std::fs::read_dir(store.join("facts"))
            .expect("lists")
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().contains(&grant))
            .expect("grant file exists");
        std::fs::remove_file(grant_file.path()).expect("removes");
        // The parse cache is derived; drop it so the removal is what fails.
        let _ = std::fs::remove_file(store.join("cache/parsed.json"));
        let error = verify(dir.path()).expect_err("removed antecedent fails");
        assert!(
            error.contains("disappeared") || error.contains("exists nowhere"),
            "{error}"
        );
    }

    #[test]
    fn verify_fails_a_compacted_unmatched_park_and_compact_refuses_to_move_one() {
        let dir = repo();
        record(
            dir.path(),
            "ruling.park",
            "2026-07-01T00:00:00Z",
            None,
            serde_json::json!({ "target": "todo.parked" }),
        );
        // A sanctioned compact refuses to move the unmatched park.
        let moved = compact(dir.path(), "2026-08-01").expect("compacts");
        assert!(moved.is_empty(), "unmatched park stays live: {moved:?}");
        verify(dir.path()).expect("still clean");

        // Force the violation by hand, as a misbehaving tool would.
        let store = dir.path().join(".git/cairn/coord");
        let park_file = std::fs::read_dir(store.join("facts"))
            .expect("lists")
            .find_map(Result::ok)
            .expect("park file exists");
        let name = park_file.file_name().to_string_lossy().to_string();
        let month_dir = store.join("archive/2026-07");
        std::fs::create_dir_all(&month_dir).expect("archive month");
        std::fs::rename(park_file.path(), month_dir.join(&name)).expect("moves");
        let _ = std::fs::remove_file(store.join("cache/parsed.json"));
        let error = verify(dir.path()).expect_err("compacted unmatched park fails");
        assert!(error.contains("unmatched `ruling.park`"), "{error}");
    }

    #[test]
    fn compact_moves_old_facts_but_keeps_live_chain_antecedents() {
        let dir = repo();
        let old = record(
            dir.path(),
            "lease.grant",
            "2026-07-01T00:00:00Z",
            None,
            serde_json::json!({ "unit_id": "todo.x" }),
        );
        record(
            dir.path(),
            "lease.renew",
            "2026-08-05T00:00:00Z",
            Some(old),
            serde_json::json!({ "unit_id": "todo.x" }),
        );
        record(
            dir.path(),
            "ruling.run",
            "2026-07-02T00:00:00Z",
            None,
            serde_json::json!({ "target": "plan-0123456789abcdef" }),
        );
        let moved = compact(dir.path(), "2026-08-01").expect("compacts");
        assert_eq!(moved.len(), 1, "only the unchained old ruling moves");
        assert!(moved[0].contains("ruling.run"), "{moved:?}");
        verify(dir.path()).expect("archive keeps the store verifiable");
    }
}
