---
node: cairn.reconcile
status: blocked
created: 2026-08-09
blocked_by: [todo.node-symbol-coverage-ruling, todo.node-symbol-coverage-reconcile]
parent: todo.node-symbol-coverage
---

# Node Symbol Coverage Query


## Goal

After the ruling and transient extractor land, expose query-visible records
only through navigation queries. Use
`res.node-symbol-coverage.investigation` to preserve the existing exported
interface surfaces.

## Scope

Add query-time context plumbing, not a stored graph field or report field, then
update `src/cli/render/node.rs`, `src/map/query.rs`,
`src/query_api/mod.rs`, `src/query_api/serialise.rs`, and
`src/query_api/handlers/locate.rs` so `cairn get <node> --symbols` and exact
`locate` invoke the transient extractor with the root, language, and claimed
files. Keep `src/cli/render/bundle.rs`,
`src/query_api/handlers/bundle.rs`, `src/scanner/checks.rs`, and
`src/scanner/snapshot.rs` on the exported field. Do not add query records to
`NodeRecord`, the cache, `map.json`, or any other persisted state. Update typed
query tests and wire tests for the new context.

The human CLI path bypasses `query_api`; it must call the same single transient
helper as structured `get` and `locate`. If the ruling widens
`src/ui/server.rs:228-233`, that route must use the helper and gain a UI
regression test; otherwise preserve its exported-only behavior. Query-time
read or parse failures must map to `QueryError`, with a TOCTOU test covering a
file changed or removed after the scan.

## Acceptance

- A failing end-to-end test first shows a private definition missing from
  `get --symbols` and `locate`.
- The test passes after implementation and both query surfaces report file,
  line, end line, kind, name, and signature.
- Bundle dependency interfaces, contract drift checking, and `map.json`
  remain exported-only, with regression tests for each boundary.
- The strict scan and hook gates pass.