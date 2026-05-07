# Distribution Tests Capability Spec

## Purpose

Records the test-first contract that phase 10 (Distribution) ships
against. Asserts that every phase-10 acceptance-criterion scenario has a
corresponding `#[cflx_planned(phase = 1000)]` stub in
`tests/phase_10_distribution.rs`. The parent phase removes these
attributes group-by-group as the underlying LSP / packaging /
reconciler-extension code lands.

This meta-spec is retained as historical record once phase-10 archives
and may be removed manually at that time; cflx has no automated
retire-meta-spec step today.

The archived delta lives at
`openspec/changes/archive/phase-10.0-tests/specs/distribution/spec.md`
under the legacy area name `distribution`. Re-running cflx's archive
consolidation against that delta would recreate
`openspec/specs/distribution/`; future maintainers should consolidate
the two manually rather than re-applying.

## Requirements

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
