# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Split `src/query_api/handlers.rs` into domain submodules to satisfy the 500-line file-size gate.
  - Commit: 03a3cae

## Next Candidates
1. Fix malformed `// Reason:` allow comments on `src/cli/commands.rs` and `src/query_api/change_queries.rs` (should be `// cairn:allow-large-module reason:`).
2. Split `src/query_api/change_queries.rs` (686 lines) by query tool domain.
3. Split `src/cli/commands.rs` (755 lines) into command-dispatch submodules.
4. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
