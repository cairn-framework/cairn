---
id: dec.close-blueprint-drift
nodes:
  - cairn.sse
  - cairn.state
  - cairn.watch
status: accepted
date: 2026-06-03
---

# Close blueprint drift for orphaned modules

## Context

Three source modules existed in the codebase but were not declared in `cairn.blueprint`:

- `src/sse.rs` (SSE event stream parser for Gas City integration spikes)
- `src/state/mod.rs` (pluggable state persistence backend: filesystem + beads)
- `src/watch.rs` (watch mode: periodic scan with finding-change events)

This drift produced `CAIRN_RECONCILE_ORPHANED_FILE` info findings on every scan and prevented `cairn lint` from exiting cleanly.

## Decision

Declare all three as top-level modules under the `cairn` System node.

## Rationale

All three modules are actively imported and used:

- `sse` is exported from `lib.rs` and consumed by the Gas City adapter.
- `state` is used by CLI commands for beads-backed persistence and by the scanner for snapshot state.
- `watch` is used by the CLI `cairn watch` command and by the query API for finding-delta events.

Leaving them orphaned weakens the dogfooding signal. The framework should model its own source tree completely.
