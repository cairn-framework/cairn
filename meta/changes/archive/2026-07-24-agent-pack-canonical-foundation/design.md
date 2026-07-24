# Design: agent-pack-canonical-foundation

## Approach

Create a dev-only workspace crate at `tools/agent-pack`. Its manifest is the ownership ledger for canonical assets and the pure Claude adapter. Every canonical row names one source by logical entry id plus explicit mode. Every adapter row references that pair and names one emitted repository-relative destination.

The renderer first parses and validates the complete manifest, builds a deterministic render plan, reads every canonical source as bytes, and only then enters write mode. Check mode compares the planned bytes with checked-in outputs and reports drift without mutation. Write mode performs collision-safe atomic replacement. Neither mode injects headers or other content markers.

The path-safety API is a library seam in the dev crate so a later installer can reuse the same lexical and resolved containment rules rather than reimplement them. Lexical validation accepts only project-relative normal components before any filesystem write. Resolved validation canonicalises the root and every existing path component, rejecting a component or symlink whose resolved location leaves the root.

## Manifest contract

The TOML manifest contains:

- `schema_version` and `bundle_version`.
- `canonical` rows with `entry`, explicit `mode`, and canonical `source`.
- `adapters` rows with `harness`, `entry`, explicit `mode`, and `destination`.

Validation normalises relative paths before comparing keys. Exactly one canonical owner is permitted for each logical entry-mode pair. Each harness may have exactly one adapter producer for that pair, allowing later harnesses to consume the same canonical asset without competing ownership. Exactly one producer is permitted for each normalised emitted destination. Diagnostics identify the bad row or both conflicting rows, include the offending path or key, and state how to correct the manifest.

## Rendered baseline

The Claude adapter emits exactly these existing assets without byte changes:

- `.claude/skills/cairn-dev/SKILL.md`
- `.claude/skills/cairn-dev/references/artefact-schemas.md`
- `.claude/skills/cairn-dev/references/blueprint-syntax.md`
- `.claude/skills/cairn-dev/references/finding-codes.md`
- `.claude/skills/cairn-explore/SKILL.md`
- `.claude/skills/cairn-propose/SKILL.md`
- `.claude/skills/cairn-apply/SKILL.md`
- `.claude/skills/cairn-archive/SKILL.md`
- `.claude/commands/cairn-loop.md`
- `.claude/skills/cairn-loop-recovery/SKILL.md`
- `.claude/skills/cairn-loop-landing/SKILL.md`

`.gitattributes` enumerates only these manifest-owned outputs as `linguist-generated`. Existing `include_str!` paths remain pointed at rendered `.claude` files, preserving package contents and runtime bytes.

## Changes

ADDED:
- `tools/agent-pack/Cargo.toml` and Rust renderer/library sources.
- `tools/agent-pack/manifest.toml`.
- Canonical byte sources under `tools/agent-pack/content/`.
- Focused renderer and containment tests.

MODIFIED:
- Workspace membership in `Cargo.toml`.
- `cairn.blueprint` ownership for `tools/agent-pack`.
- `cairn.config.yaml` claim-only ownership for the dev-only pack target.
- `.gitattributes` ownership entries for the eleven rendered destinations.
- `.claude/skills/README.md` and `src/cli/commands/project.rs` ownership guidance.

REMOVED:
- None.

RENAMED:
- None.

## Failure boundaries

The renderer performs no lifecycle operation and owns only manifest-enumerated outputs. A validation or read failure aborts before destination creation or replacement. A drift failure names the destination and the write command that regenerates it. The migration grants no new meaning to entry modes and introduces no workflow graph.
