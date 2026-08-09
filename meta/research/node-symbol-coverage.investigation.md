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
transient query extraction, scanner target context, CLI and query consumers,
and tests. The parent todo therefore decomposes into sub-todos and moves to
`blocked`. No Rust implementation is included in this research PR.

The safe direction is to keep the current exported set as the input to
interface hashes, dependency-interface bundles, contract interface checks, and
persistent map snapshots. Run a transient query-visible extraction over source
files for exact navigation queries. The query result must not become a second
persisted state, hash source, cache field, map field, or wire field, and must
not leak private definitions into the existing interface surfaces.

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

There is a second Rust-specific gate before that predicate. The
`LanguageSpec.fast_path` flag is true for Rust, and
`parse_file` at `src/reconcile/generic.rs:104-111` returns empty vectors when
the source contains no `pub ` byte sequence. An all-private binary therefore
does not reach `rust_is_exportable` at all. A query-visible widening must
bypass or replace this optimization for the query stream while retaining the
exported stream's fast-path behavior only if it remains semantically safe.

The measured failure is consequently the composition of two filters: the
pre-parse `pub ` shortcut drops files with no public marker, and the
visibility-modifier predicate drops private item nodes in files that do reach
the parser. The implementation must test both cases.

## Rust-specific pre-parse seam

The fast path is an independent seam from `rust_is_exportable`. It is an
optimization for the old public-only output, not an interface rule. If the
query-visible set includes crate-private definitions, source discovery must
parse Rust files that contain no `pub ` marker, or run a separate query walk
that does so. A predicate-only edit would leave the all-private binary at zero
coverage and would not satisfy the measured defect.

The exported hash can remain empty for a genuinely all-private file, while the
query-visible records carry its definitions. In a mixed file, exported records
continue to feed the hash and query records include both exported and private
items.

## Concrete separation seam

The current `NodeRecord.symbols` field cannot simply be widened. It is the
stored public-interface view used by bundles, contract checks, snapshots, and
the web UI. Adding a second persisted query field would violate this todo's
non-goal of no new stored state or second source of truth.

The non-stored seam is a transient query extraction over the source files at
query time:

1. Keep `ReconcileReport.symbols`, `node_symbols`, and
   `node_symbol_records` as the exported compatibility fields. Keep
   `TargetReport.symbols`, `hash`, and `symbol_records` and
   `NodeRecord.symbols` exported-only. Do not add query fields to these
   structs, do not serialize query records in the reconciler cache, and do not
   add query records to `map.json`.
2. Factor the generic tree walk so the exported reconciler and a
   query-visible extractor share `exportable_kinds`, `name_and_kind`, and
   `interface_symbol`. Add a language-specific query policy alongside
   `is_exportable`, with Rust and TypeScript admitting in-node declarations
   regardless of external visibility. Preserve TypeScript's
   `export_statement` wrapper semantics so an exported declaration is not
   emitted twice.
3. Add a crate-internal query extraction entry point in the reconcile or
   scanner seam. It receives the source root, language, and claimed relative
   files, reparses those files, and returns a transient `Vec<SymbolRecord>`.
   For Rust it must bypass the `fast_path` `pub ` shortcut, then apply the
   query policy. The source files remain the only source of truth. The
   existing exported report and hash path remains unchanged.
4. Route navigation consumers through that transient result. The CLI
   `symbols_block` in `src/cli/render/node.rs` needs root and target language
   context. The query API `get` path in `src/query_api/mod.rs:485-503` needs
   the same transient records before `node_json` emits its opt-in `Symbols`
   field. `src/query_api/handlers/locate.rs:5-51` needs root and target
   context to scan transient records. The typed `map::query::symbols` helper
   must either accept the transient records explicitly or be kept as the
   exported-only map query and have its navigation caller use the new helper;
   this choice belongs in the ruling.
5. Leave `src/cli/render/bundle.rs:59-71`,
   `src/query_api/handlers/bundle.rs:72-85`, `src/scanner/checks.rs:236-277`,
   `src/scanner/snapshot.rs:73-105`, and the web UI on
   `NodeRecord.symbols`. Their dependency interfaces, contract checks, and
   persistent snapshots therefore remain exported-only.
6. Add tests for the transient extractor, the all-private Rust fast-path case,
   the mixed Rust visibility case, TypeScript exported and unexported
   declarations, and each navigation boundary. Existing hash and bundle
   tests remain the preservation tests.

This design trades query-time parsing for the explicit no-new-state
constraint. Persisting a second query view would require an explicit decision
to supersede that constraint and is not recommended by this investigation.


## Existing consumer seam

