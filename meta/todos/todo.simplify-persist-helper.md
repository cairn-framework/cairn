---
node: cairn.root
status: done
created: 2026-07-06
---

# Shared File-Persistence Helper

Part of todo.simplify-architecture (wave 1). Depends on: nothing.
Follow the shared rules in todo.simplify-architecture.

State-file persistence is hand-rolled per module. Migrate these
enumerated call sites onto one helper:

- DraftStore: `src/summariser/store.rs:340-449` (versioned JSON files)
- Scanner state: `src/scanner/state.rs:13-180` (versioned JSON with
  explicit v1 to v2 migration)
- Scanner cache: `src/scanner/cache.rs:174-205` (reconciler-cache.json,
  version-gated)
- Scanner snapshot: `src/scanner/snapshot.rs:108-119` (map.json)
- Brownfield interview: `src/brownfield/interview.rs:217-229` (doc
  comment claims atomic write but uses plain `fs::write`; fix that here)
- Suggested edges: `src/suggested_edges/mod.rs:28-84` (versioned
  queue.json, already temp+rename atomic; adopt the helper)
- Changes: `src/changes/apply/mod.rs:218-294` (`atomic_write`,
  append-only archive log)
- Workspace: `src/workspace/mod.rs:22-69` (read-only TOML)

Extract a small `src/persist` module: `read_json`, `write_json` (pretty,
atomic), `read_toml`, `atomic_write`, version-peek.
`src/changes/apply/mod.rs::atomic_write` is the model implementation.

Constraints:

- Content-as-files invariant untouched; plumbing only, no storage model
  change. No new dependencies.
- Keep each store's schema-version semantics exactly (scanner state's
  v1 to v2 migration keeps passing its tests).
- Declare the new module in `cairn.blueprint`: either add the path to
  an owning node or add a new Module node, in which case the CH001
  architecture hook requires an accompanying decision artefact.
- Coordination: this task and todo.simplify-cut-sse both edit
  `src/lib.rs`'s module list; order-independent, trivial conflict if
  branched concurrently.

Acceptance: the enumerated call sites above all go through
`src/persist`; store tests for summariser, scanner, and changes green;
`cairn scan --strict` clean; roughly 300-500 LOC net removed.

Resolution (2026-07-07): `src/persist.rs` exposes `atomic_write`,
`atomic_write_bytes`, `write_json` (pretty, atomic, skip-if-identical),
`read_json`, `parse_json`, `read_toml`, `read_versioned_json` (single-read
version peek plus content). All eight enumerated call sites migrated;
`changes/apply`'s `atomic_write` moved in as the model implementation; the
brownfield interview's documented-but-missing atomic write is fixed;
suggested_edges' `unique_temp_path`/COUNTER helpers deleted. Scanner
state's v1-to-v2 migration tests pass unchanged. Blueprint gained
`Module Persist id cairn.persist @no-contract`
(dec.simplify-persist-module). The 300-500 net LOC estimate did not hold:
tracked call-site source shrank while the new module adds ~290 lines
(roughly half tests); net positive. The duplication removal and the
single-implementation invariant were the point; code was not compressed
to chase the estimate.
