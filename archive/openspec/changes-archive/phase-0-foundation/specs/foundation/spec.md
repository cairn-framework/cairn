# Foundation Capability Spec

## ADDED Requirements

### Requirement: Provide a strict Rust project skeleton

The repository SHALL define a reproducible Rust workspace containing a `cairn` library target and a `cairn` binary target before any Cairn domain logic is implemented.

#### Scenario: Cargo metadata is present

- **GIVEN** a clean checkout of the repository
- **WHEN** a headless agent inspects the root project files
- **THEN** `Cargo.toml` defines a Cargo workspace and package for `cairn`
- **AND** the `cairn` package uses Rust edition 2024
- **AND** `Cargo.lock` is committed
- **AND** `src/lib.rs` exists
- **AND** `src/main.rs` exists

#### Scenario: Workspace lint configuration is enforced

- **GIVEN** the Cargo workspace created by this phase
- **WHEN** a headless agent inspects `Cargo.toml`
- **THEN** `[workspace.lints.rust]` sets `unsafe_code = "forbid"` and `missing_docs = "deny"`
- **AND** `[workspace.lints.clippy]` sets `all`, `pedantic`, and `cargo` to deny level
- **AND** `[workspace.lints.clippy]` sets `dbg_macro` and `todo` to deny level
- **AND** each workspace member's `[lints]` section contains `workspace = true`
- **AND** `src/lib.rs` and `src/main.rs` contain no crate-level lint attributes

#### Scenario: Domain logic is absent

- **GIVEN** Phase 0 has been implemented
- **WHEN** a headless agent reviews the Rust modules and CLI behavior
- **THEN** the code does not implement a DSL parser, ontology graph, structural query command, scanner, reconciler, artefact reader, or hook semantics
- **AND** the binary exposes `cairn --version` as deterministic foundation-level smoke behavior

### Requirement: Enforce Rust quality gates locally

The repository SHALL provide local quality gates that fail on formatting drift, Clippy warnings, lint regressions, or failing tests.

#### Scenario: Git pre-commit hook runs fast format check

- **GIVEN** the Phase 0 hook installer has been run
- **WHEN** `.git/hooks/pre-commit` executes from the repository root
- **THEN** it runs `cargo fmt --check`
- **AND** it exits non-zero if formatting is wrong

#### Scenario: Hook can be recreated in a new clone

- **GIVEN** `.git/hooks/` is not committed by Git
- **WHEN** a headless agent starts from a clean clone
- **THEN** the repository contains `scripts/install-pre-commit-hook.sh`
- **AND** running the script recreates `.git/hooks/pre-commit`
- **AND** the recreated hook has executable permissions

#### Scenario: Conflux pre-archive gate runs strict gates

- **GIVEN** a Conflux archive operation is about to finalize a change
- **WHEN** `scripts/pre-archive-rust-gates.sh` runs from the repository root
- **THEN** it runs `cargo fmt --check`
- **AND** it runs `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`
- **AND** it runs `cargo test`
- **AND** it exits non-zero if any command fails

### Requirement: Quality infrastructure rejects bad code

The quality suite, pre-commit hook, and archive gate SHALL catch formatting violations, lint warnings, and test failures before code enters the repository.

#### Scenario: Pre-commit hook rejects unformatted code

- **GIVEN** the Phase 0 hook installer has been run
- **AND** a Rust source file contains formatting that violates `rustfmt.toml`
- **WHEN** the developer attempts to commit
- **THEN** the pre-commit hook runs `cargo fmt --check`
- **AND** the hook exits non-zero with a message identifying the formatting violation
- **AND** the commit is blocked

#### Scenario: Quality suite rejects Clippy warnings

- **GIVEN** a Rust source file triggers a Clippy warning
- **WHEN** the local quality suite (`just check` or equivalent) runs
- **THEN** it runs `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`
- **AND** the suite exits non-zero with a message identifying the Clippy violation

#### Scenario: Quality suite rejects failing tests

- **GIVEN** a Rust test in the workspace fails
- **WHEN** the local quality suite (`just check` or equivalent) runs
- **THEN** it runs `cargo test`
- **AND** the suite exits non-zero with a message identifying the test failure

#### Scenario: Crate compiles cleanly under strict lint policy

- **GIVEN** a clean checkout of the repository after Phase 0 implementation
- **WHEN** `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` runs
- **THEN** the command exits zero with no warnings or errors
- **AND** `Cargo.toml` contains `[workspace.lints]` with the required lint configuration

### Requirement: Verify checked-in fixtures without parsing them

The foundation tests SHALL prove that existing Cairn DSL fixtures are available to Rust tests while avoiding parser or ontology behavior reserved for later phases.

#### Scenario: Root fixture is readable

- **GIVEN** `test/fixtures/cairn.dsl` exists in the repository
- **WHEN** `cargo test` runs
- **THEN** a Rust test reads the file from disk
- **AND** asserts that the file is non-empty
- **AND** asserts that the file contains a non-comment line beginning with `System`, `Container`, `Module`, or `Actor`

#### Scenario: Bootstrap fixture is readable

- **GIVEN** `test/fixtures/cairn-bootstrap/cairn.dsl` exists in the repository
- **WHEN** `cargo test` runs
- **THEN** a Rust test reads the file from disk
- **AND** asserts that the file is non-empty
- **AND** asserts that the file contains a non-comment line beginning with `System`, `Container`, `Module`, or `Actor`

### Requirement: Keep verification evidence explicit

The Phase 0 task list SHALL require command-level evidence for every Rust gate and for OpenSpec validation.

#### Scenario: Required commands are listed

- **GIVEN** a headless Codex agent is implementing Phase 0
- **WHEN** it reads `openspec/changes/phase-0-foundation/tasks.md`
- **THEN** the task list includes `cargo build`
- **AND** includes `cargo fmt --check`
- **AND** includes `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`
- **AND** includes `cargo test`
- **AND** includes `cargo test --locked`
- **AND** includes strict OpenSpec validation for `phase-0-foundation`
