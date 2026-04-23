# Brownfield Tests Capability Spec

## ADDED Requirements

### Requirement: Phase 9 acceptance criteria have failing test coverage

Phase 9 Brownfield acceptance criterion scenarios SHALL each have a corresponding `#[ignore = "awaits phase-9"]` test in `tests/phase_9_brownfield.rs`. The pre-phase archives on a green `cargo test` because the tests are ignored. Phase 9 removes each `#[ignore]` attribute as the corresponding feature code lands.

#### Scenario: Pre-phase archives green with ignored brownfield tests

- **GIVEN** `phase-9.0-tests` has been applied and `tests/phase_9_brownfield.rs` exists with all tests marked `#[ignore = "awaits phase-9"]`
- **WHEN** `cargo test` runs
- **THEN** all brownfield tests are skipped
- **AND** the gate passes with zero failures

#### Scenario: Ignored tests are present and enumerable

- **GIVEN** `phase-9.0-tests` has been applied
- **WHEN** `cargo test -- --ignored 2>&1` runs
- **THEN** the output lists at least 17 test names from `tests/phase_9_brownfield.rs`
- **AND** each listed test fails (not errors) because the feature code does not yet exist

#### Scenario: Phase 9 removes ignore attributes as criteria are met

- **GIVEN** `phase-9.0-tests` has archived and Phase 9 implements a feature group
- **WHEN** Phase 9 removes `#[ignore = "awaits phase-9"]` from the corresponding tests and runs `cargo test`
- **THEN** those tests pass
- **AND** remaining ignored tests are still skipped

#### Scenario: All brownfield scenarios have test coverage

- **GIVEN** `phase-9.0-tests` has been applied
- **WHEN** the test file is inspected
- **THEN** a test exists for each of the following criteria:
  - `init__discovery_does_not_require_existing_blueprint`
  - `init__candidate_heuristics_are_deterministic`
  - `init__creates_brownfield_change_directory`
  - `init__existing_change_protected_without_force`
  - `init__force_replaces_existing_change`
  - `refine__proposes_additions_for_new_directories`
  - `refine__does_not_replace_current_truth`
  - `review__false_positive_deletion_respected`
  - `mcp__brownfield_tools_absent_in_default_mode`
  - `mcp__brownfield_tools_present_in_mutating_mode`
  - `heuristics__coupling_score_high_confidence`
  - `heuristics__coupling_score_medium_confidence`
  - `heuristics__coupling_score_low_confidence`
  - `heuristics__directory_candidate_min_three_files`
  - `heuristics__directory_depth_limit_four`
  - `heuristics__edge_threshold_two_import_observations`
  - `heuristics__summariser_disabled_uses_path_derived_names`
