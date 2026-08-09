---
node: cairn.kernel.query
status: open
created: 2026-08-09
---

# Driver In Repo Selector Wire

## Scope
Establish the passive ready-set query contract used by the later driver. The
wire returns the commit and schema version plus every dispatchable unit at
that commit, with a stable unit id, node closure, selection ground, and
reproducible evidence lines. Its first-member rule must prove equality with
the one-unit selection a manual Orient step makes at the same commit. Every
additional member carries the same eligibility evidence and pairwise write-set
disjointness. The query performs no dispatch or orchestration.

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

The parent `## Grill rulings`, Q8, fixes the seam order and query milestone:

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

The same Q8 ruling says `cairn watch` remains finding-change only, the driver
re-reads authoritative queries before acting, and polling is sufficient until
measurably painful. The parent `## Acceptance` requires the dry-run wave to
print each unit's id, selection ground, and evidence lines.

## Dependencies
The blueprint-node sub-todo is first and supplies the query node and path
boundary. This sub-todo is the explicit prerequisite for the reaction-loop
sub-todo. The workflow-artefacts sub-todo is a separate passive artefact
surface; this query does not evaluate it. The lease surface follows this
selector seam under Q8 and is not part of this sub-todo.

## Sizing
M. The later implementation is one query and selection subsystem plus focused
wire, first-member-equality, evidence, and disjointness tests, kept under
roughly 600 changed lines. It must remain a passive query.

## Non-goals
Do not dispatch units, claim leases, evaluate workflows, widen `cairn watch`,
or make one Error finding disable unrelated read queries.
