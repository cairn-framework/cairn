---
node: cairn.kernel.changes
---

# Contract: cairn.kernel.changes

## Purpose

Discovers, parses, validates, applies, and archives active changes under
`meta/changes`. A change directory bundles a proposal, an optional design, a
`blueprint.delta` document, and mirrored artefact operations. This module turns
that on-disk proposal into a typed delta, checks it against current truth, and
(on archive) mutates the blueprint plus artefacts atomically before moving the
change into the archive.

## Public interface

- `discover(root)`: returns every active `Change` under `meta/changes`.
- `load_change(root, path)`: loads one change directory into a `Change`.
- `archive(root, blueprint_path, change_id)`: validates, mutates, rescans, and
  archives a change, returning `ArchiveReport` (archive path plus summary) or an
  error string.
- `create_rename_change(root, blueprint_path, old_id, new_id)`: writes a
  reviewable rename change without mutating current truth.
- `operation_summary(change)`: human-readable operation counts.
- `operations_for_nodes(changes, nodes)`: proposed operations touching a node or
  direct neighbour.
- `active_changes_lines(changes)`: renders active changes for generated `map.md`.
- `parse_blueprint_delta(file, source)` and `validate_change(change, graph)`:
  delta parsing and reference validation.
- Types: `Change`, `BlueprintDelta` (added/modified/removed/renamed nodes and
  edges), `Rename`, `EdgeRename`, `ArtefactOperation`, `ChangeOperation`
  (`Added`, `Modified`, `Removed`, `Renamed`), and `ArchiveReport`.

## Invariants

- `BlueprintDelta::is_empty` is true only when no node or edge operation exists.
- `validate_change` rejects duplicate adds, renames of missing nodes, and rename
  targets that already exist.
- `archive` snapshots every mutation path first and restores those snapshots on
  any validation, mutation, scan, or archive failure, so the tree is left intact.
- `create_rename_change` refuses when the old node is missing or the target id
  already exists.

## Dependencies

Outgoing blueprint edges: `cairn.kernel.changes -> cairn.kernel.scanner`
(validates deltas before applying via a rescan), `-> cairn.kernel.blueprint`
(parses blueprint deltas against blueprint node and edge types), and
`-> cairn.kernel.map` (reads graph state). Inbound: `cairn.kernel.query` reads
change state.

## Tests

Unit tests live in `src/changes/tests.rs` plus module-level `#[cfg(test)]`
suites in `apply/tests.rs`, `validate/tests.rs`, and `delta/tests.rs`, covering
discovery, delta parsing, validation rules, apply and snapshot restore, and
archive reporting.
