---
node: cairn.reconcile
status: open
created: 2026-08-09
---

# Node Symbol Coverage Ruling


## Goal

Author and ratify `dec.node-symbol-coverage`, informed by
`res.node-symbol-coverage.investigation`, before any behavioural code change.

## Scope

Record the binding distinction between exported interface symbols and
query-visible definition symbols. The ruling must state that interface hashes,
dependency-interface bundles, contract interface checks, and persistent map
snapshots stay on the exported set, while exact `get --symbols` and `locate`
queries may use the query-visible set. Cover both Rust and TypeScript and state
the cache and wire compatibility expectations.

## Acceptance

- `meta/decisions/node-symbol-coverage.md` exists with
  `informed_by: [res.node-symbol-coverage.investigation]` and accepted status.
- The decision names the exported and query-visible fields or types that the
  implementation sub-todos must preserve.
- No implementation starts until the decision is accepted.