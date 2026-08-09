//! Protocol, fixture safety, hotspot location, and the command surface.

mod authoreval_support;

use authoreval_support::*;
use cairn::authoreval::{Outcome, Record, run_prompt_file};
use serde_json::json;

#[test]
fn test_a_prompt_with_no_replay_script_cannot_run_offline() {
    let (_guard, dir) = scratch();
    let path = dir.join("bare.json");
    std::fs::write(
        &path,
        r#"{"schema_version": 1, "id": "bare", "instruction": "do a thing", "expects": ["a.md"]}"#,
    )
    .expect("write prompt");

    let error = run_prompt_file(&config(3), &path).expect_err("offline run must be refused");
    assert!(error.to_string().contains("no replay script"));
}

#[test]
fn test_shipped_smoke_prompt_runs_clean_end_to_end() {
    let prompt = manifest_dir().join("harness/authoreval/prompts/smoke.decision-authoring.json");
    let record = run_prompt_file(&config(3), &prompt).expect("smoke prompt must produce a record");

    assert_eq!(record.prompt_id, "smoke.decision-authoring");
    assert_eq!(record.outcome, Outcome::CleanFirstShot);
    assert!(record.first_shot_valid);
    assert!(record.hotspots.is_empty());
    assert_eq!(record.backend.model, MODEL);
}

#[test]
fn test_the_checked_in_fixture_is_byte_identical_after_a_run() {
    let before = tree_digest(&fixture());

    let record = run(
        1,
        "fixture-safety",
        &[write_turn(BROKEN_DECISION), write_turn(VALID_DECISION)],
    );
    assert_eq!(record.outcome, Outcome::CleanAfterRepair);

    assert_eq!(
        before,
        tree_digest(&fixture()),
        "a run must never touch the checked-in fixture"
    );
}

#[test]
fn test_one_command_runs_the_smoke_prompt_and_exits_zero() {
    let (_guard, dir) = scratch();
    let out = dir.join("records.jsonl");

    let status = std::process::Command::new(env!("CARGO_BIN_EXE_cairn-authoreval"))
        .args([
            "run",
            manifest_dir()
                .join("harness/authoreval/prompts/smoke.decision-authoring.json")
                .as_str(),
            "--fixture",
            fixture().as_str(),
            "--cairn",
            env!("CARGO_BIN_EXE_cairn"),
            "--out",
            out.as_str(),
        ])
        .status()
        .expect("run cairn-authoreval");

    assert!(status.success(), "the smoke command must exit 0");

    let written = std::fs::read_to_string(&out).expect("records were written");
    let lines: Vec<&str> = written.lines().collect();
    assert_eq!(lines.len(), 1, "one prompt emits exactly one record");

    let record: Record = serde_json::from_str(lines[0]).expect("the record is valid JSON");
    assert_eq!(record.prompt_id, "smoke.decision-authoring");
    assert_eq!(record.outcome, Outcome::CleanFirstShot);
}

#[test]
fn test_a_response_that_authors_nothing_cannot_score_clean() {
    let record = run(
        3,
        "authors-nothing",
        &[json!({"kind": "response", "files": [], "tokens": {"prompt": 10, "completion": 0}})],
    );

    assert_eq!(
        record.outcome,
        Outcome::BackendFailure,
        "the fixture already scans clean, so authoring nothing must never read as clean"
    );
    let error = record.error.expect("error");
    assert_eq!(error.class, cairn::authoreval::BackendErrorClass::Protocol);
    assert!(
        error.detail.contains(TARGET),
        "the error must name the path that went unauthored: {}",
        error.detail
    );
}

#[test]
fn test_a_response_that_writes_an_unrelated_file_cannot_score_clean() {
    let record = run(
        3,
        "unrelated-file",
        &[json!({
            "kind": "response",
            "files": [{"path": "meta/decisions/something-else.md", "contents": VALID_DECISION}],
            "tokens": {"prompt": 10, "completion": 5},
        })],
    );

    assert_eq!(
        record.outcome,
        Outcome::BackendFailure,
        "a clean scan earned by authoring the wrong artefact is not authorability"
    );
}

#[test]
fn test_a_prompt_with_no_expected_paths_is_refused() {
    let (_guard, dir) = scratch();
    let path = dir.join("no-expects.json");
    std::fs::write(
        &path,
        r#"{"schema_version": 1, "id": "x", "instruction": "do a thing", "expects": []}"#,
    )
    .expect("write prompt");

    let error = run_prompt_file(&config(3), &path).expect_err("must be refused");
    assert!(error.to_string().contains("no `expects` paths"));
}

#[test]
fn test_hotspots_carry_the_location_the_lint_wire_publishes() {
    let record = run(0, "hotspot-location", &[write_turn(BROKEN_DECISION)]);

    let hotspot = record
        .hotspots
        .iter()
        .find(|hotspot| hotspot.code == "CAIRN_ARTEFACT_MISSING_FIELD")
        .expect("the failed scan's hotspots must be carried");
    assert_eq!(
        hotspot.path.as_deref(),
        Some(TARGET),
        "the wire publishes `path`; the hotspot must carry it rather than a field lint never sends"
    );
}

#[test]
fn test_a_response_with_an_unusable_path_is_a_protocol_failure() {
    let record = run(
        3,
        "unusable-path",
        &[json!({
            "kind": "response",
            "files": [{"path": "../escape.md", "contents": "x"}],
            "tokens": {"prompt": 7, "completion": 3},
        })],
    );

    assert_eq!(
        record.outcome,
        Outcome::BackendFailure,
        "a path the workspace refuses is the backend's fault, not the instrument's"
    );
    assert_eq!(record.iterations, 1);
    assert!(!record.first_shot_valid);
    assert!(record.hotspots.is_empty());
    assert_eq!(
        record.tokens.total, 10,
        "the response was paid for even though it was unusable"
    );

    let error = record.error.expect("error");
    assert_eq!(error.class, cairn::authoreval::BackendErrorClass::Protocol);
    assert!(error.detail.contains("escape.md"), "{}", error.detail);
}
