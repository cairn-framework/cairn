//! `cairn ruling run <plan-digest>`: the maintainer consent surface
//! (`res.parallel-dispatch-rung-3` Part 1, clause 1).
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

/// True when `digest` is `plan-` plus sixteen lowercase hex characters.
fn valid_plan_digest(digest: &str) -> bool {
    digest
        .strip_prefix("plan-")
        .is_some_and(|hex| hex.len() == 16 && hex.bytes().all(|b| b.is_ascii_hexdigit()))
}

/// `cairn ruling run <plan-digest>`: the maintainer consent surface.
///
/// Recomputes the wave at HEAD. On recompute-equality it records exactly
/// one `ruling.run` fact whose `payload.target` is the digest; otherwise it
/// records `outcome.run_declined` with a reason from the closed enum and
/// `dispatched: []`, because dispatch is all-or-nothing per wave. The CLI
/// holds only the consented digest, never its preimage, so the recomputed
/// preimage is recorded as the diffable side and attribution beyond
/// `parked` and `already-consumed` falls to `unit-set-moved`.
pub(crate) fn run_ruling_run(parsed: &ParsedArgs, root: &Path, digest: &str) -> CliResult {
    if !valid_plan_digest(digest) {
        return err(1, copy::lookup("ruling.run-invalid-digest"));
    }
    let scan_result = match crate::scanner::load_project(root, &parsed.file) {
        Ok(result) => result,
        Err(message) => return err(1, &message),
    };
    let wave = match crate::query_api::wave::compose::compose_wave(
        root,
        &scan_result.graph,
        &scan_result.artefacts.todos,
        None,
    ) {
        Ok(wave) => wave,
        Err(message) => return err(1, &message),
    };
    let commit = match crate::coord::git::head_commit(root) {
        Ok(commit) => commit,
        Err(message) => return err(1, &message),
    };
    let observed_at = crate::coord::time::rfc3339_utc(std::time::SystemTime::now());
    let actor = crate::coord::envelope::Actor {
        kind: "maintainer".to_owned(),
        id: std::env::var("USER").unwrap_or_else(|_| "maintainer".to_owned()),
    };

    let facts = match crate::coord::read::read_facts(root) {
        Ok(crate::coord::read::StoreRead::Ready(facts)) => facts,
        Ok(crate::coord::read::StoreRead::Uninitialised) => Vec::new(),
        Err(message) => return err(1, &message),
    };
    let consumed = match consumed_digest(root, &facts, digest) {
        Ok(consumed) => consumed,
        Err(message) => return err(1, &message),
    };

    if !consumed && wave.digest == digest {
        return record_consent(parsed, root, digest, observed_at, actor, commit);
    }
    record_decline(
        parsed,
        root,
        digest,
        consumed,
        &wave,
        observed_at,
        actor,
        commit,
    )
}

fn consumed_digest(
    root: &Path,
    facts: &[crate::coord::read::NamedFact],
    digest: &str,
) -> Result<bool, String> {
    if facts.iter().any(|named| {
        named.fact.kind == "outcome.run_consumed"
            && named
                .fact
                .payload
                .get("target")
                .and_then(serde_json::Value::as_str)
                == Some(digest)
    }) {
        return Ok(true);
    }
    crate::coord::verify::archived_fact_has_target(root, "outcome.run_consumed", digest)
}

/// Appends the single `ruling.run` fact for a digest that still holds.
fn record_consent(
    parsed: &ParsedArgs,
    root: &Path,
    digest: &str,
    observed_at: String,
    actor: crate::coord::envelope::Actor,
    commit: String,
) -> CliResult {
    let appended = crate::coord::append::append_fact(
        root,
        crate::coord::append::NewFact {
            kind: "ruling.run".to_owned(),
            recorded_at: observed_at,
            recorded_by: actor,
            commit,
            supersedes: None,
            payload: serde_json::json!({ "target": digest }),
        },
    );
    match appended {
        Ok(path) => {
            if parsed.json {
                ok(serde_json::json!({
                    "schema_version": crate::query_api::SCHEMA_VERSION,
                    "recorded": "ruling.run",
                    "target": digest,
                    "fact": path.display().to_string(),
                })
                .to_string())
            } else {
                ok(copy::lookup("ruling.run-recorded").replace("{digest}", digest))
            }
        }
        Err(message) => err(1, &message),
    }
}

