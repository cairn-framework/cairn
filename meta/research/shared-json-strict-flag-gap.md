---
id: res.shared-json-strict-flag-gap
nodes:
  - cairn.kernel.cli
date: 2026-07-29
method: primary
tags: [cli, wire, gates]
---
# The shared-JSON dispatch silently dropped `--strict`

## Observation

While wiring the published strict verdict for `dec.loop-selection-strict-green-fold`
(obligation 1: one predicate feeds the field and every strict exit path), the
unit found that `shared_exit_code` (`src/cli/commands/mod.rs`) keyed its exit
solely on an `error`-severity finding for `lint`/`scan`/`hook` and never
consulted the parsed `--strict` flag. Because `uses_shared_json` routes every
`lint --json` and `scan --json` invocation (without `--node`) through
`execute_json_request`, the documented `--strict` contract ("Exit 1 on Warning
findings (scan/lint)") was void on the JSON surface: `cairn scan --strict
--json` exited 0 on a warning-only set. The text surface honoured the flag
(`src/cli/mod.rs`, `src/cli/commands/workspace.rs`), so the two surfaces
disagreed about the same gate.

The plan did not predict this. `todo.lint-selection-folding` item 2 assumed
"whenever `--strict` would exit zero" named one well-defined verdict; before
the fix it named two, depending on output format.

## Verification

Reproduced on the contract node-shape-drift warning fixture: text
`scan --strict` exited 1, JSON `scan --strict --json` exited 0 before the fix.
The fix makes the strict exit read the published `strict_green` field itself,
so wire and exit agree by construction; the regression is pinned by
`test_cli_scan_reports_node_shape_drift_and_never_writes_the_baseline`
(both exits plus the published field) and the `shared_exit_code` unit tests.

## Limits

Only the `--strict` flag was audited on the shared-JSON dispatch. Other
per-command flags could be dropped the same way by that dispatch shape; they
were not audited here. The defect class matches the remediate bare-command
defect recorded in `todo.lint-selection-folding`'s evidence (#513): a surface
reachable only through one dispatch path drifting from the contract the other
path honours.
