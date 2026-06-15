# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Split `src/state/mod.rs` into `beads.rs` and `tests.rs` submodules.
  - Bead: cairn-e2v
  - Commit: bd534e0

## Next Candidates
1. Add valid file-size allow directives or split remaining oversized files in `src/changes/`.
2. Address `src/changes/mod.rs` (671 lines) or `src/changes/apply.rs` (640 lines).
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
