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
- `cairn_health`
- `cairn_remediate`
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
- `cairn_init_from_code`
- `cairn_refine`
- `cairn_summarise`
- `cairn_draft_accept`
- `cairn_draft_edit`
- `cairn_draft_discard`

Mutating tool calls must also pass `"mutating": true` in their arguments. Some tools accept additional flags:

- `cairn_draft_accept` accepts `"edited": true` to apply the editable draft file instead of the generated text.

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

## MAS Orchestration Loop

Cairn can guide a multi-agent system (MAS) from a dirty state to a clean state using two tools:

1. **`cairn_health`** — Returns a single authoritative assessment: `clean` (boolean), summary counts, and detailed findings from both lint and hooks. Use this to determine whether work remains.
2. **`cairn_remediate`** — Returns an ordered list of recommended actions based on current findings. Each action includes a priority, a CLI command, and a description.

### Recommended agent loop

```text
1. Call `cairn_health` to assess current state.
2. If `clean` is true, stop.
3. Call `cairn_remediate` to get the next action.
4. Execute the highest-priority action (CLI command or file edit).
5. Re-run `cairn_health` to verify progress.
6. Repeat until `clean` is true.
```

### Example agent prompt fragment

> You are working in a codebase tracked by Cairn. Before making changes, run `cairn_health`. If the project is not clean, run `cairn_remediate` and execute the highest-priority action. After every edit, re-run `cairn_health` to confirm progress. Do not declare the task complete until `clean` is true.