# AI Provenance Foundation Tests Capability Spec

## ADDED Requirements

### Requirement: Phase 7.6 acceptance criteria have failing ignored tests

Phase 7.6 acceptance-criterion scenarios SHALL each have a corresponding `#[test]` in `tests/phase_7_6_ai_provenance.rs` marked `#[cflx_planned(phase = 706)]`. The pre-phase archives on a green `cargo test` because the tests are skipped. Phase 7.6 removes each `#[cflx_planned]` attribute as the corresponding feature code lands.

#### Scenario: Pre-phase compiles and archives clean

- **GIVEN** `phase-7.6.0-tests` has been applied and `tests/phase_7_6_ai_provenance.rs` is committed
- **WHEN** `cargo test` runs as part of the archive gate
- **THEN** the 24 new tests are reported as ignored
- **AND** the gate passes

#### Scenario: Ignored tests fail when run explicitly

- **GIVEN** `phase-7.6.0-tests` has been applied
- **WHEN** `cargo test -- --ignored` runs before phase-7.6 lands
- **THEN** all 24 tests in `tests/phase_7_6_ai_provenance.rs` fail with `unimplemented!()` panics
- **AND** each failure message names its scenario

#### Scenario: All phase-7.6 scenarios have a corresponding test

- **GIVEN** `tests/phase_7_6_ai_provenance.rs` is committed
- **WHEN** the file is inspected
- **THEN** it contains exactly one `#[test]` for each of the following scenarios:
  - `test_sidecar_is_state_versioned`
  - `test_sidecar_covers_four_native_stages`
  - `test_prompt_content_reserved_but_empty`
  - `test_higher_version_fails_with_clear_error`
  - `test_trace_human_output_labels_each_stage`
  - `test_trace_json_output_is_schema_with_version`
  - `test_trace_missing_sidecar_exits_cleanly`
  - `test_trace_command_delegates_to_library_reader`
  - `test_queue_file_is_state_versioned`
  - `test_entry_carries_source_target_relation_and_triage_state`
  - `test_triage_state_defaults_to_pending`
  - `test_queue_is_sibling_not_delta_operation`
  - `test_validate_without_strict_surfaces_warning`
  - `test_validate_strict_fails_cc002_on_pending`
  - `test_validate_strict_passes_when_all_non_pending`
  - `test_absent_queue_file_is_not_error`
  - `test_islands_returns_component_breakdown`
  - `test_islands_json_output_is_versioned`
  - `test_neighbourhood_include_orphans_surfaces_reverse_only`
  - `test_both_forms_delegate_to_library_query`
  - `test_query_islands_returns_one_entry_per_component`
  - `test_query_islands_handles_single_component`
  - `test_query_neighbourhood_include_orphans_surfaces_inbound_only`
  - `test_query_islands_response_is_versioned`

#### Scenario: Phase 7.6 removes ignored attributes as features land

- **GIVEN** `phase-7.6-ai-provenance-foundation` has been applied in full
- **WHEN** `cargo test` runs at the end of phase-7.6
- **THEN** no `#[cflx_planned(phase = 706)]` attribute remains in `tests/phase_7_6_ai_provenance.rs`
- **AND** all 24 tests pass
