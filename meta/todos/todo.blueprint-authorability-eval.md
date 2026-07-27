---
node: cairn.root
status: blocked
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

`todo.example-corpus-scan-assertions`, so the fixture substrate is trustworthy.
This needs a change proposal because it adds a declared harness or scripts
surface.

## Status note

BLOCKED on `todo.example-corpus-scan-assertions` (node `cairn.tests`). That
prerequisite may repair or delete `tests/fixtures/cairn-bootstrap`, and the
authoring family above scores model output against a temporary copy of exactly
that fixture. If the prerequisite retains and repairs the fixture, reopen this
todo. If it deletes the fixture, revise the authoring family to name the
surviving corpus before reopening; the eval has no substrate otherwise.
