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
    runner: impl FnOnce() -> Result<ExitStatus, std::io::Error>,
    fail_msg: &str,
    block_msg: &str,
) {
    match runner() {
        Ok(status) if status.success() => {
            findings.push(VerificationFinding {
                test: name.to_string(),
                state: VerificationState::Passed,
                detail: None,
            });
        }
        Ok(_) => {
            findings.push(VerificationFinding {
                test: name.to_string(),
                state: VerificationState::Failed,
                detail: Some(fail_msg.to_string()),
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

fn run_command(cmd: &str, args: &[&str], quiet: bool) -> Result<ExitStatus, std::io::Error> {
    let mut c = Command::new(cmd);
    c.args(args);
    if quiet {
        c.stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
    }
    c.status()
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
    let status = if has_failed { "error" } else { "ok" };
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
        assert_eq!(parsed["status"], "ok");
        assert_eq!(parsed["data"]["gate_outcome"], "blocked");
    }
}
