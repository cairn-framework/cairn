# Claude Code Integration

Cairn integrates with Claude Code through the MCP server (`cairn-mcp`) and
direct CLI invocation.

## Installation

Install both the main CLI and the MCP server:

```bash
cargo install --git https://github.com/George-RD/cairn.git
```

This installs `cairn`, `cairn-mcp`, and `cairn-lsp` into `~/.cargo/bin/`.

## MCP server configuration

Add `cairn-mcp` to your Claude Code MCP settings:

```json
{
  "mcpServers": {
    "cairn": {
      "command": "cairn-mcp",
      "args": ["--root", "."]
    }
  }
}
```

The MCP server exposes read-only query tools by default:

- `cairn_get` -- inspect a node by ID
- `cairn_neighbourhood` -- explore a node and its dependencies
- `cairn_lint` -- report findings
- `cairn_context` -- project overview

Pass `--allow-mutating-tools` to also expose `cairn_scan` and draft commands.

## CLI fallback

When MCP is unavailable, use shell commands directly:

```bash
cairn context              # project overview
cairn lint --json          # findings as JSON
cairn get <node>          # inspect a node
cairn neighbourhood <node> # node + neighbours
```

## Verification gate

Add `cairn scan --strict` to your agent verification workflow. It exits
non-zero on any Error or Warning finding, making it suitable for pre-commit
gates and CI pipelines.

## References

- `docs/mcp.md` -- full MCP tool list and response schema
- `docs/agent-prompts.md` -- example prompts for agent workflows
- `docs/quickstart.md` -- installation and first-run walkthrough
