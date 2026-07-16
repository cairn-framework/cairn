---
node: cairn.kernel.query
status: done
created: 2026-07-16
---

# Symbol Locate Query

Add a repo-wide symbol-to-location lookup: `cairn locate <symbol>` (CLI
plus MCP tool) returning `{node_id, file, line, end_line, kind,
signature}` matches. Built as a reverse index over the SymbolRecords the
reconciler already extracts and persists per node
(`src/reconcile/symbol.rs`, `src/scanner/snapshot.rs`); no new extraction
needed. Handle name collisions by returning all matches with their owning
node ids (richer than CodeAtlas's first-wins plus `file#name` keys).

Motivation: per `res.codeatlas-analysis` (finding 1, verified), cairn can
answer "what symbols does node X have" but not "where is symbol X
defined", which CodeAtlas's benchmarks show is the single
highest-leverage agent navigation query (50 to 60 percent fewer
exploration steps). Carrying the owning node id in results also lands the
agent on the node's contract and decisions, which a bare symbol index
cannot do.

Needs a change proposal before implementation (new CLI verb plus MCP
tool surface).

## Resolution (2026-07-16)

Implemented the exact-name `locate` query over reconciled SymbolRecords,
including all collision matches with owning node ids and source ranges.
Added the shared query-api registry surface, CLI human/JSON command, and MCP
`cairn_locate` tool with a symbol argument schema. Added production-path
location and collision tests, MCP contract coverage, and deliberately updated
the api metadata wire snapshot. Manual verification covered a real unique
symbol (`normalize_symbol`), a collision (`build`), and a no-match query;
all returned exit 0, with JSON no-match data containing an empty `matches`
array. The full workspace test suite, format, clippy, and strict scan gates pass.
