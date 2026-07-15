---
node: cairn.kernel.cli
status: done
created: 2026-07-12
---

# Per Command Help

gh:#244

Per-command `--help` falls back to the global command list.

## Evidence (verified on main, 2026-07-12)
- `cairn accept --help`, `cairn frontier --help`, `cairn neighbourhood --help`
  all print the global usage.
- `src/cli/mod.rs:83` short-circuits to global help whenever any argument is
  `--help`/`-h`, before per-command parsing.

## Task
Route `cairn <cmd> --help` to a per-command usage page (args, flags such as
`--include-orphans`, `--include-research`, `--depth`, `--scope`). Strings go in
docs/design-system/copy.toml.
