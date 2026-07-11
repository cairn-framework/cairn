# Design: Web UI consumes query_api (strategy fork)

## Wire-shape mismatch (evidence)

| Endpoint | UI builder | query_api handler | Wire match |
|---|---|---|---|
| `/api/meta` | `meta_json` (api.rs:9) | none (CLI registry) | gap |
| `/api/status` | `status_json` (api.rs:80) | `project::status_json` (different shape) | no |
| `/api/graph` | `graph_json` (api.rs:18) | none (no full-graph dump) | gap |
| `/api/lint` | `lint_json` (api.rs:72) | `findings_json` (same data, query_api adds `schema_version`) | near |
| `/api/node/{n}` | `node_json` (api.rs:28) | `node_json` (adds `owns_files`, `span`; `Debug` case) | no |
| `.../contract` | `contract_response_json` (api.rs:93) | `node::contract_json` (structured) | no |
| `.../symbols` | `symbols_response_json` (api.rs:107) | `node::symbols_json` (serde lowercase) | near |
| `.../decisions|todos|research|sources` | `artefact_response_json` (api.rs:140) | `artefacts::*_response_json` (structured) | no |
| `.../beads` | `beads_response_json` (api.rs:199) | none | gap |
| `.../rationale` | `rationale_json` (api.rs:206) | `node::rationale_json` (structured) | no |
| `/api/depends|dependents/{n}` | `dependency_json` (api.rs:46) | `graph::dependency_json` (ID lists) | no |
| `/api/blueprint` | `blueprint_json` (server.rs:170) | none | gap |

14 of these are byte-identical-guarded in `tests/wire_format_snapshots.rs`
(all except `beads`/`symbols`).

## Strategy A — server translates

- UI server calls `query_api` handler fns directly (not `execute`, to avoid
  double `schema_version` stamping) and maps to the *existing* UI wire.
- Snapshots stay byte-identical; `app.js` untouched; `ux_defect_score` 0.
- New spine ops added: `beads`, `ui_meta`, `blueprint` (return UI-shaped
  data, or the server maps query_api-shaped data).
- Keeps the `Server` caching (`load_project()` RefCell cache, server.rs:30-32,
  188-210) — `query_api` does not cache, so the server must keep calling
  `load_project()` and pass the `ScanResult` in.
- Tension: `src/ui` still does a thin mapping. Acceptable as "thin consumer"
  interpretation, but does not satisfy the literal acceptance wording.

## Strategy B — adopt query_api wire (clean end state)

- Server returns `query_api::envelope_json(...).data` verbatim.
- 14 snapshots updated as a recorded schema decision
  (`meta/decisions/query-json-schema-version.md` convention).
- `app.js` rewritten to consume query_api shapes (TitleCase enums, structured
  artefacts, ID lists). Large, risky; needs incremental per-endpoint PRs and
  visual-harness re-baseline.
- Fully satisfies acceptance.

## Decision (ratified 2026-07-08)

Maintainer ratified **Strategy B** — adopt the `query_api` canonical wire.
The webui server becomes a thin router over `query_api::execute`;
`src/ui/api.rs` + `src/ui/serialise.rs` are deleted; `app.js` and
`src/ui/mod.rs` are rewritten to consume canonical shapes; the 14
byte-identical snapshots are rebased as recorded schema decisions. Rationale
and trade-off: see `meta/decisions/dec.ui-query-api-strategy.md` and
`meta/research/res.ui-query-api-wire-fork.md`.

**Implementation deferred**: the session is signing off; the migration is
sequenced per-endpoint (see Migration order below) and lands in a later
dev-loop session. The 3 spine-op gaps (`meta`, `blueprint`, `beads`) are
additive and built first.

## Migration order (per-endpoint, measured)

1. Add spine ops: `beads`, `ui_meta`, `blueprint`.
2. Flip near-matching endpoints first: `lint`, `symbols`.
3. Flip structured-artefact endpoints with server-side mapping:
   `decisions`, `todos`, `research`, `sources`, `rationale`, `contract`,
   `depends`, `dependents`.
4. Flip `node`, `status`, `graph` (largest shape gaps).
5. Delete `src/ui/api.rs` + `src/ui/serialise.rs` once nothing needs them.

## Guards

- `tests/wire_format_snapshots.rs`: byte-identical (A) or recorded schema
  decision (B) — never silent update.
- `node harness/eval.mjs`: `ux_defect_score` must stay 0 (no Rust gate wraps
  it; run manually).
- `biome check --error-on-warnings`, `check-design-tokens.sh`, `check-a11y.sh`
  (if app.js touched under B).
