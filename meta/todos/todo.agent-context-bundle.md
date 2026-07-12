---
node: cairn.kernel.query
status: open
created: 2026-07-12
---

# Agent Context Bundle

## Problem
When an agent (or subagent) starts implementing a todo, it assembles its
working context by hand: read the todo file, then separately query the
node's surfaces (`cairn neighbourhood`, `rationale`, `deps`; sometimes
`sources`, `todos`). The 2026-07-12 loop session used those separate
surfaces per unit (res.loop-efficiency-observations); that a single
composed bundle would be cheaper and better for briefing subagents is a
workflow hypothesis this todo tests, not an established fact. The bundle
idea: one token-budgeted command returning what an implementer needs.

## Approach (investigation first)
1. Inventory what already exists before proposing anything new: `cairn
   context` (has `--depth` and `--scope <node>`), `neighbourhood`,
   `rationale`, `deps`, `todos`, `sources` - what each emits, where they
   overlap, and what a per-todo composition is missing.
2. Only then evaluate a bundle surface (flag on an existing verb vs new
   verb): relations, dependencies, accepted decisions, contracts, and code
   targets for the todo's node(s), with an explicit token budget and lean
   rendering (relates to todo.output-token-efficiency).
3. Temporal context (per-node revision counts / version markers derived
   from git history and the archive trail) stays a recorded hypothesis in
   res.loop-efficiency-observations; build nothing there unless the
   inventory surfaces a concrete need.

## Non-goals
- No RAG or full-text search over source contents (dec.cairn-identity).
- No new stored state: the bundle is a read-projection of existing graph
  and artefact data.

## Acceptance
- A written inventory of existing context surfaces and their gaps (extend
  res.loop-efficiency-observations or a linked research artefact).
- A concrete recommendation: compose existing verbs vs a bundle surface,
  justified with measured output sizes (not just estimates). The
  investigation defines and documents its own deterministic sample rule
  and size metric (which nodes, why, and how measured) so the numbers are
  reproducible; escalate to a decision artefact if the CLI surface
  changes.

Informed by: res.loop-efficiency-observations
