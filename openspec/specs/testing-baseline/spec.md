# Testing Baseline Capability Spec

## Requirements

### Requirement: Public JSON wire formats pinned by snapshot tests

Cairn SHALL pin every public JSON wire format via an `insta` snapshot test. Any change to a pinned response shape SHALL require an explicit `cargo insta review` accept step before the change can pass `cargo test`.

#### Scenario: Unchanged wire format passes gate

- **GIVEN** the webui responds to `/api/graph` with its currently-pinned shape
- **WHEN** `cargo test` runs
- **THEN** the snapshot assertion passes

#### Scenario: Wire-format change requires review

- **GIVEN** a code change alters the JSON shape returned by any `/api/*` endpoint
- **WHEN** `cargo test` runs
- **THEN** the snapshot test fails with a diff
- **AND** the implementor must run `cargo insta review` to accept or reject the new shape

### Requirement: Rust source file size ceiling enforced by pre-archive gate

Cairn SHALL fail the pre-archive gate when any Rust source file under `src/` exceeds 500 lines, unless the file's first non-blank line is `// cairn:allow-large-module reason: <non-empty>`.

#### Scenario: New oversized file blocks archive

- **GIVEN** a change introduces `src/foo.rs` at 600 lines without an allow-list comment
- **WHEN** `scripts/pre-archive-rust-gates.sh` runs
- **THEN** the script exits non-zero
- **AND** reports `foo.rs: 600 lines > 500`

#### Scenario: Annotated oversized file is permitted

- **GIVEN** `src/changes.rs` is 1286 lines and its first non-blank line is `// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split`
- **WHEN** the gate runs
- **THEN** `src/changes.rs` is skipped without a violation

#### Scenario: Allow-list comment without reason is rejected

- **GIVEN** a file's first non-blank line is `// cairn:allow-large-module reason:` (empty reason)
- **WHEN** the gate runs
- **THEN** the script reports the file as an invalid allow-list entry and exits non-zero

#### Scenario: Split submodules satisfy the ceiling

- **GIVEN** Phase 7.5b has been applied and all five god modules have been split
- **WHEN** `scripts/pre-archive-rust-gates.sh` runs
- **THEN** no file under `src/` triggers a size violation
- **AND** `grep -r "cairn:allow-large-module" src/` returns empty output

### Requirement: God modules carry unit test coverage

After Phase 7.5b, the five previously-monolithic files are split into directory modules. Test coverage migrates to the new layout as follows:

| Coverage target | Host file after split |
|---|---|
| `parse_blueprint_delta`, `apply_blueprint_delta`, `validate_change` | `src/changes/mod.rs` |
| `run()`, command dispatch | `src/cli/mod.rs` |
| `registry()`, `visible_tools()`, `execute()`, `envelope_json()`, `error_json()` | `src/query_api/mod.rs` |
| `load_artefacts()`, `parse_*` status functions | `src/artefacts/registry/mod.rs` |
| `start_background()`, `request_path()`, route dispatch | `src/ui/mod.rs` |

Each `mod.rs` SHALL contain the `#[cfg(test)] mod tests` block that was in the original god module. Tests that call functions moved to submodules SHALL add the minimum necessary `use super::<submodule>::<fn>;` imports inside the test block to resolve the moved symbols.

#### Scenario: Refactor under tests

- **GIVEN** Phase 7.5b has split the five god modules into directory modules
- **WHEN** `cargo test` runs against the split tree
- **THEN** all inline unit tests previously in the god modules still pass without assertion changes
- **AND** all `/api/*` snapshot tests still pass without `cargo insta review`
- **AND** no `// cairn:allow-large-module` comment remains under `src/`

### Requirement: Test-first pre-phase convention

Feature phases that introduce new acceptance criteria SHOULD be preceded by a paired pre-phase `phase-<N>.0-tests` whose apply task writes failing test assertions against the feature's acceptance criteria. Pre-phase tests SHALL be marked `#[ignore = "awaits phase-<N>"]` so the pre-phase archives on a green `cargo test`. The feature phase's first task per group SHALL remove the matching `#[ignore]` attribute as the corresponding feature code lands.

#### Scenario: Pre-phase archives green with ignored red tests

- **GIVEN** `phase-<N>.0-tests` has been applied and its tests are committed with `#[ignore = "awaits phase-<N>"]`
- **WHEN** `cargo test` runs as part of the archive gate
- **THEN** the ignored tests are skipped and the gate passes

#### Scenario: Feature phase turns ignored tests green

- **GIVEN** `phase-<N>.0-tests` has archived and `phase-<N>` applies the feature
- **WHEN** `cargo test` runs at the end of `phase-<N>`
- **THEN** the previously-ignored tests now run and pass without their `#[ignore]` attribute

### Requirement: CFLX phase ordering supports decimal and suffix phase ids

`scripts/cflx-analyze-cairn-phases.py` SHALL accept phase directory names of the form `phase-<major>[.<minor>][<suffix>]-<name>` where `<major>` and `<minor>` are non-negative integers and `<suffix>` is a single lowercase letter. The resulting sort order SHALL be lexicographic over the tuple `(major, minor, suffix)` with missing components treated as `0` and empty string respectively.

#### Scenario: Decimal phase sorts between integer phases

- **GIVEN** directories `phase-7-mcp`, `phase-7.5a-test-fortification`, `phase-8-summariser` exist
- **WHEN** the analyze script runs
- **THEN** the reported order is `[phase-7-mcp, phase-7.5a-test-fortification, phase-8-summariser]`

#### Scenario: Suffix orders within the same decimal

- **GIVEN** `phase-7.5a-test-fortification` and `phase-7.5b-cleansing-splits` exist
- **WHEN** the analyze script runs
- **THEN** `phase-7.5a-test-fortification` appears before `phase-7.5b-cleansing-splits`
- **AND** `phase-7.5b-cleansing-splits` depends on `phase-7.5a-test-fortification`

#### Scenario: Test-first pre-phase sorts before its feature phase

- **GIVEN** `phase-8.0-tests` and `phase-8-summariser` exist
- **WHEN** the analyze script runs
- **THEN** `phase-8.0-tests` appears before `phase-8-summariser`
- **AND** `phase-8-summariser` depends on `phase-8.0-tests`
