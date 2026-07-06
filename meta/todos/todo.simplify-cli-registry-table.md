---
node: cairn.kernel.cli
status: open
created: 2026-07-06
---

# Derive CLI Dispatch and Help From the query_api Registry

Part of todo.simplify-architecture (wave 4).
Depends on: todo.simplify-cli-draft-family,
todo.simplify-cli-change-family, todo.simplify-cli-subset-folds (do this
once, after the command surface has stopped moving, or the table churns
with every rename).

`src/cli/mod.rs:484-640` hand-maintains `EXTRA_CLI_COMMANDS`,
`all_command_names`, `command_description`, `help_text`, and
edit-distance suggestions; `CommandMetadata` mirrors `ToolMetadata`. This
is a manually synced copy of `query_api::registry()`
(`src/query_api/registry.rs:7-281`).

- Drive command names, descriptions, help text, and unknown-command
  suggestions from the registry plus a small static table for the
  CLI-only commands (init, ui, workspace, import, onboard).
- Either a declarative table or clap derive; pick whichever deletes more
  code while keeping the single-binary, no-heavy-deps posture (check
  `docs/conventions.md` before adding clap as a dependency).
- `tests/command_reference_consistency.rs` should then verify docs
  against the registry, not against a second hand-written list.

Acceptance: adding a query_api operation makes it appear in CLI help
without touching `src/cli/mod.rs` lists; roughly 400-600 LOC of
bookkeeping deleted; gates green.
