---
node: cairn.kernel.query
status: open
created: 2026-08-09
parent: todo.driver-in-repo
blocked_by: [todo.driver-in-repo-blueprint-node]
---

# Driver In Repo Selector Wire

## Scope
Establish the passive raw ready-set query contract used by the later driver.
The wire returns the commit and schema version plus every dispatchable unit at
that commit, with a stable unit id, node closure, selection ground, and
reproducible evidence lines. Wave composition is a separate consumer step: it
applies first-member equality with the one-unit selection a manual Orient step
makes at the same commit, then filters additional wave members by pairwise
write-set disjointness. Those pairwise rules apply to the composed wave, not to
the raw ready set, whose eligible units may overlap. The query performs no
dispatch or orchestration.

## Parent constraints
The parent todo is `todo.driver-in-repo`, under `## Task`, item 4:

> The selector wire the loop needs: a ready-set query, per the Q1 and
> Q8 rulings below. `cairn next` today exposes no stable unit id and
> no reproducible selection evidence, groups findings into remediation
> actions, and orders todos by creation date and path rather than the
> loop's own precedence. Establish the ready-set contract (commit and
> schema version; per unit: unit id, node closure, selection ground,
> reproducible evidence) and land it as a passive query, or file the
> exact missing field against the owning node. This is the
> prerequisite for the acceptance contract: the wave's first member
> equals a manual Orient selection at the same commit, and every
> additional member carries the same eligibility evidence plus
> pairwise write-set disjointness.

The parent task records the finding-first constraint:

> The wire's design must also settle how finding-first states become
> dispatchable, under the four constraints recorded in
> `todo.parallel-dispatch-granularity`'s Q1 ruling (the ephemeral
> -findings resolution, the Info-only parked fold, the passive query,
> and first-member equality); until it does, the driver dispatches
> todo-sourced units only.

The parent `## Grill rulings`, Q8, is provisional grill direction pending an
accepted owning decision:

> **Q8, core seam order: selector wire, then lease surface, then
> findings blast radius; `cairn watch` widening deferred indefinitely**
> (task 4). The selector wire is upgraded by the Q1 ruling from a
> single recommendation to a ready-set query: every dispatchable unit
> at this commit with unit id, node closure, selection ground, and
> reproducible evidence lines; its first milestone is the dry-run
> driver: the printed wave's first member provably equals the one-unit
> selection a manual Orient step makes at the same commit, and every
> additional member satisfies the same eligibility evidence plus
> pairwise write-set disjointness.

The same Q8 passage provides provisional direction that `cairn watch` remains
finding-change only, the driver re-reads authoritative queries before acting,
and polling is sufficient until measurably painful. The parent `## Acceptance`
requires the dry-run wave to print each unit's id, selection ground, and
evidence lines.

`todo.parallel-dispatch-granularity` records that its Q1 and Q2 answers are
provisional grill direction, while the pre-existing ratified slate constraint
keeps its own authority: derive write-sets from node closure over committed
state and promote to declared write-sets only on measured false-overlap
evidence. The Q1 set-valued filtering and first-member equality remain
provisional pending an owning decision.

Accepted `dec.rung-three-coordination-substrate`, clause 1, is authoritative
for recompute-equality plan identity and commit provenance when a wave is
composed. Its accepted contract does not make the Q8 ready-set milestone or
wave filter a raw query behavior.

## Dependencies
The blueprint-node dependency is a typed `blocked_by` edge on this child and
supplies the query node and path boundary. This child is the explicit
prerequisite for the reaction-loop child, represented by that child's typed
`blocked_by` edge. The workflow-artefacts sub-todo is a separate passive
artefact surface; this query does not evaluate it. The lease surface follows
this selector seam under Q8 and is not part of this sub-todo. The parent
carries typed `blocked_by` edges to all four children.

## Sizing
M. The later implementation is one query and selection subsystem plus focused
raw-wire, first-member-equality, wave-filter, evidence, and disjointness tests,
kept under roughly 600 changed lines. It must remain a passive query.

## Non-goals
Do not dispatch units, claim leases, evaluate workflows, widen `cairn watch`,
or make one Error finding disable unrelated read queries.
