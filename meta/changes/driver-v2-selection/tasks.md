# Tasks: driver-v2-selection

- [ ] 1. Read-surface audit: at a pinned commit, capture `cairn next --json`,
  `cairn pending --json`, `cairn frontier --json`, and `cairn lint --json`,
  and record whether a mission line (unit id, node, selection ground,
  reproducible evidence) can be built from them alone. Land the result as
  `meta/research/driver-v2-read-surface.md`.
- [ ] 2. File every gap the audit finds as its own todo against the owning
  node, with the missing field named. Do not widen this change to close them.
- [ ] 3. Driver-side (external repository, tracked here only because the
  acceptance run needs it): mission constructor, unit-id ledger, and
  `--dry-run`.
- [ ] 4. Evidence: the dry-run equivalence transcript against a live loop
  session's Orient at the same commit, plus a terminal-state run on a drained
  backlog exiting zero without dispatching.
