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

### Remaining (not yet flipped — all "no" shape gaps)
These require `app.js` to consume the canonical shape while staying tolerant of the
frozen legacy fixtures the visual harness serves (dual-mode reads). Each is a
per-endpoint commit per the ratchet rule.
- [ ] Flip structured-artefact endpoints: `decisions`, `todos`, `research`,
      `sources`, `rationale`, `contract`.
- [ ] Flip `depends` / `dependents` (canonical `deps` returns ID lists, not the
      legacy `{id,name,slug,state,kind}` entries — needs `app.js` rework).
- [ ] Flip `node` (canonical `node_json` adds `owns_files`/`span` + `Debug`-case enums).
- [ ] Flip `status` (canonical `status_json` is a different dashboard model:
      `active_changes`, `open_todos`, `recent_log_entries`, `next_recommended` — no
      `nodes`/`edges`/`findings`; significant `app.js` rework).
- [ ] Flip `graph` (no canonical full-graph dump tool exists; stays on legacy
      `graph_json` unless a spine op is added).
- [ ] Delete `src/ui/api.rs` + `src/ui/serialise.rs` once nothing needs them.

## Gate notes (this session)
- Rust gate `scripts/pre-archive-rust-gates.sh`: **green** for every commit
  (`GATE_EXIT=0`: fmt, clippy `--all-targets --all-features -D warnings`, `cargo test`,
  500-line check; `src/query_api/mod.rs` carries `// cairn:allow-large-module`).
- Visual harness `node harness/eval.mjs`: `ux_defect_score=41`
  (`missing_landmarks=1` ×40, `tiny_tap_targets=1` ×1; `scenarios_ready=11/11`).
  This is the **pre-existing baseline** — the harness serves the live `app.js`
  against frozen legacy fixtures, and neither `app.js` nor the fixtures were touched
  by this Rust-only migration, so the score is unchanged. It is NOT a regression
  from the endpoint flips. Reaching 0 is a separate webui-polish task (add the
  missing landmark + enlarge the tiny tap target in `app.js`), out of scope for the
  query_api spine migration.
- `biome check --error-on-warnings`: pre-existing warnings only — `app.js:1266` lint
  warning plus 488 errors in dotfiles/fixtures (biome.json `includes: ["**"]`). No
  `.rs` change affects biome; none introduced by this migration.
- `scripts/check-design-tokens.sh`, `scripts/check-a11y.sh`: pass.
