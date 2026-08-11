---
name: cairn-loop-reconcile
description: "Plan-reconciliation procedure for one cairn-dev loop iteration: record what the unit's evidence changed, amend the todos, specs, and decisions it invalidated, move dependants through the sanctioned write verbs. Loaded by cairn-dev loop mode after Verify and before Land; not for ordinary development sessions."
license: MIT
compatibility: Requires Cairn CLI.
---

# Reconcile the remaining plan

Loaded by `cairn-dev` loop mode after Verify and before Land. Inputs: the
selected unit (slug or finding code), its resolved `node`, its validated todo
body when it has one, the proof Verify produced, and the bound `$CAIRN`.

A campaign runs across many fresh sessions with no memory between them. The plan
the next session reads is whatever this commit leaves on main. Much of the work
is research based, so a landed unit routinely invalidates a later one. Recording
that is part of the unit, not paperwork after it.

Declared exits, exactly one, as the last line you return to loop mode:

- `RECONCILED`: the plan on disk matches this unit's evidence, and the edits sit
  in the worktree for Land to stage.
- `LOOP HALTED`: reconciling needs a maintainer ruling this session cannot make.

Provenance only. This recipe never selects the next unit, never repeats, never
retries, never interprets a terminal token, and never commits or pushes. Land
owns the commit; the invoking user or harness owns repetition
(`dec.no-orchestrator`, `dec.loop-command-harness-model`).

## Authoring paths

Every obligation below has one path. `$CAIRN research`, `$CAIRN todos`, and
`$CAIRN decisions` are read-only queries, not creation verbs.

| Obligation | Path |
|---|---|
| Observation and result | hand-author `meta/research/<slug>.md` (id `res.<slug>`, `nodes:` required) |
| A rule changed | `$CAIRN decision new <slug> --node <id>`, then author the body |
| Status change | `$CAIRN todo set <slug> <open\|in_progress\|done\|blocked>` |
| New prerequisite | `$CAIRN todo new <slug> --node <id>` |
| Todo body, Depends on, umbrella Waves, change spec | direct file edit |

## 1. Record the observation

If this unit produced evidence (a measurement, a rejected option, a behaviour
the plan did not predict), write it as a Research artefact naming the node it
constrains. If it changed a rule rather than only informing one, write a
Decision as well and state retain, revise, or remove for the hypothesis the unit
tested.

A unit that changed no rule and revealed nothing the plan did not already
predict records nothing here. Say so in the summary; do not manufacture an
artefact.

## 2. Amend what the evidence invalidated

Read the downstream todos and any change specs that assumed what this unit just
disproved. Correct their bodies directly: `$CAIRN todo set` changes status only.
A body left standing after its assumption died is how the next session inherits
a stale plan.

## 3. Move dependants

- Set a todo `open` only after verifying that every entry in its own `Depends
  on` list is done. One landing unit may satisfy one dependency while others
  remain, so check all of them, not just the one this unit closed.
- Set newly ineligible todos `blocked`.
- If this unit created or revealed a new prerequisite, author it, add it to an
  appropriate umbrella Waves tier so the selector can reach it, add it to the
  dependant's `Depends on` list, and set that dependant `blocked` until the
  prerequisite is delivered.
- A child gated on a verdict rather than on a unit does not follow the generic
  done-rule. An evaluation round is `done` on `revise` too, and a `retain`
  record may still be `proposed`. Keep such a child `blocked` and open it only
  against an accepted (`status: accepted`) retaining decision. On a `revise`
  round, author the successor round and leave the child `blocked`. Drop it only
  through an accepted decision that supersedes the ruling it depended on.

## 4. Never contradict an accepted decision silently

If this unit's evidence invalidates an assumption inside an accepted decision,
author the superseding decision now: restate the target's surviving obligations
inside it, set it `status: accepted` only if that is yours to do, mark the
target `status: superseded`, and link both. `refines` is informational and never
overrides a ruling. If acceptance is the maintainer's call, land the proposed
decision plus a blocked todo and say so plainly. Never self-ratify a binding
decision. A local-tier decision may be accepted only through the receipt protocol:
two independent lens receipts bound to the subject hash and a `ratified_by: machine`
marker when the loop signs (`todo.decision-ratification-tiers`). Do not leave the
contradiction unrecorded.

## 5. Hand the edits to Land

Leave every artefact and tracker edit in the worktree. Land stages them by
explicit path inside this unit's single commit, so the next fresh session reads
the reconciled plan from main with no external memory. Nothing here is committed
separately, and nothing is deferred to "after the merge": a plan update outside
this commit does not exist for the next session.

## Worked scenario

An evaluation unit measures a candidate projection and finds it worse than
composing existing verbs. A later todo in the same programme is written to build
that projection.

1. `meta/research/context-projection-measurement.md` records the corpus, the
   result, and the limits of what it proves.
2. `$CAIRN decision new context-projection-declined --node cairn.kernel.query`
   records the removed hypothesis and links the research through `informed_by`.
3. The dependant todo's body is edited to cite that decision, and
   `$CAIRN todo set build-context-projection blocked` takes it out of selection.
4. All four files are left for Land, which stages them with the unit's code and
   commits once.

The next session sees a blocked todo whose body names the decision that blocked
it, from `$CAIRN status` alone.

## 6. Return

Report what you recorded, which todos changed status and why, which bodies you
corrected, and any decision you authored or superseded. If nothing needed
reconciling, say that and why. Then output your single exit token as the final
line.
