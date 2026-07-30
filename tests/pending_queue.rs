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
