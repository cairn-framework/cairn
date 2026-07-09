---
id: dec.ui-query-api-wire-adoption
nodes: [cairn.ui]
status: accepted
date: 2026-07-10
informed_by: [res.ui-query-api-wire-fork]
---

# Web UI wire adoption: recorded schema decisions

## Purpose
Per `dec.ui-query-api-strategy` (Strategy B), the webui server (`src/ui/server.rs`)
is a thin router over `query_api::execute` and returns the `data` payload verbatim.
Every endpoint whose wire shape changes is recorded here as a schema decision;
the `tests/wire_format_snapshots.rs` assertions are rebased (never silently
updated) alongside the flip.

## Spine ops added (registry)
Three read-only spine ops were added to `query_api::registry` so the server can
dispatch them through `execute_with_scan` without UI-side shape building:

- `ui_meta` (`cairn_ui_meta`) — replaces the legacy `api.rs::meta_json`.
- `blueprint` (`cairn_blueprint`) — replaces the legacy `server.rs::blueprint_json`.
- `beads` (`cairn_beads`) — replaces the legacy `api.rs::beads_response_json`.

These are intentional CLI-surface additions (the prior session chose to give the
spine ops proper CLI presence), so they are documented in `docs/commands.md`,
`docs/integration-contract.md`, and wired in `src/cli/mod.rs` (`uses_shared_json`).

## Recorded wire changes

### `/api/meta` (FLIPPED)
- Old wire (`meta_json`): `{"version": "...", "available_commands":[{name,request,response,safety}]}`.
- New wire (`ui_meta`): `{"commands":[{name,request,response,safety}], "schema_version":1, "version":"..."}`.
- Snapshot `wire_format_snapshots__api_meta.snap` rebased to the canonical shape.
- `app.js` consumers of `/api/meta` must read `commands` (array of objects) instead of `available_commands`.

### `/api/blueprint` (FLIPPED, wire-compatible)
- Now served via `execute("blueprint")`. The spine op returns the same raw
  blueprint-file string the legacy `blueprint_json` produced, so the snapshot
  `wire_format_snapshots__api_blueprint.snap` required no delta.

### `/api/beads` (FLIPPED)
- Now served via `execute("beads", {node})`. Wire: `{"node":"...","beads":[...]}`.
- Not covered by `wire_format_snapshots.rs` (no snapshot row), so no rebasing.
### `/api/lint` (FLIPPED, wire-identical)
- Now served via `execute("lint")`. The `query_api` `findings_json` produces
  `{"findings":[{code,severity,message,node,path}],"schema_version":1}` with the
  same field order and lowercase `severity` as the legacy `api.rs::lint_json`,
  so the snapshot `wire_format_snapshots__api_lint.snap` required no delta and
  `app.js` needs no change (it reads `lint.findings[].{code,severity,node,path}`).

## Pending flips (not yet rebased here)
The following endpoints still serve legacy `api.rs` shapes and are flipped in
later per-endpoint steps; each will record its wire delta here when rebased:
`status`, `graph`, `node`, `node/contract`, `node/symbols`,
`node/decisions`, `node/todos`, `node/research`, `node/sources`,
`node/rationale`, `depends`, `dependents`.
