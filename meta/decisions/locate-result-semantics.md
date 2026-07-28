---
id: dec.locate-result-semantics
nodes:
  - cairn.kernel.query
  - cairn.kernel.cli
  - cairn.mcp
status: superseded
date: 2026-07-17
informed_by: [res.codeatlas-analysis]
---

# cairn locate returns every exact match with its owning node id

## Decision

`cairn locate <symbol>` (CLI and MCP) is an exact-name reverse lookup over
the public SymbolRecords the reconciler already persists. Name collisions
return every match, each carrying its owning node id alongside file, line,
end line, kind, and signature. No fuzzy matching, no ranking beyond
returning all matches, and no separate extraction pass.

## Rationale

`res.codeatlas-analysis` (finding 1, verified) showed symbol-to-location is
the highest-leverage agent navigation query, and that CodeAtlas's index keeps
the first location under the unqualified name, filing later cross-file
collisions under qualified `file#name` keys, so a plain exact-name lookup
does not return all collisions as one result set. Carrying the owning
node id in every result is the cairn-native advantage: it lands the agent on
the node's contract and decisions, which a bare symbol index cannot do.

## Consequences

Future changes to locate must preserve all-matches collision semantics and
the node-id payload. A first-wins or file-keyed index is a rejected
alternative; cite this decision instead of re-proposing it. Shipped in
v0.4.0 via todo.symbol-locate-query.
