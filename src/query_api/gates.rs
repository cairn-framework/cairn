//! Helper to format quality gates in human and JSON representations.
use std::fmt::Write as _;
use std::path::Path;

/// Formats the quality gates recipe dynamically for bundle and brief.
/// If the project configuration declares `gates:`, lists the configured steps
/// (name + command); otherwise surfaces the language-battery selection result.
pub(crate) fn format_gates(root: &Path) -> String {
    let mut out = crate::copy::lookup("brief.gates").to_owned();
    match crate::scanner::gate_recipe::resolve_gate_recipe(root) {
        Ok(recipe) => match recipe {
            crate::scanner::gate_recipe::GateRecipe::Configured(steps)
            | crate::scanner::gate_recipe::GateRecipe::Fallback(steps) => {
                if steps.is_empty() {
                    out.push_str("\n  (zero language battery steps configured)");
                } else {
                    for step in steps {
                        let display = if step.name == step.command {
                            step.name.clone()
                        } else {
                            format!(
                                "{name}: {command}",
                                name = step.name,
                                command = step.command
                            )
                        };
                        if step.command.trim().is_empty() {
                            let _ = write!(
                                out,
                                "\n  {display:52} # FAILED: {} has no command; configure a non-empty `command:` in cairn.config.yaml",
                                step.name
                            );
                        } else {
                            let _ = write!(out, "\n  {display:52} # exit 0");
                        }
                    }
                }
            }
            crate::scanner::gate_recipe::GateRecipe::SkipInfo { language } => {
                let template = crate::copy::lookup("brief.gates-skip");
                let msg = template.replace("{language}", &language);
                let _ = write!(out, "\n{msg}");
            }
        },
        Err(err) => {
            let _ = write!(out, "\n  (could not load gates: {err})");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_configured_command_is_rendered_as_failure() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("cairn.config.yaml"),
            "gates:\n  - name: unit tests\n    command: \"   \"\n",
        )
        .unwrap();

        let rendered = format_gates(dir.path());

        let gate_line = rendered
            .lines()
            .find(|line| line.contains("unit tests"))
            .expect("configured gate line");
        assert!(gate_line.contains("FAILED"));
        assert!(gate_line.contains("no command"));
        assert!(!gate_line.contains("# exit 0"));
    }
}
