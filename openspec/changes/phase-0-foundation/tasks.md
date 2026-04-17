# Tasks: Phase 0 Foundation

## 0. Rust Workspace

- [x] 0.1 Create root `Cargo.toml` defining a Cargo workspace, the `cairn` package with library and binary targets, and `[workspace.lints]` with the required lint configuration.
- [x] 0.2 Set the `cairn` package to Rust edition 2024, add `[lints] workspace = true`, and commit `Cargo.lock` generated from the foundation package.
- [x] 0.3 Create `src/lib.rs` with a minimal foundation metadata API.
- [x] 0.4 Create `src/main.rs` with deterministic smoke CLI behavior.
- [x] 0.5 Add `.gitignore` entries for `target/`, Rust/editor temporary files, and system noise without ignoring `Cargo.lock`, `test/fixtures/`, or OpenSpec files.

## 0b. Quality Infrastructure

- [x] 0b.1 Create `rustfmt.toml` at the workspace root with `edition = "2021"`, `max_width = 100`, and `use_field_init_shorthand = true`.
- [x] 0b.2 Confirm `[workspace.lints.rust]` sets `missing_docs = "deny"` and every public item has a doc comment.
- [x] 0b.3 Confirm `[workspace.lints.rust]` sets `unsafe_code = "forbid"`.
- [x] 0b.4 Create the Git pre-commit hook script at `scripts/install-pre-commit-hook.sh` that writes `.git/hooks/pre-commit` running `cargo fmt --check`. The hook MUST exit non-zero on failure and print which gate failed.
- [x] 0b.5 Create a `just check` or `make check` (or `Justfile`/`Makefile` target) that runs the full local quality suite: `cargo fmt --check`, `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`, `cargo test`, and `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`.
- [x] 0b.6 Document setup instructions in `README.md`: how to run `scripts/install-pre-commit-hook.sh` after cloning, and how to run the local quality suite via `just check` or `make check`.

## 1. Fixture Smoke Tests

- [x] 1.1 Add Rust tests that read `test/fixtures/cairn.dsl` and assert it exists, is non-empty, and contains a non-comment line beginning with `System`, `Container`, `Module`, or `Actor`.
- [x] 1.2 Add Rust tests that read `test/fixtures/cairn-bootstrap/cairn.dsl` and assert it exists, is non-empty, and contains a non-comment line beginning with `System`, `Container`, `Module`, or `Actor`.
- [x] 1.3 Add a CLI smoke assertion proving `cairn --version` prints the package name and version.

## 2. Git Quality Gate

- [x] 2.1 Add `scripts/install-pre-commit-hook.sh` that writes `.git/hooks/pre-commit` with `cargo fmt --check`.
- [ ] 2.2 Run the installer and verify `.git/hooks/pre-commit` exists and is executable.
- [ ] 2.3 Verify the pre-commit hook command succeeds from the repository root.

## 3. Conflux Archive Gate

- [x] 3.1 Add `scripts/pre-archive-rust-gates.sh` for Conflux archive enforcement.
- [x] 3.2 Wire repository-local Conflux `pre_archive` configuration to `scripts/pre-archive-rust-gates.sh` if the local Conflux tool exposes a machine-readable hook location; otherwise document the required `pre_archive` command in `README.md`.
- [x] 3.3 Ensure the archive gate runs `cargo fmt --check`, `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`, and `cargo test`.
- [x] 3.4 Verify `scripts/pre-archive-rust-gates.sh` succeeds from the repository root.

## 4. Documentation and Scope Control

- [x] 4.1 Document how a new clone recreates the Git pre-commit hook.
- [x] 4.2 Document that Phase 0 intentionally excludes parser, graph, query, scanner, artefact, and reconciler behavior.
- [x] 4.3 Confirm no future CLI subcommands or domain modules are introduced before Phase 1.

## 5. Required Verification

- [x] 5.1 `cargo build` passes with zero warnings.
- [x] 5.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 5.3 `cargo fmt --check` passes.
- [x] 5.4 `cargo test` passes.
- [x] 5.5 `cargo test --locked` passes.
- [x] 5.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-0-foundation --strict` passes.

## Implementation Blocker #1

category: external_non_mockable

summary: Real Git hook installation cannot be completed from this sandboxed linked worktree.

evidence: `scripts/install-pre-commit-hook.sh && hook_path=$(git rev-parse --git-path hooks/pre-commit) && test -x "$hook_path" && "$hook_path"` failed with `Operation not permitted` while writing `/Users/george/repos/cairn/.git/hooks/pre-commit`. The sandbox writable roots include this worktree but not `/Users/george/repos/cairn/.git/hooks/`.

impact: Tasks 2.2 and 2.3 cannot be truthfully completed in this execution environment, although the committed installer script has been created and the format gate itself passes.

unblock_actions: Run `scripts/install-pre-commit-hook.sh` from an unsandboxed shell or a normal clone with permission to write the Git hooks directory. Then re-run `hook_path=$(git rev-parse --git-path hooks/pre-commit) && test -x "$hook_path" && "$hook_path"` to verify installation and hook execution.

owner: maintainer

decision_due: 2026-04-18
