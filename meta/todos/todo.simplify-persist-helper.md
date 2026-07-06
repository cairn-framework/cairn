---
node: cairn.root
status: open
created: 2026-07-06
---

# Shared File-Persistence Helper

Part of todo.simplify-architecture (wave 1). Depends on: nothing.

Four file-backed persistence patterns each hand-roll
read/write/serialise/version logic:

- DraftStore: `src/summariser/store.rs:340-449` (versioned JSON files)
- Scanner state: `src/scanner/state.rs:13-180` (versioned JSON with
  explicit v1 to v2 migration)
- Changes: `src/changes/apply/mod.rs:218-294` (`atomic_write`,
  append-only archive log)
- Workspace: `src/workspace/mod.rs:22-69` (read-only TOML)

Extract a small `src/persist` (or `src/store`) module: `read_json`,
`write_json` (pretty, atomic), `read_toml`, `atomic_write`,
version-peek. `src/changes/apply/mod.rs::atomic_write` is the model
implementation. Migrate the three JSON call sites onto it; workspace
adopts `read_toml`.

Constraints: content-as-files invariant is untouched (this is plumbing,
not a storage model change); no new dependencies; keep each store's
schema-version semantics exactly (the v1 to v2 migration in scanner
state must keep passing its tests).

Acceptance: no module under src/ hand-rolls
read_to_string + serde_json + fs::write for state files; store tests for
summariser, scanner, and changes green; roughly 300-500 LOC net removed.
