//! Integration tests for `cairn watch`.

use std::process::Command;

fn temp_project() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let blueprint = r#"System App "App" id "app" {
    Module Auth "Auth" id "app.auth" {
        contract "meta/contracts/auth.md"
    }
}"#;
    let blueprint_path = root.join("cairn.blueprint");
    std::fs::create_dir_all(root.join("meta/contracts")).unwrap();
    std::fs::write(&blueprint_path, blueprint).unwrap();
    std::fs::write(
        root.join("meta/contracts/auth.md"),
        "---\nnode: app.auth\n---\n# Auth\n\nOriginal.",
    )
    .unwrap();

    (dir, blueprint_path)
}

fn temp_project_with_missing_contract() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let blueprint = r#"System App "App" id "app" {
    Module Auth "Auth" id "app.auth" {
        contract "meta/contracts/auth.md"
    }
}"#;
    let blueprint_path = root.join("cairn.blueprint");
    std::fs::create_dir_all(root.join("meta/contracts")).unwrap();
    std::fs::write(&blueprint_path, blueprint).unwrap();
    // Deliberately do NOT create the contract file so scan produces a finding.

    (dir, blueprint_path)
}

#[test]
fn watch_once_emits_finding_for_missing_contract() {
    let (dir, _bp) = temp_project_with_missing_contract();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(dir.path())
        .args(["watch", "--once"])
        .output()
        .expect("cairn watch --once should run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "watch --once should succeed. stderr: {stderr}"
    );

    let mut found = 0usize;
    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let value: serde_json::Value =
            serde_json::from_str(line).expect("each line must be valid JSON");
        assert_eq!(value["event"].as_str(), Some("finding_added"));
        assert!(value["timestamp"].is_string());
        let finding = value.get("finding").expect("finding object required");
        assert!(finding.get("code").is_some(), "finding must have a code");
        assert!(
            finding.get("message").is_some(),
            "finding must have a message"
        );
        found += 1;
    }
    assert!(
        found >= 1,
        "expected at least one finding_added event, found {found}"
    );
}

#[test]
fn watch_once_emits_current_findings() {
    let (dir, _bp) = temp_project();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(dir.path())
        .args(["watch", "--once"])
        .output()
        .expect("cairn watch --once should run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "watch --once should succeed. stderr: {stderr}"
    );
    // With a valid but minimal project, there may be zero findings.
    // The important invariant is that the output is valid ND-JSON.
    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let value: serde_json::Value =
            serde_json::from_str(line).expect("each line must be valid JSON");
        assert_eq!(value["event"].as_str(), Some("finding_added"));
        assert!(value["timestamp"].is_string());
        assert!(value["finding"].is_object());
    }
}

#[test]
fn watch_interval_rejects_zero() {
    let (dir, _bp) = temp_project();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(dir.path())
        .args(["watch", "--interval", "0"])
        .output()
        .expect("cairn watch should run");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "watch with --interval 0 should fail"
    );
    assert!(
        stderr.contains("at least 1 second"),
        "error should mention minimum interval. got: {stderr}"
    );
}

#[test]
fn watch_interval_rejects_missing_value() {
    let (dir, _bp) = temp_project();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(dir.path())
        .args(["watch", "--interval"])
        .output()
        .expect("cairn watch should run");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "watch with bare --interval should fail"
    );
    assert!(
        stderr.contains("requires a value"),
        "error should mention missing value. got: {stderr}"
    );
}
