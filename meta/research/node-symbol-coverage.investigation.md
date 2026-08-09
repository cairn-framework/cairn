---
id: res.node-symbol-coverage.investigation
nodes: [cairn.reconcile]
sources: []
method: primary
date: 2026-08-09
---

# Node symbol coverage investigation

## Verdict

The measured gap is real. Interface-hash exportability and query-visible
coverage are different predicates, and they should be separated. The change is
L-sized and multi-seam, not an S fix: it crosses the generic reconciler,
reconciler cache, scanner target assembly, graph storage, CLI and query
consumers, and tests. The parent todo therefore decomposes into sub-todos and
moves to `blocked`. No Rust implementation is included in this research PR.

The safe direction is to keep the current exported set as the input to
interface hashes, dependency-interface bundles, contract interface checks, and
persistent map snapshots. Add a second, query-only record set for exact
navigation queries. The query-only set must not become a second hash source or
leak private definitions into the existing interface surfaces.

## Two-predicate question

The current pipeline has one filtered set serving two jobs:

1. `E_export`, the interface set. A language spec lists eligible tree-sitter
   item kinds, then its `is_exportable` predicate admits only items that belong
   to the language's externally visible surface. For Rust,
   `PUBLIC_ITEM_KINDS` is in `src/reconcile/code.rs:11-22`, and
   `rust_is_exportable` at `src/reconcile/code.rs:63-69` requires a
   `visibility_modifier`. This includes `pub`, `pub(crate)`, and related Rust
   visibility forms, but excludes crate-private items.
2. `Q_query`, the definition-site set. `cairn get <node> --symbols` and exact
   `cairn locate <symbol>` are navigation queries over the code owned by the
   node. An agent routed to a crate-private Rust module still needs its
   definitions and source locations, even though those definitions are not a
   downstream interface.

`src/reconcile/generic.rs:44-57` currently exposes one `exportable_kinds` list
and one `is_exportable` callback. `parse_file` at lines 93-134 returns one
flattened signature vector and one record vector. `collect_public_symbols` at
lines 137-160 applies the same callback before constructing either output.
There is no independent query-visible predicate or record stream.

## Where the exported set feeds hashes

The exported set must remain unchanged for interface identity:

- `collect_public_symbols` builds signatures and `SymbolRecord` values from the
  same accepted item.
- `sequential_reconcile` sorts the flattened signatures and computes
  `InterfaceFingerprint::from_sorted(&symbols)` at
  `src/reconcile/generic.rs:260-298`.
- `parallel_reconcile` does the same at `src/reconcile/generic.rs:301-377`.
- `ReconcileReport.node_symbols` is documented as the per-node input to
  interface fingerprints in `src/reconcile/mod.rs:60-62`.
- `src/scanner/mod.rs:309-345` reads `node_symbols` and computes each
  `TargetReport.hash` with `InterfaceFingerprint::from_symbols`.
- The cached path repeats that exact hash calculation in
  `src/scanner/cache.rs:249-299`.

The existing private-symbol regression test in `tests/kernel.rs:925-968`
confirms the intended invariant: changing private Rust names does not change
the interface hash. A query-coverage fix must keep that test and its contract.

## Where the set feeds query-visible output

The structured exported records currently travel through the same report path:

- `ReconcileReport.node_symbol_records` is populated beside `node_symbols`.
- `src/scanner/mod.rs:314-323` copies the per-node records to
  `TargetReport.symbol_records`, and lines 539-543 attach those records to
  `map::graph::NodeRecord.symbols`.
- `src/scanner/cache.rs:285-297` reconstructs the same field from the
  serialized report cache.
- `src/cli/render/node.rs:291-321` renders `NodeRecord.symbols` for
  `cairn get <node> --symbols`.
- `src/map/query.rs:198-209` returns `NodeRecord.symbols` through the typed
  `symbols` query.
- `src/query_api/serialise.rs:7-32` emits that field for the opt-in `Symbols`
  flag on `get` responses.
- `src/query_api/handlers/locate.rs:5-51` scans `node.symbols` and returns
  exact name, file, line, end line, kind, and signature matches.

This same `NodeRecord.symbols` is not query-only today. It is also consumed by
`src/cli/render/bundle.rs:59-71` and
`src/query_api/handlers/bundle.rs:72-85` as dependency interfaces, by
`src/scanner/checks.rs:236-277` for contract interface drift, and by
`src/scanner/snapshot.rs:73-105` for the persistent `map.json` public-symbol
snapshot. The web UI reads the snapshot and graph symbol field as module
interface evidence. Widening this field in place would therefore publish
crate-private implementation details through those surfaces and would change
more than `get` and `locate`.

## Concrete separation seam

The smallest coherent design is an exported stream plus a query-visible stream
that are produced during the same tree walk:

1. Extend `LanguageSpec` with a query-visible policy, or with a callback that
   answers both policies without duplicating parsing. Keep `is_exportable` as
the interface policy. For Rust and TypeScript, query visibility should admit
   the existing item kinds regardless of public visibility. The walk must still
   exclude unsupported tree-sitter nodes and preserve the existing TypeScript
   `export_statement` handling.
2. Split the generic extraction result. `parse_file` and the collection helper
   should continue returning the exported signatures and exported records, and
   additionally return query-visible `SymbolRecord` values. An item admitted by
   the exported policy should occur in both streams. The interface signature
   must be computed once and the exported stream must remain the input to
   `InterfaceFingerprint`.
