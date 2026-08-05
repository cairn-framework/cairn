# Tasks: driver-v2-selection

- [ ] 1. Read-surface audit: at a pinned commit, capture `cairn next --json`,
  `cairn pending --json`, `cairn frontier --json`, and `cairn lint --json`,
  and record whether a mission line (unit id, node, selection ground,
  reproducible evidence) can be built from them alone. Land the result as
  `meta/research/driver-v2-read-surface.md`.
- [ ] 2. File every gap the audit finds as its own todo against the owning
  node, with the missing field named. Do not widen this change to close them.

Tasks 3 and 4 (the external mission constructor, unit-id ledger,
dry-run, and their acceptance evidence) are retired as of 2026-08-04:
`dec.orchestration-placement` (accepted) moved the successor in-repo,
and `todo.driver-in-repo` now owns the dry-run equivalence contract and
the fail-closed catalogue. This change keeps only the read-surface
audit (task 1) and gap filing (task 2).
