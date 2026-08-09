//! Regression coverage for the ruling-run consent surface.
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
    let scan = crate::scanner::load_project(root, &root.join("cairn.blueprint")).expect("loads");
    crate::query_api::wave::compose::compose_wave(root, &scan.graph, &scan.artefacts.todos, None)
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

fn append_sidecar_read_fact(root: &Path, recorded_at: &str, sidecar: &str) {
    crate::coord::append::append_fact(
        root,
        crate::coord::append::NewFact {
            kind: "outcome.run_declined".to_owned(),
            recorded_at: recorded_at.to_owned(),
            recorded_by: crate::coord::envelope::Actor {
                kind: "driver".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: None,
            payload: serde_json::json!({
                "target": "plan-0123456789abcdef",
                "reason": "unit-set-moved",
                "preimage_diff_sidecar": sidecar,
            }),
        },
    )
    .expect("decline fact appends");
}

#[test]
fn repeat_oversized_declines_use_observation_sidecars() {
    let dir = project();
    let content = "x".repeat(4097);
    let path = spill_preimage(
        dir.path(),
        "plan-0123456789abcdef",
        "2026-08-07T03:45:12Z",
        &content,
    )
    .expect("spills preimage");
    let retry = spill_preimage(
        dir.path(),
        "plan-0123456789abcdef",
        "2026-08-07T03:45:13Z",
        &content,
    )
    .expect("a later observation gets a distinct sidecar");
    append_sidecar_read_fact(dir.path(), "2026-08-07T03:45:12Z", &path);
    append_sidecar_read_fact(dir.path(), "2026-08-07T03:45:13Z", &retry);
    assert_ne!(path, retry, "observation seconds key retries");
    assert!(
        path.starts_with("sidecars/"),
        "immutable facts must not reference cache sidecars: {path}"
    );
    let store = crate::coord::store::store_root(dir.path()).expect("store root");
    assert!(store.join(&path).is_file(), "sidecar exists at {path}");
    assert!(
        store.join(&retry).is_file(),
        "retry sidecar exists at {retry}"
    );
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
    assert_eq!(facts.len(), 2, "both decline facts remain readable");
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

#[test]
fn a_compacted_consumed_digest_stays_single_use() {
    let dir = project();
    let digest = current_digest(dir.path());
    crate::coord::append::append_fact(
        dir.path(),
        crate::coord::append::NewFact {
            kind: "outcome.run_consumed".to_owned(),
            recorded_at: "2026-07-01T00:00:00Z".to_owned(),
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
    crate::coord::verify::compact(dir.path(), "2026-08-01").expect("compacts");

    let result = run_ruling_run(&parsed(dir.path(), &["run", &digest]), dir.path(), &digest);
    assert_eq!(result.code, 1);
    let declines = facts_of_kind(dir.path(), "outcome.run_declined");
    assert_eq!(declines.len(), 1);
    assert_eq!(declines[0].payload["reason"], "already-consumed");
    assert!(
        facts_of_kind(dir.path(), "ruling.run").is_empty(),
        "archived consent remains single-use"
    );
}
