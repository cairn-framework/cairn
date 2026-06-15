# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Extracted `src/query_api/mod.rs` tests into `src/query_api/tests.rs`.
  - Bead: cairn-38g
  - Commit: 48696a4

## Next Candidates
1. Add valid file-size allow directives or split remaining oversized files in `src/changes/` and `src/scanner/` subtrees.
2. Address `src/scanner/config.rs` (723 lines) by extracting its large inline test block.
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
