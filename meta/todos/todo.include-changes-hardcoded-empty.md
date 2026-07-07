---
node: cairn.kernel.query
status: open
created: 2026-07-07
---

# Include Changes Hardcoded Empty

`--include-changes` still returns a hardcoded empty `active_changes` on the
neighbourhood surfaces: `src/query_api/handlers/graph.rs` `neighbourhood_json`
(`data["active_changes"] = json!([])`) and the human branch of
`src/cli/render/node.rs` `render_neighbourhood` ("Active changes:\nNone").
Sibling of the `cairn status` fix (todo.status-active-changes-bug): unlike
status, this needs `root` threaded into `neighbourhood_json`, and the list
must be node-scoped to match the neighbourhood's other fields (decisions,
todos, research), via `changes::operations_for_nodes(&changes, &node_ids)`,
not the full `changes::discover` list.

Also noted during the status fix: `render_status` and `render_node` carry
dead `parsed.json` branches (shared-json commands route `--json` to
`query_api` via `uses_shared_json`, `src/cli/mod.rs:307`), which still
hardcode `"active_changes":[]`. Fold their removal into the simplify-cli
cleanup family or delete alongside this fix.

Related gap (review finding, 2026-07-07): `changes::discover` hardcodes
`meta/changes`, so `cairn --changes-dir <dir> status` (and `changes`,
`show`) ignore the flag even though hook/health/remediate/archive respect
it. If threading `root` into `neighbourhood_json`, consider threading
`changes_dir` too, or into `discover` itself.
