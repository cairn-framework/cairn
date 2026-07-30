---
id: dec.cairn-mission
nodes:
  - cairn
status: accepted
date: 2026-07-30
informed_by: [src.mission-ratification-2026-07-30]
related:
  - dec.north-star-continuous-loop
---

# The Cairn mission

Accepted 2026-07-30 by maintainer ratification: chat approval during the
supervised B-queue session, recorded in `src.mission-ratification-2026-07-30`.
Every other goal statement in the graph is read against the mission it
records.

## The mission, verbatim as ratified

> Cairn exists so that AI agents can automate software development to the
> extreme, and the software still comes out maintainable, investigable,
> extendable, and fit for its purpose.

The System node description in `cairn.blueprint` carries the condensed form
of this sentence; this decision carries the binding one.

## The four properties as project terms

These four words are formal terms in this repository. They describe the
software developed under cairn, this repository included, and each is
defined by an observable quality of that software. The machinery that
protects them is the Mechanism section's subject, not part of the terms. A
change that degrades one of them works against the mission, whatever else
it improves.

- **Maintainable**: the cost of a change is proportional to the change, not
  to accumulated entropy. A competent later session, human or agent, can
  modify the software safely without re-deriving lost context by
  archaeology.
- **Investigable**: behaviour and structure can be explained after the
  fact. Diagnosis traces from symptom to the responsible code, and from
  that code to the reasoning behind its shape, out of the record rather
  than out of anyone's memory.
- **Extendable**: new requirements are accommodated at predictable cost and
  without disproportionate disruption to what already works. Reshaping
  existing code to make room is normal work under this term, not a
  violation of it.
- **Fit for its purpose**: the software meets the needs it exists to serve,
  as those needs are recorded, and keeps meeting them as it changes.
  Fitness is judged against recorded intent, not impressions.

## Mechanism

The graph records what exists: nodes, edges, and the paths they own.
Decisions record why: the provenance chain ties each rule to the evidence
behind it. Contracts and gates enforce what they encode: findings, hooks,
and CI turn declared intent into blocking checks.

## Limit

Enforcement reaches only what is encoded. A property that no contract, gate,
or decision encodes is not protected by the machinery; human review and
judgment cover the rest. The mission does not claim the tooling replaces
review, and prose that implies otherwise misstates it.

## Rationale: where the north star stands

`dec.north-star-continuous-loop` (linked via `related:`) states five
operational goals: agents keep working, guardrails keep the result aligned,
the maintainer signs only the binding surface, no surprise signatures, and a
visible signature queue. Those goals describe how this project runs its own
development so that the mission holds at high throughput. They are
operational strategy in service of this mission, not the mission itself:
continuous agent work and a quiet queue count for exactly what they
contribute to software that is maintainable, investigable, extendable, and
fit for its purpose. Where a reading of the north star and a reading of this
mission pull apart, this mission wins and the goal is re-derived from it.
The north star is not superseded: its goals, its rubric, and its
orchestration boundary stand as accepted.

## The rubric, applied to this decision

- **Tier**: `binding`. It anchors on the root System node and sits above
  every other goal statement. The maintainer ratified it directly on
  2026-07-30 (`src.mission-ratification-2026-07-30`), so it enters the graph
  as `accepted` rather than queueing at `proposed`.
- **Unblocks**: nothing mechanically. It is the referent later alignment
  sections cite above the north star's goals.
- **Alignment**: goal 1 serves the mission only while its terminal states
  name what the software still needs; goal 2 is the mission's correction
  loop; goal 3 keeps ratification cost proportional, which is what makes
  extreme automation affordable; goals 4 and 5 keep the human signature a
  working control rather than a bottleneck. Each goal is judged by the
  software it produces.
- **Options**: (a) leave the mission implicit in guidance prose and session
  memory, invisible to `cairn rationale`; (b) supersede the north star with
  one merged mission-plus-goals decision, re-litigating five accepted goals
  to state one sentence; (c) this decision: the mission as root referent
  with the north star linked and subordinate. Recommendation: (c). Cost of
  no: alignment keeps being argued from operational goals with no recorded
  mission above them.

## Consequences

- The System node description in `cairn.blueprint` becomes the condensed
  mission line, and `cairn context` prints that headline plus the
  pending-signature count, so the mission and the human-control surface are
  both visible at the default agent entry point.
- The four properties are citable terms for later decisions and reviews.
- The ratification source stands `unverified`, so one `CAIRN_SOURCE_UNVERIFIED`
  Info finding is expected to stand; the source record explains why.
