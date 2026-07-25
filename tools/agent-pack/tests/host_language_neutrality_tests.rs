//! Host-language neutrality of the shipped change-lifecycle guidance.
//!
//! `cairn change accept` resolves a project's verification battery from the
//! host repository: an explicit `gates:` list wins, a Rust project falls back to
//! a built-in cargo battery, and every other language gets an informational
//! note. The guidance that ships to a user's repository must not contradict that
//! by prescribing one language's battery to every host.
//!
//! These tests pin that contract on the canonical sources, which are the
//! authored bytes; `checked_in_claude_outputs_match_the_real_manifest` separately
//! proves the rendered `.claude` destinations match them.

use std::path::{Path, PathBuf};

/// Skills that walk a user through implementing, accepting, and archiving a
/// change. These are the ones a non-Rust host follows end to end.
const LIFECYCLE_SKILLS: [&str; 3] = [
    "content/skills/cairn-propose/SKILL.md",
    "content/skills/cairn-apply/SKILL.md",
    "content/skills/cairn-archive/SKILL.md",
];

/// Command prefixes that only make sense in a Rust host.
const LANGUAGE_SPECIFIC_COMMANDS: [&str; 6] = [
    "cargo build",
    "cargo test",
    "cargo clippy",
    "cargo fmt",
    "cargo run",
    "cargo check",
];

fn canonical(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

/// A line is prescriptive if it tells the reader to run something: a fenced
/// command line, a shell prompt, or a bullet listing commands to execute.
/// Prose that *describes* what the binary does ("falls back to a built-in cargo
/// battery") is not an instruction and is allowed.
fn prescriptive_lines(body: &str) -> Vec<(usize, String)> {
    let mut in_fence = false;
    let mut hits = Vec::new();
    for (index, raw) in body.lines().enumerate() {
        let line = raw.trim();
        if line.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        let is_command_position = in_fence || line.starts_with("$ ");
        if !is_command_position {
            continue;
        }
        if LANGUAGE_SPECIFIC_COMMANDS
            .iter()
            .any(|command| line.contains(command))
        {
            hits.push((index + 1, raw.to_owned()));
        }
    }
    hits
}

#[test]
fn lifecycle_skills_prescribe_no_language_specific_battery() {
    for skill in LIFECYCLE_SKILLS {
        let path = canonical(skill);
        let body = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()));
        let hits = prescriptive_lines(&body);
        assert!(
            hits.is_empty(),
            "{skill} instructs a host to run a language-specific battery, so a \
             non-Rust project following it would run the wrong gates: {hits:?}"
        );
    }
}

#[test]
fn apply_skill_routes_the_host_to_its_own_gates_and_to_accept() {
    let body = std::fs::read_to_string(canonical("content/skills/cairn-apply/SKILL.md")).unwrap();
    assert!(
        body.contains("cairn change accept"),
        "cairn-apply must still reach Cairn's acceptance boundary"
    );
    assert!(
        body.contains("cairn.config.yaml"),
        "cairn-apply must name the configured-gates surface a host can set"
    );
    for authority in ["AGENTS.md", "CONTRIBUTING.md"] {
        assert!(
            body.contains(authority),
            "cairn-apply must defer to the host repository's own instructions ({authority})"
        );
    }
}

#[test]
fn apply_skill_explains_cc002_rather_than_dangling_the_code() {
    let body = std::fs::read_to_string(canonical("content/skills/cairn-apply/SKILL.md")).unwrap();
    let Some(position) = body.find("CC002") else {
        return; // the code may be dropped entirely; only a dangling mention is a defect
    };
    let explanation = &body[position..];
    for expected in ["suggested-edges.json", "triage_state", "pending"] {
        assert!(
            explanation.contains(expected),
            "CC002 is mentioned without explaining {expected}, leaving the reader \
             with an unactionable error code"
        );
    }
}

#[test]
fn propose_skill_requires_an_acceptance_boundary_and_its_evidence() {
    let body = std::fs::read_to_string(canonical("content/skills/cairn-propose/SKILL.md")).unwrap();
    for expected in ["Outcome", "Acceptance boundary", "Evidence", "Exclusions"] {
        assert!(
            body.contains(expected),
            "cairn-propose must make the author name {expected}"
        );
    }
}

#[test]
fn apply_skill_keeps_scheduling_out_of_cairn() {
    let body = std::fs::read_to_string(canonical("content/skills/cairn-apply/SKILL.md")).unwrap();
    assert!(
        body.contains("Cairn does not schedule"),
        "the parallel-work guidance must deny that Cairn schedules agents \
         (dec.no-orchestrator)"
    );
}
