# Summariser Capability Spec (Phase 8.0 Tests Addition)

## ADDED Requirements

### Requirement: Phase 8 acceptance criteria have failing ignored tests

Phase 8 acceptance-criterion scenarios SHALL each have a corresponding `#[test]` in `tests/phase_8_summariser.rs` marked `#[cflx_planned(phase = 800)]`. The pre-phase archives on a green `cargo test` because the tests are skipped. Phase 8 removes each `#[cflx_planned(phase = 800)]` attribute as the corresponding feature code lands.

#### Scenario: Pre-phase compiles and archives clean

- **GIVEN** `phase-8.0-tests` has been applied and `tests/phase_8_summariser.rs` is committed
- **WHEN** `cargo test` runs as part of the archive gate
- **THEN** the 12 new tests are reported as ignored
- **AND** the gate passes

#### Scenario: Ignored tests fail when run explicitly

- **GIVEN** `phase-8.0-tests` has been applied
- **WHEN** `cargo test -- --ignored` runs before phase-8 lands
- **THEN** all 12 tests in `tests/phase_8_summariser.rs` fail with `unimplemented!()` panics
- **AND** each failure message names its scenario

#### Scenario: All phase-8 scenarios have a corresponding test

- **GIVEN** `tests/phase_8_summariser.rs` is committed
- **WHEN** the file is inspected
- **THEN** it contains exactly one `#[test]` for each of the following scenarios:
  - `disabled_summariser_does_not_generate_draft`
  - `configured_backend_creates_pending_draft`
  - `local_command_protocol_is_stable`
  - `backend_failure_does_not_create_draft`
  - `accept_applies_draft_and_records_interface_hash`
  - `edit_writes_editable_file_without_applying`
  - `edited_accept_applies_editable_content`
  - `invalid_draft_accept_rolls_back`
  - `discard_leaves_contradiction_unresolved`
  - `generation_never_auto_applies`
  - `draft_query_tools_are_read_only_mcp_tools`
  - `draft_mutation_tools_require_mutating_mcp_mode`

#### Scenario: Phase 8 removes ignored attributes as features land

- **GIVEN** `phase-8-summariser` has been applied in full
- **WHEN** `cargo test` runs at the end of phase-8
- **THEN** no `#[cflx_planned(phase = 800)]` attribute remains in `tests/phase_8_summariser.rs`
- **AND** all 12 tests pass
