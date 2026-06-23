# Session Handoff: 2026-06-23 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main`.

## What Was Done

One iteration landed and merged this session. The backlog and `cairn lint` were
both empty/clean at orient, so the unit was drawn from phase-10 priority 4: the
self-contained improvement documented as the prior handoff's next candidate (the
webui `/api/*` `schema_version` inconsistency). It was flagged a maintainer
decision; the maintainer chose "standardize the webui surface".

- **Uniform `schema_version` across the webui `/api/*` surface**
  (PR #155, squash `e46e2af`). Resolved bead `cairn-bdj`.
  - The webui versioned only `/api/meta` and `/api/status`; `/api/graph`,
    `/api/lint`, `/api/blueprint`, `/api/node/*` (and its `contract`,
    `decisions`, `todos`, `research`, `sources`, `beads`, `rationale` suffixes),
    `/api/depends/*`, and `/api/dependents/*` carried no version.
  - Stamped at a single choke point: the `server::json` `Response` constructor
    (the sole JSON builder for the API surface). A `versioned()` helper splices
    `schema_version` as the first key of the always-object body. The redundant
    inline stamps in `meta_json`/`status_json` were removed so the choke point is
    the single source of truth. This mirrors the `query_api` `execute()`
    choke-point philosophy from `dec.query-json-schema-version`.
  - Nested node records inside `/api/graph` stay **unversioned**: `graph_json`
    builds them via `node_json` as plain strings that never pass through
    `json()`. The version describes the envelope, not each embedded record
    (confirmed by the regenerated `api_graph` snapshot).
  - An `assert!` (not `debug_assert!`) enforces the object precondition in
    release too, which also makes the `&body[1..]` slice panic-safe.
  - The webui keeps its own `ui::SCHEMA_VERSION`, independent of
    `query_api::SCHEMA_VERSION` (separate wire surfaces, separate lifecycles).
  - Decision: `meta/decisions/webui-json-schema-version.md`
    (`dec.webui-json-schema-version`, node `cairn.ui`).
  - Test: `wire_format_snapshots` now asserts every `/api/*` endpoint carries a
    numeric `schema_version`, so a future endpoint without the stamp fails the
    gate. The 12 affected golden `.snap` fixtures were regenerated (diffs are the
    added key only).
  - Pre-submit review: code-review and simplify subagents both returned correct.
    Their shared finding (the release-mode precondition) was addressed by
    promoting `debug_assert!` to `assert!` before submit.
  - Gates: `cargo build` (0 warnings), `clippy --all-targets --all-features -D
    warnings` (clean), `cargo fmt --check` (clean), `cargo test` (1382 pass),
    `cairn scan --strict` (0 findings), `cairn hook all` (exit 0). Beads export
    reconciled (single `cairn-bdj` close added, diff-verified) and committed.
  - CI on PR #155: `check` / `hooks` / `webui` / `dogfood` / CodeRabbit green;
    `claude-review` is the known non-blocking hang on unprotected `main`.

## Current State

- `cairn lint` / `cairn scan --strict` clean (0 findings) on `main`. No open PRs.
- **Backlog is empty**: `cairn next` reports "nothing to do. Project is clean."
  and `bd ready` shows no open issues.
- The loop reached its sanctioned **stop condition**: the backlog is empty and
  `cairn lint` is clean. There is no next unit to draw under priorities 1-3.

## Next Candidate

- None queued. Both the `query_api` command surface (`dec.query-json-schema-version`)
  and the webui HTTP surface (`dec.webui-json-schema-version`) now carry uniform,
  independent `schema_version` contracts. The `cairn export` envelope and the
  summariser request/response wire schemas deliberately keep their own
  independent version constants (per `dec.query-json-schema-version` consequences),
  so they are not a standardization gap.
- Resume when new work is filed (`bd`) or a `cairn lint` finding appears.

## Agent Entry Points

- `cairn context`; `cairn get <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
- Every webui `/api/*` JSON response now carries a top-level `schema_version`
  (currently 1); bump `ui::SCHEMA_VERSION` to change the webui contract, and
  `query_api::SCHEMA_VERSION` for the CLI/MCP command contract.
