---
node: cairn.brownfield
status: blocked
created: 2026-08-10
blocked_by:
  - todo.brownfield-extraction-external-validation
parent: todo.brownfield-extraction-flow
---

# Carry an extracted draft through the maintainer ruling

Not agent-actionable until the maintainer has ruled. This todo holds the one
acceptance criterion of `todo.brownfield-extraction-flow` whose completion
authority is the maintainer: an extracted draft the maintainer path accepts. No
iteration may set `status: accepted` on an extracted decision
(`dec.brownfield-extraction-mechanism`, and `AGENTS.md`, "Put decisions to the
maintainer in-session").

It hangs off `todo.brownfield-extraction-flow` rather than off
`todo.brownfield-extraction-external-validation` deliberately. A parked child
inside the validation unit would let a sibling landing complete that parent,
because landing closes a parent whose last OPEN child lands and a parked child
is never open. Here the criterion sits at the level whose own acceptance already
requires an accepted draft.

The single `blocked_by` edge on `todo.brownfield-extraction-external-validation`
carries the ordering. Validation is done only when both the fixture test and the
external run are done, so this todo is not reachable before the drafts exist and
the fixture criterion is met. The edge is a status-line away from being bypassed
by hand, so the precondition is also an acceptance clause below: an iteration
that finds this todo `open` while validation is not `done` sets it back to
`blocked` with `cairn todo set` and stops, rather than landing it.

When validation lands, the iteration that completes it removes this resolved
edge, leaving this todo blocked with no declared blocker, the silent park under
`dec.todo-relationship-model` clause 4, and hands the maintainer the command
that returns it to selection once they have ruled:

```
cairn todo set brownfield-extraction-maintainer-ruling open
```

The command travels with the drafts because a blocked todo is skipped before its
body is read. If the edge is left in place, the resulting blocked-with-all-done
state raises `CAIRN_TODO_STATUS_CONTRADICTION`, which is the intended loud
signal rather than a silent stall.

## Task

The ruling is read from the artefacts, never from session memory: an accepted
draft carries `status: accepted`, an undecided one is still `proposed`, and the
run's research artefact carries each outcome and every rejection reason, written
before this todo was reopened. Start by listing the drafts this todo's body
names and reading their status and that record. If the reopen happened without
it, that record is the first thing to obtain from the maintainer; do not infer a
ruling from a status line alone.

Record the maintainer's ruling on the drafts the external run retained, in that
run's research artefact: which draft was accepted and on what evidence, and
every rejection with what it says about the evidence boundary. Test each
rejection against the `revisit_triggers` frontmatter of
`dec.brownfield-extraction-mechanism`, which that artefact names as the sole
source of reconsideration conditions, and report a fired trigger as one.

A rejected draft is then retired to `status: deprecated` with its reason in the
Decision section, the schema's non-accepted terminal state and the
`todo.autodocs-arm-a-item-7-ratification` precedent. Left `proposed`, it sits in
`cairn pending` and is handed back to the maintainer every time.

If every draft is rejected, the flow has not yet produced a decision the
maintainer path accepts and this unit is not done. Take the reroute path in
`cairn-loop-scope` section 4 rather than a normal landing: author the smallest
follow-up the recorded rejections imply, which is a redraft from the retained
evidence when the prose or the selection was at fault, a second external run
when the evidence itself was too thin, and a mechanism revisit only when a
`revisit_trigger` fired. Add it to this todo's `blocked_by`, set this todo
`blocked` through `cairn todo set`, and land those tracker edits as the
iteration. Landing's set-the-selected-todo-done
step assumes a unit whose acceptance is met; the conflict between that step and
a reroute landing is recorded in `todo.loop-landing-completion-conditions` and
must be resolved there, not improvised here.

That follow-up's own Acceptance carries the return path, or this todo parks
unreachably: on its landing it removes its resolved edge here, leaves this todo
in the silent park, hands the new drafts and the reopen command to the
maintainer, and does not open this todo itself.

## Non-goals

No change to the command surface, the shipped reference, or the evidence index.
No redraft, second external run, or mechanism revisit inside this unit: any of
those is the follow-up above, authored and landed as tracker edits.

## Acceptance

- `todo.brownfield-extraction-external-validation` is `done` at the time this
  todo lands. An early reopen is a status-line edit that no scanner rejects, so
  this clause is the guard: found `open` against an unfinished validation, this
  todo goes back to `blocked` and the iteration stops.
- At least one draft from the external-repository run carries `status:
  accepted`, set by the maintainer rather than by an iteration.
- The accepted draft's retained evidence names at least one source path in the
  external repository, at the recorded commit, that carried no cairn-specific
  annotation before the run. An accepted draft derived from pre-existing cairn
  markers does not satisfy this unit.
- Every rejection is recorded in the run's research artefact, with a
  fired-or-not verdict on any `revisit_trigger` it touches.
- This is the last child of `todo.brownfield-extraction-flow`, so the iteration
  that lands it also sets that parent to `done` and
  `todo.brownfield-extraction-pointer` to `open`.
- `cairn scan --strict` exits 0.
