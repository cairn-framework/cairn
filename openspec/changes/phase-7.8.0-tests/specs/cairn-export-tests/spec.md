# Cairn Export Capability Spec (Phase 7.8.0 Tests Addition)

## ADDED Requirements

### Requirement: Phase 7.8 acceptance criteria have failing ignored tests

Phase 7.8 acceptance-criterion scenarios SHALL each have a corresponding `#[test]` in `tests/phase_7_8_cairn_export.rs` marked `#[ignore = "awaits phase-7.8"]`. The pre-phase archives on a green `cargo test` because the tests are skipped. Phase 7.8 removes each `#[ignore]` attribute as the corresponding feature code lands.

#### Scenario: Pre-phase compiles and archives clean

- **GIVEN** `phase-7.8.0-tests` has been applied and `tests/phase_7_8_cairn_export.rs` is committed
- **WHEN** `cargo test` runs as part of the archive gate
- **THEN** the 8 new tests are reported as ignored
- **AND** the gate passes

#### Scenario: Ignored tests fail when run explicitly

- **GIVEN** `phase-7.8.0-tests` has been applied
- **WHEN** `cargo test -- --ignored` runs before phase-7.8 lands
- **THEN** all 8 tests in `tests/phase_7_8_cairn_export.rs` fail with `todo!()` panics
- **AND** each failure message names its scenario

#### Scenario: All phase-7.8 scenarios have a corresponding test

- **GIVEN** `tests/phase_7_8_cairn_export.rs` is committed
- **WHEN** the file is inspected
- **THEN** it contains exactly one `#[test]` for each of the following scenarios:
  - `default_format_is_json`
  - `markdown_format_selected_via_flag`
  - `json_envelope_carries_schema_version`
  - `markdown_payload_contains_no_em_dashes`
  - `output_flag_is_required`
  - `invalid_format_value_is_rejected`
  - `export_is_lifecycle_orthogonal`
  - `render_delegates_to_shared_library_service`

#### Scenario: Phase 7.8 removes ignored attributes as features land

- **GIVEN** `phase-7.8-cairn-export` has been applied in full
- **WHEN** `cargo test` runs at the end of phase-7.8
- **THEN** no `#[ignore = "awaits phase-7.8"]` attribute remains in `tests/phase_7_8_cairn_export.rs`
- **AND** all 8 tests pass
