---
node: cairn.kernel.query
status: open
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
