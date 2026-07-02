---
id: dec.generative-bundles-and-gaps
nodes:
  - cairn.kernel.query
status: accepted
date: 2026-07-02
informed_by: [res.vision-refactor-audit]
revisit_triggers:
  - "cairn bundle's composition drifts from cairn brief's composer after both evolve independently, and the duplication becomes a maintenance cost"
  - "The gap protocol's file-per-question convention proves too coarse once a node accumulates many concurrent gaps"
---

# Generative bundles and gaps: direction, not just checking

## Context

`res.vision-refactor-audit` finding 5: every existing query answers "does
reality match declared intent" (`get`, `neighbourhood`, `files`, `order`).
None composes everything an agent needs to *build* a declared-but-unbuilt
(ghost) node, and none gives an agent a structured way to say "I don't know
how to proceed" other than guessing or stalling silently.

## Decision

Two new surfaces:

1. **`cairn bundle <node>`** (query_api tool, read-only): composes node
   metadata, contract (including its `interface:` block once
   `dec.symbol-reality-layer`'s sibling contract work lands), decisions,
   rationale, and dependency interfaces (from `NodeRecord.symbols` of outbound
   `depends` targets) into one generation-ready response. Reuses existing
   composers (`query::get`, `contract_json`, the artefact handlers, the
   `BriefData` helpers) rather than re-implementing them.
2. **`cairn gap <node> --question "<text>"`** (CLI-only, mutating): when an
   agent hits genuine underspecification, it writes a `gap: true`,
   `status: proposed` decision artefact naming the question, instead of
   guessing or silently blocking. A new `CAIRN_GAP_UNRESOLVED` lint warning
   surfaces every open gap until a human resolves it (edits to
   `status: accepted`, filling `## Resolution`, or deletes it).

## Rationale

This is direction, not orchestration: `cairn bundle` returns everything an
agent needs to *write* code for a ghost node without cairn writing that code
or deciding to invoke anyone. `cairn gap` turns "the agent guessed" into "the
agent logged a decision-required question a human can see in `cairn lint`",
which is a *gate*, not a scheduler. Neither surface picks a task, spawns an
agent, or retries: `dec.no-orchestrator` is unaffected.

## Consequences

- `cairn bundle` is read-only (`SafetyClass::ReadOnly`); `cairn gap` is the
  only mutating surface this decision introduces, and it only ever creates one
  artefact type cairn already has (a decision), scoped by the existing
  provenance conventions.
- Depends on `dec.symbol-reality-layer` (dependency interfaces) and the
  `interface:` contract block (contract composition) landing first.
