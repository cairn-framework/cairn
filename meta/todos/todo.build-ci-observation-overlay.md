---
node: cairn.reconcile
status: blocked
created: 2026-07-31
related: [src.reddit-ontology-pointers, dec.domain-expandability]
blocked_by:
  - todo.todo-relationship-schema-implementation
  - todo.parallel-dispatch-granularity
  - todo.driver-v2-read-surface-audit
---

# Build/CI observation overlay: one bounded dogfood experiment

Unratified candidate from the slate's post-ratification intake
(res.inversion-convergence-minutes). Next horizon, not next unit: nothing
selects this while the relationship-schema and driver-v2 chains have
open work.

The external ontology critique (src.reddit-ontology-pointers) poses a
fair test cairn currently fails: the repo cannot map itself through its
own build, CI, or runtime. dec.domain-expandability keeps non-code
domains analysis-only until a reconciler exists. This todo is the
smallest honest probe of that boundary.

## Task

One bounded overlay, dogfooded on cairn itself: model Cargo workspace
members, build targets, and GitHub workflow jobs as derived, disposable
observations over the existing graph.

Constraints, from the intake verbatim: repo truth stays canonical; the
overlay is derived and rebuildable, never authored; no Joern, RDF, or
BFO dependencies enter the kernel. Design should follow the evidence
rules being set for the multi-ref index
(todo.parallel-dispatch-granularity): every derived fact carries source,
extractor plus version, observed_at, freshness, completeness.

## Acceptance

- A research artefact plus prototype (or a written kill decision)
  showing cairn's own workspace members, build targets, and CI jobs as
  overlay observations linked to existing nodes.
- The kernel gains no new required dependency; deleting the overlay
  leaves the graph exactly as it was.
- An explicit recommendation: promote to a reconciler domain, keep as
  an overlay, or kill; recorded as a decision either way.

## Mission disposition

2026-08-02: blocked against dec.cairn-mission. Serves extendable. Its dependency-gated horizon has declared blockers, so it remains parked without false readiness.

2026-08-07 audit (todo.roadmap-assumption-audit): status set open this session: both declared blockers (todo.todo-relationship-schema-implementation, todo.parallel-dispatch-granularity) are done, and lint confirmed the contradiction. Scope itself still reads current.

2026-08-10 loop iteration: status set `blocked` on
`todo.driver-v2-read-surface-audit`, which stems the unchecked tasks 1 and 2 of
`meta/changes/driver-v2-selection`. The horizon clause above stays as written;
that edge is its machine-visible counterpart, because a clause naming chains in
prose cannot gate selection and every session reaching this unit re-argues it.
The two edges the 2026-08-07 audit found satisfied are retained as resolved
blockers rather than deleted. Nothing was implemented against the gate.

`todo.driver-in-repo` is deliberately NOT an edge here, which is the question
this note exists to settle. That change retired its tasks 3 and 4 into it on
2026-08-04 (`dec.orchestration-placement`), so it is arguably live driver work.
It is excluded on two grounds: `res.inversion-convergence-minutes` R5 parked
driver-in-repo outside the slate that authored the clause, so the clause never
named it, and it is itself `blocked` on four children, so the edge would park a
bounded experiment behind a large programme with no clearing date. If a later
session wants that dependency, it is a decision, not a re-reading of this
clause.
