# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Extracted `src/hooks/mod.rs` tests into `src/hooks/tests.rs`.
  - Bead: cairn-z5a
  - Commit: 971cd5b

## Next Candidates
1. Address `src/changes/apply.rs` (640 lines) by extracting tests or splitting into submodules.
2. Address `src/artefacts/registry/validate.rs` (659 lines).
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
