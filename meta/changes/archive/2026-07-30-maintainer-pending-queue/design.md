# Design: maintainer-pending-queue

## Approach

`pending` becomes a 44th `query_api` registry tool, so the CLI name, help
entry, docs battery, MCP tool, and webui route all derive from the one
`TOOL_REGISTRY` row (single-registry discipline, `dec.kernel-tooling`,
`dec.cli-agent-workflow-consolidation`). The JSON payload is a typed
`PendingResponse` struct (serde + schemars) with a committed
`schemas/PendingResponse.schema.json`, keeping the unschema'd allowlist
burn-down (`todo.wire-format-schemas`) from growing. `schema_version` is
stamped at the existing `execute` choke point (`dec.query-json-schema-version`);
the handler adds no stamp of its own.

Rows are computed from `scan_result.artefacts.decisions` alone: filter
`status == Proposed`, age is the signed whole-day difference between the
`date:` frontmatter and today (no clamping; a future date yields a negative
age), sort age descending then id ascending. `ratification` renders the
documented default `binding` for every row until the field exists in the
artefact schema. "Today" is injected into the row builder so tests are
deterministic; only the dispatch shim reads the wall clock. A proposed
decision with an unparseable date is a deterministic `QueryError`
(`CAIRN_PENDING_INVALID_DATE`); accepted or superseded decisions never parse.

`pending` does not join the `requires_valid_map` gate: it resolves no node,
and the queue must stay readable exactly when the graph is broken.

The webui gains a read-only Pending channel in the existing `ChannelBar`
(consistent with `dec.user-surfaces`: the explorer stays read-only), fed by
`GET /api/pending`, which serves the same `query_api` payload through the
existing `spine` helper. All user-facing strings live in
`docs/design-system/copy.toml`.

## Changes

ADDED:
- `src/query_api/handlers/pending.rs`: `PendingDecision`, `PendingResponse`,
  date arithmetic, row builder, dispatch shim, unit tests.
- `schemas/PendingResponse.schema.json`: committed wire schema.
- `tests/pending_queue.rs`: fixture-based acceptance tests (states filter,
  order, flip-to-accepted, invalid date, CLI human and JSON output).
- `/api/pending` route in `src/ui/server.rs`, `fetchPending` in
  `src/ui_assets/utils.js`, a `pending` channel in
  `src/ui_assets/channel-bar.js`, bootstrap wiring in
  `src/ui_assets/app-data.js` and `src/ui_assets/app.js`.
- Copy keys: `[pending]` CLI section, `webui.channel.pending`,
  `webui.empty.pending`, `webui.bootstrap-pending-failed`,
  `[help.commands.pending]`.
- Error-code registry entry CM015 (`CAIRN_PENDING_INVALID_DATE`).

MODIFIED:
- `src/query_api/registry.rs`: new tool row; registry size test 43 to 44.
- `src/query_api/mod.rs`: dispatch arm, re-exports.
- `src/query_api/handlers/mod.rs`: module wiring.
- `src/cli/mod.rs`: human render arm, `uses_shared_json`, core-commands test
  case.
- `src/cli/help/mod.rs`: help spec row.
- `docs/commands.md`, `docs/integration-contract.md`: command rows.
- `tests/schema_validation.rs`: live-validation and drift pairs for
  `PendingResponse`.
- `tests/wire_format_snapshots.rs`: `api_pending` endpoint with `age_days`
  redaction; fixture gains one proposed decision (affected snapshots
  regenerate).
- `tests/graph_explorer.rs`: `/api/pending` route assertion.
- `tests/phase_7_7_ux_foundation.rs`: CHANNELS assertion gains `pending`.

REMOVED:
- Nothing.

RENAMED:
- Nothing.
