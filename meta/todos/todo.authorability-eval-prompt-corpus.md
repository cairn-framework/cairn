---
node: cairn.root
status: blocked
created: 2026-08-09
blocked_by: [todo.authorability-eval-instrument, todo.authoreval-lint-error-envelope]
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
`todo.authorability-eval-instrument`, as a typed `blocked_by` edge: there was
nothing to run a corpus against until the runner and scorer existed. That
dependency is discharged. The instrument landed on 2026-08-09 and flipped this
todo to `open` in its own single commit, which is why the `blocked_by` edge
above is now satisfied rather than pending.

What it delivers, and what this unit therefore builds on: `cairn-authoreval run
<prompt.json>...`, a prompt schema carrying `id`, `instruction`, the `expects`
paths an answer must author, and an optional offline `replay` script, and one
JSON Lines record per prompt. Real runs pass `--backend command --command
<program> --model <name>`; the offline replay backend does not satisfy this
unit's acceptance.

`todo.authoreval-lint-error-envelope`, on node `cairn.authoreval`, added
2026-08-10 as a second typed `blocked_by` edge. It is a prerequisite, not a
child of this todo, so completing it does not complete this one.

The precise blocking claim, because a weaker one is false. A corpus whose
module prompt does not pre-teach where a `path` declaration goes cannot
complete one unattended run: this model misplaced `path` into the module header
five times out of five, an unparseable blueprint aborts the invocation, and the
abort discards the records of every prompt that already succeeded. A corpus
whose module prompt does teach it completes today, measured. That route is
refused, not unavailable: 5 of 5 is the parent's headline finding for the
blueprint format, and a prompt that hands over the grammar to keep the harness
alive reports a first-shot number that is an artefact of the coaching. The
prerequisite buys the honest version of both, since a recorded parse failure is
a scored blueprint syntax hotspot. Evidence and options are in that todo,
measurements in `res.authoreval-corpus-first-run` section 1.

## Prior attempt, 2026-08-10

A first attempt built the corpus and ran it. Nothing from it landed. Read
`res.authoreval-corpus-first-run` before rebuilding the corpus: all six
intended prompt shapes are verified satisfiable, and two of them need more than
the obvious file.

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
- The `blueprint.delta` prompt's record is reported as unmeasured and excluded
  from first-shot validity and from any per-format hotspot count. Amended
  2026-08-10 against `res.authoreval-corpus-first-run` section 2: the scorer
  runs `scan --strict` and `lint --json`, neither of which validates a delta, so
  a delta naming a node that does not exist scores identically to a correct one.
  A validator exists behind `cairn change apply`, which the scorer never
  invokes. Reaching it means changing shipped scoring behaviour, which this
  unit's non-goals forbid, so the prompt stays in the corpus (the parent's
  family names it) and its number stays out of the metrics.
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
