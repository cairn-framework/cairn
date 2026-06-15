# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Extracted `src/mcp.rs` inline tests into `src/mcp/tests.rs`.
  - Bead: cairn-xcc
  - Commit: 0bd4cec

## Next Candidates
1. Split `src/summariser/prompt.rs` (743 lines) or add a valid file-size allow directive.
2. Add valid file-size allow directives or split remaining oversized files in `src/changes/` and `src/scanner/` subtrees.
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
