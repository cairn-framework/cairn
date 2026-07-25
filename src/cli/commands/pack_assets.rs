//! The agent-pack asset tables compiled into the binary.
//!
//! Canonical bytes live under `tools/agent-pack/content/`; the dev-only
//! renderer writes the `.claude` destinations below, which remain the
//! `include_str!` inputs so what ships is byte-identical to the checked-in
//! harness assets (`dec.agent-pack-packaging` clause 3).

/// One installable file: its destination relative to the project root, and the
/// bytes the binary carries for it.
pub(crate) struct PackAsset {
    /// Destination path relative to the target project root.
    pub(crate) path: &'static str,
    /// Bundled content compiled into the binary.
    pub(crate) content: &'static str,
}

const fn asset(path: &'static str, content: &'static str) -> PackAsset {
    PackAsset { path, content }
}

/// The base pack: interactive routing and the lifecycle skills. Installed by
/// default, and the set `cairn init` emits.
pub(crate) const BASE_ASSETS: &[PackAsset] = &[
    asset(
        ".claude/skills/cairn-dev/SKILL.md",
        include_str!("../../../.claude/skills/cairn-dev/SKILL.md"),
    ),
    asset(
        ".claude/skills/cairn-dev/references/blueprint-syntax.md",
        include_str!("../../../.claude/skills/cairn-dev/references/blueprint-syntax.md"),
    ),
    asset(
        ".claude/skills/cairn-dev/references/finding-codes.md",
        include_str!("../../../.claude/skills/cairn-dev/references/finding-codes.md"),
    ),
    asset(
        ".claude/skills/cairn-dev/references/artefact-schemas.md",
        include_str!("../../../.claude/skills/cairn-dev/references/artefact-schemas.md"),
    ),
    asset(
        ".claude/skills/cairn-dev/references/command-reference.md",
        include_str!("../../../.claude/skills/cairn-dev/references/command-reference.md"),
    ),
    asset(
        ".claude/skills/cairn-dev/references/graph-navigation.md",
        include_str!("../../../.claude/skills/cairn-dev/references/graph-navigation.md"),
    ),
    asset(
        ".claude/skills/cairn-dev/references/task-bug-investigation.md",
        include_str!("../../../.claude/skills/cairn-dev/references/task-bug-investigation.md"),
    ),
    asset(
        ".claude/skills/cairn-dev/references/task-refactoring.md",
        include_str!("../../../.claude/skills/cairn-dev/references/task-refactoring.md"),
    ),
    asset(
        ".claude/skills/cairn-dev/references/task-architecture-discovery.md",
        include_str!("../../../.claude/skills/cairn-dev/references/task-architecture-discovery.md"),
    ),
    asset(
        ".claude/skills/cairn-dev/references/task-feature-implementation.md",
        include_str!("../../../.claude/skills/cairn-dev/references/task-feature-implementation.md"),
    ),
    asset(
        ".claude/skills/cairn-explore/SKILL.md",
        include_str!("../../../.claude/skills/cairn-explore/SKILL.md"),
    ),
    asset(
        ".claude/skills/cairn-propose/SKILL.md",
        include_str!("../../../.claude/skills/cairn-propose/SKILL.md"),
    ),
    asset(
        ".claude/skills/cairn-apply/SKILL.md",
        include_str!("../../../.claude/skills/cairn-apply/SKILL.md"),
    ),
    asset(
        ".claude/skills/cairn-archive/SKILL.md",
        include_str!("../../../.claude/skills/cairn-archive/SKILL.md"),
    ),
];

/// Loop mode and its required asset closure, plus the adapter-native command
/// that resolves to it. Opt in only: the shipped `cairn-dev` router reads the
/// absence of `references/loop-mode.md` as "loop mode is unavailable in this
/// repository", so installing it by default would make that signal a lie.
pub(crate) const LOOP_ASSETS: &[PackAsset] = &[
    asset(
        ".claude/skills/cairn-dev/references/loop-mode.md",
        include_str!("../../../.claude/skills/cairn-dev/references/loop-mode.md"),
    ),
    asset(
        ".claude/skills/cairn-loop-scope/SKILL.md",
        include_str!("../../../.claude/skills/cairn-loop-scope/SKILL.md"),
    ),
    asset(
        ".claude/skills/cairn-loop-implement/SKILL.md",
        include_str!("../../../.claude/skills/cairn-loop-implement/SKILL.md"),
    ),
    asset(
        ".claude/skills/cairn-loop-recovery/SKILL.md",
        include_str!("../../../.claude/skills/cairn-loop-recovery/SKILL.md"),
    ),
    asset(
        ".claude/skills/cairn-loop-reconcile/SKILL.md",
        include_str!("../../../.claude/skills/cairn-loop-reconcile/SKILL.md"),
    ),
    asset(
        ".claude/skills/cairn-loop-landing/SKILL.md",
        include_str!("../../../.claude/skills/cairn-loop-landing/SKILL.md"),
    ),
    asset(
        ".claude/commands/cairn-loop.md",
        include_str!("../../../.claude/commands/cairn-loop.md"),
    ),
];

/// Every asset the pack can install, base first then loop.
pub(crate) fn all_assets(with_loop: bool) -> Vec<&'static PackAsset> {
    BASE_ASSETS
        .iter()
        .chain(if with_loop { LOOP_ASSETS } else { &[] })
        .collect()
}
