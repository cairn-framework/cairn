# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Extended unit tests for the JSON serialization helpers in `src/cli/format/json.rs`, covering `finding_json`, `node_json`, `todos_json`, `decisions_json`, `reviews_json`, `sources_json`, and `research_json`.
  - Bead: cairn-h1f
  - Commit: 87102ff

## Result
- `scripts/pre-archive-rust-gates.sh` passes (fmt, clippy, tests, file-size check).
- `cairn lint` reports no findings.
- `use_self = "deny"` is in `[workspace.lints.clippy]` in `Cargo.toml`, preventing regression.
- Child beads `cairn-v1t.1` and `cairn-v1t.2` are blocked with notes: the decisions pointer and schema migration must land together as a deliberate repo-wide effort.
- All `#[allow(...)]` and `#![allow(...)]` directives in `src/` and `tests/` document their rationale.
- All extracted submodules have module-level docs.

## Next Candidates
1. Continue adding targeted unit tests for extracted helpers in recently-split modules (cli/render submodules, scanner cache/checks, etc.).
2. Audit remaining documentation gaps in public APIs.
3. Address the open epic `cairn-v1t` only as a deliberate, scoped milestone: migrate all decision files, add the blueprint pointer, and write covering decisions in one coordinated effort.
