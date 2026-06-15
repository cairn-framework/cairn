# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Extracted `src/changes/mod.rs` tests into `src/changes/tests.rs`.
  - Bead: cairn-e0m
  - Commit: c7ffe92

## Next Candidates
1. Add valid file-size allow directives or split remaining oversized files in `src/changes/apply.rs` and `src/changes/validate.rs`.
2. Address `src/artefacts/registry/validate.rs` (659 lines) or `src/hooks/mod.rs` (619 lines).
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
