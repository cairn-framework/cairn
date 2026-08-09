---
node: cairn.kernel.map
status: open
created: 2026-08-09
---

# Edge Provenance Wire Projection

Carry edge provenance through every externally visible wire with an independent
version bump per accepted schema-version decision:

- Add provenance to `SnapshotEdge` and the `map.json` schema.
- Bump the `/api/graph` query and UI schema versions together.
- Add provenance to context JSON.
- Assign an independent export version for provenance.