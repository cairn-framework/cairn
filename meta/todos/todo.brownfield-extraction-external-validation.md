---
node: cairn.brownfield
status: blocked
created: 2026-08-10
blocked_by:
  - todo.brownfield-onboard-decisions-index
  - todo.brownfield-extraction-authoring-reference
parent: todo.brownfield-extraction-flow
---

# Validate the extraction flow end to end and on an external repository

Validation unit split out of `todo.brownfield-extraction-flow` under the sizing
rule. It carries the two acceptance criteria of the parent that need a working
flow: the end-to-end drafted-artefact assertion and the external-repository run.
Blocked until both `todo.brownfield-onboard-decisions-index` and
`todo.brownfield-extraction-authoring-reference` land.

## Task

Exercise the drafting entry point end to end against a fixture repository that
contains ADR-like material: run the deterministic index, then
`cairn decision new <slug> --node <id> --informed-by <research-id>` with the
node the index resolved, and assert on the resulting artefact rather than on the
report. No fixture this unit adds is the dogfood repo.

Then run the flow against at least one real external repository and record that
run as provenance: a `meta/research/` artefact carrying the repository, the
commit, the exact commands, the evidence counts per source class, the node
bindings produced, and the unbound evidence, citing a `meta/sources/` artefact
with the `file:` and `verification:` fields the source schema requires
(`docs/conventions.md` section 10).

Verify the review handoff the shipped reference owns
(`todo.brownfield-extraction-authoring-reference`): the
`cairn onboard decisions` report that produced a draft is kept with it, and the
draft goes to the maintainer for a ruling. Nothing in the flow, and nothing in
this unit, sets `status: accepted` itself; acceptance is the maintainer's, per
`AGENTS.md` ("Put decisions to the maintainer in-session").

The parent closes only on an accepted draft. Its acceptance criterion, and the
grandparent's, require the flow to produce a decision the maintainer path
accepts, so a run whose every draft is rejected does not complete this unit:
record the rejections and what they say about the evidence boundary in the
research artefact, leave this todo open, and leave the parent blocked. A
rejection that indicts the deterministic index fires a revisit_trigger and is
reported as one.

Give a fired-or-not verdict on every entry in the `revisit_triggers` list in the
`dec.brownfield-extraction-mechanism` frontmatter, which that artefact names as
the sole source of reconsideration conditions; do not work from a paraphrase of
it. A fired trigger is recorded in the research artefact as a maintainer-facing
finding, not silently absorbed.

## Non-goals

No change to the command surface or the shipped reference. A fired
revisit_trigger is recorded and raises a follow-up todo rather than widening
this unit.

## Acceptance

- A test exercises the drafting entry point against a fixture repository with
  ADR-like material and asserts that the drafted decision artefact's `nodes:`
  binding names a node that exists in the fixture's blueprint and that its
  status is exactly `proposed`.
- The change's fixture list contains no dogfood-repo fixture, and the fixture
  carries no cairn artefact directory and no cairn-specific annotation beyond
  the `cairn.blueprint` the flow requires.
- A `meta/research/` artefact records the external-repository run with repository,
  commit, commands, evidence counts, bindings, and unbound evidence, and cites a
  `meta/sources/` artefact carrying `file:` and `verification:`.
- The research artefact retains every draft the external run produced together
  with the evidence report that produced it, and states, for every entry in the
  `revisit_triggers` frontmatter of `dec.brownfield-extraction-mechanism`,
  whether the run fired it.
- At least one draft from the external-repository run is accepted through the
  maintainer path, and no code path in the flow sets `status: accepted` itself.
  Every rejection is recorded in the research artefact.
- The accepted draft's retained evidence names at least one source path in the
  external repository, at the recorded commit, that carried no cairn-specific
  annotation before the run. The parent requires the accepted artefact itself to
  start from code the user never annotated for cairn, so an accepted draft
  derived from pre-existing cairn markers does not satisfy this unit.
- `cairn scan --strict` exits 0.
- On landing, set `todo.brownfield-extraction-flow` to `done` and
  `todo.brownfield-extraction-pointer` to `open`.
