# Tasks: maintainer-pending-queue

- [x] 1. Handler: `src/query_api/handlers/pending.rs` with `PendingDecision`,
  `PendingResponse`, strict `YYYY-MM-DD` to epoch-days parsing, injected-today
  row builder, `CAIRN_PENDING_INVALID_DATE` error, and unit tests (filter,
  order, tie-break, `binding` default, signed age, malformed date).
- [x] 2. Registry and dispatch: `TOOL_REGISTRY` row (44), size test, dispatch
  arm in `execute_data_with_scan`, handler exports and `pub use` of the
  response types.
- [x] 3. Schema: `schemas/PendingResponse.schema.json` plus
  `tests/schema_validation.rs` live-validation and drift entries.
- [x] 4. CLI: human render arm from copy templates, `uses_shared_json`,
  help spec, copy keys, core-commands test case.
- [x] 5. Docs and registries: `docs/commands.md` row,
  `docs/integration-contract.md` row, error-code registry CM015.
- [x] 6. Webui: `/api/pending` route, `fetchPending`, bootstrap wiring,
  `pending` channel with copy keys, CHANNELS test update,
  `tests/graph_explorer.rs` route assertion, `tests/wire_format_snapshots.rs`
  endpoint with redaction and fixture decision. Also restored the frozen
  replay contract: `/api/pending` joined `harness/capture-fixtures.mjs` TOP
  and `harness/fixtures/` was recaptured (copy.json now carries the Pending
  keys), verified by driving the SPA against the replay server.
- [x] 7. Acceptance tests: `tests/pending_queue.rs` fixture with all four
  decision states, order check, flip-to-accepted rerun, invalid-date error.
- [x] 8. Evidence: full gates (`cargo build`, clippy, `cargo test`,
  `cairn scan --strict`, `cairn hook all`), `cairn change accept`, and the
  boundary proof: run `cairn pending` and `cairn pending --json` on this
  repository and `GET /api/pending` on the embedded server.
