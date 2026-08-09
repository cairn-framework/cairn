---
node: cairn.root
status: blocked
created: 2026-08-09
blocked_by: [todo.authorability-eval-instrument]
parent: todo.blueprint-authorability-eval
---

# Authorability Eval Prompt Corpus

## Scope
Author the real authoring corpus, run it unattended against the instrument, and
publish what it showed.

- Five to ten prompts covering the parent's authoring family: a module claiming
  named files, a `blueprint.delta` for a refactor, a decision covering named
  nodes, and the artefact frontmatter forms the fixture already loads.
- Every prompt stays inside the fixture's loaded authority corpus (modules,
  contracts, decisions, todos, reviews), per the parent's substrate constraint.
- One unattended run against the real harness backend and a named model, never
  the instrument's offline smoke backend, producing the records the instrument
  defines.
- One research artefact under `meta/research/` reporting what the run showed.

## Parent constraints
The parent todo is `todo.blueprint-authorability-eval`. Its `## Authoring
family` section fixes the prompt shapes and the scoring surfaces:

> Run them against a temporary copy of `tests/fixtures/cairn-bootstrap`, apply the
> model output, and score with `cairn scan --strict` and `cairn lint --json`.
> Primary metric: iterations and tokens to a clean scan under the deterministic
> repair loop. Secondary metrics: first-shot validity and per-format failure
> hotspots.

Its third and fourth acceptance bullets bind this child:

> - Results identify whether failures belong to syntax, generated guidance, or
>   missing repair affordances.
> - No CI scheduling, issue filing, or dataset apparatus is added before the
>   instrument proves useful.

The authoring-corpus half of the parent's first acceptance bullet also belongs
here; the scorer half belongs to `todo.authorability-eval-instrument`.

The parent's scope correction also bounds this child. The navigation family
belongs to `todo.agent-guidance-baseline` (done), not here: this unit measures
authoring validity only.

## Dependencies
`todo.authorability-eval-instrument`, as a typed `blocked_by` edge. There is
nothing to run a corpus against until the runner and scorer exist. This todo is
`blocked` rather than open because `blocked` is the status that MISSION
precedence reports and exits on, while it performs no `blocked_by` check. The
instrument's last Acceptance bullet owns the transition that opens this todo.

## Acceptance
- Five to ten prompts exist, covering a module claiming named files, a
  `blueprint.delta`, a decision covering named nodes, and at least one further
  loaded artefact frontmatter form (a contract, a todo, or a review).
- Every prompt targets only the loaded authority kinds (modules, contracts,
  decisions, todos, reviews) and requests no `research` or `sources` pointer,
  asserted against the corpus itself rather than inferred from a scan.
- Each run starts from a fixture copy that scans clean, so a dirty start is
  never mistaken for a model failure.
- One unattended run over the whole corpus completes against the real harness
  backend and a named model and emits one record per prompt. The backend and
  model identity are carried in the records and repeated in the research
  artefact; a run scored against the offline smoke backend does not satisfy this
  unit.
- `meta/research/<slug>.md` reports the parent's metrics (iterations and tokens
  to a clean scan, first-shot validity, per-format hotspots) and the failures,
  and attributes each hotspot to syntax, generated guidance, or a missing
  repair affordance. A prompt that never reached a clean scan reports the
  iterations and tokens it spent under its outcome state, not a clean-scan
  figure it never earned.
- `cairn scan --strict` exits 0 on this repository after the artefact lands.
- No change directory is created: this unit adds prompts, one run, and one
  research artefact, no declared code or scripts surface. The instrument's own
  change was accepted and archived when that unit landed.

## Sizing
M. Prompts, one run, one research artefact. The run itself is unattended, so its
cost is not review surface.

## Non-goals
Do not change shipped guidance, the blueprint grammar, or any repair affordance
in response to the results. Naming the failure classes is this unit's output;
acting on them is a later unit with its own evidence.
