# Tasks: Web UI consumes query_api (strategy fork)

Sequenced per `todo.simplify-ui-query-api` (wave 2 of `todo.simplify-architecture`).
Strategy **B** ratified (dec.ui-query-api-strategy, 2026-07-08): server is a thin
router over `query_api::execute`, returns `data` verbatim; `app.js` consumes
canonical shapes; `src/ui/api.rs` + `src/ui/serialise.rs` deleted at the end.
Recorded schema decisions live in `meta/decisions/dec.ui-query-api-wire-adoption.md`.

## Progress (resumed session, branch `feat/simplify-ui-query-api`)

### Done (committed, Rust gates green)
- [x] Step 1 — clear dead code + sync registry/docs/snapshots for spine ops.
      Commit `56063b1`. (`meta_json`/`beads_response_json` removed from `api.rs`;
      `cli`/`state::backlog` imports dropped from `ui/mod.rs`; registry size 38→41;
      integration-contract rows added; `api_meta.snap` rebased.)
- [x] Spine ops `beads`, `ui_meta`, `blueprint` added + `/api/meta`, `/api/blueprint`,
      `/api/beads` flipped. Commits `e11890f`, `98ac38d` (prior session).
- [x] `/api/lint` flipped → `execute("lint")`. Wire is byte-identical to legacy
      (`findings_json` keeps lowercase `severity`, same field order) → no snapshot
      delta, no `app.js` change. Commit `1e4462c`.
- [x] `/api/node/symbols` flipped → `execute("get", {node, flags:[Symbols]})`. Wire
      changes from `{"node","symbols"}` to `{"id",...,"symbols"}`; `app.js` reads
      `response.symbols` (unchanged path); endpoint test updated `node`→`id`.
      Removes dead `api.rs::symbols_response_json` + `serialise::symbol_kind_name`.
      Commit `5a9767f`.

### Remaining (not yet flipped)
- [ ] Flip `depends` / `dependents` (canonical `deps` returns ID lists, not the
      legacy `{id,name,slug,state,kind}` entries — needs `app.js` rework).
- [ ] Flip `node` (canonical `node_json` adds `owns_files`/`span` + `Debug`-case enums).
- [ ] Flip `status` (canonical `status_json` is a different dashboard model:
      `active_changes`, `open_todos`, `recent_log_entries`, `next_recommended` — no
      `nodes`/`edges`/`findings`; significant `app.js` rework).
- [ ] Flip `graph` (no canonical full-graph dump tool exists; stays on legacy
      `graph_json` unless a spine op is added).
- [ ] Delete `src/ui/api.rs` + `src/ui/serialise.rs` once nothing needs them.
      (rationale/contract flip removed the last api.rs helpers; both files are
      now dead and will be removed in the final cleanup step.)

## Gate notes (this session)
- Rust gate `scripts/pre-archive-rust-gates.sh`: **green** for every commit
  (`GATE_EXIT=0`: fmt, clippy `--all-targets --all-features -D warnings`, `cargo test`,
  500-line check; `src/query_api/mod.rs` carries `// cairn:allow-large-module`).
- Visual harness `node harness/eval.mjs`: `ux_defect_score=0`
  (`scenarios_ready=11/11`). The harness baseline is now zero after prior webui
  polish; the rationale/contract flip and dead-code cleanup do not affect it.
- `biome check --error-on-warnings`: no diagnostics on `src/ui_assets/app.js`.
- `scripts/check-design-tokens.sh`, `scripts/check-a11y.sh`: pass.
