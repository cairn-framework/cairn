# Spec: Phase 6 — Workspace

## Acceptance criteria

- A `cairn.workspace` with a single member `root = "."` makes
  `cairn workspace status` print one member row plus a totals row matching
  `cairn status` for this repo.
- Two `tempfile` fixture projects in one workspace produce correct summed
  totals in `cairn workspace status`.
- Deleting one member's root produces a `CAIRN_WORKSPACE_MEMBER_MISSING`
  error finding for that member while the other member is still reported.
