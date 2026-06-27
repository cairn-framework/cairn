---
node: cairn.mcp
---

# Contract: cairn.mcp

## Purpose

A minimal MCP (Model Context Protocol) stdio transport that exposes Cairn's
query API as JSON-RPC tools. It lets an orchestrator (OMP, an editor agent)
list and call Cairn queries over newline-delimited stdio without linking the
binary. Every tool call is forwarded to `query_api::execute`, so this layer is
a thin, stateless adapter: it parses arguments, runs the query, and wraps the
response envelope as MCP tool content.

## Public interface

- `ServerConfig`: root, blueprint_path, changes_dir, and allow_mutating_tools.
  `Default` is `.`, `cairn.blueprint`, `meta/changes`, and read-only.
- `serve_stdio(config)`: runs the loop over locked stdin/stdout.
- `serve(reader, writer, config)`: generic `BufRead`/`Write` loop, one request
  per line.
- `handle_line(line, config)`: dispatches one JSON-RPC request, returning the
  response `Value` for `initialize`, `tools/list`, and `tools/call`.
- `config_from_args(args)`: parses `--root`, `--file`, `--changes-dir`, and
  `--allow-mutating-tools`, returning a message on a missing value or unknown
  flag.

## Invariants

- Protocol version reported is `2024-11-05`; serverInfo names `cairn-mcp`.
- Mutating tools are hidden from `tools/list` and refused unless
  `allow_mutating_tools` is set; a per-call `mutating` argument only takes
  effect when the server itself allows mutation.
- Argument names accept aliases: `node` or `id`, `change` or `change_id`, and
  `status` or `kind`.
- Tool input schemas always set `additionalProperties: true`.
- Errors map to JSON-RPC codes (parse `-32700`, method-not-found `-32601`,
  invalid-params `-32602`, query failure `-32000`) and carry a `CAIRN_MCP_*`
  data payload.

## Dependencies

Outgoing blueprint edge: `cairn.mcp -> cairn.kernel.query`. The server is built
entirely on `query_api` (`visible_tools`, `execute`, `envelope_json`,
`error_json`) for tool discovery, dispatch, and response shaping.

## Tests

Unit tests live in `src/mcp/tests.rs` (wired as `#[cfg(test)] mod tests`),
covering argument parsing, the read-only versus mutating tool listing,
JSON-RPC dispatch for each method, and the error envelope shapes.
