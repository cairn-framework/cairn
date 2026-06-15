# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Split `src/cli/format.rs` into focused submodules (json, render, util).
  - Bead: cairn-q59
  - Commit: 0639fbd

## Next Candidates
1. Split `src/cli/render.rs` (765 lines) into per-concern rendering modules.
2. Add valid file-size allow directives or split remaining oversized files in `src/changes/` and `src/scanner/` subtrees.
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
