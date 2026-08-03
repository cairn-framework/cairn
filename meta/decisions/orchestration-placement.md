---
id: dec.orchestration-placement
nodes:
  - cairn.root
  - cairn.ui
status: proposed
ratification: binding
date: 2026-08-03
informed_by: [res.inversion-convergence-minutes]
refines: [dec.product-perimeter]
related: [dec.control-plane-programme, dec.north-star-continuous-loop, dec.no-orchestrator]
revisit_triggers:
  - "the driver's first in-repo implementation needs a capability the core graph must actively provide (would move orchestration logic into the substrate, which this record forbids)"
  - "declarative workflow definitions prove unexpressible as cairn artefacts and demand an imperative engine (reopens the layer boundary)"
---

# Orchestration placement: the substrate stays passive, the layer above it moves in

## Decision

This record is about where orchestration logic sits, not whether cairn
orchestrates. The core graph remains a passive substrate: it never
schedules, dispatches, or supervises, exactly as the live orchestration
rule in `dec.product-perimeter` (which superseded `dec.no-orchestrator`
and carries its boundary forward) requires today. What changes is the home of the layer above it: the driver
may live in this repository as a distinct layer on top of the core, the
webui fronts that driver as its steering surface, and declarative
cairn-based workflows are in scope for that layer. Recorded as intent
for the next session; nothing is built under this record until it is
signed.

## What this refines, and what it does not touch

The lineage ruled that the core is a graph other orchestrators traverse
and that moving the driver in-repo requires a new binding decision
(`dec.no-orchestrator`, now historical: `dec.product-perimeter`
superseded it and carries the rule). This is that decision, for the
placement clause of the live authority only: the
core-is-not-a-loop-engine ruling stands untouched; the
driver-outside-the-repo clause is refined so the repository may host the
driver as a separate layer above the core. Build, delivery, and runtime
facts stay inside cairn's investigation boundary; scheduling, execution,
and supervision stay outside the CORE's actuation boundary while living
inside this monorepo as their own layer.

A reader matching this against "no orchestration" should read the
boundary as layers, not repositories: the substrate (graph, reconciler,
blueprint, artefacts) does not orchestrate; the orchestration layer
(driver, workflows) is a consumer of the substrate that happens to share
the repository.

## Scope recorded for the next session

- The webui fronts the driver: the over-harness console becomes the
  driver's steering surface, under the ownership split
  `dec.control-plane-programme` signed (cairn owns policy and control;
  the driver dispatches; the console shows and records).
- The driver may live in-repo as a distinct layer above the core, with
  its own node(s) in the blueprint and no dependency from the core into
  the driver layer.
- Declarative cairn-based workflows are in scope: a task routed to a
  harness with context guidance attached, and on-outcome routing to
  another harness or destination, expressed as cairn artefacts the
  driver reads, never as core behaviour.

## The rubric

- **Tier**: `binding`. It refines the live accepted orchestration authority and
  moves a repository boundary; only the maintainer can sign it.
- **Unblocks**: the driver todo and the widened console scope; both are
  recorded now and start only after this signature.
- **Alignment**: against `dec.cairn-mission` first, it keeps the
  properties the mission guards by drawing the orchestration boundary
  explicitly in the graph instead of leaving it to session memory.
  Goal 1: agents keep working because the substrate they read stays
  stable while the layer above it evolves. Goal 2: guardrails hold
  because the layer split is recorded and queryable. Goal 3: the
  maintainer signs one placement boundary rather than re-arbitrating
  every driver change. Goal 4: the intent is enqueued before any code
  exists. Goal 5: the signature queue shows this record with its
  briefing.
- **Options considered**: (a) keep the driver outside the repository,
  which splits one product across two repos and starves the driver of
  the graph's gates; (b) build orchestration into the core, which turns
  the substrate into a loop engine and breaks every consumer that wants
  a passive graph; (c) host the driver in-repo as a distinct layer above
  the passive core, fronted by the webui, with workflows as declarative
  artefacts. This is the recommendation. The cost of rejecting (c) is a
  permanently external driver that cannot share the repository's gates,
  or a core that stops being a substrate.

## Rationale

The maintainer ratified the inversion programme framing on 2026-07-31
(`res.inversion-convergence-minutes`, row R5: cairn steers rather than
passively maps), and the 2026-08-02 campaign shipped the
read-only console under `dec.control-plane-programme`'s three-owner
split. The remaining ambiguity was never whether orchestration exists;
it was which layer owns it and which repository hosts that layer. This
record closes that ambiguity in the smallest way: placement only,
recorded before any build starts.
