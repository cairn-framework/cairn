# Cairn MCP Server

`cairn-mcp` exposes Cairn's structured query API over MCP stdio transport.

## Startup

```bash
cairn-mcp --root . --file cairn.blueprint
```

Options:

- `--root <path>` sets the project root. Defaults to `.`.
- `--file <path>` sets the blueprint path. Defaults to `cairn.blueprint`.
- `--changes-dir <path>` sets the active changes directory. Defaults to `meta/changes`.
- `--allow-mutating-tools` lists and permits mutating tools.

## Tool List

Default read-only tools:

- `cairn_get`
- `cairn_neighbourhood`
- `cairn_contract`
- `cairn_docstring`
- `cairn_files`
- `cairn_dependents`
- `cairn_depends`
- `cairn_order`
- `cairn_lint`
- `cairn_status`
- `cairn_rationale`
- `cairn_todos`
- `cairn_decisions`
- `cairn_research`
- `cairn_sources`
- `cairn_changes`
- `cairn_show_change`

Mutation-capable tools are hidden unless `--allow-mutating-tools` is passed:

- `cairn_scan`
- `cairn_archive`
- `cairn_rename`

Mutating tool calls must also pass `"mutating": true` in their arguments.

## Response Schema

Successful tool calls return a text content item containing JSON:

```json
{
  "project_context": "configured project context",
  "rules": {
    "decision": "artefact-specific rule text"
  },
  "data": {},
  "findings": []
}
```

`project_context` and `rules` are read from `cairn.config.yaml`. Missing config returns an empty string and empty object.

Errors are JSON-RPC errors whose `data` contains:

```json
{
  "code": "CAIRN_QUERY_NODE_NOT_FOUND",
  "message": "node `missing.node` was not found",
  "source_span": null,
  "remediation": null
}
```
