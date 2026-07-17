---
node: cairn.kernel.map
status: done
created: 2026-07-15
related: [todo.architecture-modularity-audit, todo.size-gate-non-rust, todo.oversized-test-file-baseline]
---

# Oversized-module scan finding


## Problem

Primary home: `cairn.kernel.map` (graph construction and integrity checks;
sister structural findings already live here). Emission is via the scan
pipeline, but the check belongs with map integrity, not scanner
orchestration.

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

## Resolution (2026-07-17)

Shipped `CAIRN_MODULE_OVERSIZED` (registry `CK031`) in `src/map/module_size.rs`,
called from `build_graph` alongside the other structural `validate_*` checks
(`src/map/build/mod.rs`). It walks every node-owned claimed file (from
`TargetReport.claimed_files`, post-reconcile, so it also covers claim-only
assets targets like `src/ui_assets`), counts newline bytes to match `wc -l`
exactly, and honours the same `cairn:allow-large-module reason: ...` marker
protocol as `scripts/check-file-sizes.sh` (Rust/JS line-comment form, CSS
single-line block form; JS/CSS vendor path segments are skipped, while Rust vendor paths remain checked to mirror the shell gate). v1 is file size
only; fan-in/fan-out quotas and import-graph edge realisation stay out of
scope, per the approach above.

At initial shipment, severity was Info, not Warning. Every then-oversized
file under `src/` already carried a valid marker (`src/cli/mod.rs`,
`src/map/query.rs`, `src/cli/render/remediate.rs`, `src/ui_assets/app.js`,
`src/ui_assets/style.css`, plus several `tests.rs` split-outs), so Warning
would have been safe against `src/` alone. But the check's scope is every
node-owned claimed file project-wide, matching `TargetReport.claimed_files`
rather than narrowing to the shell gate's `src/`-only discovery, and several
`cairn.tests`-owned files under `tests/` (`tests/kernel.rs`,
`tests/phase_9_brownfield.rs`, `tests/finding_code_coverage.rs`,
`tests/phase_7_7_ux_foundation.rs`, `tests/phase_7_6_ai_provenance.rs`,
`tests/phase_10_distribution.rs`, `tests/output_token_efficiency.rs`) were
then oversized and unmarked (the
shell gate never discovers `tests/`, so none carried a marker yet). Warning
would have failed `cairn scan --strict` on that checkout; Info kept the
signal visible without blocking. Confirmed via `cairn scan --strict` (no
`--json`, which routes through a different reply path that does not apply
`--strict` promotion): exit 0 with 7 `CAIRN_MODULE_OVERSIZED` Info findings
at that time.

Update (`todo.oversized-test-file-baseline`, 2026-07-17): the seven-file
baseline above cleared. Six files carry a `cairn:allow-large-module`
marker; the former `tests/output_token_efficiency.rs` was split at its
Part A/Part B seam into `tests/output_token_efficiency_findings.rs` and
`tests/output_token_efficiency_status_brief.rs`, both under the 500-line
limit with no marker needed. Severity is now Warning; `cairn scan --strict`
stays green on this clean checkout with zero `CAIRN_MODULE_OVERSIZED`
findings.

Wired into `cairn scan` and `cairn lint` automatically (both render
`graph.findings` generically). `cairn remediate` needed an explicit
classification arm: `remediate_json` only returns synthesised `actions`, not
raw findings, and the closest sibling (`CAIRN_TEST_COVERAGE_MISSING`) ships
with no such arm and is consequently invisible in `cairn remediate` today.
Since this todo explicitly requires remediate visibility, added a
`"CAIRN_MODULE_OVERSIZED"` arm and a priority-4 `split_module` action (same
tier as the other advisory-only actions) in
`src/query_api/handlers/remediate.rs`, rather than leaving it to fall
through the `_ => {}` catch-all like its sibling.

Tests in `src/map/module_size.rs` (`#[cfg(test)] mod tests`) cover: an
unmarked oversized file emitting the finding, a Rust-marker-suppressed file,
a CSS-block-marker-suppressed file, a vendor-pathed oversized file being
skipped, a file exactly at the 500-line boundary not firing, and a
non-policed extension (no marker syntax defined) being skipped.
`tests/finding_code_coverage.rs`'s meta-test passes: the code is registered
in `docs/registries/error-codes.md` (`CK031`) and asserted via
`f.code == "CAIRN_MODULE_OVERSIZED"` inside `find`/`contains` predicates.
