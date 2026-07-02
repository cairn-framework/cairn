---
id: dec.frontier-query
nodes:
  - cairn.kernel.query
status: accepted
date: 2026-07-02
informed_by: [res.vision-refactor-audit]
revisit_triggers:
  - "NodeState grows a Planned variant (dec.native-task-state-and-agent-guidance ruling 1 gets implemented); frontier must be revisited to decide whether Planned nodes appear in ready/blocked"
---

# Frontier query: buildable-now, never run-now

## Context

`res.vision-refactor-audit` finding 5 (direction, not just checking) has a
second half beyond `dec.generative-bundles-and-gaps`: an orchestrator walking
the graph needs to know *what* is buildable right now and in what order,
without cairn deciding to build it.

## Decision

`cairn frontier` (query_api tool, read-only) returns:

- **ready**: every `Ghost` node whose outbound edge targets are all `Synced`,
  ordered by dependency tier (reusing `order()`'s tiering).
- **blocked**: remaining `Ghost` nodes, each naming its non-synced outbound
  targets (`blocking`).

This is a pure traversal query over the existing graph, computed the same way
`order()` already is. It ships on `Ghost` only; `NodeState::Planned` does not
exist yet (`dec.native-task-state-and-agent-guidance` ruling 1 is accepted but
unimplemented) and adding it is explicitly out of scope for this decision.

## Rationale

**This explicitly reaffirms `dec.no-orchestrator`, it does not reopen it.**
Cairn exposes the traversal; it does not run anything, pick a task, spawn an
agent, or retry. The three-layer architecture stands: `cairn frontier` is
Layer 2 (semantic lane, deterministic structural authority); an external
orchestrator (Gas City, an agent harness) is Layer 3 and decides what to do
with the ready list. Without this query, every orchestrator re-implements
"which ghost nodes have all their dependencies satisfied" by re-deriving graph
traversal from `get`/`depends` calls; `frontier` is that traversal computed
once, correctly, in the kernel that already owns the graph.

## Consequences

- Depends on `dec.symbol-reality-layer` only insofar as it reuses graph state
  built by phase 1's work; `frontier` itself does not consume `SymbolRecord`.
- `cairn.workspace` (`dec.workspace-aggregation`) calls `frontier` per member
  to build its aggregate view.
