---
node: cairn.brownfield
status: blocked
created: 2026-08-10
blocked_by:
  - todo.brownfield-extraction-drafting-test
  - todo.brownfield-extraction-external-run
parent: todo.brownfield-extraction-flow
---

# Validate the extraction flow end to end and on an external repository

Validation unit split out of `todo.brownfield-extraction-flow` under the sizing
rule. It carries the two acceptance criteria of the parent that need a working
flow: the end-to-end drafted-artefact assertion and the external-repository run.
Its own two prerequisites, `todo.brownfield-onboard-decisions-index` and
`todo.brownfield-extraction-authoring-reference`, are both done, so the flow
under validation exists.

Those two criteria are not one small reviewable PR, and part of one of them
cannot be completed by an iteration at all: an extracted draft is accepted by
the maintainer or by nobody. So this unit was decomposed on 2026-08-10 under the
sizing rule, by completion authority. Blocked at decomposition time on
sub-todos: todo.brownfield-extraction-drafting-test (the fixture assertion,
closed by the test suite) and todo.brownfield-extraction-external-run (the
external run, its `meta/research/` and `meta/sources/` provenance, and the
retained drafts, closed by an iteration).

The maintainer's ruling on a draft went to
`todo.brownfield-extraction-maintainer-ruling`, a sibling of this todo under
`todo.brownfield-extraction-flow`, not a child of it. Parked inside this unit it
would let a sibling landing close this parent with the ruling still unmet,
because landing closes a parent whose last OPEN child lands and a parked child
is never open. One level up it blocks the todo whose acceptance already requires
an accepted draft.

The iteration completing the last of the two sub-todos flips this parent to
done.

## Task

The children own the work; nothing is implemented here. Each child body is the
contract for its half, so read the one you selected rather than this summary.

## Acceptance

- Both sub-todos are done. This todo closes on the recorded run, not on the
  ruling: `todo.brownfield-extraction-flow` stays blocked afterwards until
  `todo.brownfield-extraction-maintainer-ruling` lands, and that todo carries
  the final flow-to-`done` and
  `todo.brownfield-extraction-pointer`-to-`open` cascade.
