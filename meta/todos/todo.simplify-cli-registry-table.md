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
Follow the shared rules in todo.simplify-architecture.

`src/cli/mod.rs:484-640` hand-maintains `EXTRA_CLI_COMMANDS`,
`all_command_names`, `command_description`, `help_text`, and
edit-distance suggestions. `CommandMetadata` is already a type alias for
`query_api::ToolMetadata` (`src/cli/mod.rs:57`); the duplication is the
hand-synced name/description/help lists, not the struct.

- Drive command names, descriptions, help text, and unknown-command
  suggestions from `query_api::registry()` plus a small static table
  for the genuinely CLI-only commands. The authoritative list is
  `EXTRA_CLI_COMMANDS` (`src/cli/mod.rs:484-501`), currently 14
  entries (accept, backlog, brief, change, decision, check, export,
  feedback, gap, import-openspec, next, onboard, todo, workspace;
  refine and watch overlap the registry). Note `init` and `ui` ARE
  registry tools; expect the residual CLI-only set after waves 1-3 to
  be around a dozen, not a handful.
- Either a declarative table or clap derive; pick whichever deletes more
  code while keeping the single-binary, no-heavy-deps posture (check
  `docs/conventions.md` before adding clap as a dependency).
- `tests/command_reference_consistency.rs` should then verify docs
  against the registry, not against a second hand-written list.

Acceptance: adding a query_api operation makes it appear in CLI help
without touching `src/cli/mod.rs` lists; roughly 400-600 LOC of
bookkeeping deleted; gates green.
