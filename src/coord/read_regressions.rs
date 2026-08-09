//! Regression coverage for cache binding and malformed lease reads.
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
    let mut raw: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("grant bytes"))
            .expect("grant json");
    raw["payload"]["expires_at"] = serde_json::json!("not-a-timestamp");
    std::fs::write(
        &path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&raw).expect("serialises")
        ),
    )
    .expect("mutates grant");
    let Err(error) = read_facts(dir.path()) else {
        panic!("malformed grant succeeds");
    };
    assert!(
        error.contains("lease") && error.contains("expires_at"),
        "{error}"
    );
}

#[test]
fn parse_cache_is_bound_to_fact_bytes_not_only_the_filename() {
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
    let _ = read_facts(dir.path()).expect("seeds parse cache");
    let mut changed: Envelope =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("fact bytes"))
            .expect("fact parses");
    changed.payload = serde_json::json!({ "target": "plan-fedcba9876543210" });
    changed.fact_id = String::new();
    changed.fact_id = fact_id_for(&changed).expect("recomputes");
    std::fs::write(
        &path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&changed).expect("serialises")
        ),
    )
    .expect("mutates fact");
    let StoreRead::Ready(facts) = read_facts(dir.path()).expect("re-reads") else {
        panic!("store is initialised");
    };
    assert_eq!(
        facts[0].fact.payload["target"],
        serde_json::json!("plan-fedcba9876543210")
    );
}
