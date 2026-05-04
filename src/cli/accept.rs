//! `cairn accept` verification battery and gate logic.

use std::process::{Command, ExitStatus};

use crate::cli::CliResult;

/// Run the verification battery for `cairn accept`.
pub fn run_accept_gate(change_id: Option<&str>) -> CliResult {
    let mut findings = Vec::new();

    run_step(
        &mut findings,
        "cargo build",
        || run_command("cargo", &["build"]),
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
            )
        },
        "clippy warnings found",
        "could not run cargo clippy",
    );

    run_step(
        &mut findings,
        "cargo fmt",
        || run_command("cargo", &["fmt", "--check"]),
        "formatting issues found",
        "could not run cargo fmt",
    );

    run_step(
        &mut findings,
        "cargo test --workspace --locked",
        || run_command("cargo", &["test", "--workspace", "--locked"]),
        "tests failed",
        "could not run cargo test",
    );

    if let Some(id) = change_id {
        run_step(
            &mut findings,
            &format!("cflx openspec validate {id}"),
            || run_command("cflx", &["openspec", "validate", id, "--strict"]),
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

    let output = format_findings(&findings, has_blocked);

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

fn run_command(cmd: &str, args: &[&str]) -> Result<ExitStatus, std::io::Error> {
    Command::new(cmd).args(args).status()
}

fn format_findings(findings: &[VerificationFinding], has_blocked: bool) -> String {
    let mut lines = vec!["Verification Battery Results:".to_string()];

    for finding in findings {
        let state_str = match finding.state {
            VerificationState::Draft => "DRAFT",
            VerificationState::Planned => "PLANNED",
            VerificationState::Passed => "PASSED",
            VerificationState::Failed => "FAILED",
            VerificationState::Blocked => "BLOCKED",
        };
        let detail = finding
            .detail
            .as_ref()
            .map(|d| format!(" ({d})"))
            .unwrap_or_default();
        lines.push(format!("  [{}] {}{}", state_str, finding.test, detail));
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
    // Placeholder for future accept-gate unit tests.
}
