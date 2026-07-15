//! Language-aware accept-gate step selection.
//!
//! Precedence:
//! 1. Explicit `gates:` entries in `cairn.config.yaml` (highest priority).
//! 2. Else Rust projects keep the cargo battery.
//! 3. Else non-Rust / unknown projects skip the language build/test battery
//!    and emit an informational finding so acceptance can still pass via the
//!    language-agnostic `cairn lint` + suggested-edges steps.

use crate::reconcile::target::Language;
use crate::scanner::config::GateStep;

/// A single verification step the accept battery will run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptStep {
    /// Display name shown in findings / JSON output.
    pub name: String,
    /// Program to exec (first argv element).
    pub program: String,
    /// Remaining argv.
    pub args: Vec<String>,
    /// Message when the process exits non-zero.
    pub fail_msg: String,
    /// Message when the process cannot be spawned.
    pub block_msg: String,
}

/// Outcome of selecting the language build/test battery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BatterySelection {
    /// Run these steps as the language build/test battery.
    Steps(Vec<AcceptStep>),
    /// No language battery; surface an informational finding and continue.
    SkipInfo { language: String },
}

/// Select the language build/test battery for accept.
///
/// When `gates_configured` is true, run exactly `configured_gates` (which may
/// be empty: an explicit empty `gates:` list means zero language steps).
/// Otherwise Rust keeps the cargo battery; every other language (including
/// unknown) skips with an informational finding.
#[must_use]
pub fn select_language_battery(
    language: Language,
    gates_configured: bool,
    configured_gates: &[GateStep],
) -> BatterySelection {
    if gates_configured {
        let steps = configured_gates
            .iter()
            .map(|g| {
                let (program, args) = split_command(&g.command);
                let name = if g.name.is_empty() {
                    "unnamed gate".to_owned()
                } else {
                    g.name.clone()
                };
                // Blank/whitespace-only command is a config error: surface as a
                // step with empty program so the runner marks it Failed (not
                // Blocked). Legitimate missing toolchains still Blocked.
                AcceptStep {
                    name: name.clone(),
                    program,
                    args,
                    fail_msg: format!("{name} failed"),
                    block_msg: format!("could not run {name}"),
                }
            })
            .collect();
        return BatterySelection::Steps(steps);
    }

    match language {
        Language::Rust => BatterySelection::Steps(cargo_battery()),
        other => BatterySelection::SkipInfo {
            language: other.as_str().to_owned(),
        },
    }
}

/// Hardcoded cargo battery: byte-identical to the pre-language-aware accept gate.
fn cargo_battery() -> Vec<AcceptStep> {
    vec![
        AcceptStep {
            name: "cargo build".to_owned(),
            program: "cargo".to_owned(),
            args: vec!["build".to_owned()],
            fail_msg: "build failed".to_owned(),
            block_msg: "could not run cargo build".to_owned(),
        },
        AcceptStep {
            name: "cargo clippy".to_owned(),
            program: "cargo".to_owned(),
            args: vec![
                "clippy".to_owned(),
                "--all-targets".to_owned(),
                "--all-features".to_owned(),
                "--".to_owned(),
                "-D".to_owned(),
                "warnings".to_owned(),
            ],
            fail_msg: "clippy warnings found".to_owned(),
            block_msg: "could not run cargo clippy".to_owned(),
        },
        AcceptStep {
            name: "cargo fmt".to_owned(),
            program: "cargo".to_owned(),
            args: vec!["fmt".to_owned(), "--check".to_owned()],
            fail_msg: "formatting issues found".to_owned(),
            block_msg: "could not run cargo fmt".to_owned(),
        },
        AcceptStep {
            name: "cargo test --workspace --locked".to_owned(),
            program: "cargo".to_owned(),
            args: vec![
                "test".to_owned(),
                "--workspace".to_owned(),
                "--locked".to_owned(),
            ],
            fail_msg: "tests failed".to_owned(),
            block_msg: "could not run cargo test".to_owned(),
        },
    ]
}

