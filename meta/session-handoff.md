# Session Handoff: 2026-06-23 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main`.

## What Was Done

One iteration landed and merged this session, drawn from phase-10 priority 4 (a
self-contained improvement) since the backlog and `cairn lint` were both empty at
orient:

- **Removed unreachable `--json` branches in the CLI `order`/`contract` arms**
  (PR #153, squash `c0c7262`). Resolved bead `cairn-2n6`.
  - `run_project_command` intercepts `--json` at dispatch (`uses_shared_json`
    gate) and routes to `run_shared_json_command` -> `query_api::execute` before
    `render_loaded_project_command` runs. With a single private call site, the
    inline `if parsed.json { … }` branches in the `order` and `contract` arms
    were dead code. The dead `contract` branch was also divergent: it emitted
    `{node, contract}` while `query_api` emits `{node, contract, contracts}`.
    Removed both so `query_api` is the single JSON source of truth.
  - Added `test_contract_and_order_json_served_by_query_api`, which asserts the
    `contracts` field is present (a sentinel that fails if a divergent CLI-local
    formatter is reintroduced) and that the human paths still print their
    `Contract for`/`Order:` headers. Output verified byte-identical before/after.
  - Pre-submit gate: an independent reviewer subagent returned APPROVE (0
    findings, confidence 0.97); the `/debate` verdict was ship.
  - Gates: `cargo build` (0 warnings), `clippy --all-targets --all-features -D
    warnings` (clean), `cargo test --lib` (939 pass), `cairn scan` (0 findings),
    `cairn hook all` (exit 0). Beads export reconciled (single `cairn-2n6`
    open -> closed flip, diff-verified) and pushed as `e72c439`.
  - CI on PR #153: `check` / `hooks` / `webui` / `dogfood` / CodeRabbit (APPROVED)
    green; `claude-review` is the known non-blocking hang on unprotected `main`.

## Current State

- `cairn lint` / `cairn scan` clean (0 findings) on `main`. No open PRs.
- **Backlog is empty**: `cairn next` reports "nothing to do. Project is clean."
  and `bd ready` shows no open issues.
- The loop reached its sanctioned **stop condition**: the backlog is empty and
  `cairn lint` is clean. There is no next unit to draw under priorities 1-3.

## Next Candidate (maintainer decision required)

- **JSON envelope `schema_version` inconsistency.** `cairn islands --json` emits
  `{"islands":[…],"schema_version":1}`, but `order`, `contract`, and most other
  commands emit envelopes with no `schema_version`. Standardizing this is a
  user-facing output-contract change across `src/query_api/` serialization (and
  would churn snapshot/round-trip tests), so whether every JSON response should
  carry `schema_version` (and at what version) is the maintainer's call, not an
  autonomous loop unit. This aligns with the agnostic half of the gas-city slate
  #4 ("stable JSON output across all commands"). Resume here if sanctioned, or
  when new work is filed (`bd`) or a `cairn lint` finding appears.

## Agent Entry Points

- `cairn context`; `cairn get <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
- Node-linked beads surface per node: `GET /api/node/<id>/beads` (webui inspector
  "Beads" section) and a `CAIRN_BACKLOG_ORPHAN_NODE` scan warning for labels that
  point at unknown nodes. To use it, tag a bead with a `cairn-node:<id>` label
  (`bd update <id> --label cairn-node:<node-id>`).
