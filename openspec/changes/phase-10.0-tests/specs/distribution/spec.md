# Distribution Capability Spec (Phase 10.0 Delta)

## ADDED Requirements

### Requirement: Phase 10 acceptance criteria have failing ignored tests

All acceptance-criterion scenarios from `openspec/changes/phase-10-distribution/specs/distribution/spec.md` SHALL have a corresponding `#[cflx_planned(phase = 1000)]` test in `tests/phase_10_distribution.rs`. The test file SHALL compile and all tests SHALL be skipped (not failed) when `cargo test` runs without `--ignored`.

#### Scenario: Pre-phase archives with all tests ignored

- **GIVEN** `phase-10.0-tests` has been applied and `tests/phase_10_distribution.rs` is committed
- **WHEN** `cargo test` runs as part of the archive gate
- **THEN** all seven tests in `tests/phase_10_distribution.rs` are reported as ignored
- **AND** the gate exits zero

#### Scenario: Ignored tests are visible and runnable on demand

- **GIVEN** `phase-10.0-tests` has archived
- **WHEN** `cargo test -- --ignored` runs
- **THEN** all seven tests execute
- **AND** each fails with a missing-implementation error, not a compile error

#### Scenario: Phase 10 removes ignored attributes as criteria are met

- **GIVEN** `phase-10-distribution` has been applied
- **WHEN** `cargo test` runs without `--ignored`
- **THEN** all seven tests in `tests/phase_10_distribution.rs` run and pass
- **AND** no `#[cflx_planned(phase = 1000)]` attribute remains in that file
