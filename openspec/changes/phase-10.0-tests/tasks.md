# Tasks: Phase 10.0 Distribution Tests

## 1. LSP Workspace Membership Test

- [ ] 1.1 Create `tests/phase_10_distribution.rs` with a module-level comment referencing this pre-phase and the convention at `openspec/specs/testing-baseline/spec.md`.
- [ ] 1.2 Add `#[cflx_planned(phase = 1000)]` test `lsp_binary_is_workspace_member_with_workspace_lints`: parse workspace `Cargo.toml`, assert `cairn-lsp` in `workspace.members`; parse `cairn-lsp/Cargo.toml`, assert `[lints] workspace = true`; read `cairn-lsp/src/main.rs`, assert no crate-level `#![deny]` or `#![allow]` attributes.

## 2. LSP Diagnostic Parity Test

- [ ] 2.1 Add `#[cflx_planned(phase = 1000)]` test `lsp_diagnostics_match_cairn_lint_json`: run `cairn lint --json` against `test/fixtures/cairn-bootstrap/` via `std::process::Command`; send `textDocument/diagnostic` to `cairn-lsp` against the same fixture; assert finding-code sets match order-independently.

## 3. LSP Hover Test

- [ ] 3.1 Add `#[cflx_planned(phase = 1000)]` test `lsp_hover_returns_node_metadata`: open the bootstrap fixture with `cairn-lsp`; request hover on a known node ID; assert the response fields for name, description, state, paths, artefact count, and findings summary are all non-empty.

## 4. LSP Go-to-Definition Test

- [ ] 4.1 Add `#[cflx_planned(phase = 1000)]` test `lsp_definition_resolves_to_node_declaration_span`: request definition for a node ID used in an edge in the bootstrap fixture; assert the returned location points to the line of the node declaration in the blueprint file.

## 5. Plugin Documentation Test

- [ ] 5.1 Add `#[cflx_planned(phase = 1000)]` test `plugin_docs_cover_cli_mcp_and_project_context`: assert a distribution documentation file exists under `docs/`; assert it contains `cairn`, `cairn-mcp`, and coverage of project context or rules composition.

## 6. Example Project Coverage Test

- [ ] 6.1 Add `#[cflx_planned(phase = 1000)]` test `example_project_exercises_all_listed_capabilities`: assert the example project directory exists; assert its configuration or scripts reference blueprint parse, MCP startup, summariser disabled/default, brownfield fixture generation, LSP diagnostics, and fixture reconciler observations.

## 7. Fixture Reconciler Test

- [ ] 7.1 Add `#[cflx_planned(phase = 1000)]` test `fixture_reconciler_observations_enter_map_without_new_nodes`: run `cairn scan` with the fixture non-code reconciler registered; assert at least one observation from the fixture reconciler appears in map output; assert no node in map output has the fixture reconciler as its origin.

## 8. Required Verification

- [ ] 8.1 `cargo fmt --check` passes.
- [ ] 8.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 8.3 `cargo test` passes (all seven tests skipped as ignored).
- [ ] 8.4 `cargo test -- --ignored` runs all seven tests; each fails for a missing-implementation reason, not a compile error.
- [ ] 8.5 `bash scripts/pre-archive-rust-gates.sh` passes end-to-end.
