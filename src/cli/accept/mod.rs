//! `cairn change accept` verification battery and gate logic.

mod gates;

use std::path::Path;
use std::process::{Command, ExitStatus};

use crate::cli::{CliResult, format::esc};
use crate::reconcile::target::Language;
use crate::verification::VerificationState;

use gates::{AcceptStep, BatterySelection, blank_command_failure_detail, select_language_battery};

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
    // Configured gate with blank/whitespace-only command: fail closed (exit 1).
    // Do not spawn; do not classify as Blocked (Blocked does not flip the exit code).
    if let Some(detail) = blank_command_failure_detail(step) {
        findings.push(VerificationFinding {
            test: step.name.clone(),
            state: VerificationState::Failed,
            detail: Some(detail),
        });
        return;
    }
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
mod tests;
