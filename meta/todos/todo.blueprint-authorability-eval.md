---
node: cairn.root
status: blocked
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

Only `todo.authorability-eval-instrument` is open. The prompt corpus is
`blocked` behind a typed `blocked_by` edge, because `blocked` is the status
MISSION precedence reports and exits on, while it performs no `blocked_by`
check. The instrument's Reconcile step flips `instrument` to `done` and then
`prompt-corpus` to `open`, staged for its Land, so both reach main in that
unit's single commit.

The change proposal this todo's `## Depends on` section requires is not a
separate child. It belongs to the instrument, which creates the change, ticks
its tasks, and runs `cairn change accept`; Land archives it with
`cairn change apply` in the same commit. A change cannot span two units,
because Land archives any change directory the landing unit carries, so the
change and the work that completes it stay together. The prompt corpus adds
prompts, a run, and a research artefact, no declared surface, so it needs no
change of its own.

This todo stays `blocked` until both children are `done`.

No implementation occurs in this decomposition, and the partition adds no
scope: the two children cover exactly this todo's `## Authoring family` and
`## Acceptance` sections and nothing outside them.

blocked on sub-todos: todo.authorability-eval-instrument, todo.authorability-eval-prompt-corpus

## Acceptance

- The authoring corpus and scorer run unattended on demand.
- The production parser, scanner, and lint surfaces grade outputs.
- Results identify whether failures belong to syntax, generated guidance, or
  missing repair affordances.
- No CI scheduling, issue filing, or dataset apparatus is added before the
  instrument proves useful.

## Depends on

`todo.bootstrap-fixture-repair-or-delete`, so the fixture substrate is both
trustworthy and clean. This needs a change proposal because it adds a declared
harness or scripts surface.

## Status note

BLOCKED on the two sub-todos above, since 2026-08-09. The earlier gate is
discharged: `todo.bootstrap-fixture-repair-or-delete` took the REPAIR verdict
(ratified 2026-07-29, PR #528 sheet W10) and the repair landed. The fixture
scans clean and `tests/examples_gate.rs` asserts it stays that way
(`test_bootstrap_fixture_scans_clean`), so iterations to a clean scan is
measurable from zero.

Substrate constraint for prompt design: the fixture's evidence corpus
(`meta/sources/`, `meta/research/`) is deliberately unclaimed by its
blueprint (see the fixture header note and `res.bootstrap-fixture-repair`),
and prompts must preserve that. Verified failure mode: adding a `research`
pointer while `meta/sources/` stays unreached breaks the clean baseline
(`CAIRN_RESEARCH_MISSING_SOURCES` at Error, or
`CAIRN_RESEARCH_UNKNOWN_SOURCE` at Warning when the research cites the
unloaded sources). Keep eval prompts inside the loaded authority corpus
(modules, contracts, decisions, todos, reviews).

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It keeps authorability coverage in the quality gate.

cairn.root anchor justified (2026-08-07): writes only research artefacts and unowned eval scaffolding; no owned source files.

2026-08-07 audit (todo.roadmap-assumption-audit): keep; now executable post rung-3, no stale assumptions found.
