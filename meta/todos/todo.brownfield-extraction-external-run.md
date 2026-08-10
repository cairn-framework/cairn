---
node: cairn.brownfield
status: open
created: 2026-08-10
parent: todo.brownfield-extraction-external-validation
---

# Run the extraction flow against a real external repository and record it

Validation unit split out of `todo.brownfield-extraction-external-validation`
under the sizing rule. It carries the half of that unit that leaves the repo:
the external-repository run, its provenance record, and the drafts that run
retains. The in-repo fixture assertion belongs to
`todo.brownfield-extraction-drafting-test`, and the maintainer's ruling on a
draft to `todo.brownfield-extraction-maintainer-ruling`.

## Task

Run the flow against at least one real external repository and record that run
as provenance: a `meta/research/` artefact carrying the repository, the commit,
the exact commands, the evidence counts per source class, the node bindings
produced, and the unbound evidence, citing a `meta/sources/` artefact with the
`file:` and `verification:` fields the source schema requires
(`docs/conventions.md` section 10).

The source classes are the landed `kind` labels: `document`, `readme-section`,
`invariant-comment`, and `code-target`. Per
`res.onboard-decision-evidence-scope`, the `invariant-comment` count is scoped
to every source file the bounded survey observed, while `code-target` is scoped
to discovery candidates only, so the two counts are not comparable and an
`invariant-comment` count above the candidate count is expected, not a defect.

Verify the review handoff the shipped reference owns
(`todo.brownfield-extraction-authoring-reference`): the
`cairn onboard decisions` report that produced a draft is kept with it, and the
draft goes to the maintainer for a ruling. Nothing in the flow, and nothing in
this unit, sets `status: accepted` itself; acceptance is the maintainer's, per
`AGENTS.md` ("Put decisions to the maintainer in-session").

Give a fired-or-not verdict on every entry in the `revisit_triggers` list in the
`dec.brownfield-extraction-mechanism` frontmatter, which that artefact names as
the sole source of reconsideration conditions; do not work from a paraphrase of
it. A fired trigger is recorded in the research artefact as a maintainer-facing
finding, not silently absorbed.

The ruling itself is not this unit. Land the run, the research and source
artefacts, and every retained draft, then put the drafts to the maintainer in
the session. `todo.brownfield-extraction-maintainer-ruling` holds the
acceptance criterion, so this unit completes on the recorded run and does not
stall on a signature it cannot obtain. A run whose drafts the maintainer later
rejects still completes it.

## Non-goals

No change to the command surface or the shipped reference. A fired
revisit_trigger is recorded and raises a follow-up todo rather than widening
this unit. The in-repo fixture test is
`todo.brownfield-extraction-drafting-test`; the maintainer's ruling on the
drafts, and the record of any rejection, is
`todo.brownfield-extraction-maintainer-ruling`.

## Acceptance

- A `meta/research/` artefact records the external-repository run with
  repository, commit, commands, evidence counts per source class, bindings, and
  unbound evidence, and cites a `meta/sources/` artefact carrying `file:` and
  `verification:`.
- The research artefact retains every draft the external run produced together
  with the evidence report that produced it, and states, for every entry in the
  `revisit_triggers` frontmatter of `dec.brownfield-extraction-mechanism`,
  whether the run fired it.
- At least one retained draft derives from a source path in the external
  repository, at the recorded commit, that carried no cairn-specific annotation
  before the run, so the ruling unit has a candidate that can satisfy it. A
  draft derived from pre-existing cairn markers does not count.
- At this unit's landing no draft carries `status: accepted`, `ratified_by`,
  `receipts`, or `supersedes`, and no code path in the flow sets them. A later
  maintainer acceptance is the ruling unit's business, not a violation here.
- `cairn scan --strict` exits 0.
- The retained drafts are put to the maintainer in-session on landing, and
  `todo.brownfield-extraction-maintainer-ruling` gains a body line naming the
  drafts this ruling covers, each by artefact id and path, plus the research
  artefact holding their evidence. The ruling itself is then readable from the
  drafts: an accepted draft carries `status: accepted`, so the iteration that
  later lands the ruling recovers the outcome from main with no session memory.
  If `todo.brownfield-extraction-drafting-test` is still unfinished, the drafts
  go to the maintainer as context only, with no reopen command and an explicit
  note that the ruling todo stays blocked until validation completes: reopening
  it early would stand it `open` against an unresolved blocker.
- If this landing is the one that completes
  `todo.brownfield-extraction-external-validation`, it also removes the
  resolved `blocked_by` edge that ruling todo declares on that parent (per
  `dec.todo-relationship-model` clause 4) and states the handoff to the
  maintainer in two steps, in this order: record the ruling, then reopen. The
  record is the accepted draft's own `status: accepted` plus a line in the run's
  research artefact naming each draft's outcome and, for a rejection, its
  reason. Only then does
  `cairn todo set brownfield-extraction-maintainer-ruling open` apply, because
  that command writes a status line and nothing else: a reopen without the
  record leaves the next iteration with a selectable todo and no ruling. This
  unit never reopens it: an iteration cannot produce the signature it waits on.
