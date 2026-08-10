//! `cairn change accept` verification battery and gate logic.

mod gates;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use crate::cli::{CliResult, format::esc};
use crate::verification::VerificationState;

use gates::{AcceptStep, BatterySelection, blank_command_failure_detail, select_language_battery};

/// Run the verification battery for `cairn change accept`.
///
/// With `dry_run` the battery is only resolved and reported: no gate step is
/// spawned, nothing is read beyond the gate recipe, and every step is listed as
/// `planned` so a caller can preview what acceptance would run.
pub fn run_accept_gate(
    project_root: &Path,
    change_id: Option<&str>,
    json: bool,
    dry_run: bool,
) -> CliResult {
    let mut findings = Vec::new();

    match crate::scanner::gate_recipe::resolve_gate_recipe(project_root) {
        Ok(recipe) => {
            let selection = select_language_battery(recipe);
            if dry_run {
                plan_language_battery(&mut findings, selection);
            } else {
                apply_language_battery(&mut findings, selection, project_root, json);
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
        if dry_run {
            findings.push(planned_step(
                format!("cairn lint --strict {id}"),
                Some(format!("cairn lint --strict {id}")),
            ));
            findings.push(planned_step(
                "suggested edges triaged".to_owned(),
                Some(format!("meta/changes/{id}/suggested-edges.md")),
            ));
        } else {
            run_step(
                &mut findings,
                &format!("cairn lint --strict {id}"),
                || {
                    run_command(
                        running_cairn_bin()?,
                        &["lint", "--strict", id],
                        project_root,
                        json,
                    )
                },
                "validation failed",
                "could not run validation",
            );
            check_suggested_edges(&mut findings, id, project_root);
        }
    }

    let has_failed = findings
        .iter()
        .any(|f| f.state == VerificationState::Failed);
    let has_blocked = findings
        .iter()
        .any(|f| f.state == VerificationState::Blocked);

    let output = if json {
        format_json(&findings, has_failed, has_blocked, dry_run)
    } else {
        format_findings(&findings, has_blocked, dry_run)
    };

    CliResult {
        code: u8::from(has_failed),
        stdout: output,
        stderr: String::new(),
    }
}

/// A step the gate would run, recorded without spawning it.
fn planned_step(test: String, detail: Option<String>) -> VerificationFinding {
    VerificationFinding {
        test,
        state: VerificationState::Planned,
        detail,
    }
}

/// The informational finding recorded when no language battery applies.
///
/// Shared by the preview and the live battery so the two cannot drift.
fn skipped_language_finding(language: &str) -> VerificationFinding {
    VerificationFinding {
        test: format!("language battery ({language})"),
        state: VerificationState::Skipped,
        detail: Some(format!(
            "no gates configured for {language}; configure a `gates:` section in cairn.config.yaml to run build/test checks"
        )),
    }
}

/// Record the language battery as planned steps instead of running it.
///
/// A configured gate with a blank command fails here exactly as it does live:
/// the preview must not advertise a step real acceptance would refuse to run.
fn plan_language_battery(findings: &mut Vec<VerificationFinding>, selection: BatterySelection) {
    match selection {
        BatterySelection::Steps(steps) => {
            for step in steps {
                if let Some(detail) = blank_command_failure_detail(&step) {
                    findings.push(VerificationFinding {
                        test: step.name,
                        state: VerificationState::Failed,
                        detail: Some(detail),
                    });
                    continue;
                }
                let AcceptStep {
                    name,
                    program,
                    args,
                    ..
                } = step;
                let command = if args.is_empty() {
                    program
                } else {
                    format!("{program} {}", args.join(" "))
                };
                findings.push(planned_step(name, Some(command)));
            }
        }
        BatterySelection::SkipInfo { language } => {
            findings.push(skipped_language_finding(&language));
        }
    }
}

/// Apply a language-battery selection to findings.
///
/// Shared by production `run_accept_gate` and hermetic wiring tests so the
/// `SkipInfo` detail/state path cannot drift from the live battery.
fn apply_language_battery(
    findings: &mut Vec<VerificationFinding>,
    selection: BatterySelection,
    project_root: &Path,
    quiet: bool,
) {
    match selection {
        BatterySelection::Steps(steps) => {
            for step in steps {
                run_accept_step(findings, &step, project_root, quiet);
            }
        }
        BatterySelection::SkipInfo { language } => {
            findings.push(skipped_language_finding(&language));
        }
    }
}

fn run_accept_step(
    findings: &mut Vec<VerificationFinding>,
    step: &AcceptStep,
    project_root: &Path,
    quiet: bool,
) {
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
        || run_command(&step.program, &args, project_root, quiet),
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

/// The cairn binary the lint leg must grade: the one already running the gate.
///
/// Resolving `cairn` from `PATH` grades whatever binary happens to be
/// installed, so a stale `~/.cargo/bin/cairn` can fail a correct tree or pass a
/// broken one (`todo.accept-gate-stale-path-binary`). This is the shippable
/// counterpart of `scripts/dogfood.sh` reaching cairn only through
/// `cargo run --bin cairn`: that manifest-bound form is specific to this repo,
/// while acceptance also runs in adopter projects that contain no cairn crate.
fn running_cairn_bin() -> Result<PathBuf, std::io::Error> {
    std::env::current_exe()
}

fn run_command(
    cmd: impl AsRef<OsStr>,
    args: &[&str],
    project_root: &Path,
    quiet: bool,
) -> Result<(ExitStatus, String), std::io::Error> {
    let mut c = Command::new(cmd);
    c.args(args).current_dir(project_root);
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

fn format_json(
    findings: &[VerificationFinding],
    has_failed: bool,
    has_blocked: bool,
    dry_run: bool,
) -> String {
    let gate_outcome = if dry_run {
        "preview"
    } else if has_failed {
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

fn format_findings(findings: &[VerificationFinding], has_blocked: bool, dry_run: bool) -> String {
    let header = if dry_run {
        "Verification Battery Plan (dry run, nothing was run):"
    } else {
        "Verification Battery Results:"
    };
    let mut lines = vec![header.to_string()];

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
