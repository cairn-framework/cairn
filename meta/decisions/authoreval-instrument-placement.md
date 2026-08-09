---
id: dec.authoreval-instrument-placement
nodes:
  - cairn.authoreval
  - cairn.root
status: accepted
date: 2026-08-09
related:
  - dec.cli-agent-workflow-consolidation
affects:
  - src/authoreval
  - src/bin/cairn-authoreval.rs
  - harness/authoreval
---

# Authoreval instrument placement

## Context

`todo.blueprint-authorability-eval` measures whether models author valid
blueprint and artefact syntax. Its first child,
`todo.authorability-eval-instrument`, needs somewhere for the runner, the
scorer, the offline backend, and the prompt corpus to live.

The parent was anchored at `cairn.root` on the explicit basis that it writes
only research artefacts and unowned scaffolding. A declared surface re-opens
that anchor, and the child's `## Scope` makes settling it question 1.

Two constraints bound the answer. `dec.cli-agent-workflow-consolidation` records
that the CLI has grown to roughly 51 flat commands over about seven operation
families, and that collapsing surfaces rather than adding them is the
direction. The parent's own reuse constraint records that the oh-my-pi harness
owns model execution while cairn owns prompts, fixtures, production validation,
and deterministic scoring.

## Decision

The instrument is its own leaf Module, `cairn.authoreval`, claiming
`./src/authoreval`, `./src/bin/cairn-authoreval.rs`, and `./harness/authoreval`.
It ships as the separate binary `cairn-authoreval`, not as a `cairn`
subcommand. It declares no blueprint edge: it reaches every reconciled
subsystem, the scanner and the query spine included, only by invoking the
`cairn` binary as an external process. It does use the crate's shared
substrate, `CairnError` and the panic hook that `cairn.root` claims, exactly as
`cairn.lsp`, `cairn.kernel.cli`, and `cairn.brownfield` do; the blueprint edges
none of those to `cairn.root` either, and `cairn.root` carries no inbound or
outbound edge by design.

The two authorability todos stay anchored at `cairn.root`. Later work on the
instrument itself anchors at `cairn.authoreval`.

## Rationale

A `cairn` subcommand was rejected. The instrument is development tooling, not a
user surface, and `dec.cli-agent-workflow-consolidation` rules against widening
the shipped command set. `cairn-mcp` and `cairn-lsp` already establish the
separate-binary pattern for an auxiliary surface with its own module and its
own help text.

Folding the instrument into `cairn.root` was rejected. `cairn.root` is crate
entry points, shared error types, and verification; an eval harness is none of
those, and hiding a declared surface inside it would make the anchor claim
false rather than settle it.

Scoring through the `cairn` process rather than the library was chosen so the
instrument measures the shipped surfaces. Linking `query_api` would let a
scoring path drift from what `cairn scan --strict` actually does, which is the
one thing the measurement must not do. It also keeps the module edge-free.

Keeping the todos at `cairn.root` reflects what they still produce: a research
artefact and a prompt corpus, not owned source. Re-anchoring them would claim
ownership the remaining work does not exercise.

## Consequences

- A new module appears in the blueprint, with a contract at
  `meta/contracts/authoreval.md`.
- `harness/authoreval` is declared in `cairn.config.yaml` as an `assets`
  target, because a prompt corpus is data and carries no reconcilable language.
- The shipped `cairn` command set is unchanged.
- The instrument pays a process boundary per scored attempt. That is accepted:
  measuring the real gate is the point, and an eval run is not latency
  sensitive.
- `cairn-authoreval` becomes a fourth binary to keep building and linting,
  beside `cairn`, `cairn-mcp`, and `cairn-lsp`.
