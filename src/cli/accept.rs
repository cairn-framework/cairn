//! `cairn accept` verification battery and gate logic.

use std::process::{Command, ExitStatus};

use crate::cli::{CliResult, format::esc};

/// Run the verification battery for `cairn accept`.
pub fn run_accept_gate(change_id: Option<&str>, json: bool) -> CliResult {
    let mut findings = Vec::new();

    run_step(
        &mut findings,
        "cargo build",
        || run_command("cargo", &["build"], json),
        "build failed",
        "could not run cargo build",
    );

    run_step(
        &mut findings,
        "cargo clippy",
        || {
            run_command(
                "cargo",
                &[
                    "clippy",
                    "--all-targets",
                    "--all-features",
                    "--",
                    "-D",
                    "warnings",
                ],
                json,
            )
        },
        "clippy warnings found",
        "could not run cargo clippy",
    );

    run_step(
        &mut findings,
        "cargo fmt",
        || run_command("cargo", &["fmt", "--check"], json),
        "formatting issues found",
        "could not run cargo fmt",
    );

    run_step(
        &mut findings,
        "cargo test --workspace --locked",
        || run_command("cargo", &["test", "--workspace", "--locked"], json),
        "tests failed",
        "could not run cargo test",
    );

    if let Some(id) = change_id {
        run_step(
            &mut findings,
            &format!("cflx openspec validate {id}"),
            || run_command("cflx", &["openspec", "validate", id, "--strict"], json),
            "validation failed",
            "could not run validation",
        );
        check_suggested_edges(
            &mut findings,
            id,
            &std::env::current_dir().unwrap_or_default(),
        );
    }

    let has_failed = findings
        .iter()
        .any(|f| f.state == VerificationState::Failed);
    let has_blocked = findings
        .iter()
        .any(|f| f.state == VerificationState::Blocked);

    let output = if json {
        format_json(&findings, has_failed, has_blocked)
    } else {
        format_findings(&findings, has_blocked)
    };

    CliResult {
        code: u8::from(has_failed),
        stdout: output,
        stderr: String::new(),
    }
}

fn run_step(
    findings: &mut Vec<VerificationFinding>,
    name: &str,
    runner: impl FnOnce() -> Result<(ExitStatus, String), std::io::Error>,
    fail_msg: &str,
    block_msg: &str,
) {
    match runner() {
        Ok((status, _)) if status.success() => {
            findings.push(VerificationFinding {
                test: name.to_string(),
                state: VerificationState::Passed,
                detail: None,
            });
        }
        Ok((_, captured_stderr)) => {
            let detail = if captured_stderr.is_empty() {
                fail_msg.to_string()
            } else {
                format!("{fail_msg}: {captured_stderr}")
            };
            findings.push(VerificationFinding {
                test: name.to_string(),
                state: VerificationState::Failed,
                detail: Some(detail),
            });
        }
        Err(e) => {
            findings.push(VerificationFinding {
                test: name.to_string(),
                state: VerificationState::Blocked,
                detail: Some(format!("{block_msg}: {e}")),
            });
        }
    }
}

fn check_suggested_edges(
    findings: &mut Vec<VerificationFinding>,
    change_id: &str,
    root: &std::path::Path,
) {
    let change_dir = root.join("meta/changes").join(change_id);
    match crate::suggested_edges::validate_strict(change_id, &change_dir) {
        Ok(()) => {
            findings.push(VerificationFinding {
                test: "suggested edges triaged".to_string(),
                state: VerificationState::Passed,
                detail: None,
            });
        }
        Err(crate::error::CairnError::UntriagedSuggestedEdges {
            pending_count,
            file_path,
            ..
        }) => {
            findings.push(VerificationFinding {
                test: "suggested edges triaged".to_string(),
                state: VerificationState::Failed,
                detail: Some(format!(
                    "CC002: {pending_count} pending suggested edge(s) in {file_path}"
                )),
            });
        }
        Err(e) => {
            findings.push(VerificationFinding {
                test: "suggested edges triaged".to_string(),
                state: VerificationState::Blocked,
                detail: Some(format!("could not read suggested-edges queue: {e}")),
            });
        }
    }
}

#[derive(Debug, Clone)]
struct VerificationFinding {
    test: String,
    state: VerificationState,
    detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum VerificationState {
    #[allow(dead_code)] // Reason: reserved for future sidecar integration.
    Draft,
    #[allow(dead_code)] // Reason: reserved for future sidecar integration.
    Planned,
    Passed,
    Failed,
    Blocked,
}

/// Truncate `s` to at most `max_bytes` bytes, respecting UTF-8 char boundaries.
///
/// Appends `"..."` when truncation occurs. Never panics on multi-byte characters.
fn truncate_stderr(s: &str, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s.to_owned();
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &s[..end])
}

fn run_command(
    cmd: &str,
    args: &[&str],
    quiet: bool,
) -> Result<(ExitStatus, String), std::io::Error> {
    let mut c = Command::new(cmd);
    c.args(args);
    if quiet {
        c.stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped());
        let output = c.output()?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        let truncated = truncate_stderr(&stderr, 512);
        Ok((output.status, truncated))
    } else {
        let status = c.status()?;
        Ok((status, String::new()))
    }
}

