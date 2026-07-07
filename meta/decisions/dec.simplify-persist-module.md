---
id: dec.simplify-persist-module
nodes:
  - cairn.root
  - cairn.persist
status: accepted
date: 2026-07-07
related: [dec.no-orchestrator]
---
# Add cairn.persist: one shared file-persistence helper module

## Context

State-file persistence was hand-rolled per module across eight call sites
(summariser draft store, scanner state/cache/snapshot, brownfield
interview, suggested-edges queue, changes apply, workspace TOML), each
with its own read/write/version-gate plumbing. The 2026-07-06 four-audit
investigation ratified in `todo.simplify-architecture` (wave 1,
`todo.simplify-persist-helper`) found no shared `read_json` /
`atomic_write` / version-peek helper, and one caller (brownfield
interview) documented an atomic write it did not perform.

## Decision

Add a new top-level Module node `cairn.persist` (`src/persist.rs`) exposing
`atomic_write`, `read_json`, `write_json` (pretty, atomic), `read_toml`,
and a version-peek helper. All eight enumerated call sites go through it;
local copies are deleted with no re-export shims.
`src/changes/apply/mod.rs::atomic_write` is the model implementation.

## Rationale

One implementation of temp-file-plus-rename and version gating means a
durability or corruption fix lands once, not eight times. The brownfield
interview's non-atomic write is fixed as a side effect of adoption. No
new dependencies; no storage-model change.

## Consequences

- Content-as-files invariant untouched: this is plumbing, not a backend.
  `StateBackend` remains the storage-pluggability abstraction for
  artefact state; `cairn.persist` sits below it as file I/O mechanics.
- Each store keeps its schema-version semantics exactly; scanner state's
  v1-to-v2 migration tests are the behaviour lock.
- New cross-module dependency: modules with persisted state call
  `cairn.persist`. The blueprint edge list stays curated to major data
  flow, so these utility-level calls are deliberately not enumerated as
  edges.
- The node is tagged `@no-contract`: it is I/O plumbing with no domain
  contract to state. This is a deliberate, recorded exemption from the
  wire-contracts-everywhere direction of `dec.wire-leaf-contracts`.
