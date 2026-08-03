//! Acceptance tests for `cairn pending` (`todo.maintainer-pending-queue`):
//! the maintainer ratification queue lists every proposed decision and no
//! other, oldest first, and empties a row the moment its decision is
//! accepted.

use std::path::Path;
use std::process::Output;

use serde_json::Value;

fn write_fixture(root: &Path) {
    std::fs::create_dir_all(root.join("meta/decisions")).unwrap();
    std::fs::write(
        root.join("cairn.blueprint"),
        "System Test \"T\" id \"t\" {\n    decisions \"./meta/decisions\"\n}\n",
    )
    .unwrap();
    for (file, id, status, date) in [
        ("older", "dec.older", "proposed", "2020-01-01"),
        ("newer", "dec.newer", "proposed", "2021-06-01"),
        ("accepted", "dec.accepted", "accepted", "2019-01-01"),
        ("superseded", "dec.superseded", "superseded", "2019-01-02"),
        ("deprecated", "dec.deprecated", "deprecated", "2019-01-03"),
    ] {
        write_decision(root, file, id, status, date);
    }
}

fn write_decision(root: &Path, file: &str, id: &str, status: &str, date: &str) {
    std::fs::write(
        root.join(format!("meta/decisions/{file}.md")),
        format!("---\nid: {id}\nnodes: [t]\nstatus: {status}\ndate: {date}\n---\n# {id}\n"),
    )
    .unwrap();
}

fn pending(root: &Path, args: &[&str]) -> Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args(args)
        .output()
        .expect("pending command runs")
}

fn pending_ids(root: &Path) -> Vec<String> {
    let output = pending(root, &["pending", "--json"]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        value["schema_version"].is_u64(),
        "data payload must carry schema_version: {value}"
    );
    value["pending"]
        .as_array()
        .expect("pending array")
        .iter()
        .map(|row| row["id"].as_str().expect("id").to_owned())
        .collect()
}

#[test]
fn pending_lists_every_proposed_decision_and_no_other_oldest_first() {
    let dir = tempfile::tempdir().unwrap();
    write_fixture(dir.path());
    // Written out of order on purpose: the wire order must come from the
    // sort (age descending, ties by id ascending), not file insertion.
    write_decision(dir.path(), "tie-z", "dec.zzz-tie", "proposed", "2021-01-01");
    write_decision(dir.path(), "tie-a", "dec.aaa-tie", "proposed", "2021-01-01");
    write_decision(dir.path(), "future", "dec.future", "proposed", "2099-01-01");
    assert_eq!(
        pending_ids(dir.path()),
        [
            "dec.older",
            "dec.aaa-tie",
            "dec.zzz-tie",
            "dec.newer",
            "dec.future"
        ]
    );

    let output = pending(dir.path(), &["pending", "--json"]);
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    let rows = value["pending"].as_array().expect("pending array");
    let first = &rows[0];
    assert!(first["age_days"].as_i64().expect("signed age") > 0);
    assert_eq!(first["nodes"], serde_json::json!(["t"]));
    assert_eq!(first["ratification"], "binding");
    // The future-dated decision carries a negative signed age and sorts last.
    let last = rows.last().expect("future row");
    assert_eq!(last["id"], "dec.future");
    assert!(last["age_days"].as_i64().expect("signed age") < 0);
}

#[test]
fn accepting_a_decision_empties_its_row_on_the_next_run() {
    let dir = tempfile::tempdir().unwrap();
    write_fixture(dir.path());
    assert_eq!(pending_ids(dir.path()), ["dec.older", "dec.newer"]);

    // Flip the older decision to accepted with no other edit.
    write_decision(dir.path(), "older", "dec.older", "accepted", "2020-01-01");
    assert_eq!(pending_ids(dir.path()), ["dec.newer"]);
}

#[test]
fn human_output_renders_rows_from_the_same_computation() {
    let dir = tempfile::tempdir().unwrap();
    write_fixture(dir.path());
    let output = pending(dir.path(), &["pending"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pending decisions:"), "{stdout}");
    let older = stdout.find("dec.older").expect("older row");
    let newer = stdout.find("dec.newer").expect("newer row");
    assert!(older < newer, "oldest first: {stdout}");
    assert!(stdout.contains("binding"), "{stdout}");
    assert!(!stdout.contains("dec.accepted"), "{stdout}");
}

#[test]
fn empty_queue_renders_the_empty_state() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("meta/decisions")).unwrap();
    std::fs::write(
        dir.path().join("cairn.blueprint"),
        "System Test \"T\" id \"t\" {\n    decisions \"./meta/decisions\"\n}\n",
    )
    .unwrap();
    let output = pending(dir.path(), &["pending"]);
    assert!(output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("(none)"),
        "empty state must render"
    );
}

