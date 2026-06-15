# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Added unit tests for the private `blueprint_source_roots` helper in `src/scanner/cache.rs`, covering directory paths, file paths, subdirectory pruning, nested nodes, and leading `./` handling.
  - Bead: cairn-dft
  - Commit: 14f5037

## Result
- `scripts/pre-archive-rust-gates.sh` passes (fmt, clippy, tests, file-size check).
- `cairn lint` reports no findings.
- `use_self = "deny"` is in `[workspace.lints.clippy]` in `Cargo.toml`, preventing regression.
- Child beads `cairn-v1t.1` and `cairn-v1t.2` are blocked with notes: the decisions pointer and schema migration must land together as a deliberate repo-wide effort.
- All `#[allow(...)]` and `#![allow(...)]` directives in `src/` and `tests/` document their rationale.
- All extracted submodules have module-level docs.

## Next Candidates
1. Continue adding targeted unit tests for extracted helpers in recently-split modules (scanner cache/checks, cli/render submodules, etc.).
2. Audit remaining documentation gaps in public APIs.
3. Address the open epic `cairn-v1t` only as a deliberate, scoped milestone: migrate all decision files, add the blueprint pointer, and write covering decisions in one coordinated effort.
