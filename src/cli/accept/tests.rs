//! Unit tests for the accept gate.
use super::*;

#[test]
fn test_format_json_produces_valid_json() {
    let findings = vec![
        VerificationFinding {
            test: "cargo build".to_string(),
            state: VerificationState::Passed,
            detail: None,
        },
        VerificationFinding {
            test: "cargo test".to_string(),
            state: VerificationState::Failed,
            detail: Some("tests failed".to_string()),
        },
    ];
    let output = format_json(&findings, true, false, false);
    let parsed: serde_json::Value = serde_json::from_str(output.trim())
        .unwrap_or_else(|e| panic!("invalid JSON from accept --json: {e}\n{output}"));
    assert_eq!(parsed["command"], "accept");
    assert_eq!(parsed["status"], "error");
    assert_eq!(parsed["data"]["gate_outcome"], "failed");
    let steps = parsed["data"]["steps"].as_array().expect("steps array");
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0]["state"], "passed");
    assert_eq!(steps[1]["state"], "failed");
    assert_eq!(steps[1]["detail"], "tests failed");
}

#[test]
fn test_format_json_passed_status() {
    let findings = vec![VerificationFinding {
        test: "cargo build".to_string(),
        state: VerificationState::Passed,
        detail: None,
    }];
    let output = format_json(&findings, false, false, false);
    let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
    assert_eq!(parsed["status"], "ok");
    assert_eq!(parsed["data"]["gate_outcome"], "passed");
}

#[test]
fn test_format_json_blocked_status() {
    let findings = vec![VerificationFinding {
        test: "cargo build".to_string(),
        state: VerificationState::Blocked,
        detail: Some("not installed".to_string()),
    }];
    let output = format_json(&findings, false, true, false);
    let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
    assert_eq!(parsed["status"], "error");
    assert_eq!(parsed["data"]["gate_outcome"], "blocked");
}

#[test]
fn test_truncate_stderr_multibyte_chars_do_not_panic() {
    let long = "→".repeat(200);
    assert_eq!(long.len(), 600);
    assert!(!long.is_char_boundary(512));
    let result = truncate_stderr(&long, 512);
    assert!(result.ends_with("..."));
    assert!(result.len() < long.len());
}

#[test]
fn test_truncate_stderr_ascii_truncates_at_exact_limit() {
    let long = "x".repeat(600);
    let result = truncate_stderr(&long, 512);
    assert_eq!(result, format!("{}...", "x".repeat(512)));
}

#[test]
fn test_truncate_stderr_short_string_returned_unchanged() {
    assert_eq!(truncate_stderr("hello", 512), "hello");
}

#[test]
fn test_truncate_stderr_exactly_at_limit_not_truncated() {
    let exactly = "x".repeat(512);
    assert_eq!(truncate_stderr(&exactly, 512), exactly);
}

#[test]
fn test_state_str_all_variants() {
    assert_eq!(state_str(&VerificationState::Draft), "draft");
    assert_eq!(state_str(&VerificationState::Planned), "planned");
    assert_eq!(state_str(&VerificationState::Passed), "passed");
    assert_eq!(state_str(&VerificationState::Failed), "failed");
    assert_eq!(state_str(&VerificationState::Blocked), "blocked");
    assert_eq!(state_str(&VerificationState::Skipped), "skipped");
}

#[test]
fn test_format_findings_single_passed_step() {
    let findings = vec![VerificationFinding {
        test: "cargo build".to_string(),
        state: VerificationState::Passed,
        detail: None,
    }];
    let out = format_findings(&findings, false, false);
    assert!(out.contains("Verification Battery Results:"));
    assert!(out.contains("[PASSED] cargo build"));
    assert!(out.ends_with('\n'));
}

#[test]
fn test_format_findings_detail_in_parentheses() {
    let findings = vec![VerificationFinding {
        test: "cargo test".to_string(),
        state: VerificationState::Failed,
        detail: Some("3 tests failed".to_string()),
    }];
    let out = format_findings(&findings, false, false);
    assert!(out.contains("[FAILED] cargo test (3 tests failed)"));
}

#[test]
fn test_format_findings_blocked_note_appended() {
    let findings = vec![VerificationFinding {
        test: "cargo clippy".to_string(),
        state: VerificationState::Blocked,
        detail: None,
    }];
    let out = format_findings(&findings, true, false);
    assert!(out.contains("Note: Blocked outcomes do not fail the gate"));
}

