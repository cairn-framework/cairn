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

### `/api/node/decisions` (FLIPPED)
- Now served via `execute("decisions", {node})`. The spine op returns the enriched
  canonical shape `{"node":"...","schema_version":1,"decisions":[{"id","status","nodes",
  "informed_by","supersedes","refines","related","path","title","date","revisited",
  "revisit_triggers","body"}]}`.
- Legacy shape was `{"node":"...","schema_version":1,"artefacts":[{"type":"decisions",
  "path","title","frontmatter","body"}]}` (built by `src/ui/api.rs`).
- Snapshot `wire_format_snapshots__api_node_app_api_decisions.snap` rebased to the
  new canonical shape.
- `app.js` `fetchNodeArtefacts` already reads the canonical shape (`response.decisions`)
  and falls back to the legacy `response.artefacts` for frozen fixtures.

### `/api/node/research` (FLIPPED)
- Now served via `execute("research", {node})`. The spine op returns the enriched
  canonical shape `{"node":"...","schema_version":1,"research":[{"id","nodes",
  "sources","date","path","title","body"}]}`.
- Legacy shape was `{"node":"...","schema_version":1,"artefacts":[{"type":"research",
  "path","title","frontmatter","body"}]}` (built by `src/ui/api.rs`).
- Snapshot `wire_format_snapshots__api_node_app_api_research.snap` rebased to the
  new canonical shape.
- `app.js` `fetchNodeArtefacts` already reads the canonical shape (`response.research`)
  and falls back to the legacy `response.artefacts` for frozen fixtures.

### `/api/node/sources` (FLIPPED)
- Now served via `execute("sources", {node})`. The spine op returns the enriched
  canonical shape `{"node":"...","schema_version":1,"sources":[{"id","file",
  "verification","type","date","path","title","body"}]}`.
- Legacy shape was `{"node":"...","schema_version":1,"artefacts":[{"type":"sources",
  "path","title","frontmatter","body"}]}` (built by `src/ui/api.rs`).
- Snapshot `wire_format_snapshots__api_node_app_api_sources.snap` rebased to the
  new canonical shape.
- `app.js` `fetchNodeArtefacts` already reads the canonical shape (`response.sources`)
  and falls back to the legacy `response.artefacts` for frozen fixtures.

### `/api/node/rationale` (FLIPPED)
- Now served via `execute("rationale", {node})`. The spine op returns the
  canonical shape `{"node":"...","schema_version":1,"decisions":[...],"research":[...],"sources":[...]}`
  using enriched `decision_enriched_json`, `research_enriched_json`, and
  `source_enriched_json` records.
- Legacy shape was `{"node":"...","schema_version":1,"artefacts":[{"type":"decisions"|"research"|"sources", "path","title","frontmatter","body"}]}`.
- Snapshot `wire_format_snapshots__api_node_app_api_rationale.snap` rebased to
  the new canonical shape.
- `app.js` `fetchNodeArtefacts` now supports the rationale canonical shape by
  merging `response.decisions`, `response.research`, and `response.sources`
  into a single artefact list while preserving the legacy `response.artefacts`
  fallback for frozen fixtures.

### `/api/node/contract` (FLIPPED)
- Now served via `execute("contract", {node})`. The spine op returns the
  canonical shape `{"node":"...","schema_version":1,"contract":"<body>",
  "contracts":[{"path","node","declared_by","title","body"}]}`.
- Legacy shape was `{"node":"...","schema_version":1,"artefacts":[{"type":"contract",
  "path","title","frontmatter":{"node":"..."},"body"}]}`.
- **Semantic delta**: legacy matching included any contract where
  `contract.node == node || contract.declared_by == node` across the whole
  contract set. Canonical matching only considers contracts explicitly listed in
  the node's `contracts` field and then requires `contract.node == node.id`.
  A contract declared-by but not attached to the node is no longer surfaced.
- Snapshot `wire_format_snapshots__api_node_app_api_contract.snap` rebased to
  the new canonical shape.
