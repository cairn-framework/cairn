---
name: cairn-loop-scope
description: The scoping procedure for one cairn-dev loop iteration: orient on the selected unit's node, respect accepted decisions, write one verifiable success criterion, and reroute to a prerequisite rather than expanding. Loaded by cairn-dev loop mode at its Scope step; declares the typed exits that step routes on. Not for ordinary development sessions.
license: MIT
compatibility: Requires Cairn CLI.
---

# Scope one loop unit

Loaded by `cairn-dev` loop mode at its Scope step. Inputs: the selected unit
(slug or finding code), its resolved `node`, its validated todo body when it has
one, and the bound `$CAIRN`.

Declared exits, exactly one, as the last line you return to loop mode:

- `SCOPED`: a success criterion is written and implementation may begin.
- `REROUTED`: a prerequisite must land first; the tracker edits you made ARE this
  iteration, and loop mode goes straight to Land.
- `LOOP HALTED`: the unit cannot be scoped without a maintainer.

## 1. Orient

```bash
$CAIRN neighbourhood <node> --include-todos --include-changes
$CAIRN rationale <node>
$CAIRN deps <node> --direction in --transitive
```

Read all three before deciding anything. The first shows what already claims this
area, including an active change you must not collide with. The second is the
accepted authority over the node. The third is the blast radius.

If the unit has a todo body, its Scope, Depends on, and Acceptance sections bind
here. Where the body and your orientation disagree, the body is the contract and
the disagreement is worth reporting in the summary.

## 2. Respect accepted decisions

An accepted decision covering the node constrains the implementation. You may not
contradict one inside this iteration. If the unit cannot be done without
contradicting an accepted decision, that is not a coding problem:

- If the superseding decision is knowable, the gap IS the unit. Investigate,
  stress-test two to four options, persist a justified recommendation as a `meta/`
  artefact, create a blocked tracker item, and return `REROUTED` so the package
  lands.
- If it needs a ruling only the maintainer can give, do the same and say so
  plainly in the artefact. Never self-ratify a binding decision. A local-tier
  decision may be accepted only through the receipt protocol: two independent lens
  receipts bound to the subject hash and a `ratified_by: machine` marker when the
  loop signs (`todo.decision-ratification-tiers`).

## 3. Write one verifiable success criterion

One sentence, checkable by running something. "`cairn scan` reports zero
`CAIRN_CONTRACT_LEAF_UNCOVERED` findings" is a criterion. "Contracts are better"
is not.

For a bug fix the criterion is a test that fails now and passes after. For a lint
finding it is that finding's disappearance, confirmed by re-running the check that
produced it. Write it down; Verify checks it and Land reports it.

## 4. Reroute, never expand

Scope may narrow or reroute. It may never grow the unit.

If orientation reveals a prerequisite that must land first, stop before touching
code:

1. Author the prerequisite todo (`$CAIRN todo new <slug> --node <id>`).
2. Set this unit's todo `blocked`, with a body line naming the prerequisite slug
   (`todo.<slug>`) and node id.
3. Return `REROUTED`. Loop mode lands those tracker edits as the single commit.

The prerequisite is then an open todo eligible for normal selection, and the
blocked unit is skipped until it is done. Selection order is otherwise unchanged.

If the unit turns out to be too large for one small reviewable PR, that is the
sizing rule in loop mode, not a reroute: return to loop mode and say so.

## 5. Return

Report the node, what the orientation showed, the accepted decisions that
constrain the work, the success criterion, and the blast radius. Then output your
single exit token as the final line.