#[test]
fn test_format_findings_no_blocked_note_when_false() {
    let findings = vec![VerificationFinding {
        test: "cargo build".to_string(),
        state: VerificationState::Passed,
        detail: None,
    }];
    let out = format_findings(&findings, false, false);
    assert!(!out.contains("Note:"));
}

#[test]
fn test_format_json_failed_wins_over_blocked() {
    let findings = vec![VerificationFinding {
        test: "cargo test".to_string(),
        state: VerificationState::Failed,
        detail: None,
    }];
    let out = format_json(&findings, true, true, false);
    let parsed: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(parsed["data"]["gate_outcome"], "failed");
}

#[test]
fn test_skipped_info_does_not_fail_or_block() {
    let findings = vec![VerificationFinding {
        test: "language battery (typescript)".to_string(),
        state: VerificationState::Skipped,
        detail: Some(
            "no gates configured for typescript; configure a `gates:` section in cairn.config.yaml to run build/test checks"
                .to_string(),
        ),
    }];
    let has_failed = findings
        .iter()
        .any(|f| f.state == VerificationState::Failed);
    let has_blocked = findings
        .iter()
        .any(|f| f.state == VerificationState::Blocked);
    assert!(!has_failed);
    assert!(!has_blocked);
    let out = format_json(&findings, has_failed, has_blocked, false);
    let parsed: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(parsed["status"], "ok");
    assert_eq!(parsed["data"]["gate_outcome"], "passed");
    assert_eq!(parsed["data"]["steps"][0]["state"], "skipped");
    assert!(
        parsed["data"]["steps"][0]["detail"]
            .as_str()
            .unwrap()
            .contains("no gates configured for typescript")
    );
    assert_eq!(u8::from(has_failed), 0);
}

#[test]
fn test_format_findings_shows_skipped() {
    let findings = vec![VerificationFinding {
        test: "language battery (python)".to_string(),
        state: VerificationState::Skipped,
        detail: Some("no gates configured for python".to_string()),
    }];
    let out = format_findings(&findings, false, false);
    assert!(out.contains("[SKIPPED] language battery (python)"));
    assert!(out.contains("no gates configured for python"));
}

#[test]
fn test_blank_command_gate_fails_with_nonzero_exit() {
    // Configured gate with blank/whitespace-only command must be Failed, not
    // Blocked, so acceptance exits non-zero (has_failed drives the exit code).
    // Drive the full production seam: load config -> select -> apply.
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("main.ts"), "export {};\n").unwrap();
    std::fs::write(
        root.join("cairn.config.yaml"),
        "gates:\n  - name: typecheck\n    command: \"   \"\n",
    )
    .unwrap();
    let recipe = crate::scanner::gate_recipe::resolve_gate_recipe(root).unwrap();
    assert!(matches!(
        recipe,
        crate::scanner::gate_recipe::GateRecipe::Configured(_)
    ));
    let selection = select_language_battery(recipe);
    let mut findings = Vec::new();
    apply_language_battery(&mut findings, selection, root, true);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].state, VerificationState::Failed);
    assert_eq!(findings[0].test, "typecheck");
    assert!(
        findings[0]
            .detail
            .as_ref()
            .is_some_and(|d| d.contains("no command")),
        "detail: {:?}",
        findings[0].detail
    );
    let has_failed = findings
        .iter()
        .any(|f| f.state == VerificationState::Failed);
    let has_blocked = findings
        .iter()
        .any(|f| f.state == VerificationState::Blocked);
    assert!(has_failed);
    assert!(!has_blocked);
    assert_eq!(u8::from(has_failed), 1);
    let out = format_json(&findings, has_failed, has_blocked, false);
    let parsed: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(parsed["data"]["gate_outcome"], "failed");
    assert_eq!(parsed["status"], "error");
}

