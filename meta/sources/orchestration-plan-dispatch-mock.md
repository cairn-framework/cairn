---
id: src.orchestration-plan-dispatch-mock
file: studio/mocks/orchestration-plan-dispatch.html
verification: tracked
type: in-repo rendered mockup
date: 2026-08-06
---

# The dispatch preview mock, as rung 3's rendered evidence base

`todo.console-orchestration-ux-design` round 3 handed this screen to
`todo.parallel-dispatch-granularity` as the rendered evidence its task 5 owed.
`res.parallel-dispatch-rung-3` reads it as a specification of what the
coordination substrate must be able to answer.

The lines rung 3's design is written against, quoted from the file:

- `Next wave · 2 units` with `write-sets disjoint · parallel worktrees`
  (line 888 to 889): the wave is set-valued and its members are pairwise
  disjoint, which is the disjointness predicate rendered.
- `wf.default 3: ready · contract present · write-sets disjoint`
  (line 897, repeated at 906): the composer rule is quoted from an inert
  workflow artefact, so the plan identity must include the rule that produced
  it.
- `worktree ../cairn-wt/driver · lease on grant` (line 898): the worktree is
  known at composition time and the lease is a promise at preview, not yet a
  fact, which is why the lease grant's residue can be seeded with branch and
  worktree.
- `todo.lease-read-surface waits for this wave: it would change
  docs/registries/declared-items.md, and so would todo.driver-in-repo. Same
  files, one at a time: it queues behind that claim and joins the next wave.`
  (line 917): the write-set overlap case, in the plain register, naming one of
  the serialisation hotspots.
- `The console shows this plan; only the driver dispatches, and only after your
  queue is drained of what blocks it. Nothing here is a button.` (line 930):
  the console records consent and starts nothing.

Two claims in that quoted overlap sentence are more specific than any evidence
available at composition time, and `res.parallel-dispatch-rung-3` Part 3
records the correction rather than designing backwards from the pixels.
"It would change `docs/registries/declared-items.md`" asserts per-unit file
knowledge that node-closure derivation cannot produce, because
`docs/registries/` is owned by no node and because the fact that a given unit
will edit a given file does not exist in committed state. "Queues behind that
claim" asserts a lease that has not been granted. Both become true in a later
phase; the phase-0 sentence keeps "same files, one at a time" verbatim and
names the shared prefix and the permission holder instead.

Sibling screens in the same round, cited for the claim-state vocabulary:
`studio/mocks/orchestration-mixed-repository.html` and
`studio/mocks/orchestration-return-orient.html` render the expired held claim
`r-041` with residue rows, no outcome recorded, stale and unclassified, against
the backlog's `no lease recorded` cross-check line as the no-claim contrast.

Amended 2026-08-07: line 917's overlap sentence was replaced with the phase-0
form under `dec.rung-three-coordination-substrate` clause 5 (executed in its
acceptance change). The quotes above record the 2026-08-06 evidence as handed
over, which is the state the rung 3 design was made against.
