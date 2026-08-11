---
id: res.context-pass-pack-workflows.measurement
nodes:
  - cairn.kernel.cli
method: primary
date: 2026-08-11
related: [res.skill-absorption, res.reference-budget-headroom]
---

# Context pass measurement: workflow skills (explore, propose, apply, archive)

First-hand measurement from `todo.context-pass-pack-workflows` (2026-08-11),
the last child of `todo.context-pass-skill-pack`. Terms follow
`tools/agent-pack/tests/first_turn_budget_tests.rs`.

The advertised metadata terms are recorded once, in
`todo.context-pass-pack-workflows` as its task mandates, and the combined
pack first-turn measurement in the parent; this artefact carries only the
evidence the todos do not: file sizes and the compression rationale.

## File sizes (local evidence, outside the first-turn metric)

Whole-file bytes including frontmatter; these bodies load only when their
skill triggers, so only the frontmatter's name and description count toward
the first-turn metric recorded in the todo.

| File | Before | After |
|---|---|---|
| cairn-explore/SKILL.md | 3,356 | 2,940 |
| cairn-propose/SKILL.md | 3,959 | 3,885 |
| cairn-apply/SKILL.md | 6,431 | 6,398 |
| cairn-archive/SKILL.md | 2,007 | 2,029 |

The descriptions were the priority surface since they decide triggering:
explore's dropped its second sentence's restated noun list, and propose's
stopped advertising a specs artifact that `cairn change new` does not write
(it scaffolds proposal.md, design.md, tasks.md, and an empty specs/
directory), an accuracy fix as much as a cut. apply and archive descriptions
were already minimal trigger contracts and held.

Compression came from the framework, not truncation: the "I'll help you"
no-op sentences went; explore's step 2 kept only its action; propose moved
the name-collision rule from the guidelines list into step 1, where it is
checkable; apply's "Do NOT skip tests" guardrail moved whole into the
Mutation authority Never list ("skip a required test" plus the existing
failing-test clause), paired with its positive target (tests are the
contract with future maintainers); archive's
pending-tasks guardrail collapsed into step 2, which stops and reports.
Every deleted sentence's surviving copy is at the layer that owns it.

The pre-submit review corrected three claims the first draft carried or
broadened: archive's step 4 no longer says the command runs acceptance gates
(`cairn change archive` validates against the current graph; acceptance is
the `cairn change accept` prerequisite), explore's lint bullet states the
real exit contract (exit 1 on an Error finding), and explore's refine
suggestion says the command generates a change proposal rather than updating
the blueprint in place. archive is the one file that grew: its commit step
now reads the touched paths from `git status --short` and stages exactly
those, replacing `git add -A`, whose sweep of unrelated untracked files is
the hazard the explicit list removes.

## Limits

Byte counts are a proxy for context load. No CLI behaviour changed and no
accepted rule changed. This research informs the parent's combined
measurement and any future budget tightening, nothing else.
