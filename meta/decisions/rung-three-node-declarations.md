---
id: dec.rung-three-node-declarations
nodes:
  - cairn.coord
  - cairn.registries
  - cairn.blueprint-source
  - cairn.design-copy
status: accepted
ratification: binding
date: 2026-08-07
informed_by:
  - res.parallel-dispatch-rung-3
related:
  - dec.rung-three-coordination-substrate
---
# Rung three node declarations

Declares the four blueprint nodes the accepted substrate ruling requires:
`cairn.coord`, the coordination-store module sanctioned by
`dec.rung-three-coordination-substrate` clause 2, and the three hotspot owner
nodes (`cairn.registries`, `cairn.blueprint-source`, `cairn.design-copy`)
sanctioned by clause 3. This record is pure execution of an accepted binding
ruling: it introduces no new semantics. It gives the already-ruled store a
module to live in and the already-ruled hotspot files an owner each, so the
architecture gate can attribute the blueprint change to an accepted decision.

- **Tier**: Binding. It touches `cairn.blueprint`. Accepted on the
  maintainer's in-session word, 2026-08-07; the veto stands open per
  `dec.reviewer-panel-ratification`.
- **Unblocks**: the coord todo chain (`todo.coord-common-dir-helper`,
  `todo.coord-fact-store`, `todo.coord-read-surface`) and phase-0 hotspot
  subtraction in derived write-sets.
- **Alignment**: against `dec.cairn-mission`, this touches the maintainable
  and investigable properties. Maintainable: the hotspot files gain a
  declared owner, so concurrent units are steered away from them instead of
  colliding in merges. Investigable: ownership of the registries, the root
  blueprint, and the copy table becomes queryable graph fact instead of
  folklore. Extendable: the coordination store lands as its own module with
  a declared seam rather than a bolt-on inside persist. Fit for purpose: the
  declarations execute clauses 2 and 3 exactly, adding nothing the ruling
  did not sanction.
- **Options**: a new `cairn.coord` module, a submodule under
  `cairn.persist`, or deferring declaration to local-tier receipts. Chosen:
  the new module, on the maintainer's word, because the store is a seam the
  driver and the console both consume, not a persistence detail.
