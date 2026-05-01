# Design: Phase 7.7.0 Tests (UX Foundation Pre-Phase)

## References

- `openspec/changes/phase-7.7-ux-foundation/specs/cli/spec.md`: source of scenarios 1–7.
- `openspec/changes/phase-7.7-ux-foundation/specs/graph-explorer/spec.md`: source of scenarios 8–24.
- `openspec/changes/phase-7.7-ux-foundation/specs/reconciliation/spec.md`: source of scenarios 25–29.
- `openspec/specs/testing-baseline/spec.md`: test-first pre-phase convention.
- `openspec/conventions.md`: `#[ignore = "awaits phase-<N>"]` pattern.

## Test File Placement

A single new integration test file: `tests/phase_7_7_ux_foundation.rs`.

Integration test files under `tests/` are compiled as separate crates by Cargo. This placement:

- Keeps the pre-phase delta self-contained and easy to diff.
- Avoids touching any `src/` file before phase-7.7 lands.
- Matches the convention established by existing files under `tests/`.

Phase-7.7 will either flesh out these test bodies in-place or move individual tests closer to the modules they exercise, removing `#[ignore]` as each group lands.

## Test Body Convention

Every test body is `todo!("awaits phase-7.7: <scenario name>")`. This ensures:

- The file compiles.
- Running with `--ignored` produces a clear failure message naming the missing scenario.
- Grepping for `todo!` after phase-7.7 apply confirms no scenario was silently dropped.

## Scenario-to-Test Mapping

Three modules within the file, one per spec area, using `mod` blocks for grouping:

```
mod cli { ... }              // Requirement: cairn check inspection subcommand
mod empty_state { ... }      // Requirement: Empty-state CTAs name the next move
mod explorer { ... }         // Requirement: Graph Explorer UX surfaces
mod reconciliation { ... }   // Requirement: Info-severity findings for advisory states
```

| # | Spec scenario | Test function |
|---|---|---|
| 1 | Whole-map inspection without arguments | `check__whole_map_inspection_without_arguments` |
| 2 | Node-scoped inspection with a positional argument | `check__node_scoped_inspection_with_positional_argument` |
| 3 | Inspection delegates to the same library service as lint | `check__inspection_delegates_to_same_library_service_as_lint` |
| 4 | Inspection has no JSON mode in this phase | `check__inspection_has_no_json_mode` |
| 5 | No-blueprint invocation renders a CTA | `empty_state__no_blueprint_invocation_renders_cta` |
| 6 | Clean-map result renders a CTA | `empty_state__clean_map_result_renders_cta` |
| 7 | Empty-state copy is free of em-dashes (CLI) | `empty_state__copy_has_no_em_dashes` |
| 8 | Component is defined with token-only styling | `explorer__empty_state_component_uses_token_only_styling` |
| 9 | All ten inline empty-state strings are replaced | `explorer__ten_inline_empty_state_strings_replaced` |
| 10 | Missing copy keys surface a console warning | `explorer__missing_copy_keys_surface_console_warning` |
| 11 | Empty-state copy is free of em-dashes (webui) | `empty_state__copy_has_no_em_dashes` (shared with scenario 7) |
| 12 | Three severity buckets render with count badges | `explorer__three_severity_buckets_render_with_count_badges` |
| 13 | Scope toggle filters to the selected node | `explorer__scope_toggle_filters_to_selected_node` |
| 14 | Scope toggle is disabled when no node is selected | `explorer__scope_toggle_disabled_when_no_node_selected` |
| 15 | Category filter chips derive from the finding stream | `explorer__category_filter_chips_derive_from_finding_stream` |
| 16 | Panel reads only from the query-consumer API | `explorer__panel_reads_only_from_query_consumer_api` |
| 17 | Banner renders the highest-severity finding's nudge | `explorer__banner_renders_highest_severity_finding_nudge` |
| 18 | Tie-break by lowest-numbered code | `explorer__banner_tie_break_by_lowest_numbered_code` |
| 19 | Banner CTA is a copy-pasteable CLI snippet | `explorer__banner_cta_is_copy_pasteable_cli_snippet` |
| 20 | Banner is hidden when the node has no findings | `explorer__banner_hidden_when_node_has_no_findings` |
| 21 | Structural error indicator (integrity overlay) | `explorer__structural_error_indicator` |
| 22 | Interface contradiction indicator (integrity overlay) | `explorer__interface_contradiction_indicator` |
| 23 | Rationale tension indicator (integrity overlay) | `explorer__rationale_tension_indicator` |
| 24 | Info-severity findings appear in the overlay | `explorer__info_severity_findings_appear_in_overlay` |
| 25 | Info variant is defined on the kernel enum | `reconciliation__info_variant_defined_on_kernel_enum` |
| 26 | Orphaned-file state emits an Info finding | `reconciliation__orphaned_file_emits_info_finding` |
| 27 | Unverified-contract state emits an Info finding | `reconciliation__unverified_contract_emits_info_finding` |
| 28 | Info findings do not block hooks or gates | `reconciliation__info_findings_do_not_block_hooks_or_gates` |
| 29 | Info findings round-trip through serde_json | `reconciliation__info_findings_round_trip_through_serde_json` |

Total: 28 unique test functions covering 29 scenarios. Scenarios 7 and 11 assert the same property on the same centralised copy file (`docs/design-system/copy.toml`) and share one test.

## Compile Dependency

The test file imports nothing from `src/` at pre-phase time. Any use statements needed by the eventual implementations are added by phase-7.7 as it removes `#[ignore]` attributes.

## Phase-7.7 Removal Contract

Phase-7.7 tasks.md specifies that the first task in each group removes `#[ignore]` from the corresponding tests and makes them pass. The grouping here (4 mods, 28 tests) aligns directly with phase-7.7 task groups.

## Vague Scenario Flagged

The spec scenario "Component is defined with token-only styling" (Graph Explorer, scenario 8) asserts that "the count of hardcoded six-digit hex values in `components.css` matches the count from before this phase." This is difficult to assert in a Rust integration test because it requires parsing CSS and counting hex values. The pre-phase test asserts only that the empty-state component class exists in `components.css` and that no new hex values appear in the component's rule block. Phase-7.7 should add a design-system-level assertion or a snapshot test for the hex-value count if exact parity is required.
