# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Split `src/cli/commands.rs` into per-command submodules.
  - Bead: cairn-34y
  - Commit: 4287ddd

## Next Candidates
1. Add valid file-size allow directives or split remaining oversized files in `src/changes/` and `src/scanner/` subtrees.
2. Split `src/cli/render.rs` (765 lines) and `src/cli/format.rs` (725 lines) into per-concern modules.
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