- `app.js` `fetchNodeArtefacts` already reads the canonical shape
  (`response.contracts`) and falls back to the legacy `response.artefacts` for
  frozen fixtures.

### `/api/depends/:id` and `/api/dependents/:id` (FLIPPED)
- Now served via `execute("deps", {node})`; the dependents route adds the
  `Inbound` flag. Wire: `{"node":"...","nodes":["id", ...]}` with bare node-ID
  strings, matching `cairn deps`.
- Legacy shape was `{"node":"...","nodes":[{"id","name","slug","state","kind"}]}`
  with entries hydrated server-side from the graph (and a synthetic
  `state:"synced"`/`kind:"module"` fallback for unknown IDs).
- Hydration moves to the client: `app.js` maps ID strings through its loaded
  `nodesById` graph index into `{id,name,state}` for `DependencyRow`; object
  entries (legacy shape, frozen harness fixtures) pass through unchanged.
- Unknown-node errors keep the legacy 404 contract via `Server::spine_data`
  (`CAIRN_QUERY_NODE_NOT_FOUND`).
- Snapshots `wire_format_snapshots__api_depends_app_api.snap` and
  `wire_format_snapshots__api_dependents_app_api.snap` rebased.
- Removes `api.rs::dependency_json` (last `Response`-returning builder in
  `api.rs`).

### `/api/node/:id` (FLIPPED)
- Now served via `execute("get", {node})` (no flags). Wire is the canonical
  `query_api` node record: enums move to Debug case (`kind:"Container"`,
  `state:"Synced"` instead of lowercase), and the record gains `owns_files`
  (bool) and `span` (`{file,line,column,end_line,end_column}`, where `file`
  is the blueprint's absolute path, same exposure as `/api/blueprint`'s
  `path`).
- Legacy shape was `api.rs::node_json` with lowercase enum names and no
  `owns_files`/`span`.
- `app.js` never fetches the bare node endpoint (the node panel is built from
  `/api/graph` data), so no client change and no visual-harness impact.
- Unknown-node errors keep the legacy 404 contract via `Server::spine_data`
  (`CAIRN_QUERY_NODE_NOT_FOUND`), with one canonical nuance: the `get` op
  falls back to `state::backlog::find` for ids that are not graph nodes, so
  a backlog id now returns 200 with a backlog-item detail payload where the
  legacy route returned 404. This matches the flipped `/symbols` route and
  `cairn get` CLI semantics.
- Snapshot `wire_format_snapshots__api_node_app_api.snap` rebased; the
  snapshot test normalises `span.file` to `<blueprint>` like the blueprint
  endpoint's `path`. `tests/graph_explorer.rs` pins the canonical markers
  (`owns_files`, `span`, `kind:"Container"`).
- `api.rs::node_json` remained only as a helper of `graph_json` until the
  `/api/graph` flip below removed both.

### `/api/graph` (FLIPPED)
- Now served via a new read-only spine op `graph` (`cairn_graph`, registry
  41 to 42) built on `query::graph`; `execute("graph")` with no node argument.
- Wire: `{"nodes":[...],"edges":[...]}`. Nodes move from the legacy trimmed
  record to the canonical `node_json` record: Debug-case `kind`/`state`
  (`Module`, `Synced`), plus `owns_files` and `span`. Edges keep
  `{from,to,kind,description}` but `kind` becomes Debug-case
  (`Ownership`/`Dependency`).
- `app.js` normalises once at the graph ingest boundary (`normaliseGraph`),
  lowercasing node `kind`/`state` and edge `kind`, tolerant of the frozen
  legacy lowercase fixtures the visual harness replays.
- Snapshot `api_graph` rebased (span `file` normalised like the node
  endpoint); `api_meta` rebased because `ui_meta` lists the new registry
  entry. Legacy `api.rs::graph_json`/`node_json` and
  `serialise::{kind_name,state_name,graph_edge_kind_name,string_array_json}`
  deleted.

## Pending flips (not yet rebased here)
The following endpoint still serves legacy `api.rs` shapes and is flipped in
a later per-endpoint step; it will record its wire delta here when rebased:
`status`.
