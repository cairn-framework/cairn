# Proposal: agent-context-bundle

## Motivation

Agents compose implementation context from overlapping Cairn queries. Before
changing guidance or adding another projection, the programme needs an exact
inventory and a fixed evaluation substrate that cannot be tuned after results
are visible.

## Scope

- Inventory the current node-context query surface, including defaults,
  opt-ins, overlap, missing facts, and the task decision each field serves.
- Freeze a deterministic sample selector and output-accounting protocol against
  the baseline corpus.
- Register runnable compositions of existing verbs and one paper-only
  hypothetical projection for later scoring.
- Hand measurement and recommendation to
  `todo.agent-context-bundle-evaluation`.

## Out of scope

- Measuring, scoring, setting thresholds, or recommending a candidate.
- Adding or changing a public query, router, MCP tool, or stored state.
- RAG, full-text search, graph-root fingerprints, or opening the sealed
  confirmation corpus.
