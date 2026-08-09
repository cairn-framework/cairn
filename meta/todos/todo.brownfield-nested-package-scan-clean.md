---
node: cairn.brownfield
status: done
created: 2026-07-27
related: [todo.brownfield-parent-package-cycle, todo.brownfield-parent-child-edge-model]
blocked_by: [todo.order-cycle-scc-enumeration, todo.blueprint-edge-provenance, todo.order-cycle-discovery-severity]
---

# Make a brownfield round-trip scan non-blocking on discovery-observed cycles

Implementation unit split out of `todo.brownfield-parent-package-cycle` under
the sizing rule.

Blocked on ratification, not on a unit, when filed.
`dec.brownfield-discovery-cycle-severity` was `status: proposed`, because its
clause 3 narrows a consequence of the then-accepted
`dec.order-containment-rule`. The unblock condition was that
decision reaching `status: accepted` AND its "What ratification must do"
section carried out, so the contradiction with `dec.order-containment-rule` is
resolved rather than left standing in the graph; a status flip alone was not
enough. Had the maintainer rejected clause 3, the decision's fallback applied
instead: document the exit-1 first scan in the brownfield quickstart and close
this todo without code.

Satisfied 2026-07-29 by acceptance of `dec.brownfield-discovery-cycle-severity`
(maintainer ratification, sheet of record PR #528, row W5), option A carried
out: `dec.order-containment-rule` marked `superseded` with the `supersedes:`
link added in the same commit. This todo was opened accordingly, and is `blocked`
again since the 2026-08-07 decomposition below.

Also update the `topological_order` doc comment in `src/map/integrity.rs`,
whose contradiction rule this changes, when the code lands.

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
  builder, not just brownfield. Prerequisite for the severity branch below;
  cycle enumeration and the containment fall-through need no provenance.
- **Cycle enumeration (clause 5).** Replace the return-on-first-cycle behaviour
  in `cycle_findings` with clause 5's deterministic per-component findings.
- **Containment fall-through (clause 7).** `topological_order` computes
  dependency SCC findings and evaluates the combined containment and dependency
  constraints over their quotient, even when dependency cycles are present.
  This keeps an independent child-to-ancestor contradiction visible.
- **Severity branch (clause 3, applied per component under clause 5).** Apply
  per-component severity inside graph cycle detection, where edge identity
  exists. Clause 4's provenance marker is its prerequisite, not part of it.

The delta-pipeline work in the parent todo's verified fact 2 (`flatten_nodes`,
`src/changes/delta.rs`) is out of scope: the chosen rule does not nest.

This unit is large. It was split under the sizing rule on 2026-08-07; see
Decomposition below for the seam actually taken.

## Decomposition (2026-08-07)

Too large for one small reviewable PR, confirmed against the code rather than
estimated. The four bullets above span two surfaces that barely overlap: edge
provenance is an additive blueprint grammar change threaded through the AST, the
map builder, the change-apply writer, and `blueprint_delta`; cycle enumeration
and the containment fall-through are a rewrite of `cycle_findings` and
`topological_order` inside `src/map/integrity.rs` alone. The six acceptance
bullets below split cleanly along that seam, and one of them (the hand-declared
masking bug) fails against today's code with no provenance work at all.

blocked on sub-todos: todo.order-cycle-scc-enumeration, todo.blueprint-edge-provenance, todo.order-cycle-discovery-severity

The first carries the enumeration half of clause 5 and clause 7 and needs no
provenance, so it lands the pre-existing masking fix on its own. The second
carries clause 4, the marker the
decision names as the implementation prerequisite. The third carries clause 3 and
the severity half of clause 5, depends on both, and is `blocked` accordingly.
This todo flips to `done` when the third lands.

An earlier version of this body suggested edge provenance as the natural first
half. The seam runs the other way round, because enumeration is independently
valuable and provenance is not.

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

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It keeps brownfield scanning reliable for real repositories.

2026-08-07 audit (todo.roadmap-assumption-audit): keep; owns the remaining parent-package-cycle scope.
