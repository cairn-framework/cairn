# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Extracted `src/changes/validate` inline tests into `src/changes/validate/tests.rs`.
  - Bead: cairn-npg
  - Commit: b4e95ee

## Next Candidates
1. Address `src/changes/apply.rs` (640 lines) by extracting tests or splitting into submodules.
2. Address `src/artefacts/registry/validate.rs` (659 lines) or `src/hooks/mod.rs` (619 lines).
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
