---
node: cairn.kernel.query
status: open
created: 2026-07-11
---

# Query: node-overlap view for concurrent work

Multi-developer scenario (2026-07-11 discussion): when an agent or developer works on a node, it is relevant that another open change or todo touches that node or its dependency neighbourhood. Add an overlap query that answers "who else has in-flight work intersecting node X".

Scope:

- Input: a node ID. Output: open todos and active changes whose anchored nodes intersect the target node or its one-hop neighbourhood (dependencies and dependents).
- Baseline backend: committed state only (open todos and changes in git), stale by one push but zero-install. Design the query behind the existing StateBackend seam so a shared coordination backend can serve it live later.
- Surface as a CLI command (for example `cairn conflicts <node>`) or fold into `cairn neighbourhood`/`cairn status`; follow the command-reference consistency tests for any new command.

Acceptance: query returns the overlapping artefacts for a fixture with two open todos on adjacent nodes; `--json` supported; command reference tests pass.

## Coordination (added 2026-07-11 after backlog review)

Shares the StateBackend seam with `todo.unified-todo-write-surface` and
`todo.github-todo-sync`. Design that seam once and coherently across the three;
this is a soft ordering, not a hard blocker (this query reads committed state and
can ship on the file baseline first).

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It keeps query results honest when graph nodes overlap.

2026-08-07 audit (todo.roadmap-assumption-audit): re-scope against the shipped rung-3 wave composer and lease read surface before any build; the original framing predates the substrate.
