---
node: cairn.brownfield
status: open
created: 2026-08-07
parent: todo.brownfield-decision-extraction
---

# Rule which mechanism drafts decisions from an existing codebase

Decision unit split out of `todo.brownfield-decision-extraction` under the
sizing rule. Nothing user-visible ships here: this unit produces the ruling
that `todo.brownfield-extraction-flow` implements.

## Problem

The parent todo names the mechanism as an open fork: "a guided flow (prompt,
skill, or `cairn onboard` extension)". Those are three different products with
three different maintenance costs, and picking one silently in an
implementation PR would bury the load-bearing choice in a diff.

## Verified facts

1. Cairn performs no LLM inference of its own on this path.
   `cairn.summariser` is the only module that calls a model;
   `dec.build-and-extension` records that `cairn.suggested-edges` also handles
   LLM outputs, but as a triage queue for results produced elsewhere. The
   standing principle is that AI assists authoring while the reconciler stays
   deterministic (`docs/agent/principles.md`). Mining prose intent out of
   arbitrary code and ADR-like material is not a deterministic operation, so a
   pure-Rust `cairn onboard` extension cannot do the drafting part on its own.
2. Deterministic scaffolding surfaces already exist to build the other half on
   (`cairn decision new`, `cairn gap`, `src/brownfield/discovery.rs`); the
   ruling picks which, rather than inventing a new writer.
3. Tier follows shape. Adding a skill to the agent pack is binding-tier under
   `dec.decision-ratification-tiers` ("shipped pack content") and needs a
   maintainer signature; a CLI-shaped answer on `cairn.brownfield` alone may be
   local-tier.

## Task

Stress-test the three candidate mechanisms (shipped agent skill, new
`cairn`-side deterministic command, `cairn onboard` extension), including the
hybrid where cairn scaffolds and indexes candidate evidence while the harness
agent writes the prose. Persist the evidence as a `meta/research/` artefact and
the ruling as a decision artefact bound to `cairn.brownfield`.

## Acceptance

- A decision artefact records the chosen mechanism, its ratification tier, and
  what it rejects, informed by a research artefact carrying the comparison.
- The ruling names the concrete surfaces the implementation unit will touch
  (command name or skill path, and the artefact-writing entry point it reuses).
- `cairn scan --strict` exits 0.
