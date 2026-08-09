//! Store integrity: `verify` checks the append-only discipline held, and
//! `compact` moves old facts to `archive/` without ever deleting.
//!
//! Two facts are never compactable: a `ruling.park` with no matching
//! `unpark` (the ready-set projection honours it indefinitely), and any fact
//! that is the antecedent of a live `supersedes` chain.

use std::collections::BTreeSet;
use std::path::Path;

use crate::artefacts::registry::dates::date_to_days;
use crate::persist;

use super::envelope::Envelope;
use super::read::{NamedFact, StoreRead, read_store, validate};
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
            let is_json = std::path::Path::new(&name)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"));
            if !is_json {
                continue;
            }
            let raw = std::fs::read_to_string(entry.path())
                .map_err(|error| format!("cannot read archived fact `{name}`: {error}"))?;
            let fact: Envelope = serde_json::from_str(&raw)
                .map_err(|error| format!("archived fact `{name}` does not parse: {error}"))?;
            validate(&name, &fact)?;
            out.push(NamedFact { name, fact });
        }
    }
    Ok(out)
}

/// Checks archived facts without mutating the verification observation cache.
pub(crate) fn archived_fact_has_target(
    root: &Path,
    kind: &str,
    target: &str,
) -> Result<bool, String> {
    let store = store::store_root(root)?;
    let archived = read_archive(&store)?;
    Ok(archived.iter().any(|named| {
        named.fact.kind == kind
            && named
                .fact
                .payload
                .get("target")
                .and_then(serde_json::Value::as_str)
                == Some(target)
    }))
}

/// True when `park` has a matching `ruling.unpark` anywhere in the store.
fn park_is_matched(park: &Envelope, all: &[&NamedFact]) -> bool {
    all.iter().any(|candidate| {
        candidate.fact.kind == "ruling.unpark"
            && candidate.fact.payload.get("target") == park.payload.get("target")
            && candidate.fact.recorded_at >= park.recorded_at
    })
}

fn move_once(source: &Path, target: &Path) -> Result<(), std::io::Error> {
    std::fs::hard_link(source, target)?;
    std::fs::remove_file(source)
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
    if let Some(duplicate) = live_ids.intersection(&archived_ids).next() {
        return Err(format!(
            "fact identity `{duplicate}` appears in both live facts and archive"
        ));
    }
    let everything: Vec<&NamedFact> = live.iter().chain(archived.iter()).collect();

    // Append-only: every previously observed fact still exists somewhere.
    let observed_path = store.join("cache/observed.json");
    let mut observed: BTreeSet<String> = match persist::read_json(&observed_path) {
        Ok(observed) => observed,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => BTreeSet::new(),
        Err(error) => {
            return Err(format!("cannot read coordination observation: {error}"));
        }
    };
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
            if live_ids.contains(named.fact.fact_id.as_str()) && in_archive {
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
    persist::write_json(&observed_path, &observed)
        .map_err(|error| format!("cannot record coordination observation: {error}"))?;
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
    if date_to_days(before).is_none() {
        return Err(format!("`{before}` is not a YYYY-MM-DD date"));
    }
    let cutoff: String = before.chars().filter(char::is_ascii_digit).collect();
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
        move_once(&store.join("facts").join(&name), &target_dir.join(&name))
            .map_err(|error| format!("cannot archive `{name}` without replacement: {error}"))?;
        moved.push(name);
    }
    // Remove a legacy parsed cache left by builds before cache elimination;
    // current reads neither create nor trust this file.
    let _ = std::fs::remove_file(store.join("cache/parsed.json"));
    Ok(moved)
}

#[cfg(test)]
#[path = "verify_regressions.rs"]
mod tests;
