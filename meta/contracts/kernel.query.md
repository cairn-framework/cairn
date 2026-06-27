---
node: cairn.kernel.query
---

# Contract: cairn.kernel.query

## Purpose

Shared structured query API behind both the CLI `--json` surface and the MCP
server. Given a project root and a `QueryRequest`, it loads the graph, validates
preconditions, dispatches to a domain handler, and returns a stable
`QueryResponse` carrying tool data plus project context, relevant rules, and
findings. It is the single source of truth for the query tool registry and the
JSON serialisation contract.

## Public interface

- `QueryRequest`: tool name plus optional `node`, `change`, `old_id`, `new_id`,
  `status`, `language`, a `BTreeSet<QueryFlag>`, and a `mutating` allow bit.
- `QueryFlag`: optional toggles (`Transitive`, `IncludeTodos`, `IncludeResearch`,
  `IncludeReviews`, `IncludeDeprecatedDecisions`, `IncludeChanges`, `Force`,
  `Edited`).
- `QueryResponse`: `project_context`, `rules`, `data` (`serde_json::Value`),
  and `findings`. `QueryError`: stable error with `Display`/`Error`.
- `ToolMetadata` and `SafetyClass` (`ReadOnly`/`Mutating`): registry metadata.
- `execute(root, blueprint_path, changes_dir, request)`: runs a query and
  composes context and rules.
- `registry()`, `visible_tools(allow_mutating)`, `envelope_json(response)`,
  `error_json(error)`, and `SCHEMA_VERSION` (currently 1).

## Invariants

- Every JSON `data` payload is stamped with `SCHEMA_VERSION` so CLI and MCP
  consumers branch on one contract.
- `requires_valid_map` gates commands that need a clean, error-free graph
  before execution; the canonical list lives in `serialise`.
- A mutating tool runs only when `request.mutating` is set; `visible_tools`
  hides mutating tools unless explicitly allowed.
- The registry has a fixed size (36 tools), asserted in tests.

## Dependencies

Outgoing blueprint edges: `cairn.kernel.scanner` (loads `ScanResult`),
`cairn.kernel.map` (graph and neighbourhood traversal), `cairn.kernel.changes`
(change-tool dispatch via `change_queries::dispatch_change_tool`), and
`cairn.kernel.hooks` (hook metadata serialisation). Handlers and serialisers
also read artefact-registry record types.

## Tests

Unit tests live in `src/query_api/tests.rs` (`#[cfg(test)] mod tests` wired in
`mod.rs`) covering execution and serialisation, plus a `#[cfg(test)]` module in
`registry.rs` asserting registry membership, safety classes, and size.