#[test]
fn unparseable_date_on_a_proposed_decision_fails_deterministically() {
    let dir = tempfile::tempdir().unwrap();
    write_fixture(dir.path());
    write_decision(dir.path(), "older", "dec.older", "proposed", "2020-99-99");
    for args in [&["pending"][..], &["pending", "--json"][..]] {
        let output = pending(dir.path(), args);
        assert!(!output.status.success(), "invalid date must fail: {args:?}");
        let all = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(all.contains("CAIRN_PENDING_INVALID_DATE"), "{all}");
        assert!(all.contains("dec.older"), "{all}");
    }
}

#[test]
fn pending_id_filters_json_and_human_to_one_row() {
    let dir = tempfile::tempdir().unwrap();
    write_fixture(dir.path());
    let output = pending(dir.path(), &["pending", "dec.older", "--json"]);
    assert!(output.status.success(), "id-filtered json must succeed");
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    let rows = value["pending"].as_array().expect("pending array");
    assert_eq!(rows.len(), 1, "one row for the requested id: {value}");
    assert_eq!(rows[0]["id"], "dec.older");

    let output = pending(dir.path(), &["pending", "dec.older"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("dec.older"), "{stdout}");
    assert!(
        !stdout.contains("dec.newer"),
        "detail is single-row: {stdout}"
    );
}

#[test]
fn pending_unknown_id_fails_on_both_paths() {
    let dir = tempfile::tempdir().unwrap();
    write_fixture(dir.path());
    for args in [
        &["pending", "dec.absent"][..],
        &["pending", "dec.absent", "--json"][..],
    ] {
        let output = pending(dir.path(), args);
        assert!(!output.status.success(), "unknown id must fail: {args:?}");
        let all = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(all.contains("dec.absent"), "{all}");
    }
}

#[test]
fn context_surfaces_a_pending_error_instead_of_an_empty_queue() {
    let dir = tempfile::tempdir().unwrap();
    write_fixture(dir.path());
    write_decision(dir.path(), "older", "dec.older", "proposed", "2020-99-99");
    for args in [&["context"][..], &["context", "--json"][..]] {
        let output = pending(dir.path(), args);
        assert!(
            !output.status.success(),
            "context must fail on an unreadable queue: {args:?}"
        );
        let all = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(all.contains("CAIRN_PENDING_INVALID_DATE"), "{all}");
        assert!(
            !all.contains("Nothing is waiting"),
            "must not render an honest-empty state: {all}"
        );
    }
}

#[test]
fn context_opens_with_where_work_was_left_on_both_surfaces() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("meta/decisions")).unwrap();
    std::fs::create_dir_all(dir.path().join("meta/todos")).unwrap();
    std::fs::create_dir_all(dir.path().join("meta/changes/live-change")).unwrap();
    std::fs::write(
        dir.path().join("cairn.blueprint"),
        "System Test \"T\" id \"t\" {\n    decisions \"./meta/decisions\"\n    Container Work \"work\" id \"t.work\" {\n        path \"./meta/todos\"\n        todos \"./meta/todos\"\n    }\n}\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("meta/todos/todo.live-unit.md"),
        "---\nnode: t.work\nstatus: in_progress\ncreated: 2026-01-01\n---\n# Live unit\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("meta/changes/live-change/proposal.md"),
        "# Live change\n\n## Why\n\nKeep going.\n",
    )
    .unwrap();
    std::fs::create_dir_all(dir.path().join(".beads")).unwrap();
    std::fs::write(
        dir.path().join(".beads/issues.jsonl"),
        "{\"id\":\"cairn-live\",\"title\":\"Live bead\",\"status\":\"in_progress\",\"priority\":1}\n",
    )
    .unwrap();

    let output = pending(dir.path(), &["context"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Where work was left:"), "{stdout}");
    assert!(stdout.contains("todo.live-unit"), "{stdout}");
    assert!(stdout.contains("live-change"), "{stdout}");
    assert!(stdout.contains("bead cairn-live"), "{stdout}");

    let output = pending(dir.path(), &["context", "--json"]);
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    let left = &value["waiting_on_you"]["where_left"];
    assert_eq!(left["in_progress"][0]["stem"], "todo.live-unit");
    assert_eq!(left["active_changes"][0]["id"], "live-change");
    assert_eq!(left["in_progress_backlog"][0]["id"], "cairn-live");
}
