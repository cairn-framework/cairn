# Verification States Capability Spec

## Purpose

The Verification States capability gives Cairn a structured vocabulary for test outcomes beyond pass and fail. It introduces a `#[cflx_planned]` attribute to register future-phase tests, a `Blocked` state with error code `CC001` to distinguish upstream dependency issues from assertion failures, and a `VerificationState` enum that unifies Draft, Planned, Passed, Failed, and Blocked into the runtime and tooling API.

## Requirements

### Requirement: Provide a structured planned-verification attribute

Cairn SHALL provide a `#[cflx_planned(phase = N)]` attribute proc-macro that marks a test as planned for a future phase and replaces the legacy `#[ignore = "awaits phase-<N>"]` comment-string convention. The macro lives in a new `cairn-macros` Cargo workspace member declared `proc-macro = true`.

#### Scenario: Workspace member is configured strictly

- **GIVEN** Phase 7.5c has archived
- **WHEN** a headless agent inspects the `cairn-macros` package configuration
- **THEN** the package is a member of the Cargo workspace
- **AND** its `Cargo.toml` declares `proc-macro = true`
- **AND** its `[lints]` section contains `workspace = true`

#### Scenario: Attribute emits ignore plus sidecar registration

- **GIVEN** a test function is marked `#[cflx_planned(phase = 8)]`
- **WHEN** the cairn crate compiles
- **THEN** the macro emits `#[ignore = "cflx_planned: phase-8"]` on the function
- **AND** the macro writes an entry to `target/cflx/planned.json` containing the test path, the integer `8`, and the source file and line of the attribute

#### Scenario: Attribute is re-exported from the cairn crate

- **GIVEN** a downstream call site imports `use cairn::cflx_planned;`
- **WHEN** the call site applies `#[cflx_planned(phase = 9)]` to a test
- **THEN** the macro expands without requiring a direct path dependency on `cairn-macros`

#### Scenario: Invalid phase argument is rejected

- **GIVEN** a test function is marked `#[cflx_planned(phase = 0)]` or `#[cflx_planned(phase = -1)]`
- **WHEN** the macro expands at compile time
- **THEN** compilation fails with a clear error naming the rejected value

#### Scenario: Sidecar is build-derived and versioned

- **GIVEN** the sidecar at `target/cflx/planned.json`
- **WHEN** a reader inspects the file
- **THEN** the first field is `version = 1`
- **AND** the file is not committed to the repository

### Requirement: Distinguish Blocked from Failed via error code CC001

Cairn SHALL allocate the error code `CC001` to the `Blocked` verification state and surface it through the `CairnError` pipeline so `cflx accept` can classify a blocked outcome separately from a failed assertion.

#### Scenario: CC001 is allocated in the Changes category

- **GIVEN** Phase 7.5c has archived
- **WHEN** a reader inspects `openspec/registries/error-codes.md`
- **THEN** the entry `CC001 -- verification blocked by upstream dependency -- phase-7.5c` appears under the `CC -- Changes` heading

#### Scenario: Blocked verification carries CC001

- **GIVEN** a verification cannot execute because an upstream fixture, environment dependency, or referenced phase is missing
- **WHEN** the cairn runtime constructs a blocked-verification error
- **THEN** `CairnError.code()` returns `"CC001"`
- **AND** the JSON error output includes a `code` field with the value `"CC001"` and a sibling field naming the missing upstream piece

#### Scenario: cflx accept classifies CC001 as Blocked

- **GIVEN** `cflx accept` runs the verification battery
- **AND** at least one test surfaces an error carrying code `CC001`
- **WHEN** the gate reports outcomes
- **THEN** the test is labelled `Blocked` rather than `Failed`
- **AND** the gate output and JSON output preserve the upstream-cause field

### Requirement: Expose the five-state verification enum

Cairn SHALL define the `VerificationState` enum with variants `Draft`, `Planned`, `Passed`, `Failed`, `Blocked` and re-export it from the cairn crate's library API.

#### Scenario: Enum derives standard traits

- **GIVEN** the `VerificationState` type defined in `src/verification.rs`
- **WHEN** a downstream module uses the type
- **THEN** the enum derives `Debug`, `Clone`, `PartialEq`, `Eq`, `serde::Serialize`, and `serde::Deserialize`

#### Scenario: Enum round-trips through serde_json

- **GIVEN** any variant of `VerificationState`
- **WHEN** the variant is serialised to JSON and deserialised back
- **THEN** the round-trip preserves the variant exactly

#### Scenario: Planned and Blocked are the only net-new runtime states

- **GIVEN** the cairn runtime
- **WHEN** test outcomes are produced and classified
- **THEN** `Passed` and `Failed` correspond to existing `cargo test` outcomes
- **AND** `Planned` is produced only via the `#[cflx_planned]` attribute and the sidecar registry
- **AND** `Blocked` is produced only via a `CairnError` carrying code `CC001`
- **AND** `Draft` is produced only by tooling reasoning about a verification authored but not yet wired to the battery
