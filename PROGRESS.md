# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Moved `src/summariser/backend.rs` tests into a submodule.
  - Bead: cairn-4r7
  - Commit: 5aada5d

## Next Candidates
1. Split `src/summariser/prompt.rs` (743 lines) or add a valid file-size allow directive.
2. Add valid file-size allow directives or split remaining oversized files in `src/changes/` and `src/scanner/` subtrees.
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
