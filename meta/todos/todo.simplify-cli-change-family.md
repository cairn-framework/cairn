---
node: cairn.kernel.cli
status: open
created: 2026-07-06
---

# Collapse Change Lifecycle Into One change Subcommand

Part of todo.simplify-architecture (wave 2).
Depends on: todo.simplify-cli-draft-family (establishes the subcommand
pattern and its test/doc update recipe on a zero-risk family first).

Six top-level names describe one lifecycle: `change`, `changes`, `show`,
`archive`, `accept`, `rename`. Consolidate as:

- `change new <id>` (current `change`)
- `change list` (current `changes`)
- `change show <id>` (current `show`)
- `change accept <id>` (current `accept`, `src/cli/accept.rs` moves as-is)
- `change archive <id>` (current `archive`)
- `change rename <old> <new>` (current `rename`)

Cautions:

- These commands ARE load-bearing (AGENTS.md, docs/spec.md, skills,
  `/cairn-loop`). Grep and update every reference: `.claude/skills/`,
  `.claude/commands/`, `docs/`, `meta/` guidance prose, and the skills
  emitted by `cairn init` (include_str! sources), in the same change.
- MCP tools `cairn_changes` and `cairn_show_change` keep their names.
- Clean cutover, no aliases.
- Update `tests/command_reference_consistency.rs`, `docs/commands.md`,
  `docs/integration-contract.md` in the same commit.

Acceptance: full lifecycle smoke on the demo project (`change new` ->
edit -> `change accept` -> `change archive`); all gates green; zero
occurrences of the old top-level spellings outside archive/ history.
