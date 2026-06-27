---
node: cairn.kernel.hooks
---

# Contract: cairn.kernel.hooks

## Purpose

Hook engine that enforces commit and task-boundary gates over a scanned
project. It selects scanner and lint findings by hook class, detects conflicts
between active changes, checks interface-hash contradictions, and reduces the
result to a single pass or block exit decision suitable for a git or CI gate.

## Public interface

- `run(kind, root, changes_dir, scan_result)`: runs one hook class against an
  already loaded project and returns a `HookReport`.
- `detect_active_change_conflicts(changes_dir)`: returns structural `Finding`s
  for active changes that target the same blueprint, artefact, or rename target.
- `render_human` and `render_json`: report rendering.
- `HookKind`: `Structural`, `Interface`, `Tension`, `ArchitectureDecision`, and
  `All` (combined blocking semantics).
- `ExitDecision`: `Pass` or `Block`.
- `HookReport`: carries the kind, selected findings, conflict findings, decision,
  elapsed milliseconds, and output paths; `exit_code` maps `Pass` to 0 and
  `Block` to 1.

## Invariants

- `Structural` blocks on error-severity lint findings or any active-change
  conflict; `Interface` blocks on interface-hash contradictions;
  `ArchitectureDecision` blocks on blueprint mutations lacking paired decisions.
- `Tension` never blocks: it reports rationale tensions only.
- `All` aggregates every class and blocks if any blocking class has findings.
- Duplicate-target detection emits `CAIRN_CHANGE_BLUEPRINT_CONFLICT`,
  `CAIRN_CHANGE_ARTEFACT_CONFLICT`, and `CAIRN_CHANGE_RENAME_CONFLICT` codes.

## Dependencies

Outgoing blueprint edges: `cairn.kernel.hooks -> cairn.kernel.scanner` (gates
commits on scan input, consuming `ScanResult` and target hashes) and
`-> cairn.kernel.map` (reads findings via `query::lint`). Inbound:
`cairn.kernel.cli` runs hook checks and `cairn.kernel.query` reads hook findings.

## Tests

Unit tests live in `src/hooks/tests.rs`, covering per-kind finding selection,
blocking versus passing decisions, exit codes, and active-change conflict
detection across blueprint, artefact, and rename targets.
