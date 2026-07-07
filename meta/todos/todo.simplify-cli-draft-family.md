---
node: cairn.kernel.cli
status: done
created: 2026-07-06
resolved: 2026-07-07
---

# Collapse draft_* Commands Into One draft Subcommand

Part of todo.simplify-architecture (wave 1). Depends on: nothing.
Follow the shared rules in todo.simplify-architecture.

Lowest-risk CLI consolidation: `drafts`, `draft_show`, `draft_edit`,
`draft_discard` have no workflow, skill, or script usage and 0-1 lifetime
invocations across 13k mined sessions. They ARE referenced in the
command-reference docs and their gates, so the rename sweep below is
mandatory, not optional.

- Replace the six top-level commands (`drafts`, `draft_show`,
  `draft_edit`, `draft_discard`, `draft_accept`, `summarise`) with one
  `draft` command: `draft list|show|edit|discard|accept|create`.
- The summariser engine is NOT in scope: it backs the actively used
  `init --from-code` and `refine` paths
  (`src/brownfield/summarise.rs`). Only the CLI surface moves.
- All draft MCP tool names stay: `cairn_draft_accept` plus the
  registry-exposed draft query tools asserted by
  `tests/phase_8_summariser.rs:352-380` and `docs/mcp.md:50-52`.
- Risk split: `drafts`/`draft_show`/`draft_edit`/`draft_discard` are
  the genuinely low-risk renames. `draft_accept` and `summarise` carry
  extra contract work: both are registered query_api tools
  (`src/query_api/registry.rs`) and `tests/mcp.rs` pins the
  `draft_accept` MCP schema; keep tool ids and schemas byte-stable
  while only the CLI spelling moves.
- No deprecation aliases: clean cutover, per house convention.
- Update in the same commit (guards that WILL fail otherwise):
  - `tests/command_reference_consistency.rs`, `docs/commands.md`
    (documents all four at :112-116), `docs/integration-contract.md`
    (:104-108)
  - `tests/phase_10_distribution.rs:268` (asserts docs/commands.md
    contains literal `cairn drafts`, `cairn draft_show`,
    `cairn summarise`)
  - `tests/snapshots/wire_format_snapshots__api_meta.snap` (pins
    registry cli_names on the /api/meta wire; treat per the shared
    schema-decision rule)
  - `docs/mcp.md`, `docs/summariser.md:125-133`, and any spec prose
    naming the old commands.

Acceptance: `cairn --help` lists one `draft` command;
`command_reference_consistency`, `phase_10_distribution`, wire snapshots,
and `cargo test` green; `cairn draft create <node>` and
`cairn draft accept` exercise the summariser round trip on the demo
project.

## Resolution

Implemented on 2026-07-07. The six top-level commands were collapsed
into one `draft` command with subcommands `list`, `show`, `edit`,
`discard`, `accept`, and `create`. Registry tool ids and MCP names
were kept byte-stable; only the `cli_name` field changed. The
`tests/snapshots/wire_format_snapshots__api_meta.snap` was updated
deliberately because `/api/meta` exposes the new compound cli_names as
a sanctioned schema change. The command-reference and
integration-contract docs were updated to list the new compound
spellings, and the guard tests in `tests/command_reference_consistency.rs`
and `tests/phase_10_distribution.rs` were adjusted to treat the six
draft-family tools as one `draft` entry for documentation/help purposes.
