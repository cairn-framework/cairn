# Proposal: Phase 7.5b Cleansing Splits

## Dependencies

- Requires: `phase-7.5a-test-fortification` (archived).
- Execution: MUST run before Phase 8 Summariser.

## Problem/Context

Phase 7.5a established a regression wall: insta snapshots pin all 14 `/api/*` JSON wire formats, each of the five god modules carries an inline `#[cfg(test)] mod tests` block, and `scripts/check-file-sizes.sh` enforces a 500-line ceiling. All five god modules are currently allow-listed with reason `scheduled-for-phase-7.5b-split`:

| File | Lines |
|---|---|
| `src/changes.rs` | 1444 |
| `src/cli/mod.rs` | 1471 |
| `src/query_api.rs` | 1323 |
| `src/artefacts/registry.rs` | 1239 |
| `src/ui.rs` | 1009 |

The allow-list comments are load-bearing placeholders. Each comment is a self-fulfilling technical debt marker: the framework that catches drift has declared its own drift acceptable on a schedule. This phase closes that account.

The gate battery cannot meaningfully enforce the 500-line convention while five core modules carry permanent exemptions. Every future phase that adds code to any of these modules re-opens the exemption silently.

## Proposed Solution

Split each of the five god modules at its natural cohesion seam into a `<name>/mod.rs` re-exporting all public symbols plus focused submodule files. Inline unit tests migrate with the code they exercise. Snapshot tests require zero modification because no public API surface changes. Remove the `// cairn:allow-large-module` comment from every file that drops below 500 lines after the split. Run the full gate battery after each split to confirm the regression wall holds.

## Acceptance Criteria

- Each of the five source files is replaced by a directory module (`<name>/mod.rs` + submodules).
- Every submodule file is under 500 lines.
- No `// cairn:allow-large-module` comment remains in the codebase after this phase (all five are removed as each module drops below the ceiling).
- `cargo test` passes without modification to any test assertion or snapshot file.
- `bash scripts/pre-archive-rust-gates.sh` passes end-to-end including the file-size check.
- All public items previously importable from each module remain importable at the same path via `pub use` re-exports in `mod.rs`.

## Out of Scope

- API surface changes of any kind.
- Wire-format changes. All insta snapshots MUST stay green without `cargo insta review`.
- New features in any subsystem.
- Renaming public items, types, or error codes.
- Changing test assertions or helper logic.
