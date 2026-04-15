# Design: Phase 7 MCP

## References

- `docs/spec.md` section 12 for query interface and context/rules composition.
- `docs/spec.md` section 14 for MCP wrapper phase.

## Server Shape

The project SHALL add a second binary target named `cairn-mcp`. It SHALL use stdio transport and expose tools that call the same library services used by the CLI.

The server SHALL not parse CLI text. Shared query request and response structs SHALL live in the library.

## Tool Set

The MCP server SHALL expose tools for:

- `cairn_get`
- `cairn_neighbourhood`
- `cairn_contract`
- `cairn_files`
- `cairn_dependents`
- `cairn_depends`
- `cairn_order`
- `cairn_lint`
- `cairn_scan`
- `cairn_status`
- `cairn_rationale`
- `cairn_todos`
- `cairn_decisions`
- `cairn_research`
- `cairn_sources`
- `cairn_changes`
- `cairn_show_change`

Archive and rename tools SHALL require an explicit mutating flag in the request and SHALL be omitted from default read-only tool listings unless the server starts with a mutating-tools configuration flag.

## Context and Rules Composition

The server SHALL read `cairn.config.yaml`. Responses SHALL include:

- `project_context`: the configured context block.
- `rules`: rules relevant to the query or artefact type.
- `data`: the structured query result.
- `findings`: relevant findings.

If no config exists, the context and rules fields SHALL be empty strings or empty arrays.

## Error Model

MCP errors SHALL wrap the same stable error codes used by CLI JSON output. Errors SHALL include code, message, optional source span, and optional remediation text.

## Testing

Tests SHALL cover tool registration, stdio request handling, shared query invocation, context/rule composition, read-only versus mutating tool exposure, and error mapping.
