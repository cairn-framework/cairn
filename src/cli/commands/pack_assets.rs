//! The agent-pack asset tables compiled into the binary.
//!
//! Canonical bytes live under `tools/agent-pack/content/`; the dev-only
//! renderer writes the `.claude` destinations below, which remain the
//! `include_str!` inputs so what ships is byte-identical to the checked-in
//! harness assets (`dec.agent-pack-packaging` clause 3).
//!
//! A harness changes where those bytes land, never what they say: the bodies
//! are harness-neutral, so an adapter is a pack root and nothing more. The
//! roots below mirror the `[[adapters]]` rows in
//! `tools/agent-pack/manifest.toml`, which stays the declaring authority
//! (`dec.agent-pack-packaging` clauses 1 and 2); a test holds the two in step.

use std::borrow::Cow;

/// Where the Claude adapter discovers a project pack.
pub(crate) const CLAUDE_ROOT: &str = ".claude/";
/// Where OMP's `native` provider discovers a project pack:
/// `<project>/.omp/skills/<name>/SKILL.md` and `<project>/.omp/commands/*.md`.
pub(crate) const OMP_ROOT: &str = ".omp/";

/// Harnesses with a validated adapter, and the pack root each discovers.
/// Unverified rows are contracts, not facts (`dec.agent-pack-packaging`
/// clause 2), so they are not listed here. This table is the only place a
/// harness name becomes a path.
pub(crate) const HARNESS_ROOTS: &[(&str, &str)] = &[("claude", CLAUDE_ROOT), ("omp", OMP_ROOT)];

/// A template for one installable file, before a harness roots its destination.
struct AssetTemplate {
    /// Destination the Claude adapter installs to, which is also the
    /// `include_str!` input path. Another harness swaps the pack root.
    path: &'static str,
    /// Bundled content compiled into the binary.
    content: &'static str,
}

/// One installable file: its destination relative to the project root, and the
/// bytes the binary carries for it.
pub(crate) struct PackAsset {
    /// Destination path relative to the target project root.
    pub(crate) path: Cow<'static, str>,
    /// Bundled content compiled into the binary.
    pub(crate) content: &'static str,
}

/// The pack root a harness installs into, or `None` for a harness with no
/// validated adapter.
pub(crate) fn harness_root(harness: &str) -> Option<&'static str> {
    HARNESS_ROOTS
        .iter()
        .find(|(name, _)| *name == harness)
        .map(|(_, root)| *root)
}

const fn asset(path: &'static str, content: &'static str) -> AssetTemplate {
    AssetTemplate { path, content }
}

/// The base pack: interactive routing and the lifecycle skills. Installed by
/// default, and the set `cairn init` emits.
const BASE_ASSETS: &[AssetTemplate] = &[
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
        ".claude/skills/cairn-dev/references/task-brownfield-decision-extraction.md",
        include_str!(
            "../../../.claude/skills/cairn-dev/references/task-brownfield-decision-extraction.md"
        ),
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
const LOOP_ASSETS: &[AssetTemplate] = &[
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

/// Every asset the pack can install under one pack root, base first then loop.
/// Callers pass a root from [`harness_root`], so an unvalidated harness name
/// can never reach a destination path.
pub(crate) fn all_assets(pack_root: &str, with_loop: bool) -> Vec<PackAsset> {
    BASE_ASSETS
        .iter()
        .chain(if with_loop { LOOP_ASSETS } else { &[] })
        .map(|template| PackAsset {
            path: if pack_root == CLAUDE_ROOT {
                Cow::Borrowed(template.path)
            } else {
                Cow::Owned(template.path.replacen(CLAUDE_ROOT, pack_root, 1))
            },
            content: template.content,
        })
        .collect()
}
