# Session Handoff: 2026-06-23 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main`.

## What Was Done

One iteration landed and merged this session. The backlog and `cairn lint` were
both empty at orient, so the unit was drawn from phase-10 priority 4: a
self-contained improvement that was also the documented next candidate from the
prior handoff (JSON-envelope `schema_version` inconsistency). The maintainer was
asked (`AskUserQuestion`) and chose "add `schema_version` everywhere".

- **Uniform `schema_version` across the `query_api` command surface**
  (PR #154, squash `30ac03b`). Resolved bead `cairn-njc`.
  - `cairn islands --json` carried a top-level `schema_version`, but `order`,
    `contract`, `context`, `status`, `lint`, `dependents`, and every other
    command emitted an unversioned `data` payload. Added `query_api::SCHEMA_VERSION`
    and stamp it onto every command's `data` object at the single `execute()`
    choke point (after `execute_data` returns). The CLI prints `data` directly
    and MCP wraps it, so both surfaces now share one versioned contract from one
    constant. Removed the redundant per-handler stamp in `islands_json`; the
    universal stamp owns it and the live islands output is byte-identical.
  - A `debug_assert` in `execute` guards the object-payload invariant (every
    `execute_data` arm returns a JSON object). Doc comments on
    `IslandsResponse`/`ISLANDS_SCHEMA_VERSION`/`islands()` now state that
    constant is a domain-layer invariant, not the consumer wire version.
  - Decision: `meta/decisions/query-json-schema-version.md`
    (`dec.query-json-schema-version`, node `cairn.kernel.query`).
  - Test: `test_execute_stamps_schema_version_on_read_commands` asserts the
    stamp across status/order/islands/lint/context/health/watch/remediate and
    that islands retains its component array.
  - Pre-submit gate: a commissioned reviewer subagent returned correct/approve
    (confidence 0.87); its three non-blocking findings (stale islands doc,
    missing debug_assert, test-name coverage) were all fixed before submit.
  - Gates: `cargo build` (0 warnings), `clippy --all-targets --all-features -D
    warnings` (clean), `cargo fmt --check` (clean), `cargo test` (1382 pass),
    `cairn scan` (0 findings), `cairn hook all` (exit 0). Beads export reconciled
    (single `cairn-njc` open -> closed flip, diff-verified) and committed as
    `532d093`.
  - CI on PR #154: `check` / `hooks` / `webui` / `dogfood` / CodeRabbit green;
    `claude-review` is the known non-blocking hang on unprotected `main`.

## Current State

- `cairn lint` / `cairn scan` clean (0 findings) on `main`. No open PRs.
- **Backlog is empty**: `cairn next` reports "nothing to do. Project is clean."
  and `bd ready` shows no open issues.
- The loop reached its sanctioned **stop condition**: the backlog is empty and
  `cairn lint` is clean. There is no next unit to draw under priorities 1-3.

## Next Candidate (maintainer decision required)

- **Webui `/api` `schema_version` inconsistency.** The webui HTTP surface
  (`src/ui/api.rs`, its own `SCHEMA_VERSION`) versions only `/api/meta` and
  `/api/status`; `/api/graph`, `/api/node/*`, `/api/depends/*`, and
  `/api/dependents/*` carry no version (confirmed by the `wire_format_snapshots`
  fixtures). This is the same class of user-facing output-contract change just
  made for `query_api`, on a separate surface, and may be a deliberate
  "version the handshake endpoints only" design rather than an oversight.
  Standardizing it would churn the `wire_format_snapshots` and is the
  maintainer's call. The `dec.query-json-schema-version` record explicitly
  scoped the webui out of the `query_api` unit. Resume here if sanctioned, or
  when new work is filed (`bd`) or a `cairn lint` finding appears.

## Agent Entry Points

- `cairn context`; `cairn get <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
- Every `query_api` `--json` command now carries a top-level `schema_version`
  (currently 1); bump `query_api::SCHEMA_VERSION` to change the contract.
