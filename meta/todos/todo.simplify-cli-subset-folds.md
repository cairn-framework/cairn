---
node: cairn.kernel.cli
status: open
created: 2026-07-06
---

# Fold Strict-Subset Commands Into Flags

Part of todo.simplify-architecture (wave 3).
Depends on: todo.simplify-cli-change-family (surface churn in one place
at a time; reuse its reference-update recipe).
Follow the shared rules in todo.simplify-architecture.

Three redundancies:

- `symbols`: `symbols_json` (`src/query_api/handlers/node.rs:95-104`)
  reads the same in-memory `NodeRecord` that `get` resolves, but `get`'s
  wire JSON (`node_json`, `src/query_api/serialise.rs:7-30`) does NOT
  yet expose a `symbols` field. The fold is `get --symbols` adding an
  opt-in field to get's response: a deliberate wire-shape addition,
  recorded per the query JSON schema versioning convention
  (`meta/decisions/query-json-schema-version.md`), not a free rename.
- `check <node>` is `lint` filtered by node (same
  `scanner::load_project` data). Fold into `lint --node <id>`.
- `depends` and `dependents` share one handler differing by a boolean
  (`src/query_api/handlers/graph.rs:95`). Merge into
  `deps <id> --direction in|out` (default out).

Cautions:

- All three have documented usage (spec, skills, cairn-dev workflow doc,
  emitted init skills); update every reference in the same change.
- MCP tools (`cairn_get`, `cairn_lint`, `cairn_depends`,
  `cairn_dependents`) keep their names; only CLI spellings change. The
  `get` response gains an optional field only under the new flag.
- Removing/renaming registry cli_names changes the /api/meta wire
  (`tests/snapshots/wire_format_snapshots__api_meta.snap`); treat per
  the shared schema-decision rule, not a silent snapshot update.

Acceptance: gates green; `cairn get <id> --symbols`, `cairn lint --node
<id>`, `cairn deps <id> --direction in` produce the same information the
old commands did (compare against pre-change output captured in the PR).
