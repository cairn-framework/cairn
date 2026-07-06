---
node: cairn.kernel.cli
status: open
created: 2026-07-06
---

# Collapse draft_* Commands Into One draft Subcommand

Part of todo.simplify-architecture (wave 1). Depends on: nothing.
Lowest-risk CLI consolidation: `drafts`, `draft_show`, `draft_edit`,
`draft_discard` have zero references in any doc, skill, script, test, or
gate, and 0-1 lifetime invocations across 13k mined sessions.

- Replace the six top-level commands (`drafts`, `draft_show`,
  `draft_edit`, `draft_discard`, `draft_accept`, `summarise`) with one
  `draft` command: `draft list|show|edit|discard|accept|create`.
- The summariser engine is NOT in scope: it backs the actively used
  `init --from-code` and `refine` paths
  (`src/brownfield/summarise.rs`). Only the CLI surface moves.
- MCP tool `cairn_draft_accept` keeps its name (MCP names are stable
  independent of CLI names).
- No deprecation aliases: clean cutover, per house convention.
- Update in the same commit: `tests/command_reference_consistency.rs`,
  `docs/commands.md`, `docs/integration-contract.md`, and any spec prose
  naming the old commands.

Acceptance: `cairn --help` lists one `draft` command;
`command_reference_consistency` and `cargo test` green; `cairn draft
create <node>` and `cairn draft accept` exercise the summariser round
trip on the demo project.
