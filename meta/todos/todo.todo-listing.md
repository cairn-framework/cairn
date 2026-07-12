---
node: cairn.kernel.cli
status: open
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
