---
node: cairn.ui
status: open
created: 2026-07-06
---

# Web UI Consumes query_api Instead of Reimplementing It

Part of todo.simplify-architecture (wave 2).
Depends on: todo.simplify-dedup-format-util.

`src/ui/server.rs:116-198` (`api`/`node_api`) calls
`scanner::load_project`, `query::graph`, `query::get`, `query::lint`
directly, and `src/ui/api.rs` + `src/ui/serialise.rs` rebuild JSON shapes
(graph, node, depends, dependents, lint, status, contract, symbols,
artefacts, rationale, beads) that already exist as `src/query_api`
handlers. MCP (`src/mcp/mod.rs:162`) shows the intended pattern: dispatch
through `query_api::execute`.

- Replace the endpoint bodies with `query_api::execute` calls mapping
  route params to tool args.
- Delete `src/ui/api.rs` and `src/ui/serialise.rs` once no endpoint
  needs them. If an endpoint needs a shape the canonical JSON lacks,
  enrich the query_api handler rather than keeping a UI-side builder.
- Preserve the UI's project-load caching by moving it below the execute
  boundary or into a shared cache layer; do not regress request latency.

Guards: `tests/wire_format_snapshots.rs` (byte-identical) and
`tests/visual_harness.rs` (all scores 0). If a wire snapshot must change,
that is a schema decision: record it against
`meta/decisions/webui-json-schema-version.md` conventions, not a silent
snapshot update.

Acceptance: both test suites green, `src/ui` contains no JSON
shape-building for data query_api can serve, `cairn ui` smoke-tested
against a real project.
