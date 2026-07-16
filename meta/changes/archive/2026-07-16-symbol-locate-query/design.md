# Design: symbol-locate-query

## Approach

Walk every node's already-extracted, public-only `SymbolRecord`s
(`node.symbols: Vec<SymbolRecord>` on `NodeRecord` in `src/map/graph.rs`,
populated by the four language reconcilers via `src/reconcile/symbol.rs`,
which only ever record exported/public identifiers). At query time, visit
every node in the reconciled `Graph`
(`scan_result.graph.nodes: BTreeMap<String, NodeRecord>`, deterministic
key order), filter each node's symbol list for an exact `name` match, and
collect `{node_id, file, line, end_line, kind, signature}` per match. This
is a direct linear scan, not a persisted or cached index: the walk is
O(total public symbols) per query, matching the cost of every other
whole-graph query tool (`islands`, `frontier`, `graph`). A private or
unexported symbol sharing the queried name is invisible to `locate`,
because the reconcilers never populate `SymbolRecord` for it in the first
place.

Collisions (two or more nodes declaring the same symbol name) return every
match, each carrying its own `node_id`, rather than CodeAtlas's
first-wins-plus-`file#name`-key scheme. The caller disambiguates using the
node id, which then supports `cairn get <node_id>` /
`cairn contract <node_id>` for the richer context a bare symbol index
cannot offer.

Three surfaces are added, all backed by the one query-api tool
(`cairn`'s existing single-registry pattern: one `ToolMetadata` entry
drives the CLI subcommand, the `--json` wire shape, and the MCP tool
declaration):

- **query_api handler**: `src/query_api/handlers/locate.rs::locate_json`,
  registered in `src/query_api/registry.rs` as tool `"locate"` /
  `"cairn_locate"`, dispatched from `execute_data_with_scan` in
  `src/query_api/mod.rs`.
- **CLI**: `cairn locate <symbol>` subcommand. Human rendering lives in the
  new `src/cli/render/locate.rs::render_locate`, which reuses the
  `ScanResult` `render_loaded_project_command` already loaded (via
  `query_api::execute_with_scan`, the same scan-reuse path `render_get` /
  `render_neighbourhood` / `render_bundle` use) rather than re-scanning
  the project a second time. `--json` output is intercepted earlier by
  the existing generic shared-JSON path (`uses_shared_json` +
  `commands::shared_request`), which is taught to populate the new
  `symbol` field from the positional token when `command == "locate"`.
- **MCP**: no new tool-declaration mechanism is needed; cairn's MCP server
  derives its tool list from the same `query_api::registry()`
  (`src/mcp/mod.rs::tools_json`). Adding the registry entry alone exposes
  `cairn_locate`; the only MCP-specific work is teaching
  `request_from_arguments` to read a `"symbol"` argument and teaching
  `input_schema` a `"LocateRequest"` case.

### `QueryRequest.symbol`, not `QueryRequest.node`

The obvious shortcut is to smuggle the symbol name through the existing
`QueryRequest.node` field (already overloaded as "the positional
identifier" by several tools). This was rejected: `src/mcp/mod.rs`'s
`request_from_arguments` maps the wire argument named `node` (or `id`)
into that field, so an MCP caller would have to pass a symbol name under
an argument literally called `node`, which is the wrong public contract
for a symbol-lookup tool and would read as a bug to any consumer of the
MCP tool schema. `QueryRequest` gains a new `pub symbol: Option<String>`
field instead; MCP maps a `"symbol"` argument onto it and the CLI
generic-dispatch path populates it from the positional token only for the
`locate` command, leaving `node` (and every other tool's contract)
untouched.

### Response shape

`execute()` (`src/query_api/mod.rs`) asserts every `execute_data` arm
returns a JSON object, because it stamps `schema_version` onto it as a
sibling key. `locate_json` therefore returns
`{"symbol": <queried name>, "matches": [...]}`, not a bare top-level
array: this mirrors the existing `order` tool's `{"nodes": [...]}` shape.
"Empty array for --json" (zero-match acceptance criterion) means
`matches` is `[]`, not that the whole response degenerates into an array,
which the shared envelope contract does not allow.

## Changes

ADDED:
- `src/query_api/handlers/locate.rs` (`locate_json`, direct scan of every
  node's public `SymbolRecord`s for an exact name match).
- `src/cli/render/locate.rs` (`render_locate`, human rendering).
- `QueryRequest.symbol: Option<String>` field.
- `"locate"` / `"cairn_locate"` registry entry (`LocateRequest` /
  `LocateResponse` schema names).
- `format::symbol_arg` CLI helper (mirrors `format::node_arg`, but with a
  symbol-specific usage message sourced from `copy.toml`).
- `copy.toml` entries: `[locate]` usage/missing-symbol,
  `[locate] no-matches and missing-symbol keys`, `[help.commands.locate]`.

MODIFIED:
- `src/query_api/mod.rs`: import `locate_json`; dispatch `"locate"` in
  `execute_data_with_scan`.
- `src/query_api/registry.rs`: new tool entry (description: public-symbol
  lookup); `TOOL_REGISTRY` size 42 -> 43.
- `src/query_api/serialise.rs`: `requires_valid_map` gains `"locate"` so
  an invalid graph blocks it like `get`/`files`/`contract`.
- `src/query_api/handlers/mod.rs`: `mod locate;` + re-export.
- `src/mcp/mod.rs`: `request_from_arguments` reads `"symbol"`;
  `input_schema` gains a `"LocateRequest"` case.
- `src/cli/mod.rs`: `uses_shared_json` includes `"locate"`;
  `render_loaded_project_command` dispatches `"locate"` to
  `render_locate`.
- `src/cli/commands/mod.rs`: `shared_request` populates `symbol` for the
  `locate` command.
- `src/cli/render/mod.rs`: `mod locate;` + re-export.
- `src/cli/help/mod.rs`: `COMMAND_HELP` gains a `locate` entry.
- `src/cli/format/util.rs`: new `symbol_arg` helper.
- Every existing `QueryRequest { .. }` struct literal that does not use
  `..Default::default()` gains an explicit `symbol: None,` field (a
  mechanical consequence of adding a non-defaulted-away field to a struct
  built with full literals in ~11 call sites).
- `tests/snapshots/wire_format_snapshots__api_meta.snap`: gains the
  `locate` entry in `available_commands` (registry order).

REMOVED:
- Nothing.

RENAMED:
- Nothing.
