---
node: cairn.reconcile
status: blocked
created: 2026-07-25
---

# Node Symbol Coverage

## Priority

P2. Query implementation, raised by measured retrieval evidence rather than by
output size. It gates nothing in the agent-guidance programme; no guidance may
consume it before it is delivered and verified.

## Problem

`cairn get <node> --symbols` and `cairn locate <symbol>` return almost nothing
for a Rust binary crate. `rust_is_exportable` (`src/reconcile/code.rs:63-69`)
admits an item only when it carries a `visibility_modifier`, so a crate whose
items are crate-private exposes no symbols at all. Measured on the pinned
ripgrep fixture (`4649aa97`), `cairn get crates.core.flags --symbols` returned
one symbol for a module whose `defs.rs` alone declares 104 structs, and
`cairn locate TypeList` returned an empty array. Python has no equivalent
filter, so the pinned flask fixture returned 688 symbols across sixteen files.

The consequence is measured, not hypothetical. In
`res.loop-efficiency-observations` (2026-07-25 entry) the two symbol-bearing
compositions, primitive and topology-first, scored 0.000 recall on both ripgrep
tasks and 1.000 on both flask tasks. The composition was not the variable; the
symbols were never in the graph to retrieve. The other two candidates,
bundle-centred and `context_projection_v1`, scored 0.200 on ripgrep IMP and
0.000 on ripgrep LOC, reaching 0.500 and 0.286 on the flask strata. Their single
ripgrep hit comes from `bundle.dependencies[]`, which carries dependency symbols
that `get --symbols` never returns, so that exception is itself evidence of the
coverage gap rather than a counter-example to it.

## Investigation outcome

The measured coverage gap is real, but the fix is L-sized and multi-seam, not
an S fix. `res.node-symbol-coverage.investigation` maps the separation:
interface hashes must keep the current exported predicate, while exact
navigation queries need a distinct query-visible record stream. The existing
`NodeRecord.symbols` also feeds dependency-interface bundles, contract
interface drift checks, persistent map snapshots, and the web UI, so widening
that field in place would leak private definitions into interface surfaces.

The parent is decomposed and blocked until the child units complete. No Rust
implementation lands in this research PR.

## Re-scoped implementation boundary

- Keep exported signatures, records, interface fingerprints, target hashes,
  dependency bundles, contract checks, and persistent map snapshots unchanged.
- Add a query-visible extraction stream for Rust and TypeScript in the generic
  reconciler, with cache-safe propagation through reports and scanner assembly.
- Store both views explicitly and route only `get --symbols` and exact
  `locate` to the query-visible view.
- Preserve exact-match lookup and do not add full-text, fuzzy, RAG, or stored
  duplicate state.
- Re-run the frozen context-bundle harness against the pinned ripgrep manifest
  after implementation and report the recall delta.

## Child units

- `todo.node-symbol-coverage-ruling`: author and ratify the binding decision
  informed by `res.node-symbol-coverage.investigation`.
- `todo.node-symbol-coverage-reconcile`: split extraction and report/cache
  streams while preserving exported hashes.
- `todo.node-symbol-coverage-query`: wire graph, CLI, and query API navigation
  surfaces while keeping bundles and snapshots exported-only.
- `todo.node-symbol-coverage-evaluation`: run Rust and TypeScript fixtures and
  the frozen corpus evaluation.

## Acceptance

The child units must prove query-visible definition coverage for Rust and
TypeScript, unchanged interface hashes, exported-only interface consumers, and
the frozen ripgrep recall delta. The parent remains blocked until all child
units and their gates are complete.

Research: `res.node-symbol-coverage.investigation`.

Informed by: `res.loop-efficiency-observations`.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves investigable. It improves investigation when symbols need precise graph coverage.

2026-08-07 audit (todo.roadmap-assumption-audit): keep as written.
