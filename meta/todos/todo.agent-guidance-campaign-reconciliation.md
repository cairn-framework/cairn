---
node: cairn.kernel.cli
status: open
created: 2026-07-22
---

# Agent Guidance Campaign Reconciliation

## Priority

P2. It defines the end-of-unit reconciliation step in the `cairn-dev` loop
recipe. Distinct from `todo.agent-context-bundle-evaluation`: that scores
per-task retrieval; this governs how a multi-run campaign updates its own
remaining plan.

## Depends on

`todo.agent-guidance-router-playbooks`, which establishes the `cairn-dev`
entry, its modes, and the loop procedure skills this step joins.

## Problem

A Cairn development campaign runs across many fresh-session iterations. Much of
the work is research based, so a completed unit routinely invalidates or
reshapes later units. Nothing in the current loop recipe requires an agent to
record why the plan changed or to update the remaining backlog before it ends,
so downstream todos drift from the evidence and the next iteration inherits a
stale plan.

Harness Engineering's Improve One Harnessed Job playbook closes this with a
bounded loop: job contract, baseline, earliest failed handoff, one reversible
intervention hypothesis, claim-boundary proof, fresh rerun, then retain,
revise, or remove with a recorded follow-up and retirement condition. This
todo adapts that discipline into a Cairn-native reconciliation step mapped onto
existing artefacts, without giving Cairn a scheduler.

## Scope

- Add a `reconcile-plan` recipe (working name) to the `cairn-dev` loop mode,
  loaded just in time like scope, implement, recovery, and landing.
- Because today's `/cairn-loop` selects a slug but never loads the selected
  unit's todo body (it scopes via `neighbourhood`, `rationale`, and `deps`),
  specify and implement where the loop mode reads the reconcile recipe from (the
  selected unit's todo body, or a loaded skill that reads the umbrella), so the
  step is guaranteed to run before the terminal token.
- The recipe runs after proof and before the terminal token. It requires the
  agent to:
  - record observation and result as a Research artefact, and a Decision when
    a rule changed (retain, revise, or remove the hypothesis);
  - amend downstream todos and change specs that this unit's evidence
    invalidated;
  - set newly ineligible todos to `blocked`, and set a todo `open` only after
    verifying every dependency in its own Depends on list is done, through the
    sanctioned `cairn todo set` verb. A single landing unit may satisfy one
    dependency while others remain, so check all of them. If this unit creates
    or reveals a new prerequisite, add it to an appropriate umbrella Waves tier
    so the selector can reach it, add it to the dependant's Depends on list, and
    set that dependant back to `blocked` until the prerequisite is delivered;
  - for a child gated on a verdict rather than a unit (OMP publication on the
    treatment `retain` decision), do not use the generic done-rule: a treatment
    round is `done` on `revise` too and a retain record may still be `proposed`.
    Keep it `blocked` and open it only after verifying an accepted (`status:
    accepted`) `retain` decision; on a `revise` round spawn the successor and
    leave it `blocked`; on a `remove` verdict, drop it only via an accepted
    decision that supersedes `dec.agent-pack-packaging` (restating its surviving
    obligations, that target marked `superseded`);
  - author or supersede any Decision whose assumption this unit changed, and
    never contradict an accepted decision silently.
- Stage every plan update inside this unit's single commit, so the next fresh
  session reads the reconciled state with no external memory.
- Provenance only. The recipe never selects the next unit, never repeats, and
  never interprets terminal tokens. Between units the operator or thin wrapper
  picks the next eligible child and hands its slug to a fresh `/cairn-loop`
  session, which performs and gates that one unit; per
  `dec.loop-command-harness-model` the harness owns repetition, and
  `dec.no-orchestrator` keeps any scheduler out of Cairn core.
- Map each obligation to its exact authoring path so the step is deterministic,
  not prose: hand-author the Research artefact per the `meta/` schema, use
  `cairn decision new` for a decision, use `cairn todo set` for status changes,
  and edit change specs directly. `cairn research` and `cairn todos` are
  read-only queries, not creation verbs.

## Acceptance

- A campaign scenario where one unit's research invalidates a later todo shows
  that todo blocked or amended with a linked decision in the same commit,
  before the terminal token.
- A scenario proving the loop mode loads and runs the selected unit's reconcile
  recipe before the terminal token, not merely that the recipe exists.
- The reconciled plan is visible to the next fresh session through `cairn
  status` and this programme's umbrella Waves, with no external memory; the
  supervised selector reads those to pick the next programme child, not unscoped
  `cairn next` (which selects the oldest open todo across the whole repo and
  cannot scope to this programme).
- No reconciliation path selects work, repeats, or emits more than one
  terminal token.
- The step preserves every semantic clause of `dec.loop-command-harness-model`
  and adds no scheduler.

Informed by the Harness Engineering anthology (pinned commit 226c8d35),
captured as source and research by `todo.agent-guidance-provenance`.
