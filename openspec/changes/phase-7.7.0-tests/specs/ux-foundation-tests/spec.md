# UX Foundation Tests Capability Spec (Phase 7.7.0 Tests)

## ADDED Requirements

### Requirement: Phase 7.7 acceptance criteria have failing planned tests

Phase 7.7 UX Foundation acceptance-criterion scenarios SHALL each have a corresponding `#[test]` in `tests/phase_7_7_ux_foundation.rs` marked `#[cflx_planned(phase = 77)]`. The pre-phase archives on a green `cargo test` because the tests are skipped. Phase 7.7 removes each `#[cflx_planned]` attribute as the corresponding feature code lands.

#### Scenario: Pre-phase compiles and archives clean

- **GIVEN** `phase-7.7.0-tests` has been applied and `tests/phase_7_7_ux_foundation.rs` is committed
- **WHEN** `cargo test` runs as part of the archive gate
- **THEN** the 28 new tests are reported as ignored
- **AND** the gate passes

#### Scenario: Ignored tests fail when run explicitly

- **GIVEN** `phase-7.7.0-tests` has been applied
- **WHEN** `cargo test -- --ignored` runs before phase-7.7 lands
- **THEN** all 28 tests in `tests/phase_7_7_ux_foundation.rs` fail with `unimplemented!()` panics
- **AND** each failure message names its scenario

#### Scenario: All phase-7.7 scenarios have a corresponding test

- **GIVEN** `tests/phase_7_7_ux_foundation.rs` is committed
- **WHEN** the file is inspected
- **THEN** it contains exactly one `#[test]` for each of the following scenarios (28 unique test functions covering 29 scenarios; `empty_state__copy_has_no_em_dashes` covers both CLI scenario 7 and Graph Explorer scenario 11):
  - `check__whole_map_inspection_without_arguments`
  - `check__node_scoped_inspection_with_positional_argument`
  - `check__inspection_delegates_to_same_library_service_as_lint`
  - `check__inspection_has_no_json_mode`
  - `empty_state__no_blueprint_invocation_renders_cta`
  - `empty_state__clean_map_result_renders_cta`
  - `empty_state__copy_has_no_em_dashes`
  - `explorer__empty_state_component_uses_token_only_styling`
  - `explorer__ten_inline_empty_state_strings_replaced`
  - `explorer__missing_copy_keys_surface_console_warning`
  - `explorer__three_severity_buckets_render_with_count_badges`
  - `explorer__scope_toggle_filters_to_selected_node`
  - `explorer__scope_toggle_disabled_when_no_node_selected`
  - `explorer__category_filter_chips_derive_from_finding_stream`
  - `explorer__panel_reads_only_from_query_consumer_api`
  - `explorer__banner_renders_highest_severity_finding_nudge`
  - `explorer__banner_tie_break_by_lowest_numbered_code`
  - `explorer__banner_cta_is_copy_pasteable_cli_snippet`
  - `explorer__banner_hidden_when_node_has_no_findings`
  - `explorer__structural_error_indicator`
  - `explorer__interface_contradiction_indicator`
  - `explorer__rationale_tension_indicator`
  - `explorer__info_severity_findings_appear_in_overlay`
  - `reconciliation__info_variant_defined_on_kernel_enum`
  - `reconciliation__orphaned_file_emits_info_finding`
  - `reconciliation__unverified_contract_emits_info_finding`
  - `reconciliation__info_findings_do_not_block_hooks_or_gates`
  - `reconciliation__info_findings_round_trip_through_serde_json`

#### Scenario: Phase 7.7 removes ignored attributes as features land

- **GIVEN** `phase-7.7-ux-foundation` has been applied in full
- **WHEN** `cargo test` runs at the end of phase-7.7
- **THEN** no `#[cflx_planned(phase = 77)]` attribute remains in `tests/phase_7_7_ux_foundation.rs`
- **AND** all 28 tests pass
