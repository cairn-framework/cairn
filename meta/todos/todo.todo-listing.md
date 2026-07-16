---
node: cairn.kernel.cli
status: done
created: 2026-07-12
---

# Todo Listing

gh:#242

No project-wide todo listing (the created-date bug in `cairn todo new` is fixed
on main; scratch probe stamped today's date, `src/cli/commands/todo.rs:48`).

## Evidence (verified on main, 2026-07-12)
- Bare `cairn todos` errors `CAIRN_CLI_MISSING_NODE`; `cairn todos --all` is
  parsed as a node name; `cairn todos <container>` does not aggregate
  descendants (`src/query_api/handlers/artefacts.rs:18-25` filters exact
  node id only).

## Task
Support project-wide listing (bare `cairn todos` or `--all`) and descendant
aggregation when given a container node. Relates to
todo.unified-todo-write-surface.

## Resolution (2026-07-16)
Shipped both gaps. Bare `cairn todos` (or any flag-first invocation, e.g.
`cairn todos --status open`) now lists todos project-wide in both human and
`--json` output; the JSON payload carries `node: null` for project-wide
listings. `cairn todos <container>` now aggregates the container's own todos
plus all descendants' by walking the map's containment (`children`) edges in
`src/query_api/handlers/artefacts.rs`. Exact leaf-node behaviour is
unchanged. Help copy (`docs/design-system/copy.toml`), the registry
description, `docs/commands.md`, and `docs/spec.md` were updated to match.
Tests: unit tests in `src/query_api/handlers/artefacts.rs` and
`src/cli/render/artefacts.rs`, plus an end-to-end test in `tests/kernel.rs`
(`test_todos_project_wide_and_descendant_listing`).
