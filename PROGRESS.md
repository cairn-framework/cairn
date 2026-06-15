# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Split `src/scanner/mod.rs` into `cache.rs`, `checks.rs`, and `tests.rs` submodules.
  - Bead: cairn-0s5
  - Commit: fd44dee

## Next Candidates
1. Add valid file-size allow directives or split remaining oversized files in `src/changes/` and `src/state/` subtrees.
2. Address `src/state/mod.rs` (1077 lines) by extracting tests or splitting into submodules.
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
