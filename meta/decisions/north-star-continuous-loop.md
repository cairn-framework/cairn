---
id: dec.north-star-continuous-loop
nodes:
  - cairn.root
status: accepted
date: 2026-07-29
informed_by: [res.gas-city-cairn-integration]
related:
  - dec.product-perimeter
  - dec.task-tracking-authority
  - dec.workspace-aggregation
---

# North star: continuous development with a quiet signature queue

Accepted 2026-07-29 by maintainer ratification (PR #528 sheet W9). This is the
alignment referent other artefacts cite. It changes no code by itself; it
states the goal, the rubric queued decisions must carry, and the boundary the
orchestration layer builds against. Its own queue entry carried the rubric it
defines; the rubric now lives in each queued artefact itself.

## The goal, stated once

1. Agents keep working while selectable work exists. Done is a terminal state,
   not a feeling: no ready unit remains in scope, and everything left parks
   visibly as waiting-on-signature, blocked-on-capability, or
   deferred-by-decision (the loop's `LOOP EXHAUSTED` is this state's
   per-session form). No agent waits mid-flight on a human; work that needs a
   human routes around them and continues elsewhere.
2. Guardrails keep the result aligned. When it still misses, the map and the
   decision chain make correction cheap: a later session can reverse-engineer
   why a thing is shaped the way it is, find the assumption that died, and
   re-aim the work without archaeology.
3. The maintainer signs the binding surface only: spec invariants, artefact
   schemas, registries, shipped pack content, supersessions of accepted
   decisions, and decisions whose `nodes:` span more than one container, the
   boundary `todo.decision-ratification-tiers` implements and is authoritative
   for. Everything below it is self-serve under recorded adversarial review.
4. No surprise signatures. A binding need is discovered at Scope time, enqueued
   as a proposed decision carrying the rubric below, and the loop continues
   with other work. A binding need first surfacing mid-implementation follows
   the SAME path, enqueue and reroute, with one addition: the proposed
   decision's Context section records where Scope missed it. The handling is
   identical either way, so there is nothing to gain by relabelling a binding
   need as local; the late-discovery note only feeds the audit of how Scope is
   performing.
5. The signature queue is visible in one place
   (`todo.maintainer-pending-queue`): first age-sorted over typed data, then
   sorted by what each item unblocks once `dec.todo-relationship-model` gives
   todos typed edges. The maintainer not knowing what is waiting on them is a
   defect state, not a mood.

## The rubric a queued decision must carry

- **Tier**: `local` or `binding`, with the mechanical facts behind the claim
  (node span, supersession, affected paths against the allowlist).
- **Unblocks**: the todos and findings that move when it is signed, by id.
- **Alignment**: one sentence against each goal above that it serves or trades
  off. A decision that cannot state its alignment is not ready to queue.
- **Options**: the two to four considered, the recommendation, and what saying
  no costs.
- Silence never accepts. An unanswered binding item waits visibly, and nothing
  ratifies on a timer.

## Orchestration boundary

`dec.product-perimeter` stands, including the ruling it absorbed from
`dec.no-orchestrator`: cairn does not repeat, schedule, retry, or supervise.
Continuation across iterations, worktrees, and repositories is a consumer
layer's job. That layer reads the graph (frontier, the pending queue,
selectability) and owns assignments, leases, and scheduling outside the
repository. Per-repository truth (blueprint, contracts, decisions, todos)
stays inside the repository, where it lands atomically with the code it
governs; externalised task state already has an accepted precedent in
`dec.task-tracking-authority`, which carries forward the read-only beads view,
and multi-project read aggregation in `dec.workspace-aggregation`.

Building that layer beside cairn supersedes nothing. Moving it inside cairn
would require superseding `dec.product-perimeter`, whose own revisit trigger
(a scheduling primitive inexpressible outside cairn, or demand for a
zero-dependency orchestrator outweighing integration cost) has not fired.

## The rubric, applied to this decision

- **Tier**: `binding`. It sets decision-authoring practice, and its rubric
  reaches shipped pack content through the tiers unit.
- **Unblocks**: nothing mechanically. It is the referent
  `todo.decision-ratification-tiers` (goal 3) and
  `todo.maintainer-pending-queue` (goal 5) implement against, and the
  alignment target every later queue entry cites.
- **Alignment**: goal 1, it defines the terminal state continuation drives to;
  goal 2, it names the map-and-decisions correction loop as the guardrail;
  goal 3, it fixes the signature boundary at the binding surface; goal 4, it
  makes late binding discoveries followable instead of gameable; goal 5, it
  makes the queue a surface rather than a conversation.
- **Options**: (a) no stated goal, alignment re-argued per session, which is
  the measured status quo; (b) goals as AGENTS.md prose, unratified and
  outside the graph, invisible to `cairn rationale`; (c) this decision.
  Recommendation: (c). Cost of no: the rubric has no referent, and queue
  triage stays judgement calls made fresh each session.

## Consequences

- `todo.decision-ratification-tiers` implements goal 3's boundary.
- `todo.maintainer-pending-queue` implements goal 5.
- The rubric binds decision authoring in this repository from acceptance. The
  shipped pack teaches it when the tiers unit lands, as one binding change.
- Accepting this puts `cairn.root` at 11 direct accepted decisions against the
  flat threshold of 10, so `CAIRN_DECISION_ACCUMULATION` fires there. Expected,
  advisory, and owned by `todo.lint-selection-folding` item 3; it must not be
  cleared by another consolidation rewrite before that item decides the
  threshold's shape.
