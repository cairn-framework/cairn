//! Regression coverage for malformed lease reads and immutable fact names.
use super::*;
use crate::coord::append::{NewFact, append_fact};
use crate::coord::envelope::{Actor, Envelope, fact_id_for};

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

fn rewrite_expiry_as_malformed(path: &std::path::Path) {
    let mut raw: Envelope =
        serde_json::from_str(&std::fs::read_to_string(path).expect("fact bytes"))
            .expect("fact json");
    raw.payload["expires_at"] = serde_json::json!("not-a-timestamp");
    raw.fact_id = String::new();
    raw.fact_id = fact_id_for(&raw).expect("recomputes identity");
    std::fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&raw).expect("serialises")
        ),
    )
    .expect("mutates expiry");
}

#[test]
fn malformed_lease_grant_fails_read_instead_of_being_unheld() {
    let dir = repo();
    let path = append_fact(
        dir.path(),
        NewFact {
            kind: "lease.grant".to_owned(),
            recorded_at: "2026-08-07T03:45:12Z".to_owned(),
            recorded_by: Actor {
                kind: "driver".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: None,
            payload: serde_json::json!({
                "unit_id": "todo.example",
                "expires_at": "2026-08-07T04:00:00Z",
            }),
        },
    )
    .expect("valid grant");
    rewrite_expiry_as_malformed(&path);
    let Err(error) = read_facts(dir.path()) else {
        panic!("malformed grant succeeds");
    };
    assert!(
        error.contains("lease") && error.contains("expires_at"),
        "{error}"
    );
}
#[test]
fn malformed_lease_renew_fails_read_instead_of_being_unheld() {
    let dir = repo();
    let grant_path = append_fact(
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
    .expect("valid grant");
    let grant: Envelope =
        serde_json::from_str(&std::fs::read_to_string(grant_path).expect("grant bytes"))
            .expect("grant json");
    let path = append_fact(
        dir.path(),
        NewFact {
            kind: "lease.renew".to_owned(),
            recorded_at: "2026-08-07T03:50:12Z".to_owned(),
            recorded_by: Actor {
                kind: "driver".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: Some(grant.fact_id),
            payload: serde_json::json!({
                "unit_id": "todo.example",
                "expires_at": "2026-08-07T05:00:00Z",
            }),
        },
    )
    .expect("valid renewal");
    rewrite_expiry_as_malformed(&path);
    let Err(error) = read_facts(dir.path()) else {
        panic!("malformed renewal succeeds");
    };
    assert!(
        error.contains("lease") && error.contains("expires_at"),
        "{error}"
    );
}

#[test]
fn renamed_valid_fact_fails_filename_validation() {
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
    let renamed = path.with_file_name(format!(
        "{}-renamed.json",
        path.file_stem().unwrap().to_string_lossy()
    ));
    std::fs::rename(&path, &renamed).expect("renames fact");
    let Err(error) = read_facts(dir.path()) else {
        panic!("renamed fact succeeds");
    };
    assert!(error.contains("filename"), "{error}");
}

#[test]
fn fractional_lease_expiry_is_rejected_before_fold() {
    let dir = repo();
    let Err(error) = append_fact(
        dir.path(),
        fact(
            "lease.grant",
            "2026-08-07T03:45:12Z",
            serde_json::json!({
                "unit_id": "todo.example",
                "expires_at": "2026-08-07T04:00:00.500Z",
            }),
        ),
    ) else {
        panic!("fractional lease expiry succeeds");
    };
    assert!(error.contains("expires_at"), "{error}");
}

#[test]
fn fractional_park_and_unpark_timestamps_are_rejected() {
    let dir = repo();
    for kind in ["ruling.park", "ruling.unpark"] {
        let mut candidate = fact(
            kind,
            "2026-08-07T03:45:12.500Z",
            serde_json::json!({ "target": "todo.example" }),
        );
        candidate.kind = kind.to_owned();
        let Err(error) = append_fact(dir.path(), candidate) else {
            panic!("fractional {kind} timestamp succeeds");
        };
        assert!(error.contains("recorded_at"), "{error}");
    }
}

#[test]
fn fractional_at_after_expiry_is_stale() {
    let dir = repo();
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
    .expect("valid grant");
    let facts = match read_facts(dir.path()).expect("reads") {
        StoreRead::Ready(facts) => facts,
        StoreRead::Uninitialised => panic!("store initialised"),
    };
    let at = "2026-08-07T04:00:00.500Z";
    assert!(!held(&facts, "todo.example", at));
    assert!(stale(&facts, "todo.example", at).is_some());
}

#[test]
fn offset_at_before_expiry_is_held() {
    let dir = repo();
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
    .expect("valid grant");
    let facts = match read_facts(dir.path()).expect("reads") {
        StoreRead::Ready(facts) => facts,
        StoreRead::Uninitialised => panic!("store initialised"),
    };
    let at = "2026-08-07T05:59:00+02:00";
    assert!(held(&facts, "todo.example", at));
    assert!(stale(&facts, "todo.example", at).is_none());
}

#[test]
fn reads_do_not_create_a_parsed_envelope_cache() {
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
    read_facts(dir.path()).expect("reads");
    assert!(
        !dir.path()
            .join(".git/cairn/coord/cache/parsed.json")
            .exists(),
        "full reads do not regenerate a parsed-envelope cache"
    );
}

#[test]
fn verify_rejects_a_live_successor_with_an_archived_antecedent() {
    let dir = repo();
    let grant_path = append_fact(
        dir.path(),
        fact(
            "lease.grant",
            "2026-07-01T00:00:00Z",
            serde_json::json!({
                "unit_id": "todo.x",
                "expires_at": "2026-07-01T01:00:00Z",
            }),
        ),
    )
    .expect("appends grant");
    let grant: Envelope =
        serde_json::from_str(&std::fs::read_to_string(&grant_path).expect("grant bytes"))
            .expect("grant json");
    append_fact(
        dir.path(),
        NewFact {
            kind: "lease.renew".to_owned(),
            recorded_at: "2026-08-02T00:00:00Z".to_owned(),
            recorded_by: Actor {
                kind: "driver".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: Some(grant.fact_id),
            payload: serde_json::json!({
                "unit_id": "todo.x",
                "expires_at": "2026-08-02T01:00:00Z",
            }),
        },
    )
    .expect("appends renewal");
    let store = dir.path().join(".git/cairn/coord");
    let name = grant_path.file_name().expect("grant name");
    let archive = store.join("archive/2026-07");
    std::fs::create_dir_all(&archive).expect("archive month");
    std::fs::rename(&grant_path, archive.join(name)).expect("archives grant");

    let error = crate::coord::verify::verify(dir.path()).expect_err("compacted antecedent fails");
    assert!(
        error.contains("supersedes") && error.contains("compacted"),
        "{error}"
    );
}

#[test]
fn verify_fails_when_observation_snapshot_is_malformed() {
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
    let snapshot = dir.path().join(".git/cairn/coord/cache/observed.json");
    std::fs::write(&snapshot, "not json").expect("corrupts observation");

    let error = crate::coord::verify::verify(dir.path()).expect_err("malformed snapshot fails");
    assert!(
        error.contains("observation") || error.contains("parse"),
        "{error}"
    );
}

#[test]
fn verify_fails_when_observation_snapshot_cannot_be_written() {
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
    let cache = dir.path().join(".git/cairn/coord/cache");
    let snapshot = cache.join("observed.json");
    std::fs::write(&snapshot, "[]").expect("seeds observation");
    let original_permissions = std::fs::metadata(&cache)
        .expect("cache metadata")
        .permissions();
    let mut readonly_permissions = original_permissions.clone();
    readonly_permissions.set_readonly(true);
    std::fs::set_permissions(&cache, readonly_permissions).expect("blocks cache writes");
    let result = crate::coord::verify::verify(dir.path());
    std::fs::set_permissions(&cache, original_permissions).expect("restores cache writes");

    let error = result.expect_err("snapshot write fails");
    assert!(
        error.contains("observation") || error.contains("Permission denied"),
        "{error}"
    );
}

#[test]
fn compact_rejects_invalid_calendar_dates_before_store_access() {
    let dir = repo();
    for before in ["2026/99/99", "2026-99-99"] {
        let error =
            crate::coord::verify::compact(dir.path(), before).expect_err("invalid date rejected");
        assert!(error.contains("YYYY-MM-DD"), "{error}");
    }
    assert!(
        !dir.path().join(".git/cairn").exists(),
        "invalid dates do not initialise coordination storage"
    );
}
