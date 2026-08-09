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
    let consumed = facts.iter().any(|named| {
        named.fact.kind == "outcome.run_consumed"
            && named
                .fact
                .payload
                .get("target")
                .and_then(serde_json::Value::as_str)
                == Some(digest)
    });

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
        let sidecar = match spill_preimage(root, digest, &recomputed) {
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
/// subtree; returns the store-relative path.
fn spill_preimage(root: &Path, digest: &str, recomputed: &str) -> Result<String, String> {
    let store = crate::coord::store::store_root(root)?;
    let name = format!("sidecars/preimage-{digest}.diff");
    crate::persist::atomic_write_once(&store.join(&name), recomputed)
        .map_err(|error| format!("cannot spill preimage diff: {error}"))?;
    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project() -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let run = |args: &[&str]| {
            let output = std::process::Command::new("git")
                .arg("-C")
                .arg(dir.path())
                .args(args)
                .output()
                .expect("git runs");
            assert!(output.status.success());
        };
        run(&["init", "-q"]);
        std::fs::create_dir_all(dir.path().join("src/a")).expect("src dir");
        std::fs::write(dir.path().join("src/a/lib.rs"), "pub fn a() {}\n").expect("source");
        std::fs::create_dir_all(dir.path().join("meta/todos")).expect("todos dir");
        std::fs::write(
            dir.path().join("meta/todos/todo.alpha.md"),
            "---\nnode: app.a\nstatus: open\ncreated: 2026-08-07\n---\n\n# Alpha\n",
        )
        .expect("todo");
        std::fs::write(
            dir.path().join("cairn.blueprint"),
            "System App \"app\" id \"app\" {\n    todos \"./meta/todos\"\n    Module A \"a\" id \"app.a\" {\n        path \"./src/a\"\n    }\n}\n",
        )
        .expect("blueprint");
        run(&["add", "-A"]);
        run(&[
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "commit",
            "-q",
            "-m",
            "base",
        ]);
        dir
    }

    fn parsed(root: &Path, args: &[&str]) -> ParsedArgs {
        ParsedArgs {
            json: false,
            strict: false,
            verbose: false,
            brief: false,
            file: root.join("cairn.blueprint"),
            changes_dir: PathBuf::from("meta/changes"),
            command: "ruling".to_owned(),
            command_args: std::iter::once("ruling")
                .chain(args.iter().copied())
                .map(ToOwned::to_owned)
                .collect(),
        }
    }

    fn current_digest(root: &Path) -> String {
        let scan =
            crate::scanner::load_project(root, &root.join("cairn.blueprint")).expect("loads");
        crate::query_api::wave::compose::compose_wave(
            root,
            &scan.graph,
            &scan.artefacts.todos,
            None,
        )
        .expect("composes")
        .digest
    }

    fn facts_of_kind(root: &Path, kind: &str) -> Vec<crate::coord::envelope::Envelope> {
        match crate::coord::read::read_facts(root).expect("reads") {
            crate::coord::read::StoreRead::Uninitialised => Vec::new(),
            crate::coord::read::StoreRead::Ready(facts) => facts
                .into_iter()
                .map(|named| named.fact)
                .filter(|fact| fact.kind == kind)
                .collect(),
        }
    }

    #[test]
    fn an_invalid_digest_shape_is_refused_before_any_write() {
        let dir = project();
        let result = run_ruling_run(
            &parsed(dir.path(), &["run", "plan-xyz"]),
            dir.path(),
            "plan-xyz",
        );
        assert_eq!(
            result.stderr.trim(),
            copy::lookup("ruling.run-invalid-digest")
        );
        assert_eq!(result.code, 1);
        assert!(
            !dir.path().join(".git/cairn").exists(),
            "a refused digest writes nothing"
        );
    }

    #[test]
    fn oversized_decline_preimage_is_not_written_under_disposable_cache() {
        let dir = project();
        crate::coord::append::append_fact(
            dir.path(),
            crate::coord::append::NewFact {
                kind: "ruling.run".to_owned(),
                recorded_at: "2026-08-07T03:45:12Z".to_owned(),
                recorded_by: crate::coord::envelope::Actor {
                    kind: "driver".to_owned(),
                    id: "t".to_owned(),
                },
                commit: "a".repeat(40),
                supersedes: None,
                payload: serde_json::json!({ "target": "plan-0123456789abcdef" }),
            },
        )
        .expect("fact appends");
        let content = "x".repeat(4097);
        let path =
            spill_preimage(dir.path(), "plan-0123456789abcdef", &content).expect("spills preimage");
        assert!(
            path.starts_with("sidecars/"),
            "immutable facts must not reference cache sidecars: {path}"
        );
        let store = crate::coord::store::store_root(dir.path()).expect("store root");
        assert!(store.join(&path).is_file(), "sidecar exists at {path}");
        assert!(
            !store
                .join("cache/preimage-plan-0123456789abcdef.diff")
                .exists(),
            "cache remains disposable"
        );
        let crate::coord::read::StoreRead::Ready(facts) =
            crate::coord::read::read_facts(dir.path()).expect("sidecar does not poison reads")
        else {
            panic!("store is initialised");
        };
        assert_eq!(facts.len(), 1, "the fact remains readable");
    }

    #[test]
    fn a_holding_digest_records_consent_and_a_moved_one_declines() {
        let dir = project();
        let digest = current_digest(dir.path());
        let result = run_ruling_run(&parsed(dir.path(), &["run", &digest]), dir.path(), &digest);
        assert_eq!(result.code, 0, "{}", result.stderr);
        let runs = facts_of_kind(dir.path(), "ruling.run");
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].payload["target"], serde_json::json!(digest));
        assert_eq!(runs[0].commit.len(), 40);
        assert_eq!(runs[0].recorded_by.kind, "maintainer");

        // Rewrite the task definition: the digest moves, the old one
        // declines with a closed-enum reason and dispatches nothing.
        std::fs::write(
            dir.path().join("meta/todos/todo.alpha.md"),
            "---\nnode: app.a\nstatus: open\ncreated: 2026-08-07\n---\n\n# Alpha rewritten\n",
        )
        .expect("rewrite");
        let stale = run_ruling_run(&parsed(dir.path(), &["run", &digest]), dir.path(), &digest);
        assert_eq!(stale.code, 1);
        let declines = facts_of_kind(dir.path(), "outcome.run_declined");
        assert_eq!(declines.len(), 1);
        assert_eq!(declines[0].payload["reason"], "unit-set-moved");
        assert_eq!(declines[0].payload["dispatched"], serde_json::json!([]));
        assert!(declines[0].payload["preimage_diff"].is_string());
    }

    #[test]
    fn a_consumed_digest_declines_already_consumed_with_nothing_dispatched() {
        let dir = project();
        let digest = current_digest(dir.path());
        crate::coord::append::append_fact(
            dir.path(),
            crate::coord::append::NewFact {
                kind: "outcome.run_consumed".to_owned(),
                recorded_at: "2026-08-07T03:00:00Z".to_owned(),
                recorded_by: crate::coord::envelope::Actor {
                    kind: "driver".to_owned(),
                    id: "t".to_owned(),
                },
                commit: "a".repeat(40),
                supersedes: None,
                payload: serde_json::json!({ "target": digest }),
            },
        )
        .expect("consumed fact");
        let result = run_ruling_run(&parsed(dir.path(), &["run", &digest]), dir.path(), &digest);
        assert_eq!(result.code, 1);
        let declines = facts_of_kind(dir.path(), "outcome.run_declined");
        assert_eq!(declines.len(), 1);
        assert_eq!(declines[0].payload["reason"], "already-consumed");
        assert_eq!(declines[0].payload["dispatched"], serde_json::json!([]));
        assert!(
            facts_of_kind(dir.path(), "ruling.run").is_empty(),
            "consent is single-use; no second consent is recorded"
        );
    }
}
