---
node: cairn.kernel.cli
status: done
created: 2026-07-15
---

# Health Info Count Key Mismatch

## Problem

`cairn health` human output reads `summary.total_info` (`src/cli/render/health.rs:53-56`) but the JSON payload emits `summary.info` (`src/query_api/handlers/remediate.rs:69`), so human `cairn health` shows `info: 0` while `cairn health --json` reports the real info count.

## Evidence

Discovered during PR #346 review. Present on main, pre-existing.

## Approach (backlog only, test-first when built)

Align the key (human reads `summary.info`, or emit `total_info` consistently) and add a test that human and JSON info counts agree. Do NOT implement here.

## Resolution (2026-07-16)

The JSON payload now emits `summary.total_info` (`src/query_api/handlers/remediate.rs`), matching its siblings `total_errors` and `total_warnings` and the key the human renderer already read. The JSON key was the outlier, so the emitted key changed rather than the renderer; this is a deliberate wire-shape change, noted in the PR. Regression test `test_health__human_info_count_matches_json_summary` in `tests/phase_7_7_ux_foundation.rs` asserts the human info count equals the JSON summary count for a fixture with an info finding.
