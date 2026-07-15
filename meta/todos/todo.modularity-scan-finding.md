---
node: cairn.kernel.scanner
status: open
created: 2026-07-15
---

# Oversized-module scan finding

related: [todo.architecture-modularity-audit, todo.size-gate-non-rust]

## Problem

Cairn dogfoods cairn, yet a node can own a 2000-line file and still scan
clean. Map validation checks path existence and edge *endpoints*
(`src/map/build.rs::validate_edges`, lines 100-133) but has no intra-module
health signal (size, coupling, god-module). Agent loops that only run
`cairn scan` / `cairn remediate` never see size drift that the shell gate
might catch in CI.

## Evidence (res.architecture-modularity-audit, 2026-07-15)

- `validate_edges` only emits `CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT` when an
  endpoint node is missing; declared edges need not be realised in code.
- No finding code today for file length, fan-in/fan-out, or multi-responsibility.
- Allow-listed Rust offenders still scan clean: `src/cli/mod.rs` (2205),
  `src/map/query.rs` (1104), `src/cli/render/remediate.rs` (1054), plus
  ungated `app.js` / `style.css`.
- Completed simplify-architecture programme removed some duplication but did
  not add a durable modularity signal.

## Approach (backlog only)

1. Add a Warning or Info finding (e.g. `CAIRN_MODULE_OVERSIZED`) when a
   node-owned source file exceeds N lines (default 500, matching conventions)
   without an allow-list marker.
2. Honour the same allow-list protocol as `scripts/check-file-sizes.sh` so CI
   shell and scan agree.
3. Scope v1 to file size only. Do **not** ship fan-in/fan-out quotas or full
   import-graph edge-realisation in this todo (audit judged those higher cost
   / higher false-positive risk without a longer baseline).
4. Surface the finding in `cairn scan`, `cairn lint` (advisory unless Error),
   and `cairn remediate` so agent loops see it without invoking the shell gate.
5. Register the code in `docs/registries/error-codes.md` and cover it with a
   unit or integration test.

## Priority

Second self-guardrail after todo.size-gate-non-rust. The shell gate catches CI;
this catches agent-native loops. Can land after the size-gate extension so
thresholds and allow-list syntax are shared.
