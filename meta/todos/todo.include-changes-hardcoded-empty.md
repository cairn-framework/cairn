---
node: cairn.kernel.query
status: done
created: 2026-07-07
---

# Include Changes Hardcoded Empty

`--include-changes` returned a hardcoded empty `active_changes` on the
neighbourhood surfaces: `src/query_api/handlers/graph.rs` `neighbourhood_json`
and the human branch of `src/cli/render/node.rs` `render_neighbourhood`.

Fixed 2026-07-07: `root` threaded into both, list node-scoped via
`changes::operations_for_nodes(&changes, &node_ids)` to match the
neighbourhood's other fields (decisions, todos, research). The dead
`parsed.json` branch in `render_neighbourhood` was deleted (shared-json
commands route `--json` to `query_api` via `uses_shared_json`), taking the
now-unused `cli/format` `reviews_json` with it. `render_status` and
`render_node` still carry dead `parsed.json` branches; their removal stays
with the simplify-cli cleanup family.

The `--changes-dir` gap (`changes::discover` hardcodes `meta/changes`) is
split out to todo.changes-dir-flag-ignored.
