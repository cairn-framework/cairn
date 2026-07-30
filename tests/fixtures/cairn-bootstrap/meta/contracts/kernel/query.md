---
node: cairn.kernel.query
informed_by:
  - type: decision
    id: dec.blueprint-as-current-state
  - type: decision
    id: dec.stable-ids
---

# Contract: cairn.kernel.query

The Query module answers typed questions over the map: nodes, neighbourhoods, dependencies, and the artefacts anchored to them.

## Interface

- **Input.** A query naming a stable node id and the traversal it wants (get, neighbourhood, deps, artefact listings).
- **Output.** A structured result over the current graph, serialisable for both human and JSON presentation.
- **Errors.** An id that resolves to no node returns a not-found result naming the id; queries never guess a nearest match silently.

## Invariants

- Queries are read-only: no query mutates graph or artefact state.
- Results reflect the blueprint as current-state truth, decisions as rationale.
- Traversals address nodes by stable id, never by display name.

## Out of scope

- Rendering. The CLI and other surfaces own presentation.
- Scanning. Queries read the map the scanner built; they never trigger reconciliation.
