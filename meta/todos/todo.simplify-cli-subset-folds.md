---
node: cairn.kernel.cli
status: open
created: 2026-07-06
---

# Fold Strict-Subset Commands Into Flags

Part of todo.simplify-architecture (wave 3).
Depends on: todo.simplify-cli-change-family (surface churn in one place
at a time; reuse its reference-update recipe).

Three provable redundancies:

- `symbols` is a strict subset of `get`: `symbols_json` only formats
  `NodeRecord.symbols`, which `get` already returns
  (`src/query_api/handlers/node.rs`). Fold into `get --symbols`.
- `check <node>` is `lint` filtered by node (same
  `scanner::load_project` data). Fold into `lint --node <id>`.
- `depends` and `dependents` share one handler differing by a boolean
  (`src/query_api/handlers/graph.rs:95`). Merge into
  `deps <id> --direction in|out` (default out).

Cautions:

- All three have documented usage (spec, skills, cairn-dev workflow doc,
  emitted init skills); update every reference in the same change.
- MCP tools (`cairn_get`, `cairn_lint`, `cairn_depends`,
  `cairn_dependents`) keep their names and shapes; only CLI spellings
  change.
- Registry entries for removed CLI names go away; query_api operations
  stay if MCP needs them.

Acceptance: gates green; `cairn get <id> --symbols`, `cairn lint --node
<id>`, `cairn deps <id> --direction in` produce the same information the
old commands did (compare against pre-change output captured in the PR).
