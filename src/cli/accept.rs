//! `cairn accept` verification battery and gate logic.

use std::{
    fs,
    path::PathBuf,
    process::{Command, ExitStatus},
};

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

    let _planned_tests = read_planned_tests();
    run_step(
        &mut findings,
        "cargo test",
        || run_command("cargo", &["test"]),
        "tests failed",
        "could not run cargo test",
    );

    run_step(
        &mut findings,
        "cargo test --locked",
        || run_command("cargo", &["test", "--locked"]),
        "locked tests failed",
        "could not run cargo test --locked",
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

    run_step(
        &mut findings,
        "openspec validate --specs",
        || run_command("openspec", &["validate", "--specs", "--strict"]),
        "spec validation failed",
        "could not run spec validation",
    );

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
    Passed,
    Failed,
    Blocked,
}

fn run_command(cmd: &str, args: &[&str]) -> Result<ExitStatus, std::io::Error> {
    Command::new(cmd).args(args).status()
}

fn read_planned_tests() -> Vec<PlannedTest> {
    let path = PathBuf::from("target/cflx/planned.json");
    if !path.exists() {
        return Vec::new();
    }

    let Ok(content) = fs::read_to_string(&path) else {
        return Vec::new();
    };

    // Simple manual JSON parsing for the sidecar format
    parse_planned_json(&content)
}

#[derive(Debug, Clone)]
struct PlannedTest {
    #[allow(dead_code)]
    test_path: String,
    #[allow(dead_code)]
    phase: u32,
}

fn parse_planned_json(content: &str) -> Vec<PlannedTest> {
    let mut tests = Vec::new();

    // Extract entries array
    if let Some(entries_start) = content.find("\"entries\"")
        && let Some(arr_start) = content[entries_start..].find('[')
    {
        let arr_start = entries_start + arr_start;
        if let Some(arr_end) = content[arr_start..].rfind(']') {
            let arr_text = &content[arr_start..=arr_start + arr_end];
            for entry_text in split_json_array(arr_text) {
                if let Some(test) = parse_planned_entry(entry_text) {
                    tests.push(test);
                }
            }
        }
    }

    tests
}

fn parse_planned_entry(text: &str) -> Option<PlannedTest> {
    let test_path = extract_json_string(text, "test_path")?;
    let phase_str = extract_json_string(text, "phase")?;
    let phase = phase_str.parse::<u32>().ok()?;

    Some(PlannedTest { test_path, phase })
}

fn extract_json_string(text: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{key}\": \"");
    if let Some(start) = text.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = text[start..].find('"') {
            return Some(text[start..start + end].to_string());
        }
    }

    // Try numeric value
    let pattern = format!("\"{key}\": ");
    if let Some(start) = text.find(&pattern) {
        let start = start + pattern.len();
        let end = text[start..]
            .find([',', '}', ']'])
            .unwrap_or(text[start..].len());
        return Some(text[start..start + end].trim().to_string());
    }

    None
}

fn split_json_array(arr_text: &str) -> Vec<&str> {
    let mut entries = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    let mut in_string = false;
    let mut escape = false;

    for (i, ch) in arr_text.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        match ch {
            '\\' if in_string => escape = true,
            '"' => in_string = !in_string,
            '{' if !in_string => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    entries.push(&arr_text[start..=i]);
                }
            }
            _ => {}
        }
    }

    entries
}

fn format_findings(findings: &[VerificationFinding], has_blocked: bool) -> String {
    let mut lines = vec!["Verification Battery Results:".to_string()];

    for finding in findings {
        let state_str = match finding.state {
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
    use super::*;

    #[test]
    fn test_parse_planned_json() {
        let json = r#"{
  "version": 1,
  "entries": [
    { "test_path": "test_foo", "phase": 8, "file": "src/lib.rs", "line": 10 }
  ]
}"#;
        let tests = parse_planned_json(json);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].test_path, "test_foo");
        assert_eq!(tests[0].phase, 8);
    }

    #[test]
    fn test_split_json_array() {
        let arr = "[{ \"a\": 1 }, { \"b\": 2 }]";
        let parts = split_json_array(arr);
        assert_eq!(parts.len(), 2);
    }
}
