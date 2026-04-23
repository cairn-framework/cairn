# Design: Phase 10.0 Distribution Tests

## References

- `openspec/changes/phase-10-distribution/specs/distribution/spec.md`: canonical acceptance criteria.
- `openspec/changes/phase-10-distribution/proposal.md`: parent phase problem/solution summary.
- `openspec/changes/phase-10-distribution/design.md`: LSP, packaging, reconciler, and release design.
- `openspec/specs/testing-baseline/spec.md`: "Requirement: Test-first pre-phase convention."
- `openspec/changes/archive/phase-7.5a-test-fortification/`: prior-art pre-phase for style.

## Test File Placement

All tests go into a single new integration test file: `tests/phase_10_distribution.rs`.

Integration test placement is preferred over inline `#[cfg(test)]` modules because the modules phase-10 will touch (`cairn-lsp`, reconciler extension, release checks) do not yet exist. An integration test file compiles against the current public API without requiring the target modules to be present, so the pre-phase archives cleanly.

Each test carries:

```rust
#[test]
#[ignore = "awaits phase-10"]
fn <name>() { ... }
```

## Acceptance Criteria Coverage

Seven scenarios from the phase-10 spec map to tests as follows:

| # | Spec scenario | Test function |
|---|---|---|
| 1 | LSP binary root is strict | `lsp_binary_is_workspace_member_with_workspace_lints` |
| 2 | Diagnostics match lint | `lsp_diagnostics_match_cairn_lint_json` |
| 3 | Hover returns node context | `lsp_hover_returns_node_metadata` |
| 4 | Definition resolves edge endpoint | `lsp_definition_resolves_to_node_declaration_span` |
| 5 | Claude Code setup is documented | `plugin_docs_cover_cli_mcp_and_project_context` |
| 6 | Example project exercises major capabilities | `example_project_exercises_all_listed_capabilities` |
| 7 | Fixture reconciler contributes observations | `fixture_reconciler_observations_enter_map_without_new_nodes` |

## Test Strategy Per Assertion

**Test 1: LSP binary workspace membership.**
Parse workspace `Cargo.toml` and assert `cairn-lsp` appears in `workspace.members`. Parse `cairn-lsp/Cargo.toml` and assert `[lints] workspace = true` is present. Read `cairn-lsp/src/main.rs` and assert it contains no `#![deny(...)]` or `#![allow(...)]` crate-level lint attributes.

**Test 2: Diagnostic parity.**
Using the bootstrap fixture at `test/fixtures/cairn-bootstrap/`, invoke `cairn lint --json` via `std::process::Command`. Invoke the LSP `textDocument/diagnostic` request against the same fixture via `cairn-lsp`. Assert finding codes returned by both match as an order-independent set.

**Test 3: Hover content.**
Using the bootstrap fixture, invoke LSP `textDocument/hover` on a known node ID declared in the fixture. Assert the response contains non-empty fields for name, description, state, paths, artefact count, and findings summary.

**Test 4: Go-to-definition.**
Using the bootstrap fixture, invoke LSP `textDocument/definition` on a node ID used in an edge. Assert the returned location points to the blueprint file at the line of that node's declaration.

**Test 5: Plugin docs completeness.**
Assert a distribution documentation file (e.g. `docs/distribution.md` or `docs/agent-integration.md`) exists under `docs/`. Assert it contains the strings `cairn`, `cairn-mcp`, and coverage of project context or rules composition.

**Test 6: Example project coverage.**
Assert the example project directory (e.g. `examples/full-project/`) exists. Assert its configuration or scripts reference: blueprint parse, MCP startup, summariser disabled/default config, brownfield fixture generation, LSP diagnostics, and fixture reconciler observations. String-presence checks against the example files are sufficient for the pre-phase assertion; phase-10 makes them execute correctly.

**Test 7: Fixture reconciler.**
Run `cairn scan` via `std::process::Command` on a fixture project that includes the fixture non-code reconciler registered. Assert the scan output contains at least one observation attributed to the fixture reconciler. Assert that `cairn map` output contains no node whose origin is the fixture reconciler (new nodes must go through the brownfield workflow).

## Flagged Under-Specified Criteria

The following capabilities appear in the phase-10 proposal and design but have no scenario in `specs/distribution/spec.md`. No tests are written for them in this pre-phase. Phase-10 should add scenarios to the spec or explicitly mark them out of scope for acceptance testing before implementation begins.

- **Autocomplete** (proposal AC: "LSP autocomplete suggests node IDs, artefact IDs, and command-relevant symbols"). No scenario specifies what constitutes a passing completion response, what fixture triggers it, or what the expected suggestion set is.
- **Document symbols** (design: "Document symbols for systems, containers, modules, actors, and edges"). No scenario specifies expected symbol kinds, counts, or fixture.
- **Shell completions** (release checks task 4.1). No scenario asserts which shells are supported or what the completion output must contain.
- **Manpage/command reference** (release checks task 4.2). No scenario asserts coverage or format requirements.

## Out of Scope

- Any production code.
- Snapshot files (`*.snap`). Tests in this pre-phase use assertion-based checks because the LSP and reconciler APIs do not yet exist.
- Removing `#[ignore]` attributes. Phase-10 owns that step.
