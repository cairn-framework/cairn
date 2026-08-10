---
node: cairn.root
status: done
created: 2026-07-16
blocked_by: [todo.authorability-eval-instrument, todo.authorability-eval-prompt-corpus]
---

# Blueprint Authorability Eval

## Scope correction

This todo is authoring-only. The overlapping navigation family moved to
`todo.agent-guidance-baseline`, which owns the shared agent-evaluation harness
and Cairn-versus-search conditions.

## Problem

Cairn's blueprint syntax, blueprint.delta format, and artefact frontmatter are
increasingly agent-authored, with no measurement of whether models produce
them validly. This todo measures authoring convergence through production
validators rather than building a second navigation harness.

## Authoring family

Create five to ten prompts such as:

- add a module claiming named files;
- author a blueprint.delta for a refactor;
- write a decision covering named nodes.

Run them against a temporary copy of `tests/fixtures/cairn-bootstrap`, apply the
model output, and score with `cairn scan --strict` and `cairn lint --json`.
Primary metric: iterations and tokens to a clean scan under the deterministic
repair loop. Secondary metrics: first-shot validity and per-format failure
hotspots.

Reuse the agent-guidance evaluation runner or the summariser's
`LocalCommandBackend` pattern rather than building another orchestrator. The
oh-my-pi harness owns model execution. Cairn owns prompts, fixtures, production
validation, and deterministic scoring.

## Decomposition (2026-08-09)

Sizing: L under the repository sizing rule. One PR cannot carry a change
proposal, a runner and scorer, a prompt corpus, an unattended model run, and a
published result. The unit is partitioned into exactly two sub-todos, in
dependency order: `todo.authorability-eval-instrument`, then
`todo.authorability-eval-prompt-corpus`.

`todo.authorability-eval-instrument` is `done` as of 2026-08-09: the
instrument, its offline backend, its smoke prompt, and
`dec.authoreval-instrument-placement` landed together, and its change was
accepted and archived in the same commit. `todo.authorability-eval-prompt-corpus`
was `blocked` behind a typed `blocked_by` edge until then and reopened in that
commit. On 2026-08-10 it went `blocked` again behind a second prerequisite,
`todo.authoreval-lint-error-envelope` (node `cairn.authoreval`): an
unparseable blueprint aborted the whole authoreval run and discarded every
record, and this model misplaced `path` into the module header five times out
of five, so a corpus whose module prompt did not pre-teach the grammar could
not complete one unattended run. A run with the grammar pre-taught did
complete, and that route was refused because it coaches away the finding.
Evidence in `res.authoreval-corpus-first-run`.
That prerequisite is not a third child of this todo; it was an instrument fix
the corpus waited on. It landed on 2026-08-10: the scorer now scores an
unparseable answer as a `syntax` / `blueprint` hotspot instead of aborting, and
the prompt corpus is `open` again.

The change proposal this todo's `## Depends on` section requires is not a
separate child. It belongs to the instrument, which creates the change, ticks
its tasks, and runs `cairn change accept`; Land archives it with
`cairn change apply` in the same commit. A change cannot span two units,
because Land archives any change directory the landing unit carries, so the
change and the work that completes it stay together. The prompt corpus adds
prompts, a run, and a research artefact, no declared surface, so it needs no
change of its own.

Both children are `done` as of 2026-08-10, so this todo is `done`. The corpus
landed six prompts, one unattended run over the whole corpus against the real
harness backend at `anthropic/claude-sonnet-4-5`, and the published result in
`res.authoreval-corpus-baseline`. The Acceptance section below carries the
contract that was met, including the staged-delta exemption amended into it.

No implementation occurs in this decomposition, and the partition adds no
scope: the two children cover exactly this todo's `## Authoring family` and
`## Acceptance` sections and nothing outside them.

## Acceptance

- The authoring corpus and scorer run unattended on demand.
- The production parser, scanner, and lint surfaces grade outputs, except a
  staged `blueprint.delta`, which neither `scan --strict` nor `lint --json`
  validates. That one output is published as unmeasured rather than graded
  (amended 2026-08-10 against `res.authoreval-corpus-baseline` section 2).
- Results identify whether failures belong to syntax, generated guidance, or
  missing repair affordances.
- No CI scheduling, issue filing, or dataset apparatus is added before the
  instrument proves useful.

## Depends on

`todo.bootstrap-fixture-repair-or-delete`, so the fixture substrate is both
trustworthy and clean. This needs a change proposal because it adds a declared
harness or scripts surface.

## Status note

DONE as of 2026-08-10, when the second sub-todo landed. Both earlier gates are
discharged: `todo.bootstrap-fixture-repair-or-delete` took the REPAIR verdict
(ratified 2026-07-29, PR #528 sheet W10) and the repair landed. The fixture
scans clean and `tests/examples_gate.rs` asserts it stays that way
(`test_bootstrap_fixture_scans_clean`), so iterations to a clean scan was
measurable from zero.

Substrate constraint for prompt design: the fixture's evidence corpus
(`meta/sources/`, `meta/research/`) is deliberately unclaimed by its
blueprint (see the fixture header note and `res.bootstrap-fixture-repair`),
and prompts must preserve that. Verified failure mode: adding a `research`
pointer while `meta/sources/` stays unreached breaks the clean baseline
(`CAIRN_RESEARCH_MISSING_SOURCES` at Error, or
`CAIRN_RESEARCH_UNKNOWN_SOURCE` at Warning when the research cites the
unloaded sources). Keep eval prompts inside the loaded authority corpus
(modules, contracts, decisions, todos, reviews), with one exception amended
2026-08-10: the required `blueprint.delta` prompt authors into `meta/changes/`,
which the scanner does not load, so that prompt's record is published ungraded.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It keeps authorability coverage in the quality gate.

cairn.root anchor justified (2026-08-07): writes only research artefacts and unowned eval scaffolding; no owned source files.

2026-08-07 audit (todo.roadmap-assumption-audit): keep; now executable post rung-3, no stale assumptions found.
