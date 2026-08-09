//! Regression coverage for coordination verification and compaction.
use super::*;
use crate::coord::append::{NewFact, append_fact};
use crate::coord::envelope::{Actor, fact_id_for};

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
        serde_json::json!({
            "unit_id": "todo.x",
            "expires_at": "2026-08-01T01:00:00Z",
        }),
    );
    record(
        dir.path(),
        "lease.renew",
        "2026-08-02T00:00:00Z",
        Some(grant.clone()),
        serde_json::json!({
            "unit_id": "todo.x",
            "expires_at": "2026-08-02T01:00:00Z",
        }),
    );
    verify(dir.path()).expect("clean store verifies");

    let store = dir.path().join(".git/cairn/coord");
    let grant_file = std::fs::read_dir(store.join("facts"))
        .expect("lists")
        .filter_map(Result::ok)
        .find(|entry| entry.file_name().to_string_lossy().contains(&grant))
        .expect("grant file exists");
    std::fs::remove_file(grant_file.path()).expect("removes");
    let error = verify(dir.path()).expect_err("removed antecedent fails");
    assert!(
        error.contains("disappeared") || error.contains("exists nowhere"),
        "{error}"
    );
}

#[test]
fn verify_recomputes_fact_identity_before_accepting_the_store() {
    let dir = repo();
    let path = append_fact(
        dir.path(),
        NewFact {
            kind: "ruling.run".to_owned(),
            recorded_at: "2026-08-07T03:45:12Z".to_owned(),
            recorded_by: Actor {
                kind: "driver".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: None,
            payload: serde_json::json!({ "target": "plan-0123456789abcdef" }),
        },
    )
    .expect("appends");
    let mut raw: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("fact bytes"))
            .expect("fact json");
    raw["payload"]["target"] = serde_json::json!("plan-fedcba9876543210");
    std::fs::write(
        &path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&raw).expect("serialises")
        ),
    )
    .expect("mutates fact");
    let error = verify(dir.path()).expect_err("tampered identity fails verification");
    assert!(error.contains("identity"), "{error}");
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
        serde_json::json!({
            "unit_id": "todo.x",
            "expires_at": "2026-07-01T01:00:00Z",
        }),
    );
    record(
        dir.path(),
        "lease.renew",
        "2026-08-05T00:00:00Z",
        Some(old),
        serde_json::json!({
            "unit_id": "todo.x",
            "expires_at": "2026-08-05T01:00:00Z",
        }),
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
#[test]
fn archived_fact_identity_and_filename_are_validated() {
    let dir = repo();
    record(
        dir.path(),
        "ruling.run",
        "2026-07-01T00:00:00Z",
        None,
        serde_json::json!({ "target": "plan-0123456789abcdef" }),
    );
    compact(dir.path(), "2026-08-01").expect("compacts");
    let archive = dir.path().join(".git/cairn/coord/archive/2026-07");
    let path = std::fs::read_dir(&archive)
        .expect("lists archive")
        .find_map(Result::ok)
        .expect("archived fact")
        .path();
    let mut fact: Envelope =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("fact bytes"))
            .expect("fact json");
    fact.payload = serde_json::json!({ "target": "plan-fedcba9876543210" });
    fact.fact_id = String::new();
    fact.fact_id = fact_id_for(&fact).expect("recomputes");
    std::fs::write(
        &path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&fact).expect("serialises")
        ),
    )
    .expect("mutates archive");
    let Err(error) = verify(dir.path()) else {
        panic!("tampered archive succeeds");
    };
    assert!(
        error.contains("filename") || error.contains("identity"),
        "{error}"
    );
}

#[test]
fn verify_rejects_a_fact_identity_present_in_live_and_archive() {
    let dir = repo();
    record(
        dir.path(),
        "ruling.run",
        "2026-07-01T00:00:00Z",
        None,
        serde_json::json!({ "target": "plan-0123456789abcdef" }),
    );
    compact(dir.path(), "2026-08-01").expect("compacts");
    let store = dir.path().join(".git/cairn/coord");
    let archive = store.join("archive/2026-07");
    let archived = std::fs::read_dir(&archive)
        .expect("lists archive")
        .find_map(Result::ok)
        .expect("archived fact");
    let name = archived.file_name();
    std::fs::copy(archived.path(), store.join("facts").join(&name)).expect("duplicates fact");
    let error = verify(dir.path()).expect_err("cross-set duplicate fails");
    assert!(error.contains("both live facts and archive"), "{error}");
}

#[test]
fn compact_refuses_replacing_an_existing_archive_target() {
    let dir = repo();
    let path = crate::coord::append::append_fact(
        dir.path(),
        NewFact {
            kind: "ruling.run".to_owned(),
            recorded_at: "2026-07-01T00:00:00Z".to_owned(),
            recorded_by: Actor {
                kind: "driver".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: None,
            payload: serde_json::json!({ "target": "plan-0123456789abcdef" }),
        },
    )
    .expect("appends");
    let name = path.file_name().expect("name").to_owned();
    let bytes = std::fs::read(&path).expect("source bytes");
    let store = dir.path().join(".git/cairn/coord");
    let target_dir = store.join("archive/2026-07");
    std::fs::create_dir_all(&target_dir).expect("archive month");
    let target = target_dir.join(&name);
    std::fs::write(&target, &bytes).expect("target");
    let error = compact(dir.path(), "2026-08-01").expect_err("replacement refused");
    assert!(error.contains("without replacement"), "{error}");
    assert!(path.is_file(), "source remains live");
    assert_eq!(std::fs::read(&target).expect("target bytes"), bytes);
}
