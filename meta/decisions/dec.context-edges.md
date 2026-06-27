---
id: dec.context-edges
nodes:
  - cairn.kernel.query
  - cairn.kernel.cli
status: accepted
date: 2026-06-27
informed_by: []
---

# `cairn context` surfaces the labeled dependency graph

## Context

`cairn context` is the agent orientation surface ("structured project overview"),
yet it emitted only `edge_count` (a number) and never the edges themselves, in
both human and `--json` output. The human output was a flat per-node list
(`id (name) [state] path`) whose `path` duplicated the file tree. Bead cairn-6yh.

## Decision

Both surfaces now expose the full blueprint dependency graph with labels:

- `cairn context --json` gains an `edges` array of `{source, target, label}`
  objects (full node ids), alongside the retained `edge_count`.
- `cairn context` (human) replaces the flat node list with a `Structure:` block:
  every node listed once, its outbound edges indented beneath as
  `-> <target>  # <label>`. Node ids are shown with the system-root prefix
  stripped (e.g. `kernel.scanner`); the redundant `path` and the default
  `[Synced]` state are dropped (anomalous states still print). Full ids remain
  in `--json`.

## Rationale

The graph is the point of an architecture-map overview; emitting only a count
defeated the command's purpose. The human encoding was made leaner per element
(no paths, no names, no redundant `[Synced]`, stripped prefixes) so the view is
far more information-dense than before.

## Consequences and the token tradeoff

The bead's acceptance criteria asked for "all blueprint edges with labels" AND
"token cost not higher than today". On cairn's own graph these conflict: the old
output was ~1867 chars with zero edges; the leanest complete implementation is
~2157 chars (+15.5%), and the overage is the irreducible cost of 27 labeled
edges. Even the alternative of dropping the ~13 edgeless nodes still lands
~1987 chars, because the real blueprint labels are longer than the bead's
estimate assumed. No design satisfies both clauses.

The completeness-vs-budget call was escalated to the maintainer, who chose to
**ship the complete graph** and relax the token-neutrality target: hiding ~13 of
24 modules from an orientation surface would defeat it, and the +290 chars buys
the entire labeled dependency graph. Container-level rollup and `--depth`/
`--scope` flags for very large repos are tracked separately (cairn-mqy).
