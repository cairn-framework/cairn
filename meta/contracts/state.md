---
node: cairn.state
---

# Contract: cairn.state

## Purpose

Read-only view over the beads issue backlog. `dec.change-format-only`
deletes the pluggable `StateBackend` record-storage abstraction (filesystem
and beads backends, and the `create_change_epic`/`create_task_beads`/
`list_child_tasks`/`claim_change` workflow methods that created, seeded, and
claimed beads on behalf of the change system): creating, claiming, and
sequencing work items is workflow, and cairn does not do workflow
(`dec.no-orchestrator`). What survives is `dec.beads-task-layer`'s read-only
per-node task view: beads (bd) stays the sole source of truth for tasks;
cairn only reads the passive `.beads/issues.jsonl` export.

## Public interface

- `backlog`: read-only loader exposing `BacklogItem`, `Dependency`, `read`,
  `ready`, `find`, and `for_node`.

## Invariants

- `backlog` is strictly read-only: beads stays the source of truth, malformed
  JSONL lines are skipped, and a missing export yields an empty vec.
- A bead omitting priority defaults to 99 (lowest urgency).
- `ready` and `for_node` order by priority ascending, then id.

## Dependencies

Inbound only at the blueprint level: `cairn.kernel.scanner` reads beads to
flag orphan node labels and `cairn.ui` reads node-linked beads for the
inspector. This node itself has no outgoing blueprint edges.

## Tests

Unit tests live in a `#[cfg(test)]` module in `src/state/backlog.rs`,
covering parsing, readiness ordering, lookup by id, and per-node label
filtering.
