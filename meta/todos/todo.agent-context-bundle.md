---
node: cairn.kernel.query
status: open
created: 2026-07-12
---

# Agent Context Bundle

## Priority

P0 investigation, inventory phase. Completable now, in parallel with
`todo.agent-guidance-baseline`. The measurement, scoring, and recommendation
move to `todo.agent-context-bundle-evaluation`, which waits for the baseline's
preregistered corpus.

## Problem

Agents currently compose task context from several Cairn queries. A single
read projection might be cheaper or more reliable, but `cairn bundle` already
covers much of the proposed shape. This unit maps the existing surface and
fixes the sample rule so the later evaluation has a trustworthy substrate; it
does not assume a new command.

## Scope (completable without the corpus)

- Inventory `context`, `get`, `neighbourhood`, `bundle`, `deps`, `rationale`,
  `locate`, `todos`, `sources`, and existing token-lean modes. Record overlap,
  missing facts, default versus opt-in detail, and the exact task decision each
  field serves.
- Define the reproducible sample rule: which nodes and tasks the evaluation
  will use, why they were selected, and how output is measured.
- List the candidate compositions of existing verbs and any hypothesised new
  projection to be scored later, with no scoring yet.

## Non-goals

- No RAG or full-text source search.
- No new stored state or second source of truth.
- No graph-root or commit fingerprint. `dec.graph-root-fingerprint` defers it.
- No router reference to an unimplemented surface.
- No measurement, scoring, or recommendation; that is the evaluation unit.

## Acceptance

- Extend `res.loop-efficiency-observations` or write linked research containing
  the surface inventory, the fixed sample rule, and the candidate list.
- Hand off to `todo.agent-context-bundle-evaluation` for measurement against
  the baseline corpus.

Informed by: res.loop-efficiency-observations