#[test]
fn test_load_gate_context_skip_wiring_end_to_end() {
    // Drive config load -> language infer -> selection -> production helper
    // (apply_language_battery) without spawning real gate commands.
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("main.ts"), "export {};\n").unwrap();
    let recipe = crate::scanner::gate_recipe::resolve_gate_recipe(root).unwrap();
    assert!(matches!(
        recipe,
        crate::scanner::gate_recipe::GateRecipe::SkipInfo { .. }
    ));
    let selection = select_language_battery(recipe);
    let mut findings = Vec::new();
    apply_language_battery(&mut findings, selection, root, true);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].state, VerificationState::Skipped);
    assert_eq!(findings[0].test, "language battery (typescript)");
    assert!(
        findings[0]
            .detail
            .as_ref()
            .is_some_and(|d| d.contains("no gates configured for typescript")),
        "detail: {:?}",
        findings[0].detail
    );
    let has_failed = findings
        .iter()
        .any(|f| f.state == VerificationState::Failed);
    let has_blocked = findings
        .iter()
        .any(|f| f.state == VerificationState::Blocked);
    assert!(!has_failed);
    assert!(!has_blocked);
    assert_eq!(u8::from(has_failed), 0);
    let out = format_json(&findings, has_failed, has_blocked, false);
    let parsed: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(parsed["data"]["gate_outcome"], "passed");
    assert_eq!(parsed["data"]["steps"][0]["state"], "skipped");
}

#[test]
fn test_load_gate_context_with_gates_selects_steps() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("main.ts"), "export {};\n").unwrap();
    std::fs::write(
        root.join("cairn.config.yaml"),
        "gates:\n  - name: unit\n    command: true\n",
    )
    .unwrap();
    let recipe = crate::scanner::gate_recipe::resolve_gate_recipe(root).unwrap();
    assert!(matches!(
        recipe,
        crate::scanner::gate_recipe::GateRecipe::Configured(_)
    ));
    let selection = select_language_battery(recipe);
    match &selection {
        BatterySelection::Steps(steps) => {
            assert_eq!(steps.len(), 1);
            assert_eq!(steps[0].program, "true");
            assert_eq!(steps[0].name, "unit");
        }
        BatterySelection::SkipInfo { language } => {
            panic!("gates config must not skip, got SkipInfo({language})")
        }
    }
    // Production helper must run the configured step (true exits 0).
    let mut findings = Vec::new();
    apply_language_battery(&mut findings, selection, root, true);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].state, VerificationState::Passed);
    assert_eq!(findings[0].test, "unit");
    let has_failed = findings
        .iter()
        .any(|f| f.state == VerificationState::Failed);
    assert!(!has_failed);
    assert_eq!(u8::from(has_failed), 0);
}

#[test]
fn accept_runs_gate_commands_from_project_root() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("marker.txt"), "ok\n").unwrap();
    std::fs::write(
        root.join("cairn.config.yaml"),
        "gates:\n  - name: read marker\n    command: cat marker.txt\n",
    )
    .unwrap();

    let result = run_accept_gate(root, None, true, false);

    assert_eq!(result.code, 0, "accept output: {}", result.stdout);
    assert!(result.stdout.contains("\"test\":\"read marker\""));
    assert!(result.stdout.contains("\"state\":\"passed\""));
}

#[test]
fn accept_dry_run_previews_steps_without_running_them() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(
        root.join("cairn.config.yaml"),
        "gates:\n  - name: touch sentinel\n    command: touch sentinel.txt\n",
    )
    .unwrap();

    let result = run_accept_gate(root, Some("some-change"), true, true);

    assert_eq!(result.code, 0, "accept output: {}", result.stdout);
    assert!(
        !root.join("sentinel.txt").exists(),
        "dry run must not spawn gate commands"
    );
    assert!(
        result.stdout.contains("\"gate_outcome\":\"preview\""),
        "dry run reports a preview outcome: {}",
        result.stdout
    );
    assert!(
        result.stdout.contains(
            "{\"test\":\"touch sentinel\",\"state\":\"planned\",\"detail\":\"touch sentinel.txt\"}"
        ),
        "dry run lists the resolved gate command: {}",
        result.stdout
    );
    assert!(
        result
            .stdout
            .contains("\"test\":\"cairn lint --strict some-change\",\"state\":\"planned\""),
        "dry run lists the change-scoped lint step: {}",
        result.stdout
    );
    assert!(
        !result.stdout.contains("\"state\":\"passed\""),
        "dry run never reports an executed result: {}",
        result.stdout
    );
}
