# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Extracted `src/summariser/prompt.rs` tests into `src/summariser/prompt/tests.rs`.
  - Bead: cairn-egp
  - Commit: daf42fc

## Next Candidates
1. Add valid file-size allow directives or split remaining oversized files in `src/changes/` and `src/scanner/` subtrees.
2. Address `src/query_api/mod.rs` (629 lines) by extracting tests or splitting into submodules.
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
