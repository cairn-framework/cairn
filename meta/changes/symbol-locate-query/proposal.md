# Proposal: symbol-locate-query

## Motivation

Per res.codeatlas-analysis (finding 1, verified), cairn can answer "what
symbols does node X have" but not "where is symbol X defined". CodeAtlas's
benchmarks show that the reverse lookup is the single highest-leverage
agent navigation query: 50 to 60 percent fewer exploration steps versus
grep-and-guess. Cairn already extracts and persists per-node, public-only
SymbolRecords (src/reconcile/symbol.rs, src/scanner/snapshot.rs, and the
live NodeRecord.symbols field in src/map/graph.rs); nothing new needs
extracting. Carrying the owning node id in the result also lands the agent
on that node's contract and decisions, which a bare symbol index (as in
CodeAtlas) cannot do.

## Scope

- A new `locate` query-api tool: `cairn locate <symbol>` (CLI, human and
  --json) plus the equivalent MCP tool (`cairn_locate`), returning every
  exact-name match among public symbols as `{node_id, file, line, end_line,
  kind, signature}`.
- A reverse index built at query time over the already-reconciled graph's
  `NodeRecord.symbols` (no persistence format change; map.json already
  carries `SnapshotNode.symbols`).
- Name collisions (the same identifier declared in more than one node)
  return every match with its owning node id; no ranking, no first-wins.
- Zero matches is a clean, successful result: a no-matches message for
  humans, an empty `matches` array for --json/MCP, exit code 0.

## Out of scope

- No new symbol extraction or language support; this reuses the existing
  reconciler-populated, public-symbols-only SymbolRecords verbatim. A
  private/unexported symbol with the same name is not indexed and never
  matches.
- No fuzzy, substring, or ranked search; exact name match only.
- No persistence or wire-format change to map.json; the reverse index is
  built in memory from the already-persisted per-node symbol lists.
