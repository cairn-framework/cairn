---
node: cairn.kernel.artefacts
status: open
created: 2026-08-09
---

# Driver In Repo Workflow Artefacts

## Scope
Define the declarative workflow artefact that the driver reads. It is typed,
inert policy: Cairn parses, validates, stores, and exposes it, while the driver
alone evaluates it. The shape includes a match predicate over dispatch units, a
harness route with context guidance, limits for wave size, TTL, and spend, an
outcome-class routing table, and named deterministic `require:` gates. All
vocabularies and slots are closed and executable logic is not embedded inline.

## Parent constraints
The parent todo is `todo.driver-in-repo`, under `## Task`, item 2:

> Declarative workflow artefacts the driver reads: a task routed to a
> harness with context guidance, and on-outcome routing to another
> harness or destination. These are inert policy under clause 3. Cairn
> parses, validates, stores, and exposes them and evaluates none of
> them.

The parent `## Grill rulings`, Q4, is binding for the later design:

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

The parent `## Grill rulings`, Q3, supplies the outcome classes consumed by the
routing table. The parent `## Acceptance` requires that the substrate gain no
orchestration behavior, so parsing and exposure must stay passive.

## Dependencies
The blueprint-node sub-todo is first and supplies this artefact's owning
surface. This unit is independent of the selector-wire contract, although the
reaction-loop sub-todo depends on both this policy shape and the selector wire.
The selector wire remains a passive query and must not evaluate workflows.

## Sizing
M. The later implementation is one artefact subsystem plus parser, validation,
storage, exposure, and focused contract tests, kept under roughly 600 changed
lines. It must not absorb driver execution or lease storage.

## Non-goals
Do not execute routes or gates during scanning, add executable workflow logic,
implement the reaction loop, or define the selector or lease surfaces.