/// Detail string when a configured gate has no runnable program.
///
/// Returns `Some` when `program` is empty (blank/whitespace-only command).
#[must_use]
pub fn blank_command_failure_detail(step: &AcceptStep) -> Option<String> {
    if step.program.is_empty() {
        Some(format!(
            "{} has no command; configure a non-empty `command:` in cairn.config.yaml",
            step.name
        ))
    } else {
        None
    }
}

/// Split a gate command string on whitespace into program + args.
///
/// Simple whitespace split: no shell quoting. Prefer commands without spaces
/// in arguments, or quote-free argv forms. Documented in the config schema.
fn split_command(command: &str) -> (String, Vec<String>) {
    let mut parts = command.split_whitespace().map(str::to_owned);
    let program = parts.next().unwrap_or_default();
    let args = parts.collect();
    (program, args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn configured_gates_override_rust() {
        let gates = vec![GateStep {
            name: "bun test".to_owned(),
            command: "bun test".to_owned(),
        }];
        match select_language_battery(Language::Rust, true, &gates) {
            BatterySelection::Steps(steps) => {
                assert_eq!(steps.len(), 1);
                assert_eq!(steps[0].name, "bun test");
                assert_eq!(steps[0].program, "bun");
                assert_eq!(steps[0].args, vec!["test".to_owned()]);
            }
            other @ BatterySelection::SkipInfo { .. } => panic!("expected Steps, got {other:?}"),
        }
    }

    #[test]
    fn rust_without_gates_keeps_cargo_battery() {
        match select_language_battery(Language::Rust, false, &[]) {
            BatterySelection::Steps(steps) => {
                assert_eq!(steps.len(), 4);
                assert_eq!(steps[0].name, "cargo build");
                assert_eq!(steps[1].name, "cargo clippy");
                assert_eq!(steps[2].name, "cargo fmt");
                assert_eq!(steps[3].name, "cargo test --workspace --locked");
                assert_eq!(
                    steps[1].args,
                    vec![
                        "clippy".to_owned(),
                        "--all-targets".to_owned(),
                        "--all-features".to_owned(),
                        "--".to_owned(),
                        "-D".to_owned(),
                        "warnings".to_owned(),
                    ]
                );
            }
            other @ BatterySelection::SkipInfo { .. } => {
                panic!("expected cargo Steps, got {other:?}")
            }
        }
    }

    #[test]
    fn typescript_without_gates_skips_with_info() {
        match select_language_battery(Language::TypeScript, false, &[]) {
            BatterySelection::SkipInfo { language } => {
                assert_eq!(language, "typescript");
            }
            other @ BatterySelection::Steps(_) => panic!("expected SkipInfo, got {other:?}"),
        }
    }

    #[test]
    fn python_without_gates_skips() {
        match select_language_battery(Language::Python, false, &[]) {
            BatterySelection::SkipInfo { language } => assert_eq!(language, "python"),
            other @ BatterySelection::Steps(_) => panic!("expected SkipInfo, got {other:?}"),
        }
    }

    #[test]
    fn go_without_gates_skips() {
        match select_language_battery(Language::Go, false, &[]) {
            BatterySelection::SkipInfo { language } => assert_eq!(language, "go"),
            other @ BatterySelection::Steps(_) => panic!("expected SkipInfo, got {other:?}"),
        }
    }

    #[test]
    fn unknown_without_gates_skips() {
        match select_language_battery(Language::Unknown, false, &[]) {
            BatterySelection::SkipInfo { language } => assert_eq!(language, "unknown"),
            other @ BatterySelection::Steps(_) => panic!("expected SkipInfo, got {other:?}"),
        }
    }

    #[test]
    fn configured_gates_work_for_typescript() {
        let gates = vec![
            GateStep {
                name: "typecheck".to_owned(),
                command: "tsc --noEmit".to_owned(),
            },
            GateStep {
                name: "unit tests".to_owned(),
                command: "bun test".to_owned(),
            },
        ];
        match select_language_battery(Language::TypeScript, true, &gates) {
            BatterySelection::Steps(steps) => {
                assert_eq!(steps.len(), 2);
                assert_eq!(steps[0].program, "tsc");
                assert_eq!(steps[0].args, vec!["--noEmit".to_owned()]);
                assert_eq!(steps[1].program, "bun");
            }
            other @ BatterySelection::SkipInfo { .. } => panic!("expected Steps, got {other:?}"),
        }
    }

    #[test]
    fn explicit_empty_gates_runs_zero_steps_even_for_rust() {
        match select_language_battery(Language::Rust, true, &[]) {
            BatterySelection::Steps(steps) => assert!(steps.is_empty()),
            other @ BatterySelection::SkipInfo { .. } => {
                panic!("expected empty Steps, got {other:?}")
            }
        }
    }

    #[test]
    fn split_command_empty_yields_empty_program() {
        let (program, args) = split_command("");
        assert!(program.is_empty());
        assert!(args.is_empty());
    }

    #[test]
    fn split_command_single_token() {
        let (program, args) = split_command("true");
        assert_eq!(program, "true");
        assert!(args.is_empty());
    }

    #[test]
    fn blank_command_gate_yields_empty_program_and_failure_detail() {
        let gates = vec![GateStep {
            name: "typecheck".to_owned(),
            command: "   ".to_owned(),
        }];
        match select_language_battery(Language::TypeScript, true, &gates) {
            BatterySelection::Steps(steps) => {
                assert_eq!(steps.len(), 1);
                assert!(steps[0].program.is_empty(), "whitespace-only command");
                let detail = blank_command_failure_detail(&steps[0])
                    .expect("blank command must produce failure detail");
                assert!(detail.contains("typecheck"));
                assert!(detail.contains("no command"));
            }
            other @ BatterySelection::SkipInfo { .. } => {
                panic!("expected Steps, got {other:?}")
            }
        }
    }

    #[test]
    fn missing_command_field_gate_yields_empty_program() {
        let gates = vec![GateStep {
            name: "typecheck".to_owned(),
            command: String::new(),
        }];
        match select_language_battery(Language::Rust, true, &gates) {
            BatterySelection::Steps(steps) => {
                assert!(steps[0].program.is_empty());
                assert!(blank_command_failure_detail(&steps[0]).is_some());
            }
            other @ BatterySelection::SkipInfo { .. } => {
                panic!("expected Steps, got {other:?}")
            }
        }
    }

    #[test]
    fn load_and_select_typescript_without_gates_skips() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("app.ts"), "export const x = 1;\n").unwrap();
        // No cairn.config.yaml => gates absent.
        let config = crate::scanner::config::load(root).unwrap();
        assert!(
            !config.gates_configured,
            "absent gates: must leave gates_configured false"
        );
        let language = Language::infer_from_directory(root, Path::new("."), &config.ignores)
            .unwrap_or(Language::Unknown);
        assert_eq!(language, Language::TypeScript);
        match select_language_battery(language, config.gates_configured, &config.gates) {
            BatterySelection::SkipInfo { language } => {
                assert_eq!(language, "typescript");
            }
            other @ BatterySelection::Steps(_) => panic!("expected SkipInfo, got {other:?}"),
        }
    }

    #[test]
    fn load_and_select_with_gates_config_runs_configured_steps() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("app.ts"), "export const x = 1;\n").unwrap();
        std::fs::write(
            root.join("cairn.config.yaml"),
            "gates:\n  - name: unit\n    command: true\n",
        )
        .unwrap();
        let config = crate::scanner::config::load(root).unwrap();
        assert!(config.gates_configured);
        assert_eq!(config.gates.len(), 1);
        let language = Language::infer_from_directory(root, Path::new("."), &config.ignores)
            .unwrap_or(Language::Unknown);
        match select_language_battery(language, config.gates_configured, &config.gates) {
            BatterySelection::Steps(steps) => {
                assert_eq!(steps.len(), 1);
                assert_eq!(steps[0].name, "unit");
                assert_eq!(steps[0].program, "true");
                assert!(blank_command_failure_detail(&steps[0]).is_none());
            }
            other @ BatterySelection::SkipInfo { .. } => {
                panic!("expected Steps from gates config, got {other:?}")
            }
        }
    }
}
