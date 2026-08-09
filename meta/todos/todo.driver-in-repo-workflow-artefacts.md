---
node: cairn.kernel.artefacts
status: open
created: 2026-08-09
parent: todo.driver-in-repo
---

# Driver In Repo Workflow Artefacts

## Scope
Define the declarative workflow artefact that the driver reads. It is typed,
inert policy: Cairn parses, validates, stores, and exposes it, while the driver
alone evaluates it. The shape includes a match predicate over dispatch units, a
harness route with context guidance, limits for wave size, TTL, a per-unit
spend cap, and a per-wave spend cap, an outcome-class routing table, and named
deterministic `require:` gates. It also carries a `serialises:` list of path
prefixes for hotspot policy. All vocabularies and slots are closed and
executable logic is not embedded inline.

## Parent constraints
The parent todo is `todo.driver-in-repo`, under `## Task`, item 2:

> Declarative workflow artefacts the driver reads: a task routed to a
> harness with context guidance, and on-outcome routing to another
> harness or destination. These are inert policy under clause 3. Cairn
> parses, validates, stores, and exposes them and evaluates none of
> them.

The parent `## Grill rulings`, Q4, is provisional grill direction pending an
accepted owning decision. Its relevant passage is:

> **Q4, workflow definition: an inert typed cairn artefact, evaluated
> only by the driver** (task 2, confirming clause 3 of the placement
> decision). Shape: a match predicate over dispatch units, a harness
> route with context (skills, briefing), limits (wave size, TTL), and
> an outcome-class routing table over Q3's derived classes. All slots
> are closed vocabularies cairn validates at scan time; a workflow
> never carries executable logic inline. Workflows name gates as rules
> of engagement: a `require:` list of registered deterministic checks
> (rust gates, scan strict, review receipts) bound to a named moment;
> the driver enforces them on driver-observed evidence before the
> routed action fires. Policy updates are ordinary artefact edits
> landing through PR gates with provenance; the driver re-reads policy
> each cycle, so a landed change binds at the next dispatch decision.

Accepted `dec.orchestrator-skills-layering`, clause 4, ratifies the three
terminal tokens and the no-orchestration rule as pack content. The Q4 routing
table and its remaining derived outcome-class details stay provisional pending
the owning decision.

Accepted `dec.rung-three-coordination-substrate`, clause 3, ratifies the
hotspot policy requirement:

> Therefore, in the derived-first phase, hotspot paths are in no unit's
> derived write-set, every derived write-set is stamped `completeness:
> "partial"` naming the uncovered prefixes, and contention is resolved by
> policy: the inert workflow artefact carries a `serialises:` list of
> **path prefixes**, cairn validates them and evaluates nothing, and the
> driver grants the hotspot permission to one unit per wave in deterministic
> order.

`todo.workflow-serialises-validation` is the follow-up owner for that field.
It requires `serialises:` to contain path prefixes, not node ids, and requires
an unknown prefix to be a validation finding while a valid prefix is stored and
never evaluated by the core.

The parent `## Grill rulings`, Q9, adds the full budget contract:

> (2) Budget: the workflow `limits:` slot carries per-unit and per-wave
> spend caps beside wave size and TTL; the driver refuses dispatch past a
> cap and queues a budget-exhausted human moment.

The parent `## Grill rulings`, Q3, supplies provisional outcome classes for the
routing table. The parent `## Acceptance` requires that the substrate gain no
orchestration behavior, so parsing and exposure must stay passive.

## Dependencies
The blueprint-node sub-todo is first and supplies this artefact's owning
surface. This child remains open, so its inter-child ordering is recorded in
this prose rather than an unresolved `blocked_by` edge; the parent carries
typed `blocked_by` edges to all four children. This unit is independent of the
selector-wire contract, although the reaction-loop sub-todo consumes both this
policy shape and the selector wire. The selector wire remains a passive query
and must not evaluate workflows.

## Sizing
M. The later implementation is one artefact subsystem plus parser, validation,
storage, exposure, and focused contract tests, kept under roughly 600 changed
lines. It must not absorb driver execution or lease storage.

## Non-goals
Do not execute routes or gates during scanning, add executable workflow logic,
omit separate per-unit or per-wave budget caps, add per-unit authoring, or
implement the reaction loop, selector, or lease surfaces.
