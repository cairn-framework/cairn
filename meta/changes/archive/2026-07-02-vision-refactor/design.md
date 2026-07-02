# Design: Vision refactor

Implements the seven ratified decisions:
`dec.symbol-reality-layer`, `dec.persistent-map-snapshot`,
(structured-contract interface verification, no standalone decision beyond
those two — it is the mechanism `dec.generative-bundles-and-gaps` composes),
`dec.generative-bundles-and-gaps`, `dec.frontier-query`,
`dec.workspace-aggregation`, `dec.change-format-only`.

Repo conventions that bind every step: test-first (`docs/conventions.md`); no
em-dashes in user-facing copy; British spelling; user-facing strings via
`copy.toml`; 500-line file gate; `missing_docs = deny` and
`unsafe_code = forbid`; edition 2024; no new heavyweight deps.

## Phase order

Phases 1→2→3→4 are sequential (each builds on the last: symbols → snapshot →
contract verification → generation bundles). Phase 5 needs phase 1 (graph
state). Phase 6 needs phase 5 (`frontier` per workspace member). Phase 7 is
independent of 1–6 and can land any time after phase 0. Phase 8 is cleanup and
lands last.

## Phase 1 — Symbol records

New `src/reconcile/symbol.rs`: `SymbolRecord { name, kind, signature, file,
line, end_line }`, `SymbolKind` enum. All four reconcilers build a record at
the exact point they already push the flattened interface-hash string, without
changing that string's computation (fingerprint stability is load-bearing).
Records thread through `ReconcileReport`/`TargetReport` (cache version bump
4→5), attach to `NodeRecord.symbols` via `build_graph`, and surface through a
new query_api tool `cairn symbols <node>` (CLI + MCP + webui module inspector
section).

## Phase 2 — `map.json`

New `src/scanner/snapshot.rs`: `MapSnapshot { schema_version, interface_hash,
nodes, edges, findings }`, built deterministically (`BTreeMap` order, no
timestamps) from `ScanResult`. Written by `scan()` alongside `map.md`;
`load_project()` stays pure. Committed, not gitignored.

## Phase 3 — Structured contracts

`Contract` gains `interface: Vec<String>` (optional frontmatter list, empty
default). Shared `normalize_symbol` (promoted from `src/reconcile/code.rs` to
`src/reconcile/symbol.rs`) makes a declared entry match iff
`normalize_symbol(entry) == record.signature` for some symbol of the
contract's node. Mismatches emit `CAIRN_CONTRACT_INTERFACE_DRIFT` (Warning).
One contract (the map module's) is dogfooded with a real `interface:` block.

## Phase 4 — Generative direction

`cairn bundle <node>` (query_api tool, read-only): composes node + contract +
decisions + rationale + dependency interfaces (from neighbours'
`NodeRecord.symbols`) + gates, reusing existing composers
(`query::get`, `contract_json`, artefact handlers, `BriefData` helpers).
`cairn gap <node> --question "<text>"` (CLI-only, mutating): writes a
`gap: true`, `status: proposed` decision artefact. `CAIRN_GAP_UNRESOLVED`
(Warning) lints every open gap.

## Phase 5 — `cairn frontier`

Query over the graph: `ready` = `Ghost` nodes whose outbound targets are all
`Synced` (tiered via `order()`'s logic); `blocked` = remaining `Ghost` nodes
with their non-synced targets named. Ships on `Ghost` only (no `Planned`
variant exists yet).

## Phase 6 — Workspace

`cairn.workspace` (TOML, `[[project]] name/root`). New `src/workspace/mod.rs`:
`Workspace`/`WorkspaceProject`/`load()`. `cairn workspace <status|lint|frontier>`
loops members through the existing `scanner::load_project`, aggregating
results; a missing member contributes `CAIRN_WORKSPACE_MEMBER_MISSING`
(Error) and the loop continues.

## Phase 7 — Change-system trim

Delete `create_change_epic`/`create_task_beads`/`list_child_tasks`/
`claim_change` on `BeadsStateBackend` and their call sites in
`src/cli/commands/change.rs`. Reduce `src/state/mod.rs` to a thin shell
(`pub(crate) mod backlog;` only); delete `StateBackend`/`StateRecord`/
`FilesystemStateBackend`/`storage_backend()`/`src/state/beads.rs`. The
read-only backlog view (`crate::state::backlog`) is untouched, byte-identical.
Remove the `state_backend` config key.

## Phase 8 — Cleanup

Update `AGENTS.md`/`README.md`/`docs/commands.md`; commit the regenerated
root `map.json`; run the file-size gate; run the final battery; archive this
change through cairn's own apply/archive path.
