# Tasks: Phase 0 Foundation

## 0. Rust Workspace

- [ ] 0.1 Create root `Cargo.toml` defining a Cargo workspace and the `cairn` package with library and binary targets.
- [ ] 0.2 Set the `cairn` package to Rust edition 2024 and commit `Cargo.lock` generated from the foundation package.
- [ ] 0.3 Create `src/lib.rs` with the required strict crate-level lint attributes and a minimal foundation metadata API.
- [ ] 0.4 Create `src/main.rs` with the required strict crate-level lint attributes and deterministic smoke CLI behavior.
- [ ] 0.5 Add `.gitignore` entries for `target/`, Rust/editor temporary files, and system noise without ignoring `Cargo.lock`, `test/fixtures/`, or OpenSpec files.

## 1. Fixture Smoke Tests

- [ ] 1.1 Add Rust tests that read `test/fixtures/cairn.dsl` and assert it exists, is non-empty, and contains a non-comment line beginning with `System`, `Container`, `Module`, or `Actor`.
- [ ] 1.2 Add Rust tests that read `test/fixtures/cairn-bootstrap/cairn.dsl` and assert it exists, is non-empty, and contains a non-comment line beginning with `System`, `Container`, `Module`, or `Actor`.
- [ ] 1.3 Add a CLI smoke assertion proving `cairn --version` prints the package name and version.

## 2. Git Quality Gate

- [ ] 2.1 Add `scripts/install-pre-commit-hook.sh` that writes `.git/hooks/pre-commit` with `cargo fmt --check`, strict `cargo clippy`, and `cargo test`.
- [ ] 2.2 Run the installer and verify `.git/hooks/pre-commit` exists and is executable.
- [ ] 2.3 Verify the pre-commit hook command succeeds from the repository root.

## 3. Conflux Archive Gate

- [ ] 3.1 Add `scripts/pre-archive-rust-gates.sh` for Conflux archive enforcement.
- [ ] 3.2 Wire repository-local Conflux `pre_archive` configuration to `scripts/pre-archive-rust-gates.sh` if the local Conflux tool exposes a machine-readable hook location; otherwise document the required `pre_archive` command in `README.md`.
- [ ] 3.3 Ensure the archive gate runs `cargo fmt --check`, `cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery`, and `cargo test`.
- [ ] 3.4 Verify `scripts/pre-archive-rust-gates.sh` succeeds from the repository root.

## 4. Documentation and Scope Control

- [ ] 4.1 Document how a new clone recreates the Git pre-commit hook.
- [ ] 4.2 Document that Phase 0 intentionally excludes parser, graph, query, scanner, artefact, and reconciler behavior.
- [ ] 4.3 Confirm no future CLI subcommands or domain modules are introduced before Phase 1.

## 5. Required Verification

- [ ] 5.1 `cargo build` passes with zero warnings.
- [ ] 5.2 `cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery` passes.
- [ ] 5.3 `cargo fmt --check` passes.
- [ ] 5.4 `cargo test` passes.
- [ ] 5.5 `cargo test --locked` passes.
- [ ] 5.6 `python3 /Users/george/repos/cairn/.agents/skills/cflx-proposal/scripts/cflx.py validate phase-0-foundation --strict` passes.
