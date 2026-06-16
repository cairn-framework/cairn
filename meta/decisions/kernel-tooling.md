---
id: dec.kernel-tooling
nodes:
  - cairn.kernel.changes
  - cairn.kernel.hooks
  - cairn.kernel.query
  - cairn.kernel.cli
status: accepted
date: 2026-06-16
---

# Kernel tooling: changes, hooks, query API, and CLI

## Context

Beyond graph construction, cairn needs mechanisms to mutate and query the graph, gate commits, and expose everything to users.

## Decision

Provide four tooling modules under `cairn.kernel`:

- **Changes**: change directories, delta parsing, and archive acceptance.
- **Hooks**: commit and task-boundary gates (`structural`, `interface`, `tension`, `all`).
- **Query API**: structured query handlers used by the CLI, web UI, and MCP wrapper.
- **CLI**: primary user surface, command parsing, and output formatting.

## Rationale

Keeping these separate from the core graph pipeline lets the core stay focused on map integrity while tooling evolves independently. The query API is the stable boundary between cairn's internals and its surfaces.

## Consequences

- New commands start in `cairn.kernel.cli` and usually call into `cairn.kernel.query`.
- Hook changes affect `scripts/` and CI, so they need extra test coverage.
- The changes module is the only kernel submodule that writes to the working tree outside generated outputs.
