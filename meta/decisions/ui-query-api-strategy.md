---
id: dec.ui-query-api-strategy
nodes: [cairn.ui]
status: accepted
date: 2026-07-08
informed_by: [res.ui-query-api-wire-fork]
---

# Web UI adopts query_api canonical wire (Strategy B)

## Decision
For `todo.simplify-architecture` wave 2 unit `simplify-ui-query-api`, the webui
HTTP server (`src/ui/server.rs`) becomes a thin router that calls
`query_api::execute` and returns the data verbatim. `src/ui/api.rs` and
`src/ui/serialise.rs` are deleted. `src/ui_assets/app.js` and the
`src/ui/mod.rs` smoke test are rewritten to consume the canonical
(`query_api`) shapes. The 14 byte-identical wire snapshots are rebased as
recorded schema decisions under `meta/decisions/query-json-schema-version.md`.

## Rationale
- The accepted architectural invariant (dec.no-orchestrator,
  todo.simplify-architecture) is one canonical JSON shape in `src/query_api`,
  with every surface a thin consumer. MCP already dispatches through
  `query_api::execute`; the webui must do the same.
- The todo acceptance is literal: "src/ui contains no JSON shape-building for
  data query_api can serve." Only removing the UI builders satisfies it.
- Strategy A (server translates) and the hybrid (`format=ui`) both leave a
  second, independently-maintained wire contract in the server or spine, which
  drifts from the canonical shape until a later migration loop.

## Trade-off
Strategy B is a larger, riskier migration than A: it rewrites ~15 `app.js`
parse sites and rebases 14 snapshots (each a recorded schema decision), with
`ux_defect_score` regression exposure. It is sequenced into per-endpoint PRs
(see `meta/changes/simplify-ui-query-api/design.md` migration order) to keep
each landing mergeable. The three spine-op gaps (`meta`, `blueprint`, `beads`)
are additive and built first under either strategy.

## Status
Ratified 2026-07-08. Implementation deferred to a later dev-loop session;
this artefact records the decision so it survives the session boundary.