The exported set's current consumers are:

- `ReconcileReport.node_symbol_records`, scanner target assembly, and cache
  reconstruction feed `NodeRecord.symbols`.
- `src/cli/render/node.rs:291-321` renders `NodeRecord.symbols` for
  `cairn get <node> --symbols`.
- `src/map/query.rs:198-209` returns `NodeRecord.symbols` through the typed
  `symbols` query.
- `src/query_api/serialise.rs:7-32` emits that field for the opt-in `Symbols`
  flag on `get` responses.
- `src/query_api/handlers/locate.rs:5-51` scans `node.symbols` and returns
  exact name, file, line, end line, kind, and signature matches.

This same `NodeRecord.symbols` is also consumed by
`src/cli/render/bundle.rs:59-71` and
`src/query_api/handlers/bundle.rs:72-85` as dependency interfaces, by
`src/scanner/checks.rs:236-277` for contract interface drift, and by
`src/scanner/snapshot.rs:73-105` for the persistent `map.json` public-symbol
snapshot. The web UI reads the snapshot and graph symbol field as module
interface evidence. The transient design changes only the navigation
consumers, not this stored interface view.

The implementation will touch generic extraction, query context plumbing,
and navigation tests, but it deliberately avoids a report/cache/schema
migration. The query-time parse and multi-surface plumbing still make it
multi-seam rather than S-sized.

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

A first fresh lane-binary smoke fixture made the all-private failure
reproducible without cloning the corpus. The fixture had two Rust files under
one owned module, with five eligible item nodes: `mod defs`, `fn main`,
`struct TypeList`, `fn new`, and `fn private_function`. Every item was
crate-private. With the freshly built lane binary:

- `cairn get fixture.core --symbols` printed `Symbols for fixture.core:`
  followed by `(none)`.
- `cairn get fixture.core --symbols --json` returned `"symbols":[]` while
  `cairn files fixture.core` still listed `src/defs.rs` and `src/lib.rs`.
- `cairn locate TypeList` printed `No public symbol definitions found for
  \`TypeList\`.`
- The target hash was `bd60acb658c79e45`. Renaming the private definitions and
  rerunning `cairn files fixture.core` produced the same hash, demonstrating
  the desired hash behavior even while query coverage was zero.

Because that all-private fixture contains no `pub ` marker, it also exercises
the Rust pre-parse shortcut at `src/reconcile/generic.rs:104-111`; it does not
reach `rust_is_exportable`. To isolate the visibility predicate, a second
version added one `pub fn exported()` and kept the private definitions:

- `cairn get fixture.core --symbols` returned only `exported` at
  `src/lib.rs:3`.
- `cairn locate RenamedTypeList` still printed
  `No public symbol definitions found for \`RenamedTypeList\`.`
- The mixed target hash was `671b61de98147df2`. Renaming the private
  definitions to `ChangedTypeList` and `changed_private_function` left that
  hash unchanged.

Together the two runs show both necessary implementation seams: parsing must
not be skipped for query-only all-private files, and parsed private item nodes
must bypass the exportability predicate only for the query stream.

These fixtures are measurements of current behavior, not proposed test
fixtures. The eventual implementation should turn their observable query
results into deterministic regression tests.

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

This is L-sized under the overnight rule because it is a multi-subsystem
change, not because the predicate itself is difficult. The observable boundary
spans:

- generic Rust and TypeScript extraction, including the Rust pre-parse gate;
- a transient query extractor and scanner context plumbing that works with
  both fresh and cached exported reports;
- CLI and query API navigation paths that need root, language, and claimed-file
  context;
- typed query tests and preservation tests for bundles, contract drift,
  snapshots, hashes, and the unchanged cache and map wire shapes; and
- the frozen ripgrep and TypeScript evaluation evidence.

The parent todo is therefore decomposed into these implementable sub-todos:

- `todo.node-symbol-coverage-ruling`: author and ratify the decision that
  defines exported interface symbols versus transient query-visible definition
  extraction, including bundle and snapshot boundaries and the no-new-state
  constraint.
- `todo.node-symbol-coverage-reconcile`: factor shared extraction and add the
  transient query extractor without adding query records to reports, caches,
  graph nodes, or wire artifacts.
- `todo.node-symbol-coverage-query`: pass source-root and target context to
  `get --symbols` and `locate`, and keep all stored interface consumers on the
  exported records.
- `todo.node-symbol-coverage-evaluation`: exercise Rust and TypeScript
  fixtures plus the frozen context-bundle harness, and report recall against
  the pinned ripgrep manifest.

No sizing claim here authorizes implementation without the ruling sub-todo.
