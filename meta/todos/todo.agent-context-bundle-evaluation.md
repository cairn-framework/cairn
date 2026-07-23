---
node: cairn.kernel.query
status: blocked
created: 2026-07-22
---

# Agent Context Bundle Evaluation

## Priority

P2 investigation. The measurement half of the context-bundle question,
separated so the inventory can complete now while scoring waits for real
tasks.

## Depends on

`todo.agent-guidance-baseline` (preregistered task corpus and ground truth)
and `todo.agent-context-bundle` (surface inventory, sample rule, candidate
list).

## Scope

- Derive required facts from the baseline's pinned development tasks and ground
  truth only.
- Score the candidate compositions of existing verbs and any hypothesised new
  projection for required-fact recall, precision, duplication, characters, and
  estimated tokens, using the fixed sample rule from the inventory unit.
- Prefer composition of existing verbs unless it misses a declared recall or
  precision threshold.

## Non-goals

- No RAG or full-text source search.
- No new stored state or second source of truth.
- No graph-root or commit fingerprint. `dec.graph-root-fingerprint` defers it.
- No confirmation prompt, ground truth, metadata, or candidate run. The
  treatment unit owns first access to the sealed split.

## Acceptance

- Extend `res.loop-efficiency-observations` or write linked research with the
  measurements, thresholds, and a recommendation from retrieval evidence, not
  output size alone.
- Preserve the sealed confirmation split unopened for treatment evaluation.
- Recommend existing composition or a specific new surface.
- If a new surface is justified, create a separate query implementation todo
  and require verified delivery before any guidance consumes it.
- Escalate to a decision artefact only if the public query surface changes.

Informed by: res.loop-efficiency-observations
