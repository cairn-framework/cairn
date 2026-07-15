---
node: cairn.kernel.cli
status: open
created: 2026-07-15
---

# Health Info Count Key Mismatch

## Problem

`cairn health` human output reads `summary.total_info` (`src/cli/render/health.rs:53-56`) but the JSON payload emits `summary.info` (`src/query_api/handlers/remediate.rs:69`), so human `cairn health` shows `info: 0` while `cairn health --json` reports the real info count.

## Evidence

Discovered during PR #346 review. Present on main, pre-existing.

## Approach (backlog only, test-first when built)

Align the key (human reads `summary.info`, or emit `total_info` consistently) and add a test that human and JSON info counts agree. Do NOT implement here.
