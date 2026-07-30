---
node: cairn.root
status: open
created: 2026-07-16
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

OPEN. `todo.bootstrap-fixture-repair-or-delete` took the REPAIR verdict
(ratified 2026-07-29, PR #528 sheet W10) and the repair landed: the fixture
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
