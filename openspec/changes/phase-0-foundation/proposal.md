# Proposal: Phase 0 Foundation

## Dependencies

- Requires: none.
- Execution: MUST run before every later phase.

## Problem/Context

Cairn v0.6 defines a phased Rust implementation plan, but the repository currently contains only specification material and fixtures. Before the kernel or any domain logic can be implemented, the project needs a reproducible Rust foundation that every later phase can rely on.

The foundation MUST establish strict Rust quality gates at project creation time. Later phases SHALL inherit a workspace that already denies warnings, denies strict Clippy lint groups, forbids unsafe code, runs formatting checks, and exercises the test harness from a clean checkout.

## Proposed Solution

Create the Rust project skeleton and tooling required for subsequent OpenSpec changes:

- A Cargo workspace with a `cairn` package exposing both a library and CLI binary.
- Rust edition 2024 as the required package edition.
- `src/lib.rs` and `src/main.rs` containing the required crate-level lint attributes.
- A `.gitignore` covering Rust build outputs and editor/system noise.
- A committed `scripts/install-pre-commit-hook.sh` script that writes a Git pre-commit hook running `cargo fmt --check`, strict `cargo clippy`, and `cargo test`.
- A committed `scripts/pre-archive-rust-gates.sh` script for the Conflux `pre_archive` gate, enforcing the same Rust gates before archive.
- A minimal library and CLI smoke test proving the package builds and the test harness can read the checked-in DSL fixtures.
- Documentation in the change tasks that makes the verification evidence explicit for a headless Codex agent.

## Acceptance Criteria

- `cargo build` succeeds with zero warnings.
- `cargo fmt --check` succeeds.
- `cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery` succeeds.
- `cargo test` succeeds.
- Running `scripts/install-pre-commit-hook.sh` creates an executable `.git/hooks/pre-commit` hook that blocks on any failed Rust gate.
- Running `scripts/pre-archive-rust-gates.sh` enforces the same Rust gates that Conflux MUST wire into `pre_archive`.
- Test fixtures under `test/fixtures/` remain checked in and are reachable from Rust tests without network access.
- `cargo test --locked` succeeds, proving the committed lockfile is usable.
- No parser, graph, query, scanner, or artefact-domain behavior is implemented in this phase.

## Out of Scope

- DSL lexer, parser, AST, or ontology graph implementation.
- CLI commands beyond a deterministic foundation-level smoke command.
- Tree-sitter, scanner, reconciler, hook semantics, MCP, summariser, brownfield extraction, or distribution packaging.
- Publishing crates or installing external services.
