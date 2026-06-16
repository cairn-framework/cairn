---
id: dec.user-surfaces
nodes:
  - cairn.ui
  - cairn.mcp
status: accepted
date: 2026-06-16
---

# User surfaces: web UI and MCP wrapper

## Context

Not all consumers want a CLI. A read-only web graph explorer and an MCP tool wrapper make cairn accessible to agents and browser-based users.

## Decision

Provide two surfaces:

- **UI (`cairn.ui`)**: an embedded HTTP server serving a read-only graph explorer.
- **MCP (`cairn.mcp`)**: a Model Context Protocol server that exposes cairn queries as tools.

## Rationale

The web UI is useful for human review of the architecture map. The MCP wrapper lets agent harnesses call cairn without shelling out, reducing latency and surface area.

## Consequences

- Both surfaces must consume the same query API as the CLI to avoid semantic drift.
- UI assets live under `src/ui_assets/` and are served statically.
- MCP schema changes require updating `src/mcp.rs` and any dependent clients.
