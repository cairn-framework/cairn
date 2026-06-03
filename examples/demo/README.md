# Cairn Demo Project

A minimal task-manager API demonstrating Cairn's blueprint structure,
artefacts, changes, and reconciler observations.

## Structure

- `cairn.blueprint` — the declarative system map
- `src/` — source code for each module (exercises code reconciler)
- `meta/` — artefacts:
  - `contracts/` — interface contracts per node
  - `todos/` — open tasks
  - `decisions/` — design decisions
  - `sources/` — external references
  - `research/` — background research
- `meta/changes/` — pending change directories

## Commands

```bash
# Inspect findings (non-blocking)
cairn check

# Run lint gate (blocking on errors)
cairn lint

# View a node
cairn get tasks.api

# List dependencies
cairn depends tasks.api

# Start the web UI
cairn ui

# Run the MCP server
cairn mcp
```

## Capabilities demonstrated

- **Blueprint parse**: `cairn.blueprint` declares a System with Modules, paths, contracts, and edges.
- **Artefacts**: contracts, todos, decisions, sources, and research are loaded and validated.
- **Changes**: the `meta/changes/demo-change/` directory shows a pending change with proposal, design, and tasks.
- **Reconciler**: source files in `src/` are reconciled against blueprint node paths.
- **Hooks**: run `cairn lint` while a change is active to trigger hook checks.
- **MCP**: run `cairn mcp` to expose the map as an MCP server.
- **Summariser**: configured as disabled by default; set `[summariser] enabled = true` to enable.
