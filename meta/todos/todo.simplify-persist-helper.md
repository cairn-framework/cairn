---
node: cairn.root
status: open
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
