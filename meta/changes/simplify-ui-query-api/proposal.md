# Proposal: Web UI consumes query_api instead of reimplementing it

## Motivation

`todo.simplify-architecture` (wave 2) targets `src/ui`: `src/ui/server.rs`
calls `scanner::load_project` / `query::*` directly, and `src/ui/api.rs` +
`src/ui/serialise.rs` rebuild ~10 JSON shapes (`graph`, `node`, `depends`,
`dependents`, `lint`, `status`, `contract`, `symbols`, `artefacts`,
`rationale`) that already exist as `src/query_api` handlers. MCP
(`src/mcp/mod.rs:162`) already dispatches through `query_api::execute` and is
the template. This change makes `src/ui` a thin consumer of the spine.

## The fork (must be ratified before code)

Investigation (`explore` agent, 2026-07-08) found the UI and query_api wire
shapes **differ for every one of the 15 endpoints** (lowercase vs `Debug`
enums, raw vs structured artefacts, full records vs ID lists in `depends`).
`tests/wire_format_snapshots.rs` is **byte-identical for 14 of 15 endpoints**.
So the migration cannot be a drop-in. Two strategies:

- **B — adopt query_api wire.** Server returns `query_api` data verbatim.
  Satisfies the todo acceptance ("no JSON shape-building in src/ui") but
  requires (a) updating 14 byte-identical snapshots as a *recorded schema
  decision* and (b) rewriting `app.js` to consume query_api shapes. Large,
  risky (visual-harness `ux_defect_score` exposure).
- **A — server translates.** Server calls `query_api` and reshapes to the
  *existing* UI wire. Snapshots stay byte-identical, `app.js` untouched, but
  `src/ui` still performs a thin mapping (mild tension with the literal
  acceptance wording).

Three endpoints have **no** query_api handler and need new spine ops under
either strategy: `/api/meta` (CLI registry listing), `/api/blueprint` (raw
source), `/api/node/{n}/beads` (node-scoped beads).

## Scope (this change)

Scaffold the migration plan and the spine-op gaps so the strategy decision is
recorded and the work is decomposable into per-endpoint PRs. Code landing is
gated on the maintainer ratifying A vs B (see design.md).

## Out of scope until ratified

- The 14 snapshot updates and `app.js` rewrite (Strategy B only).
- Deleting `src/ui/api.rs` / `src/ui/serialise.rs` (only once no endpoint
  needs them).

## Acceptance (depends on ratified strategy)

- If B: 14 snapshots updated as a recorded schema decision; `app.js` consumes
  query_api shapes; `src/ui` does no shape-building; `ux_defect_score` 0.
- If A: 14 snapshots byte-identical; `app.js` unchanged; `src/ui` thin
  translation only; `ux_defect_score` 0.
