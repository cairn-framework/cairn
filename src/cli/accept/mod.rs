//! `cairn change accept` verification battery and gate logic.

mod gates;

use std::path::Path;
use std::process::{Command, ExitStatus};

use crate::cli::{CliResult, format::esc};
use crate::reconcile::target::Language;
use crate::verification::VerificationState;

use gates::{AcceptStep, BatterySelection, select_language_battery};

/// Run the verification battery for `cairn change accept`.
pub fn run_accept_gate(change_id: Option<&str>, json: bool) -> CliResult {
    let mut findings = Vec::new();
    let root = std::env::current_dir().unwrap_or_default();

    match load_gate_context(&root) {
        Ok((gates_configured, configured_gates, language)) => {
            match select_language_battery(language, gates_configured, &configured_gates) {
                BatterySelection::Steps(steps) => {
                    for step in steps {
                        run_accept_step(&mut findings, &step, json);
                    }
                }
                BatterySelection::SkipInfo { language } => {
                    findings.push(VerificationFinding {
                        test: format!("language battery ({language})"),
                        state: VerificationState::Skipped,
                        detail: Some(format!(
                            "no gates configured for {language}; configure a `gates:` section in cairn.config.yaml to run build/test checks"
                        )),
                    });
                }
            }
        }
        Err(message) => {
            // Do not fall back by language when config is unreadable: that would
            // silently bypass an explicit `gates:` block.
            findings.push(VerificationFinding {
                test: "load cairn.config.yaml".to_string(),
                state: VerificationState::Blocked,
                detail: Some(message),
            });
        }
    }

    if let Some(id) = change_id {
        run_step(
            &mut findings,
            &format!("cairn lint --strict {id}"),
            || run_command("cairn", &["lint", "--strict", id], json),
            "validation failed",
            "could not run validation",
        );
        check_suggested_edges(&mut findings, id, &root);
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

fn load_gate_context(
    root: &Path,
) -> Result<(bool, Vec<crate::scanner::config::GateStep>, Language), String> {
    match crate::scanner::config::load(root) {
        Ok(config) => {
            let language = Language::infer_from_directory(root, Path::new("."), &config.ignores)
                .unwrap_or(Language::Unknown);
            Ok((config.gates_configured, config.gates, language))
        }
        Err(error) => Err(format!("could not load cairn.config.yaml: {error}")),
    }
}

fn run_accept_step(findings: &mut Vec<VerificationFinding>, step: &AcceptStep, quiet: bool) {
    let args: Vec<&str> = step.args.iter().map(String::as_str).collect();
    run_step(
        findings,
        &step.name,
        || run_command(&step.program, &args, quiet),
        &step.fail_msg,
        &step.block_msg,
    );
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

fn check_suggested_edges(findings: &mut Vec<VerificationFinding>, change_id: &str, root: &Path) {
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
        VerificationState::Skipped => "skipped",
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
        assert!(!out.contains("Note:"));
    }

    #[test]
    fn test_format_json_failed_wins_over_blocked() {
        let findings = vec![VerificationFinding {
            test: "cargo test".to_string(),
            state: VerificationState::Failed,
            detail: None,
        }];
        let out = format_json(&findings, true, true);
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
        let out = format_json(&findings, has_failed, has_blocked);
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
        let out = format_findings(&findings, false);
        assert!(out.contains("[SKIPPED] language battery (python)"));
        assert!(out.contains("no gates configured for python"));
    }
}
