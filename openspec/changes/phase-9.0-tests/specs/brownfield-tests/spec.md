# Brownfield Tests Capability Spec

## ADDED Requirements

### Requirement: Phase 9 acceptance criteria have failing test coverage

Phase 9 Brownfield acceptance criterion scenarios SHALL each have a corresponding `#[ignore = "awaits phase-9"]` test in `tests/phase_9_brownfield.rs`. The pre-phase archives on a green `cargo test` because the tests are ignored. Phase 9 removes each `#[ignore]` attribute as the corresponding feature code lands. After the Wave 4 rescope, the brownfield spec carries 8 requirements with 23 acceptance-criterion scenarios; this requirement covers all of them plus the 7 heuristic invariants from `phase-9-brownfield/design.md`.

#### Scenario: Pre-phase archives green with ignored brownfield tests

- **GIVEN** `phase-9.0-tests` has been applied and `tests/phase_9_brownfield.rs` exists with all tests marked `#[ignore = "awaits phase-9"]`
- **WHEN** `cargo test` runs
- **THEN** all brownfield tests are skipped
- **AND** the gate passes with zero failures

#### Scenario: Ignored tests are present and enumerable

- **GIVEN** `phase-9.0-tests` has been applied
- **WHEN** `cargo test -- --ignored 2>&1` runs
- **THEN** the output lists at least 30 test names from `tests/phase_9_brownfield.rs`
- **AND** each listed test fails (not errors) because the feature code does not yet exist
- **AND** stubs that depend on Phase 9 fixtures fail via `unimplemented!()` runtime panic rather than compile error

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
  - `suggest__engine_writes_to_queue_file`
  - `suggest__entry_triage_state_is_pending`
  - `suggest__entry_provenance_carries_trace_phase`
  - `suggest__pending_entries_block_archive_with_cc002`
  - `suggest__no_auto_accept_on_high_confidence`
  - `suggest__manual_test_entries_accept_empty_provenance`
  - `interview__session_persists_across_invocations`
  - `interview__final_transcript_lands_at_genesis_path`
  - `interview__session_state_never_leaks_outside_change_dir`
  - `templates__matching_template_guides_stub_authoring`
  - `templates__non_matching_candidates_fall_back_to_builtin`
  - `templates__ill_formed_template_does_not_block_authoring`
  - `obligations__populated_when_field_exists`
  - `obligations__reviewable_before_archive`
  - `obligations__no_op_when_field_absent`
  - `heuristics__coupling_score_high_confidence`
  - `heuristics__coupling_score_medium_confidence`
  - `heuristics__coupling_score_low_confidence`
  - `heuristics__directory_candidate_min_three_files`
  - `heuristics__directory_depth_limit_four`
  - `heuristics__edge_threshold_two_import_observations`
  - `heuristics__summariser_disabled_uses_path_derived_names`

#### Scenario: Wave 4 stubs preserve failing-state contract without compile errors

- **GIVEN** `phase-9.0-tests` has been applied and the Wave 4 stubs (suggest, interview, templates, obligations) are present
- **WHEN** `cargo build` runs
- **THEN** the build passes with zero warnings
- **AND** stubs lacking Phase 9 fixtures compile through `unimplemented!()` so they fail at runtime when run with `--ignored`, not at compile time

#### Scenario: Conditional obligations stubs reflect current schema state

- **GIVEN** the decision artefact schema in this phase does not currently expose an `obligations` field
- **WHEN** `cargo test -- --ignored` runs after `phase-9.0-tests` has applied
- **THEN** `obligations__populated_when_field_exists` and `obligations__reviewable_before_archive` panic via `unimplemented!()` until Phase 9 either adds the field or confirms it stays absent
- **AND** `obligations__no_op_when_field_absent` asserts the no-op rider directly and is the always-callable branch
