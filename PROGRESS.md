# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Added a convention test (`tests/conventions.rs::test_every_source_file_has_module_doc`) that walks `src/**/*.rs` and enforces a top-level `//!` module doc in the first five lines, and added missing module docs to the two extracted test submodules that lacked them.
  - Bead: cairn-id4
  - Commit: f9df7fe

## Result
- `scripts/pre-archive-rust-gates.sh` passes (fmt, clippy, tests, file-size check).
- `cairn lint` reports no findings.
- `use_self = "deny"` is in `[workspace.lints.clippy]` in `Cargo.toml`, preventing regression.
- Child beads `cairn-v1t.1` and `cairn-v1t.2` are blocked with notes: the decisions pointer and schema migration must land together as a deliberate repo-wide effort.
- All `#[allow(...)]` and `#![allow(...)]` directives in `src/` and `tests/` document their rationale.
- All source files now have module-level docs, enforced by the conventions test.

## Next Candidates
1. Continue adding targeted unit tests for extracted helpers in recently-split modules.
2. Audit remaining documentation gaps in public APIs.
3. Address the open epic `cairn-v1t` only as a deliberate, scoped milestone: migrate all decision files, add the blueprint pointer, and write covering decisions in one coordinated effort.
