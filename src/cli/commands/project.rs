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
    let already_initialized = root.join("cairn.blueprint").exists();
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
    let mut backfilled: Vec<&str> = Vec::new();
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
        if !full.exists() {
            if fs::write(&full, content).is_err() {
                return err(1, &format!("failed to write {}", full.display()));
            }
            backfilled.push(path);
        }
    }
    if already_initialized {
        return ok(reinit_message(&backfilled));
    }
    ok(format!(
        "{}\n\n{}\n",
        copy::lookup("init.done"),
        copy::lookup("init.next-steps")
    ))
}

/// Build the message for `cairn init` run in an already-initialized project:
/// report what (if anything) was backfilled and the next steps that make sense
/// when the blueprint already exists, instead of claiming a fresh scaffold.
fn reinit_message(backfilled: &[&str]) -> String {
    let mut msg = copy::lookup("init.already").to_string();
    if backfilled.is_empty() {
        msg.push('\n');
        msg.push_str(copy::lookup("init.nothing-backfilled"));
    } else {
        msg.push_str("\n\n");
        msg.push_str(copy::lookup("init.backfilled"));
        for path in backfilled {
            msg.push_str("\n  ");
            msg.push_str(path);
        }
    }
    msg.push_str("\n\n");
    msg.push_str(copy::lookup("init.next-steps-existing"));
    msg.push('\n');
    msg
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

        // The explore skill must teach provenance queries, not just structure,
        // so a regenerated template cannot silently drop the rationale path.
        let explore_skill =
            std::fs::read_to_string(dir.path().join(".claude/skills/cairn-explore/SKILL.md"))
                .unwrap();
        assert!(
            explore_skill.contains("cairn rationale"),
            "emitted cairn-explore skill must teach the provenance query path"
        );

        // The agent guide points the agent at the loop skills.
        let guide = std::fs::read_to_string(dir.path().join(".cairn/AGENTS.md")).unwrap();
        assert!(
            guide.contains(".claude/skills/cairn-dev"),
            "agent guide must reference the dev loop skills"
        );
    }

    #[test]
    fn test_reinit_on_existing_project_does_not_claim_fresh_scaffold() {
        let dir = tempfile::tempdir().unwrap();
        // First init writes the full scaffold and reports a fresh project.
        let first = init_project(dir.path());
        assert_eq!(first.code, 0, "first init should succeed: {}", first.stderr);
        assert!(
            first.stdout.contains(copy::lookup("init.done")),
            "fresh init should report the project was initialized"
        );

        // Second init in the same dir must recognize the existing blueprint,
        // not re-announce a fresh scaffold or claim a starter was written.
        let second = init_project(dir.path());
        assert_eq!(second.code, 0, "re-init should succeed: {}", second.stderr);
        assert!(
            second.stdout.contains(copy::lookup("init.already")),
            "re-init must report the project is already initialized"
        );
        assert!(
            second
                .stdout
                .contains(copy::lookup("init.nothing-backfilled")),
            "re-init with full scaffold present must report nothing to do"
        );
        assert!(
            !second.stdout.contains("a starter was written"),
            "re-init must not claim a starter blueprint was written"
        );
    }

    #[test]
    fn test_reinit_backfills_only_missing_scaffolding() {
        let dir = tempfile::tempdir().unwrap();
        // Simulate a project that has a blueprint but is missing the rest of
        // the scaffold (e.g. created by hand or partially deleted).
        std::fs::write(dir.path().join("cairn.blueprint"), "System X id \"x\" {}\n").unwrap();

        let result = init_project(dir.path());
        assert_eq!(result.code, 0, "re-init should succeed: {}", result.stderr);
        assert!(
            result.stdout.contains(copy::lookup("init.already")),
            "re-init must report the project is already initialized"
        );
        assert!(
            result.stdout.contains(copy::lookup("init.backfilled")),
            "re-init with missing files must report what was backfilled"
        );
        // The existing blueprint must be preserved, not overwritten.
        let blueprint = std::fs::read_to_string(dir.path().join("cairn.blueprint")).unwrap();
        assert_eq!(
            blueprint, "System X id \"x\" {}\n",
            "existing blueprint must not be clobbered on re-init"
        );
        // Missing scaffolding is backfilled.
        assert!(
            dir.path().join("cairn.config.yaml").exists(),
            "missing config must be backfilled"
        );
        assert!(
            dir.path().join(".cairn/AGENTS.md").exists(),
            "missing agent guide must be backfilled"
        );
    }
}
