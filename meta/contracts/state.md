---
node: cairn.state
---

# Contract: cairn.state

## Purpose

Pluggable state persistence backend. The `StateBackend` enum abstracts artefact
state storage (status, claim, ready-queries) away from the filesystem default,
allowing a beads key-value backend as an alternative. Content (markdown bodies,
blueprint text) stays as files unconditionally; only structured record state is
routed through this layer. It also hosts the read-only beads issue backlog
loader.

## Public interface

- `StateBackend`: enum over `Filesystem(FilesystemStateBackend)` and
  `Beads(BeadsStateBackend)`, dispatching `load`, `save`, `list`, `remove`,
  `query_by_type`, `query_by_label`, and `query_by_dependency`.
- `StateError`: error enum with `Io(io::Error)` and `Serialization(String)`;
  implements `Display`, `Error` (with `source`), and `From` for `io::Error` and
  `serde_json::Error`.
- `StateRecord`: trait providing `record_type`, `labels`, and `dependencies` for
  queryable records.
- `FilesystemStateBackend`: JSON files under a root, keyed by filename.
- `BeadsStateBackend`: JSON strings in the beads kv store via the `bd` CLI,
  keyed with a `cairn:state:` prefix.
- `storage_backend(name, root)`: factory, `filesystem` (default) or `beads`.
- `backlog`: read-only loader exposing `BacklogItem`, `Dependency`, `read`,
  `ready`, `find`, and `for_node`.

## Invariants

- `load` returns `Ok(None)` for a missing record; `remove` is idempotent.
- `save` overwrites existing records.
- The beads backend prefixes every key with `cairn:state:`.
- `backlog` is strictly read-only: beads stays the source of truth, malformed
  JSONL lines are skipped, and a missing export yields an empty vec.
- A bead omitting priority defaults to 99 (lowest urgency).
- `ready` and `for_node` order by priority ascending, then id.

## Dependencies

Inbound only at the blueprint level: `cairn.kernel.scanner` reads beads to flag
orphan node labels and `cairn.ui` reads node-linked beads for the inspector.
This node itself has no outgoing blueprint edges; the beads backend shells out
to the external `bd` binary for kv operations.

## Tests

Unit tests live in `src/state/tests.rs` (`#[cfg(test)] mod tests` wired in
`mod.rs`) covering backend round-trips and queries, plus a `#[cfg(test)]` module
in `src/state/backlog.rs` covering parsing, readiness ordering, lookup by id,
and per-node label filtering.
