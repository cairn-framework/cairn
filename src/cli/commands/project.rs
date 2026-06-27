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

/// Agent-facing guide written by `cairn init`, appended to a project's CLAUDE.md or AGENTS.md.
const AGENT_GUIDE: &str = include_str!("../agent_guide.md");

/// Curated cairn dev-loop agent skills bundled into the binary and emitted by
/// `cairn init` so an agent landing in a fresh repo has an on-ramp to the loop.
/// Single source of truth is cairn's own `.claude/skills/` tree, compiled in
/// via `include_str!`, so the emitted pack never drifts from the skills cairn
/// develops itself with.
const SKILL_FILES: &[(&str, &str)] = &[
    (
        ".claude/skills/cairn-dev/SKILL.md",
        include_str!("../../../.claude/skills/cairn-dev/SKILL.md"),
    ),
    (
        ".claude/skills/cairn-dev/references/blueprint-syntax.md",
        include_str!("../../../.claude/skills/cairn-dev/references/blueprint-syntax.md"),
    ),
    (
        ".claude/skills/cairn-dev/references/finding-codes.md",
        include_str!("../../../.claude/skills/cairn-dev/references/finding-codes.md"),
    ),
    (
        ".claude/skills/cairn-dev/references/artefact-schemas.md",
        include_str!("../../../.claude/skills/cairn-dev/references/artefact-schemas.md"),
    ),
    (
        ".claude/skills/cairn-explore/SKILL.md",
        include_str!("../../../.claude/skills/cairn-explore/SKILL.md"),
    ),
    (
        ".claude/skills/cairn-propose/SKILL.md",
        include_str!("../../../.claude/skills/cairn-propose/SKILL.md"),
    ),
    (
        ".claude/skills/cairn-apply/SKILL.md",
        include_str!("../../../.claude/skills/cairn-apply/SKILL.md"),
    ),
    (
        ".claude/skills/cairn-archive/SKILL.md",
        include_str!("../../../.claude/skills/cairn-archive/SKILL.md"),
    ),
];

pub(crate) fn init_project(root: &Path) -> CliResult {
    let writes = [
        (
            "cairn.blueprint",
            "# Describe your system here. Every source file (tests included) should\n# fall under some module's path. Grammar reference:\n# https://github.com/cairn-framework/cairn/blob/HEAD/docs/blueprint.md\nSystem Example \"Starter architecture\" id \"example\" {\n    Module App \"Starter app\" id \"example.app\" {\n        path \"./src\"\n    }\n}\n",
        ),
        (
            "cairn.config.yaml",
            "reconcilers:\n  - id: rust-code\n    version: phase-1\n    config:\n      ignore:\n        - target\ncontext: \"\"\nrules: {}\n",
        ),
        ("meta/contracts/.gitkeep", ""),
        (".cairn/state/.gitkeep", ""),
        (".cairn/AGENTS.md", AGENT_GUIDE),
    ];
    for &(path, content) in writes.iter().chain(SKILL_FILES) {
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
    ok(format!(
        "{}\n\n{}\n",
        copy::lookup("init.done"),
        copy::lookup("init.next-steps")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_project_emits_cairn_skills_and_loop_guide() {
        let dir = tempfile::tempdir().unwrap();
        let result = init_project(dir.path());
        assert_eq!(result.code, 0, "init should succeed: {}", result.stderr);

        // Every bundled cairn-* skill lands in the fresh repo.
        for (path, _content) in SKILL_FILES {
            assert!(
                dir.path().join(path).exists(),
                "init must emit skill file {path}"
            );
        }

        // Content is actually copied, not stubbed.
        let dev_skill =
            std::fs::read_to_string(dir.path().join(".claude/skills/cairn-dev/SKILL.md")).unwrap();
        assert!(
            dev_skill.contains("name: cairn-dev"),
            "emitted cairn-dev skill must carry its frontmatter"
        );

        // The agent guide points the agent at the loop skills.
        let guide = std::fs::read_to_string(dir.path().join(".cairn/AGENTS.md")).unwrap();
        assert!(
            guide.contains(".claude/skills/cairn-dev"),
            "agent guide must reference the dev loop skills"
        );
    }
}
