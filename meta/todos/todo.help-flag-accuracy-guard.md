---
node: cairn.kernel.cli
status: open
created: 2026-07-15
related: [todo.per-command-help]
---

# Help Flag Accuracy Guard

## Problem

`src/cli/help/mod.rs` `COMMAND_HELP` hand-maintains each command's advertised flag list, with no single source of flag truth (the CLI is hand-rolled, no clap). During PR #347 three successive manual audits kept finding drift, so flag accuracy is not durably guaranteed.

## Evidence

#347 found `scan` advertising `--node` it ignores; `refine`/`change new` over-advertising `--json`; `feedback`/`gap`/`change archive`/`change apply` under-advertising `--json`/`--verbose`; the `todo` family page over-listing `--node`.

## Approach (backlog only)

Add a structural guard so advertised flags cannot drift from real parsing, e.g. a test that cross-checks each command's `COMMAND_HELP` flag set against the flags its parser actually consumes (or introduce a shared per-command flag registry that both help and parsing read). Do NOT implement here; this is the backlog entry.
