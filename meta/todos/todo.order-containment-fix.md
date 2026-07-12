---
node: cairn.kernel.query
status: open
created: 2026-07-12
---

# Order Containment Fix

gh:#237

`cairn order` is inconsistent about container-vs-child ordering.

## Evidence (verified on main, 2026-07-12)
- In this repo, `cairn order --json` lists `cairn.kernel.blueprint`,
  `cairn.kernel.artefacts`, `cairn.kernel.map` before their parent
  `cairn.kernel`; in a scratch project with no dependency edges the parent
  sorts first (degenerate key order).
- `src/map/integrity.rs:79-91` topologically sorts over dependency edges only;
  containment edges are not part of the ordering.

## Scope note
The frontier half of gh:#237 is resolved by design: dec.frontier-query defines
frontier as Ghost-only, so synced leaf modules are intentionally excluded.

## Task
Define and enforce a deterministic containment rule in `cairn order` (children
before parent, or parent first, consistently) by including containment edges in
the sort. Add a test.