fn state_str(state: &VerificationState) -> &'static str {
    match state {
        VerificationState::Draft => "draft",
        VerificationState::Planned => "planned",
        VerificationState::Passed => "passed",
        VerificationState::Failed => "failed",
        VerificationState::Blocked => "blocked",
    }
}

fn format_json(findings: &[VerificationFinding], has_failed: bool, has_blocked: bool) -> String {
    let gate_outcome = if has_failed {
        "failed"
    } else if has_blocked {
        "blocked"
    } else {
        "passed"
    };
    let status = if has_failed || has_blocked {
        "error"
    } else {
        "ok"
    };
    let steps: Vec<String> = findings
        .iter()
        .map(|f| {
            let detail = f
                .detail
                .as_ref()
                .map(|d| format!(",\"detail\":\"{}\"", esc(d)))
                .unwrap_or_default();
            format!(
                "{{\"test\":\"{}\",\"state\":\"{}\"{}}}",
                esc(&f.test),
                state_str(&f.state),
                detail
            )
        })
        .collect();
    format!(
        "{{\"command\":\"accept\",\"status\":\"{status}\",\"data\":{{\"gate_outcome\":\"{gate_outcome}\",\"steps\":[{}]}}}}\n",
        steps.join(",")
    )
}

fn format_findings(findings: &[VerificationFinding], has_blocked: bool) -> String {
    let mut lines = vec!["Verification Battery Results:".to_string()];

    for finding in findings {
        let label = state_str(&finding.state).to_ascii_uppercase();
        let detail = finding
            .detail
            .as_ref()
            .map(|d| format!(" ({d})"))
            .unwrap_or_default();
        lines.push(format!("  [{label}] {}{detail}", finding.test));
    }

    if has_blocked {
        lines.push(String::new());
        lines.push(
            "Note: Blocked outcomes do not fail the gate by default in this phase.".to_string(),
        );
    }

    lines.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
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
        let output = format_json(&findings, true, false);
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
        let output = format_json(&findings, false, false);
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
        let output = format_json(&findings, false, true);
        let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
        assert_eq!(parsed["status"], "error");
        assert_eq!(parsed["data"]["gate_outcome"], "blocked");
    }

    // ── truncate_stderr ───────────────────────────────────────────────────────

    #[test]
    fn test_truncate_stderr_multibyte_chars_do_not_panic() {
        // U+2192 RIGHTWARDS ARROW is 3 UTF-8 bytes (0xE2 0x86 0x92).
        // 170 arrows = 510 bytes; byte 512 falls inside the 171st arrow
        // (a continuation byte), which is NOT a char boundary.
        // &s[..512] on this input panics in the original code.
        let long = "→".repeat(200); // 600 bytes
        assert_eq!(long.len(), 600);
        assert!(
            !long.is_char_boundary(512),
            "byte 512 must not be a char boundary"
        );
        let result = truncate_stderr(&long, 512);
        assert!(
            result.ends_with("..."),
            "truncated output must end with '...'"
        );
        assert!(result.len() < long.len(), "must be shorter than input");
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

    // ── state_str ─────────────────────────────────────────────────────────────

    #[test]
    fn test_state_str_all_variants() {
        assert_eq!(state_str(&VerificationState::Draft), "draft");
        assert_eq!(state_str(&VerificationState::Planned), "planned");
        assert_eq!(state_str(&VerificationState::Passed), "passed");
        assert_eq!(state_str(&VerificationState::Failed), "failed");
        assert_eq!(state_str(&VerificationState::Blocked), "blocked");
    }

    // ── format_findings ───────────────────────────────────────────────────────

    #[test]
    fn test_format_findings_single_passed_step() {
        let findings = vec![VerificationFinding {
            test: "cargo build".to_string(),
            state: VerificationState::Passed,
            detail: None,
        }];
        let out = format_findings(&findings, false);
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
        let out = format_findings(&findings, false);
        assert!(out.contains("[FAILED] cargo test (3 tests failed)"));
    }

    #[test]
    fn test_format_findings_blocked_note_appended() {
        let findings = vec![VerificationFinding {
            test: "cargo clippy".to_string(),
            state: VerificationState::Blocked,
            detail: None,
        }];
        let out = format_findings(&findings, true);
        assert!(out.contains("Note: Blocked outcomes do not fail the gate"));
    }

    #[test]
    fn test_format_findings_no_blocked_note_when_false() {
        let findings = vec![VerificationFinding {
            test: "cargo build".to_string(),
            state: VerificationState::Passed,
            detail: None,
        }];
        let out = format_findings(&findings, false);
        assert!(
            !out.contains("Note:"),
            "blocked note must not appear when has_blocked is false"
        );
    }

    // ── format_json gaps ──────────────────────────────────────────────────────

    #[test]
    fn test_format_json_failed_wins_over_blocked() {
        // When both has_failed and has_blocked are true, gate_outcome must be
        // "failed" — failed takes precedence in the if-else chain.
        let findings = vec![VerificationFinding {
            test: "cargo test".to_string(),
            state: VerificationState::Failed,
            detail: None,
        }];
        let out = format_json(&findings, true, true);
        let parsed: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
        assert_eq!(parsed["data"]["gate_outcome"], "failed");
    }
}
