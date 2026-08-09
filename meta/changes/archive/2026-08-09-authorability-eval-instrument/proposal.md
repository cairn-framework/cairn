# Proposal: authorability-eval-instrument

## Motivation

Cairn's blueprint grammar and artefact frontmatter are increasingly authored by
models, and nothing measures whether a model produces them validly. The parent
todo `todo.blueprint-authorability-eval` asks for that measurement, graded by the
production validators rather than by a second copy of the finding logic.

The measurement needs an instrument before it needs a corpus: something that
takes one prompt, drives a model through a bounded repair loop against a scratch
copy of `tests/fixtures/cairn-bootstrap`, scores each attempt with
`cairn scan --strict` and `cairn lint --json`, and emits exactly one record. The
prompt corpus (`todo.authorability-eval-prompt-corpus`) is blocked behind this
unit precisely because a corpus with no instrument measures nothing.

## Scope

- A declared module, `cairn.authoreval`, owning the instrument.
- A backend seam so an offline backend and a real harness-driven backend are
  interchangeable: one trait, one JSON request shape, one JSON response shape,
  a per-call timeout, and a fixed failure classification.
- A deterministic offline replay backend, so the whole path runs with no
  network, no API key, and no installed harness.
- A bounded deterministic repair loop.
- One record schema covering four outcomes, with the backend and model identity
  the run used.
- A failure taxonomy that attributes a hotspot to syntax, to generated guidance,
  or to a missing repair affordance.
- One smoke prompt and a `cairn-authoreval` binary that runs it end to end.

## Out of scope

- The authoring prompt corpus and any run against a real model. Those belong to
  `todo.authorability-eval-prompt-corpus`.
- Any CI job, schedule, issue filing, or dataset apparatus.
- Any change to the `cairn` CLI surface. The instrument ships as its own binary
  so the shipped command set is untouched.