/// Appends `outcome.run_declined`: dispatched is `[]` on every branch,
/// because dispatch is all-or-nothing per wave.
#[allow(clippy::too_many_arguments)]
// Reason: the decline path threads the already-resolved envelope fields; a struct would restate them once.
fn record_decline(
    parsed: &ParsedArgs,
    root: &Path,
    digest: &str,
    consumed: bool,
    wave: &crate::query_api::wave::compose::Wave,
    observed_at: String,
    actor: crate::coord::envelope::Actor,
    commit: String,
) -> CliResult {
    use std::fmt::Write as _;
    let (reason, causes) = decline_reason(digest, consumed, wave);
    let mut recomputed = String::new();
    for line in wave.preimage.lines() {
        let _ = writeln!(recomputed, "+{line}");
    }
    let mut payload = serde_json::json!({
        "target": digest,
        "reason": reason,
        "observed_at": observed_at,
        "head": commit,
        "causes": causes,
        "recomputed_digest": wave.digest,
        "dispatched": [],
    });
    if recomputed.len() <= 4096 {
        payload["preimage_diff"] = serde_json::json!(recomputed);
    } else {
        let sidecar = match spill_preimage(root, digest, &observed_at, &recomputed) {
            Ok(path) => path,
            Err(message) => return err(1, &message),
        };
        payload["preimage_diff_sidecar"] = serde_json::json!(sidecar);
    }
    let appended = crate::coord::append::append_fact(
        root,
        crate::coord::append::NewFact {
            kind: "outcome.run_declined".to_owned(),
            recorded_at: observed_at,
            recorded_by: actor,
            commit,
            supersedes: None,
            payload,
        },
    );
    match appended {
        Ok(_) => {
            if parsed.json {
                err(
                    1,
                    &serde_json::json!({
                        "schema_version": crate::query_api::SCHEMA_VERSION,
                        "recorded": "outcome.run_declined",
                        "target": digest,
                        "reason": reason,
                        "dispatched": [],
                    })
                    .to_string(),
                )
            } else {
                err(
                    1,
                    &copy::lookup("ruling.run-declined")
                        .replace("{digest}", digest)
                        .replace("{reason}", reason),
                )
            }
        }
        Err(message) => err(1, &message),
    }
}

/// Chooses the decline reason and its structured causes from what the CLI
/// can observe: a consumed digest, parked units, else recompute mismatch.
fn decline_reason(
    digest: &str,
    consumed: bool,
    wave: &crate::query_api::wave::compose::Wave,
) -> (&'static str, Vec<serde_json::Value>) {
    if consumed {
        return (
            "already-consumed",
            vec![serde_json::json!({
                "unit": serde_json::Value::Null,
                "predicate": "already-consumed",
                "blocking_fact_id": serde_json::Value::Null,
                "detail": format!("digest {digest} already carries an outcome.run_consumed fact"),
            })],
        );
    }
    let parked: Vec<serde_json::Value> = wave
        .held
        .iter()
        .filter(|entry| entry.reason == "parked")
        .map(|entry| {
            serde_json::json!({
                "unit": entry.id,
                "predicate": "parked",
                "blocking_fact_id": entry.blocking_fact_id,
                "detail": format!("{} is parked by a live ruling.park fact", entry.id),
            })
        })
        .collect();
    if !parked.is_empty() {
        return ("parked", parked);
    }
    (
        "unit-set-moved",
        vec![serde_json::json!({
            "unit": serde_json::Value::Null,
            "predicate": "recompute-mismatch",
            "blocking_fact_id": serde_json::Value::Null,
            "detail": format!(
                "the recomputed composition yields {}, not {digest}",
                wave.digest
            ),
        })],
    )
}

/// Spills an oversized recomputed preimage to the immutable `sidecars/`
/// subtree, keyed by digest and observation second; returns the store-relative
/// path.
fn spill_preimage(
    root: &Path,
    digest: &str,
    recorded_at: &str,
    recomputed: &str,
) -> Result<String, String> {
    let store = crate::coord::store::store_root(root)?;
    let compacted = crate::coord::time::compact_rfc3339(recorded_at);
    let name = format!("sidecars/preimage-{digest}-{compacted}.diff");
    crate::persist::atomic_write_once(&store.join(&name), recomputed)
        .map_err(|error| format!("cannot spill preimage diff: {error}"))?;
    Ok(name)
}

#[cfg(test)]
#[path = "ruling_run_regressions.rs"]
mod tests;
