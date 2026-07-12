---
node: cairn.kernel.query
status: open
created: 2026-07-12
---

# Remediate Copy Centralisation

## Problem
All 14 remediation action `description` strings in
`src/query_api/handlers/remediate.rs` (lines ~224-343) are hardcoded at the
emission site; the handler never consults `src/copy.rs`, `copy.toml` has no
remediation section, and `src/cli/render/remediate.rs` passes the JSON
`description` through verbatim. This violates the convention that
user-facing strings live in `docs/design-system/copy.toml`. Surfaced during
PR #271/#272 review of the `CAIRN_ORDER_CYCLE` wording change.

## Task
Add a `[remediate.actions]` section to `docs/design-system/copy.toml` keyed
by action id (`fix_order`, `fix_research`, ...), resolve descriptions in the
handler via `src/copy.rs` lookups, and move all 14 strings in one pass (no
split convention). Add a test asserting an emitted remediation description
matches the copy entry.
