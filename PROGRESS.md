# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Split `src/cli/render.rs` into per-concern rendering modules.
  - Bead: cairn-d7v
  - Commit: 86a5229

## Next Candidates
1. Add valid file-size allow directives or split remaining oversized files in `src/changes/` and `src/scanner/` subtrees.
2. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
3. Add missing test coverage for the newly split CLI format/render modules.
