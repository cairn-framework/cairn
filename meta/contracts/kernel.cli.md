---
node: cairn.kernel.cli
---

# Contract: cairn.kernel.cli

## Purpose

The primary user surface: command-line registry, argument parsing, command
execution, and human and JSON output rendering. It parses raw args into a
`ParsedArgs`, routes to project, change, or shared-JSON command paths, and
delegates most structured queries to the shared query API while owning the CLI
specific commands (init, onboard, import, watch, accept, export, ui, hooks).

## Public interface

- `run(args)`: parses arguments and dispatches, returning a `CliResult`.
- `CliResult`: process `code` (`u8`), `stdout`, and `stderr`.
- `registry()`: the command registry (the shared query `ToolMetadata` slice).
- `CommandMetadata`: alias for `query_api::ToolMetadata`; `SafetyClass`
  re-exported for command classification.
- Per-command modules under `commands/` (`init_project`, `run_onboard_command`,
  `run_import_openspec`, `run_watch_command`, `run_hook_command`,
  `run_decision_command`, `run_archive_command`, change runners,
  `run_shared_json_command`, `run_ui_command`), plus `accept`, `export`, and the
  `render`/`format` renderer layers.

## Invariants

- `--version` and `--help`/`-h` short-circuit before any project load.
- Unknown commands produce a suggestion using Levenshtein `edit_distance`.
- `EXTRA_CLI_COMMANDS` and `MCP_ONLY_TOOLS` reconcile the CLI command set with
  the shared registry (`init_from_code` is MCP-only and hidden from the CLI).
- Commands gated by `requires_valid_map` refuse to run on an errored graph.
- Shared-JSON commands print `response.data` verbatim with a derived exit code.

## Dependencies

Outgoing blueprint edges: `cairn.kernel.query` (delegates structured queries via
`query_api::execute`), `cairn.kernel.scanner` (loads `ScanResult` for
project-loaded commands), `cairn.kernel.hooks` (`run_hook_command`,
`HookKind`), `cairn.ui` (`run_ui_command`), and `cairn.brownfield` (onboard and
OpenSpec import paths feeding code-first initialisation).

## Tests

Unit tests live in the `#[cfg(test)] mod tests` block in `src/cli/mod.rs`,
covering argument parsing, command routing, help and unknown-command output, and
exit codes, backed by insta snapshots under `src/cli/snapshots/` for
`check`-style output.
