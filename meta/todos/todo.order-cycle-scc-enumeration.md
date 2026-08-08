---
node: cairn.kernel.map
status: done
created: 2026-08-07
parent: todo.brownfield-nested-package-scan-clean
related: [dec.brownfield-discovery-cycle-severity, dec.order-containment-rule]
---

# Report every cyclic component, and stop dependency cycles masking containment

Implementation unit split out of `todo.brownfield-nested-package-scan-clean`
under the sizing rule. It carries the enumeration half of clause 5 and all of
clause 7 of `dec.brownfield-discovery-cycle-severity`, which are the two
properties that decision names as prerequisites for any severity branch. It
needs no edge provenance and changes no severity: every finding this unit emits
stays an Error, exactly as today. Clause 5's advisory-versus-Error rule is
`todo.order-cycle-discovery-severity`, not this unit.

Half of it is a pre-existing defect, fixable and worth fixing on its own:
today a hand-declared dependency cycle hides an independent hand-declared
child-to-ancestor contradiction, so `dec.order-containment-rule`'s promise that
declared contradictions block does not hold across the two detection paths.

## Verified facts

1. `cycle_findings` uses deterministic SCC enumeration. It reports one
   `CAIRN_ORDER_CYCLE` per cyclic dependency component, ordered by the
   component's smallest member id, and no longer stops at the first back edge.
2. `topological_order` computes dependency findings and then evaluates the
   combined containment-and-dependency constraints over a dependency-SCC
   quotient, so dependency cycles cannot mask containment contradictions.
3. The blast radius is contained. Outside `src/map/integrity.rs`, the only
   callers are `src/map/query.rs:288`, `:301`, and `:377`; the identically
   named `cycle_findings` in `src/artefacts/registry/validate/relations.rs` is
   an unrelated artefact-relation check over a different graph and must not be
   touched.

## Task

- Replace the return-on-first-cycle behaviour in `cycle_findings` with
  deterministic per-component enumeration, per clause 5: one finding per cyclic
  strongly connected component of the dependency graph, never one per simple
  cycle and never a cycle basis. A "cyclic SCC" is a component of more than one
  node, or a single node carrying a dependency edge to itself; a
  self-dependency must not vanish because its component has size one. Order
  components by their lexicographically smallest member id, and pick the
  representative path within a component deterministically.
- Each finding keeps its current shape: code `CAIRN_ORDER_CYCLE`, severity
  `Error`, a message carrying a cycle path. Clause 6 is explicit that what
  changes here is how many findings are emitted, not what they contain or which
  graph they read.
- Run the containment pass in `topological_order` whenever any dependency
  component is reported, rather than returning at the first one, so an
  independent containment contradiction is still evaluated and reported. The
  SCC quotient preserves reachability through dependency components. Both sets
  of findings reach the caller.
- Update the `topological_order` doc comment, which states the contradiction
  rule in the shape this changes.

## Acceptance

- A test pins a map with two disjoint hand-declared dependency cycles: two
  findings are emitted, one per component, and the scan exits non-zero.
  Reporting one fails this test.
- A test pins a hand-declared dependency cycle plus an independent
  hand-declared child-to-ancestor contradiction: both are reported and the scan
  exits non-zero. This is acceptance bullet 4 of the parent todo and it fails
  against today's code.
- A test pins one component containing several simple cycles over the same
  nodes: exactly one finding is emitted for that component, and the output is
  unchanged when the blueprint's declaration order is permuted. This is the
  enumeration half of acceptance bullet 5 of the parent todo; its
  mixed-provenance severity half belongs to
  `todo.order-cycle-discovery-severity`.
- A test pins a single node with a dependency edge to itself: one finding is
  emitted.
- `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cairn scan --strict` all pass on the dogfood blueprint, which is acyclic and
  must stay finding-free.

## Non-goals

Edge provenance, any severity change, and anything under `src/brownfield/`.
Those are `todo.blueprint-edge-provenance` and
`todo.order-cycle-discovery-severity`.
