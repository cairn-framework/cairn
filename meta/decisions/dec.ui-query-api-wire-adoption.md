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

## Enriched artefact helpers (already landed)
The canonical artefact handlers now expose `path`, `title`, and `body` plus any
kind-specific fields so the webui can render the same cards without legacy
`src/ui/api.rs` builders. The helpers live in `src/query_api/serialise.rs` and
are unit-tested:

- `title_from_body(body, fallback)` — extracts the first level-one Markdown
  heading from an artefact body, falling back to the provided title.
- `relative_path(path, root)` — strips the project root prefix so JSON wires are
  stable and portable.
- `todo_enriched_json` — adds `path`, `title`, and `body` to `todo_json`.
- `decision_enriched_json` — adds `path`, `title`, `date`, `revisited`,
  `revisit_triggers`, and `body` to `decision_json`.
- `research_enriched_json` — adds `path`, `title`, and `body` to `research_json`.
- `source_enriched_json` — adds `path`, `title`, and `body` to `source_json`.

`src/query_api/handlers/artefacts.rs` wires these helpers into the response
builders for the `todos`, `decisions`, `research`, and `sources` spine op
responses; `src/query_api/handlers/node.rs` uses them for the `contract` and
`rationale` records (the `contract` record gained a `title` field, while
`rationale` uses the enriched source/research shapes for its referenced items).

## Recorded wire changes

### `/api/meta` (FLIPPED)
- Wire keys unchanged: `{"version":"...","available_commands":[{name,request,response,safety}]}`
  (the `ui_meta` spine op reproduces the legacy `meta_json` shape).
- Actual delta: `available_commands` gains three rows because the registry now
  carries the `ui_meta`, `blueprint`, and `beads` spine ops. Snapshot
  `wire_format_snapshots__api_meta.snap` rebased (+18 lines, the three rows).
- `app.js` has no `/api/meta` consumer; no UI change.

### `/api/blueprint` (FLIPPED, wire-compatible)
- Now served via `execute("blueprint")`. The spine op returns the same raw
  blueprint-file string the legacy `blueprint_json` produced, so the snapshot
  `wire_format_snapshots__api_blueprint.snap` required no delta.
- Legacy 404-on-read-failure preserved: the server maps a non-null `error`
  field from the spine op to HTTP 404 (`Server::spine_data` caller), matching
  the legacy `server.rs::blueprint_json` status behaviour.

## Wire versioning rule
`query_api::execute` stamps `schema_version` into its data payload; the UI
server's `json()` constructor also stamps the envelope. On flipped endpoints
the server strips the inner stamp (`Server::spine_data`) so the wire carries
exactly one `schema_version` key, owned by the server constant.

### `/api/beads` (FLIPPED)
- Now served via `execute("beads", {node})`. Wire: `{"node":"...","beads":[...]}`.
- Not covered by `wire_format_snapshots.rs` (no snapshot row), so no rebasing.
### `/api/lint` (FLIPPED, wire-identical)
- Now served via `execute("lint")`. The `query_api` `findings_json` produces
  the same field order and lowercase `severity` as the legacy `api.rs::lint_json`,
  so the snapshot `wire_format_snapshots__api_lint.snap` required no delta and
  `app.js` needs no change (it reads `lint.findings[].{code,severity,node,path}`).

### `/api/node/todos` (FLIPPED)
- Now served via `execute("todos", {node})`. The spine op returns the enriched
  canonical shape `{"node":"...","schema_version":1,"todos":[{"id","node","status",
  "created","satisfies","path","title","body"}]}`.
- Legacy shape was `{"node":"...","schema_version":1,"artefacts":[{"type":"todos",
  "path","title","frontmatter","body"}]}` (built by `src/ui/api.rs`).
- Snapshot `wire_format_snapshots__api_node_app_api_todos.snap` rebased to the
  new canonical shape.
- `app.js` `fetchNodeArtefacts` already reads the canonical shape (`response.todos`)
  and falls back to the legacy `response.artefacts` for frozen fixtures.

### `/api/node/symbols` (FLIPPED, wrapper change)
- Now served via `execute("get", {node, flags:[Symbols]})`. The canonical
  `node_json` embeds `symbols` inside the full node record, so the wire changes
  from `{"node":"...","symbols":[...]}` to `{"id":"...",...,"symbols":[...]}`.
- No `wire_format_snapshots` row exists for symbols, so no snapshot rebasing.
- `app.js` reads `response.symbols` (unchanged path), so no UI change is needed.
- `src/ui/mod.rs::test_ui_symbols_endpoint_returns_extracted_symbols` updated to
  assert `"id":"app.api"` instead of `"node":"app.api"`.

## Pending flips (not yet rebased here)
The following endpoints still serve legacy `api.rs` shapes and are flipped in
later per-endpoint steps; each will record its wire delta here when rebased:
`status`, `graph`, `node`, `node/contract`,
`node/decisions`, `node/research`, `node/sources`,
`node/rationale`, `depends`, `dependents`.
