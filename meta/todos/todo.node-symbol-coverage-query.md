---
node: cairn.reconcile
status: open
created: 2026-08-09
---

# Node Symbol Coverage Query


## Goal

After the ruling and reconciler stream split, expose query-visible records only
through navigation queries. Use `res.node-symbol-coverage.investigation` to
preserve the existing exported interface surfaces.

## Scope

Add an explicit query-visible field to the graph node or its typed query
response, then update `src/cli/render/node.rs`, `src/map/query.rs`,
`src/query_api/serialise.rs`, and `src/query_api/handlers/locate.rs` so
`cairn get <node> --symbols` and exact `locate` return crate-private Rust and
unexported TypeScript definitions. Keep
`src/cli/render/bundle.rs`, `src/query_api/handlers/bundle.rs`,
`src/scanner/checks.rs`, and `src/scanner/snapshot.rs` on the exported field.
Update graph construction and all affected fixtures and wire tests.

## Acceptance

- A failing end-to-end test first shows a private definition missing from
  `get --symbols` and `locate`.
- The test passes after implementation and both query surfaces report file,
  line, end line, kind, name, and signature.
- Bundle dependency interfaces, contract drift checking, and `map.json`
  remain exported-only, with regression tests for each boundary.
- The strict scan and hook gates pass.