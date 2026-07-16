//! Resolves the project's accept-gate recipe: the explicit `gates:` block in
//! `cairn.config.yaml` when configured, otherwise the language build/test
//! battery. This is the single source of truth for "what will `cairn change
//! accept` run", shared by the accept executor (`cli::accept::gates`, which
//! adapts it into executable steps) and the bundle/brief renderers (which
//! display it so an agent sees the exact recipe before starting work).

use std::path::Path;

use crate::reconcile::target::Language;

use super::config::GateStep;

/// A single step in a resolved gate recipe: display name and the exact
/// command line that will run (whitespace-split into argv by the executor;
/// no shell quoting).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GateRecipeStep {
    /// Display name for the step.
    pub name: String,
    /// Full command line, e.g. `"cargo clippy --all-targets -- -D warnings"`.
    pub command: String,
}

/// Outcome of resolving a project's gate recipe.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GateRecipe {
    /// User-configured gates. May be empty.
    Configured(Vec<GateRecipeStep>),
    /// Fallback default gates by language.
    Fallback(Vec<GateRecipeStep>),
    /// No language battery: informational only, naming the detected (or
    /// unsupported) language.
    SkipInfo {
        /// The language accept looked for a battery for.
        language: String,
    },
}

/// Hardcoded cargo battery: byte-identical to the pre-language-aware accept
/// gate. The canonical Rust step list; both the accept executor and the
/// recipe renderers read it from here so it cannot drift between the two.
#[must_use]
pub fn cargo_battery() -> Vec<GateRecipeStep> {
    vec![
        GateRecipeStep {
            name: "cargo build".to_owned(),
            command: "cargo build".to_owned(),
        },
        GateRecipeStep {
            name: "cargo clippy".to_owned(),
            command: "cargo clippy --all-targets --all-features -- -D warnings".to_owned(),
        },
        GateRecipeStep {
            name: "cargo fmt".to_owned(),
            command: "cargo fmt --check".to_owned(),
        },
        GateRecipeStep {
            name: "cargo test --workspace --locked".to_owned(),
            command: "cargo test --workspace --locked".to_owned(),
        },
    ]
}

/// Selects the language build/test battery for a project.
///
/// Precedence: explicit `gates:` config (may be empty: an explicit empty
/// list deliberately runs zero language steps) beats language inference;
/// Rust without config keeps the cargo battery; every other language
/// (including unknown) skips with an informational note.
#[must_use]
pub fn select_gate_recipe(
    language: Language,
    gates_configured: bool,
    configured_gates: &[GateStep],
) -> GateRecipe {
    if gates_configured {
        let steps = configured_gates
            .iter()
            .map(|gate| GateRecipeStep {
                name: if gate.name.is_empty() {
                    "unnamed gate".to_owned()
                } else {
                    gate.name.clone()
                },
                command: gate.command.clone(),
            })
            .collect();
        return GateRecipe::Configured(steps);
    }

    match language {
        Language::Rust => GateRecipe::Fallback(cargo_battery()),
        other => GateRecipe::SkipInfo {
            language: other.as_str().to_owned(),
        },
    }
}

/// Loads `cairn.config.yaml` under `root`, infers the project's language,
/// and resolves the gate recipe in one call. The single entry point both
/// the accept executor and the bundle/brief renderers use so language
/// inference and precedence cannot drift between "what will run" and "what
/// is displayed".
///
/// # Errors
///
/// Returns an error string when `cairn.config.yaml` fails to load.
pub fn resolve_gate_recipe(root: &Path) -> Result<GateRecipe, String> {
    let config = super::config::load(root)
        .map_err(|error| format!("could not load cairn.config.yaml: {}", error.message))?;
    let language = Language::infer_from_directory(root, Path::new("."), &config.ignores)
        .unwrap_or(Language::Unknown);
    Ok(select_gate_recipe(
        language,
        config.gates_configured,
        &config.gates,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_gates_override_rust() {
        let gates = vec![GateStep {
            name: "bun test".to_owned(),
            command: "bun test".to_owned(),
        }];
        match select_gate_recipe(Language::Rust, true, &gates) {
            GateRecipe::Configured(steps) => {
                assert_eq!(steps.len(), 1);
                assert_eq!(steps[0].name, "bun test");
                assert_eq!(steps[0].command, "bun test");
            }
            other @ (GateRecipe::Fallback(_) | GateRecipe::SkipInfo { .. }) => {
                panic!("expected Configured, got {other:?}")
            }
        }
    }

    #[test]
    fn rust_without_gates_keeps_cargo_battery() {
        match select_gate_recipe(Language::Rust, false, &[]) {
            GateRecipe::Fallback(steps) => {
                assert_eq!(steps.len(), 4);
                assert_eq!(steps[0].name, "cargo build");
                assert_eq!(
                    steps[1].command,
                    "cargo clippy --all-targets --all-features -- -D warnings"
                );
            }
            other @ (GateRecipe::Configured(_) | GateRecipe::SkipInfo { .. }) => {
                panic!("expected Fallback, got {other:?}")
            }
        }
    }

    #[test]
    fn non_rust_without_gates_skips_with_info() {
        match select_gate_recipe(Language::TypeScript, false, &[]) {
            GateRecipe::SkipInfo { language } => assert_eq!(language, "typescript"),
            other => panic!("expected SkipInfo, got {other:?}"),
        }
    }

    #[test]
    fn explicit_empty_gates_runs_zero_steps_even_for_rust() {
        match select_gate_recipe(Language::Rust, true, &[]) {
            GateRecipe::Configured(steps) => assert!(steps.is_empty()),
            other => panic!("expected empty Configured, got {other:?}"),
        }
    }

    #[test]
    fn resolve_gate_recipe_reads_config_and_infers_language() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("app.ts"), "export const x = 1;\n").unwrap();
        match resolve_gate_recipe(root).unwrap() {
            GateRecipe::SkipInfo { language } => assert_eq!(language, "typescript"),
            other => panic!("expected SkipInfo, got {other:?}"),
        }
    }

    #[test]
    fn resolve_gate_recipe_honours_configured_gates() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("app.ts"), "export const x = 1;\n").unwrap();
        std::fs::write(
            root.join("cairn.config.yaml"),
            "gates:\n  - name: unit\n    command: true\n",
        )
        .unwrap();
        match resolve_gate_recipe(root).unwrap() {
            GateRecipe::Configured(steps) => {
                assert_eq!(steps.len(), 1);
                assert_eq!(steps[0].name, "unit");
                assert_eq!(steps[0].command, "true");
            }
            other => panic!("expected Configured, got {other:?}"),
        }
    }
}
