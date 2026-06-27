---
node: cairn.tests
---

# Contract: cairn.tests

## Purpose

The repository integration and smoke test suite under `tests/`. It is the
cross-module verification surface that exercises whole subsystems end to end,
complementing the in-crate `#[cfg(test)]` unit tests owned by each module. These
tests drive the kernel, reconcilers, hooks, MCP server, and web surfaces against
real fixtures and assert behaviour that no single module can verify in isolation.

## Public interface

This node has no runtime API; its surface is the set of integration test entry
points (each `tests/*.rs` file is a separately compiled test binary). The main
categories are:

- Kernel and graph: `kernel.rs`, `graph_explorer.rs`, `blueprint_lexer.rs`,
  `blueprint_parser.rs`, `wire_format_snapshots.rs` (with `snapshots/`),
  `scanner_interface_hash.rs`.
- Reconcile per language: `reconcile_rust.rs`, `reconcile_typescript.rs`,
  `reconcile_python.rs`, `reconcile_go.rs`, `reconcile_target_fingerprint.rs`.
- Hooks and MCP: `hooks_architecture.rs`, `mcp.rs`.
- Artefacts and conventions: `artefacts_contract.rs`,
  `artefacts_frontmatter.rs`, `conventions.rs`, `decision_claims.rs`,
  `command_reference_consistency.rs`, `gitignore_lint.rs`,
  `check_file_sizes.rs`.
- Phased feature suites: `phase_7_6_ai_provenance.rs`, `phase_7_7_ux_foundation.rs`,
  `phase_7_8_cairn_export.rs`, `phase_8_summariser.rs`, `phase_9_brownfield.rs`,
  `phase_10_distribution.rs`.
- Web surfaces and smoke: `check_a11y.rs`, `check_design_tokens.rs`,
  `landing_assets.rs`, `ui_mobile.rs`, `watch.rs`, `fixtures_smoke.rs`.

## Invariants

- Each file is an independent test target; failures are isolated per binary.
- Snapshot assertions compare against committed `tests/snapshots/*.snap` files,
  pinning the wire format and API output shapes.
- Tests requiring sample projects build them from `tests/fixtures/`.
- Tests for unshipped phases are queued via `#[cairn_planned(phase = N)]` from
  `cairn.macros` so the suite stays green until the phase implementation lands.

## Dependencies

Leaf node with no outgoing blueprint edges. As a verification harness it links
the library crate under test and, for queued phase tests, the `cairn.macros`
crate. Tagged `@test` (it is the test surface) and `@no-test-coverage`
(integration tests are not themselves measured by the coverage gate).

## Tests

This node is the test suite; it has no separate tests of its own. Its categories
above constitute the integration coverage that backs the other blueprint nodes.
