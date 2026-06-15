//! CLI project-scaffolding command implementations.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn run_ui_command(parsed: &ParsedArgs) -> CliResult {
    match ui::UiOptions::from_args(&parsed.command_args) {
        Ok(mut options) => {
            options.blueprint_path.clone_from(&parsed.file);
            ui::serve_current_thread(options).map_or_else(
                |error| err(1, &error.to_string()),
                |message| ok(format!("{message}\n")),
            )
        }
        Err(message) => err(2, &message),
    }
}

pub(crate) fn init_project(root: &Path) -> CliResult {
    let writes = [
        (
            "cairn.blueprint",
            "System Example \"Starter architecture\" id \"example\" {\n    Module App \"Starter app\" id \"example.app\" {\n        path \"./src\"\n    }\n}\n",
        ),
        (
            "cairn.config.yaml",
            "reconcilers:\n  - id: rust-code\n    version: phase-1\n    config:\n      ignore:\n        - target\ncontext: \"\"\nrules: {}\n",
        ),
        ("meta/contracts/.gitkeep", ""),
        (".cairn/state/.gitkeep", ""),
    ];
    for (path, content) in writes {
        let full = root.join(path);
        if let Some(parent) = full.parent()
            && let Err(error) = fs::create_dir_all(parent)
        {
            return err(
                1,
                &format!("failed to create {}: {error}", parent.display()),
            );
        }
        if !full.exists() && fs::write(&full, content).is_err() {
            return err(1, &format!("failed to write {}", full.display()));
        }
    }
    ok("initialized Cairn project\n".to_owned())
}
