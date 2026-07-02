# Spec: Phase 5 — Orchestrator surface

## Acceptance criteria

- `cairn frontier --json` on this repo (all nodes synced) returns
  `ready: []`, `blocked: []`.
- A fixture graph with a ghost node whose dependencies are all synced places
  that node in `ready`, tiered correctly; a ghost node with an unsynced
  dependency appears in `blocked` naming that dependency.
- A cyclic graph makes `frontier` fail with the same structural-cycle error
  shape as `order`.
