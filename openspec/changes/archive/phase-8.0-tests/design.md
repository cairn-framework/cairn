# Design: Phase 8.0 Tests (Summariser Pre-Phase)

## References

- `openspec/changes/phase-8-summariser/specs/summariser/spec.md`: source of the 12 scenarios.
- `openspec/specs/testing-baseline/spec.md`: test-first pre-phase convention.
- `openspec/conventions.md`: `#[cflx_planned(phase = <N>)]` pattern.

## Test File Placement

A single new integration test file: `tests/phase_8_summariser.rs`.

Integration test files under `tests/` are compiled as separate crates by Cargo. This placement:

- Keeps the pre-phase delta self-contained and easy to diff.
- Avoids touching any `src/` file before phase-8 lands.
- Matches the convention established by existing files under `tests/`.

Phase-8 will either flesh out these test bodies in-place or move individual tests closer to the modules they exercise, removing `#[cflx_planned(phase = 800)]` as each group lands.

## Test Body Convention

Every test body is `unimplemented!("awaits phase-8: <scenario name>")`. This ensures:

- The file compiles.
- Running with `--ignored` produces a clear failure message naming the missing scenario.
- Grepping for `todo!` after phase-8 apply confirms no scenario was silently dropped.

## Scenario-to-Test Mapping

Three modules within the file, one per requirement, using `mod` blocks for grouping:

```
mod backend_and_config { ... }   // Requirement: Generate drafts through an optional backend
mod resolution_actions { ... }   // Requirement: Require explicit draft resolution
mod mcp_exposure { ... }         // Requirement: Expose summariser commands through MCP
```

Each `mod` contains the tests for its requirement's scenarios. All tests carry both `#[test]` and `#[cflx_planned(phase = 800)]`.

## Compile Dependency

The test file imports nothing from `src/` at pre-phase time. Any use statements needed by the eventual implementations are added by phase-8 as it removes `#[cflx_planned(phase = 800)]` attributes.

## Phase-8 Removal Contract

Phase-8 tasks.md specifies that the first task in each group removes `#[cflx_planned(phase = 800)]` from the corresponding test and makes it pass. The grouping here (3 mods, 12 tests) aligns directly with phase-8 task groups.

## Vague Scenario Flagged

The spec scenario "Configured backend creates draft" (Req 1, scenario 2) says Cairn "builds grounded prompt inputs" without specifying what fields are required. The pre-phase test asserts only that a draft file is created under `.cairn/state/summariser/` and that its `status` field equals `"pending"`. Phase-8 should add a sub-scenario specifying the minimum required fields in a stored draft's `prompt_inputs` reference, or the test will remain a coarse existence check.