3. Keep `ReconcileReport.symbols`, `node_symbols`, and
   `node_symbol_records` as the exported compatibility fields. Add a clearly
   named per-node query record map, such as `node_query_symbol_records`, and
   carry it through sequential and parallel aggregation. Update the fixture
   report constructor and the report serialization shape.
4. Keep `TargetReport.symbols`, `hash`, and `symbol_records` as exported
   interface data. Add a query-visible record field and copy it in both the
   fresh and cached scanner paths. Bump the reconciler cache schema from the
   current version 5 in `src/scanner/cache.rs:13-18`; old caches must be
   discarded rather than silently serving a report without query records.
5. Store both views on the graph node, preferably with a field whose name makes
   the boundary explicit, such as `NodeRecord.query_symbols`. `symbols` stays
   exported so bundles, contract checks, snapshots, and existing map consumers
   retain their current meaning. The scanner attaches exported records to
   `symbols` and query records to the new field in separate operations.
6. Change only navigation consumers to read the query field: the CLI
   `symbols_block`, `map::query::symbols`, `query_api::serialise::node_json` for
   the `Symbols` flag, and `query_api::handlers::locate`. Keep both bundle
   renderers, contract drift checking, and snapshot construction on exported
   `symbols`. Add tests proving each boundary, not just a field assignment.

The graph field addition is intentionally explicit. A side map hidden in one
query handler would create a second source of truth and would fail for cached
or serialized graphs. A blanket rename of `NodeRecord.symbols` would instead
leak private symbols into dependency interfaces and persistent map output.

The implementation will also touch the many `NodeRecord` test fixtures and the
reconcile baseline fixtures. That mechanical spread, plus cache compatibility,
is why this is not a one or two file change.

## Rust evidence

The prior frozen evaluation record is explicit. `res.loop-efficiency-observations`
(2026-07-25 entry, lines 444-461) records that the pinned ripgrep revision
`4649aa9700619f94cf9c66876e9549d83420e16c` produced one symbol for
`crates.core.flags` although `defs.rs` alone declared 104 structs, and that
`cairn locate TypeList` returned an empty array. The same record contrasts the
flask fixture's 688 symbols across sixteen files and reports ripgrep recall of
0/9 for the primitive and topology-first compositions. The one apparent hit
came from `bundle.dependencies[]`, not from `get --symbols`, which supports the
interpretation that the graph substrate lacked definitions.

A fresh lane-binary smoke fixture made the predicate failure reproducible
without cloning the corpus. The fixture had two Rust files under one owned
module, with five eligible item nodes: `mod defs`, `fn main`, `struct
TypeList`, `fn new`, and `fn private_function`. Every item was crate-private.
With the freshly built lane binary:

- `cairn get fixture.core --symbols` printed `Symbols for fixture.core:`
  followed by `(none)`.
- `cairn get fixture.core --symbols --json` returned `"symbols":[]` while
  `cairn files fixture.core` still listed `src/defs.rs` and `src/lib.rs`.
- `cairn locate TypeList` printed `No public symbol definitions found for
  \`TypeList\`.`
- The target hash was `bd60acb658c79e45`. Renaming the private definitions and
  rerunning `cairn files fixture.core` produced the same hash, demonstrating
  the desired hash behavior even while query coverage was zero.

This fixture is a measurement of current behavior, not a proposed test
fixture. The eventual implementation should turn its observable query result
into a deterministic regression test.

## TypeScript disposition

`src/reconcile/typescript.rs:89-95` has the same two-job coupling. Its
`ts_is_exportable` accepts an `export_statement` or a declaration with a
`visibility_modifier` or `export` child. Unexported top-level interfaces,
functions, and variables are therefore absent from the current report even
though they are useful definition sites inside the owned module. An analogous
TypeScript fixture containing `PublicUser`, `InternalUser`, `helper`, and a
local variable returned only `PublicUser`; `cairn locate InternalUser` returned
no match.

TypeScript should follow the same separation, not receive a Rust-only special
case. The exported predicate remains the module interface policy. The query
predicate admits the existing `EXPORTABLE_KINDS` declarations and preserves the
special `export_statement` name and kind logic. The follow-up tests must cover
both exported and unexported declarations and verify that private TypeScript
changes do not alter the interface hash.

## Sizing and decomposition

This is L-sized under the overnight rule because it is a multi-subsystem change,
not because the predicate itself is difficult. The observable boundary spans:

- generic Rust and TypeScript extraction;
- `ReconcileReport` aggregation and serde;
- fresh scanner and cache report reconstruction;
- graph node storage and all node fixture constructors;
- CLI and query API navigation paths;
- preservation tests for bundles, contract drift, snapshots, and hashes; and
- the frozen ripgrep and TypeScript evaluation evidence.

The parent todo is therefore decomposed into these implementable sub-todos:

- `todo.node-symbol-coverage-ruling`: author and ratify the decision that
  defines exported interface symbols versus query-visible definition symbols,
  including bundle and snapshot boundaries.
- `todo.node-symbol-coverage-reconcile`: add the dual extraction streams to
  `LanguageSpec`, the generic reconciler, `ReconcileReport`, cache schema, and
  `TargetReport`, with tests that prove private items never enter hashes.
- `todo.node-symbol-coverage-query`: carry query-visible records into graph
  nodes and route `get --symbols` and `locate` to them while keeping bundles,
  contract checks, and snapshots on exported records.
- `todo.node-symbol-coverage-evaluation`: exercise Rust and TypeScript fixtures
  plus the frozen context-bundle harness, and report recall against the pinned
  ripgrep manifest.

No sizing claim here authorizes implementation without the ruling sub-todo.
