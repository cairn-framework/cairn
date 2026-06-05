# Testing Baseline Capability Spec

## MODIFIED Requirements

### Requirement: Test-first pre-phase convention

Feature phases that introduce new acceptance criteria SHOULD be preceded by a paired pre-phase `phase-<N>.0-tests` whose apply task writes failing test assertions against the feature's acceptance criteria. Pre-phase tests SHALL be marked `#[cflx_planned(phase = <N>)]` so the pre-phase archives on a green `cargo test`. The feature phase's first task per group SHALL remove the matching `#[cflx_planned(phase = <N>)]` attribute as the corresponding feature code lands.

The `#[cflx_planned]` attribute is a proc-macro provided by the `cairn-macros` workspace member. The macro emits `#[ignore = "cflx_planned: phase-<N>"]` on the attached function so `cargo test` continues to skip planned tests, and registers the test path and target phase in a build-derived sidecar at `target/cflx/planned.json`. Authors and agents SHALL NOT mark a planned test with a hand-written `#[ignore]` attribute; the proc-macro is the sole authorised entry point for the planned state. Authors and agents SHALL NOT parse the `#[ignore]` reason string to discover the target phase; the structured attribute is the source of truth.

#### Scenario: Pre-phase archives green with planned tests

- **GIVEN** `phase-<N>.0-tests` has been applied and its tests are committed with `#[cflx_planned(phase = <N>)]`
- **WHEN** `cargo test` runs as part of the archive gate
- **THEN** the planned tests are skipped and the gate passes

#### Scenario: Feature phase removes planned attributes as features land

- **GIVEN** `phase-<N>.0-tests` has archived and `phase-<N>` applies the feature
- **WHEN** `cargo test` runs at the end of `phase-<N>`
- **THEN** the previously-planned tests now run and pass without any `#[cflx_planned(phase = <N>)]` attribute

#### Scenario: Stacking with explicit ignore is rejected

- **GIVEN** a test function carries both `#[cflx_planned(phase = <N>)]` and a hand-written `#[ignore = "<reason>"]`
- **WHEN** the macro expands at compile time
- **THEN** compilation fails with a clear error directing the author to choose one mechanism

## ADDED Requirements

### Requirement: Verification states attached to test attributes

Cairn SHALL recognise five verification states (`Draft`, `Planned`, `Passed`, `Failed`, `Blocked`) attached to test outcomes. The state is a logical contract surfaced by the proc-macro, the `cargo test` pass/fail signal, and the cairn error pipeline. The states are not a kernel artefact type; they live on test attributes and in build-derived state.

#### Scenario: Planned tests report as Planned

- **GIVEN** a test marked `#[cflx_planned(phase = 8)]` exists in `tests/`
- **WHEN** `cflx accept` reads `target/cflx/planned.json` after a build
- **THEN** the test outcome is reported as `Planned`
- **AND** the same test is reported as ignored under `cargo test`

#### Scenario: Blocked tests carry CC001 and surface a cause

- **GIVEN** a test surfaces an upstream missing fixture during execution and constructs a blocked-verification error
- **WHEN** the error reaches the cairn error pipeline
- **THEN** the error code returned by `CairnError.code()` is `"CC001"`
- **AND** the JSON error output includes a sibling field naming the missing upstream piece

#### Scenario: cflx accept distinguishes Blocked from Failed

- **GIVEN** `cflx accept` runs the verification battery
- **AND** one test panics with an assertion failure
- **AND** another test returns a `CC001` blocked-verification error
- **WHEN** the gate classifies outcomes
- **THEN** the panicking test is reported as `Failed`
- **AND** the blocked test is reported as `Blocked`
- **AND** the gate exits with a status that distinguishes the two outcomes

#### Scenario: Blocked does not fail accept by default in phase 7.5c

- **GIVEN** a test surfaces a `CC001` blocked-verification error during `cflx accept`
- **WHEN** the gate completes after this phase archives
- **THEN** the gate does NOT fail the phase solely because of the blocked outcome
- **AND** the gate's help text records the deferred-tightening note for a future phase
