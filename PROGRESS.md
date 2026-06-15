# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Add valid file-size allow directive to `src/query_api/change_queries.rs`.
  - Bead: cairn-6gk
  - Commit: c7b8877

## Next Candidates
1. Add valid file-size allow directive to `src/cli/commands.rs` (same malformed `// Reason:` prefix issue).
2. Split `src/query_api/mod.rs` (629 lines) into smaller submodules.
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
