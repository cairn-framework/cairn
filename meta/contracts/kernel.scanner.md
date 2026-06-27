---
node: cairn.kernel.scanner
---

# Contract: cairn.kernel.scanner

## Purpose

The scanner is the orchestration hub of cairn. It loads config, parses the
blueprint, loads typed artefacts and contracts, reconciles every target,
builds the map graph, runs the scan-time integrity checks, and persists
machine state plus generated outputs. `load_project` is the pure read path;
`scan` adds the side-effecting writes.

## Public interface

- `load_project(root, blueprint_path)`: loads config, parses the blueprint,
  loads contracts and artefacts, builds targets, reconciles them, dedups
  findings, builds the graph, and runs the provenance, claims,
  gitignored-path, orphan-bead, and blueprint-change checks. Returns a
  `ScanResult` or an error string.
- `scan(root, blueprint_path)`: calls `load_project`, then writes the interface
  hash, blueprint snapshot, `map.md`, and the scan log concurrently via
  `thread::scope`, returning the first persistence error if any.
- `ScanResult`: graph, artefacts, contracts, interface_hash, target_reports,
  target_hashes, and blueprint_snapshot.
- `TargetReport`: per-target reconciliation result.
- `config`: `Config`, `TargetConfig`, `IntentionalAsymmetry`, `load`, and
  `is_ignored`.
- `state`: interface-hash and `BlueprintSnapshot` read/write plus v1-to-v2
  migration.
- `outputs`: `write_map` and `append_log`.

## Invariants

- `dedup_findings` collapses findings sharing identity `(code, node, path,
  target)`, preserving first occurrence and order; the message is display-only.
- `scan` never poisons its mutex (writes run inside `thread::scope`); the first
  write error is surfaced and the rest discarded.
- The reconciler cache (version 4, extensions rs/ts/tsx/py/go) is keyed on
  every reconciliation input; a stale or version-mismatched cache is ignored,
  and cache writes silently swallow errors.
- Blueprint-change gating skips dependency-edge drift until a schema-v2
  snapshot exists, so an upgrade does not flag every edge as new.

## Dependencies

Outgoing blueprint edges: `cairn.kernel.scanner -> cairn.kernel.blueprint`
(parses blueprint files), `-> cairn.kernel.artefacts` (loads artefact
metadata), `-> cairn.kernel.map` (builds the graph), `-> cairn.reconcile`
(invokes registered reconcilers), and `-> cairn.state` (reads beads to flag
orphan node labels). Inbound: hooks, changes, cli, query, and ui all drive or
read scanner runs.

## Tests

Unit tests live in `src/scanner/tests.rs` (wired as `#[cfg(test)] mod tests`)
covering scan orchestration and divergence detection, plus `#[cfg(test)]`
modules in `cache.rs`, `checks.rs`, `outputs.rs`, `state.rs`, and
`config/tests.rs` covering caching, the scan-time checks, output rendering,
snapshot migration, and config parsing.
