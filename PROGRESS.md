# Cairn Atomic Improvement Loop

## Current Status
- In-progress: none

## Last Completed
- Add valid file-size allow directive to `src/query_api/change_queries.rs`.
  - Bead: cairn-6gk
  - Commit: c7b8877

## Next Candidates
1. Add a `// cairn:allow-large-module reason:` line as line 1 of `src/cli/commands.rs`, keeping the existing `// Reason:` wildcard-import rationale intact.
2. Split `src/query_api/mod.rs` (629 lines) into smaller submodules.
3. Address remaining oversized files surfaced by `scripts/check-file-sizes.sh` one per commit.
