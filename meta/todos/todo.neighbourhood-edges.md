---
node: cairn.kernel.query
status: done
created: 2026-07-12
---

# Neighbourhood Edges

gh:#236

Human `cairn neighbourhood` hides inbound edges that the JSON output reports.

## Evidence (verified on main, 2026-07-12)
- `cairn neighbourhood cairn.kernel.query --json` emits
  `{"inbound":["cairn.kernel.cli","cairn.mcp"],...}` while the human output
  prints `Inbound: None`.
- `src/map/query.rs:426-428` (`neighbourhood_with_options`) clears
  `response.inbound` unless `--include-orphans` is passed; the default human
  renderer takes that path.

## Task
Show inbound/outbound edges in the default human output; `--include-orphans`
must not gate edge display. Add a regression test comparing human and JSON edge
visibility.

## Resolution (2026-07-16)

The default human renderer now calls `query::neighbourhood`, the same query
the JSON handler uses, so both surfaces always show the same inbound and
outbound edges (`src/cli/render/node.rs`).

Why inbound was cleared: phase 7.6 specified `include_orphans` on the premise
that the default neighbourhood skips inbound-only neighbours. It never did;
`query::neighbourhood` has always returned both directions, and the JSON path
kept doing so. `neighbourhood_with_options` implemented the premise by
clearing `response.inbound` by default, which only the human renderer hit.
The flag controlled nothing else, so there was no orphan-specific behaviour
to preserve: `neighbourhood_with_options` and the `--include-orphans` flag
are removed (function, help spec, help copy, flag token list). Passing the
retired flag is silently ignored, matching how the CLI treats unknown flags.
The canonical spec (docs/spec.md, neighbourhood query) already documents the
new behaviour: the node plus its inbound and outbound edges by default.

Regression tests: `test_neighbourhood_human_edges_match_json` in
`tests/phase_7_6_ai_provenance.rs` compares human and JSON edge visibility on
the bootstrap fixture; `neighbourhood_returns_inbound_and_outbound_edges` in
`src/map/query.rs` pins the query result;
`test_query_neighbourhood_reports_both_edge_directions` covers the library
surface. The kernel.map contract prose no longer lists the removed function.
