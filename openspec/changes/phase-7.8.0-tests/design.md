# Design: Phase 7.8.0 Tests (Cairn Export Pre-Phase)

## References

- `openspec/changes/phase-7.8-cairn-export/specs/cli/spec.md`: source of the 8 scenarios.
- `openspec/specs/testing-baseline/spec.md`: test-first pre-phase convention.
- `openspec/conventions.md`: `#[ignore = "awaits phase-<N>"]` pattern.

## Test File Placement

A single new integration test file: `tests/phase_7_8_cairn_export.rs`.

Integration test files under `tests/` are compiled as separate crates by Cargo. This placement:

- Keeps the pre-phase delta self-contained and easy to diff.
- Avoids touching any `src/` file before phase-7.8 lands.
- Matches the convention established by existing files under `tests/`.

Phase-7.8 will either flesh out these test bodies in-place or move individual tests closer to the modules they exercise, removing `#[ignore]` as each lands.

## Test Body Convention

Every test body is `todo!("awaits phase-7.8: <scenario name>")`. This ensures:

- The file compiles.
- Running with `--ignored` produces a clear failure message naming the missing scenario.
- Grepping for `todo!` after phase-7.8 apply confirms no scenario was silently dropped.

## Scenario-to-Test Mapping

All tests live in a single flat module (no `mod` blocks needed because there is only one requirement).

| # | Spec scenario | Test function |
|---|---|---|
| 1 | Default format is JSON | `default_format_is_json` |
| 2 | Markdown format is selected via flag | `markdown_format_selected_via_flag` |
| 3 | JSON envelope carries a schema version | `json_envelope_carries_schema_version` |
| 4 | Markdown payload contains no em-dashes | `markdown_payload_contains_no_em_dashes` |
| 5 | Output flag is required | `output_flag_is_required` |
| 6 | Invalid format value is rejected | `invalid_format_value_is_rejected` |
| 7 | Export is lifecycle-orthogonal | `export_is_lifecycle_orthogonal` |
| 8 | Render delegates to a shared library service | `render_delegates_to_shared_library_service` |

## Compile Dependency

The test file imports nothing from `src/` at pre-phase time. Any use statements needed by the eventual implementations are added by phase-7.8 as it removes `#[ignore]` attributes.

## Phase-7.8 Removal Contract

Phase-7.8 tasks.md specifies that each task removes `#[ignore]` from the corresponding test and makes it pass. The mapping here (8 tests, 1 requirement) aligns directly with phase-7.8 task groups.

## Vague Scenario Flagged

None. All 8 scenarios are precise enough to assert unambiguously once the export command and shared library service exist.
