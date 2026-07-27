---
node: cairn.brownfield
status: blocked
created: 2026-07-27
related: [todo.brownfield-parent-package-cycle, todo.brownfield-parent-child-edge-model]
---

# Make a brownfield round-trip scan non-blocking on discovery-observed cycles

Implementation unit split out of `todo.brownfield-parent-package-cycle` under
the sizing rule.

Blocked on ratification, not on a unit. `dec.brownfield-discovery-cycle-severity`
is `status: proposed`, because its clause 3 narrows a consequence of the accepted
`dec.order-containment-rule`. Unblock with
`cairn todo set brownfield-nested-package-scan-clean open` only once that
decision is `status: accepted` AND its "What ratification must do" section has
been carried out, so the contradiction with `dec.order-containment-rule` is
resolved rather than left standing in the graph. A status flip alone is not
enough. If the maintainer
rejects clause 3, take the decision's fallback instead: document the exit-1 first
scan in the brownfield quickstart and close this todo without code.

Also update `src/map/integrity.rs:74-76`, whose doc comment states the
contradiction rule this changes, when the code lands.

In short, the rule: discovery keeps both observed edges and does not nest, and
`CAIRN_ORDER_CYCLE` becomes advisory when every edge inside the cyclic component
is discovery-observed, staying an Error whenever the component holds a
hand-declared edge. Severity is per component, not per printed path.

## Scope

This is not a change to what discovery emits. `derive_import_edges`
(`src/brownfield/import_edges.rs`), `discover` (`src/brownfield/discovery.rs`),
and `blueprint_delta` (`src/brownfield/mod.rs`) keep their current output except
where provenance has to be recorded.

The decision is authoritative for the semantics; this list names where each
clause lands in the code. Where the two ever disagree, the decision wins and this
body is stale.

- **Edge provenance (clause 4).** Give edges written by `blueprint_delta` a
  marker that survives into the blueprint and the graph, so they are
  distinguishable from hand-written ones. Reaches blueprint syntax and the map
  builder, not just brownfield. Prerequisite for everything below.
- **Cycle enumeration (clause 5).** Replace the return-on-first-cycle behaviour
  in `cycle_findings` with clause 5's deterministic per-component findings.
- **Containment fall-through (clause 7).** `topological_order` returns as soon as
  dependency-only `cycle_findings` is non-empty (`src/map/integrity.rs:83-85`),
  before the combined containment and dependency deadlock branch runs. Run the
  containment pass whenever any dependency component is reported, advisory or
  Error alike. Restricting it to advisory components leaves today's masking bug
  in place.
- **Severity branch (clauses 3 and 4).** Apply per-component severity inside
  graph cycle detection, where edge identity exists.

The delta-pipeline work in the parent todo's verified fact 2 (`flatten_nodes`,
`src/changes/delta.rs`) is out of scope: the chosen rule does not nest.

This unit is large. Split it if it grows past one reviewable PR; edge provenance
is the natural first half.

## Acceptance

- After `cairn init --from-code` and `cairn change apply brownfield-init` on a
  project with a package root and a subpackage that import each other,
  `cairn scan` reports the `CAIRN_ORDER_CYCLE` naming that cycle as a
  non-blocking advisory and exits zero. The finding must be present: silently
  dropping discovery-only cycles satisfies "exits zero" while violating clause 3,
  so assert on the advisory, not just on the exit code.
- A test pins a mixed map holding one discovery-only dependency cycle and one
  hand-declared dependency cycle. The scan reports both, the discovery-only one
  as an advisory and the hand-declared one as an Error, and exits non-zero.
  Reporting only the Error fails this test.
- A test pins the containment case: one discovery-only dependency cycle plus a
  hand-declared child-to-ancestor contradiction in the same map. The scan reports
  the discovery-only cycle as an advisory and the contradiction as an Error, and
  exits non-zero. Without this the severity branch can mask the contradiction,
  because the current traversal never reaches the containment pass once a
  dependency cycle exists.
- A test pins the same masking bug on the blocking side: a hand-declared
  dependency cycle plus an independent hand-declared child-to-ancestor
  contradiction. The scan reports both and exits non-zero. This one fails against
  today's code too, since the containment pass is unreachable once any dependency
  cycle exists; it is the pre-existing half of the defect.
- A test pins the reporting unit itself: one component containing several simple
  cycles over the same nodes, with mixed provenance, where at least one
  hand-declared edge is absent from the path the finding prints. Exactly one
  finding is emitted for that component and it is an Error. This is the case that
  separates clause 5 from the alternatives: an implementation emitting every
  simple cycle, or a cycle basis, or deciding severity from the printed path,
  passes every other bullet here and fails this one. Assert the output is
  unchanged when the blueprint's declaration order is permuted.
- A test pins that both observed edge directions survive discovery unmodified, so
  a future edge-suppression shortcut fails loudly.
