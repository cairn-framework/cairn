# Design: Phase 0 Foundation

## Objective

Phase 0 creates the Rust substrate that all later Cairn phases use. It is intentionally limited to repository structure, build tooling, strict lint policy, hooks, and smoke tests. The implementation MUST NOT include Cairn domain behavior.

## Cargo Workspace

The repository root SHALL contain a Cargo workspace with a single package named `cairn`:

```text
Cargo.toml
Cargo.lock
src/
  lib.rs
  main.rs
tests/
  fixtures_smoke.rs
```

`Cargo.toml` SHALL define:

- `edition = "2024"`.
- A library target at `src/lib.rs`.
- A binary target named `cairn` at `src/main.rs`.
- No runtime dependencies unless the implementation needs a tiny crate for a foundation concern and documents why the standard library is insufficient.

`Cargo.lock` SHALL be committed so `cargo test --locked` can run reproducibly in later phases.

## Strict Rust Attributes

Every Rust crate root created in this phase SHALL begin with these attributes:

```rust
#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![forbid(unsafe_code)]
```

The implementation MUST keep code simple enough to satisfy these lints without `#[allow(...)]` attributes. Any later phase that needs a lint exception MUST justify it in that phase's design document.

## Library and CLI Surface

The library SHALL expose only foundation-level metadata needed by smoke tests, such as a package name or version helper. This API exists solely to prove that `src/lib.rs` compiles under the strict lint policy.

The binary SHALL provide a deterministic foundation-level smoke behavior: `cairn --version` prints the package name and version. It MUST NOT parse `cairn.dsl`, build an ontology, or expose future query commands.

## Fixture Wiring

Rust tests SHALL read the existing checked-in fixtures:

- `test/fixtures/cairn.dsl`
- `test/fixtures/cairn-bootstrap/cairn.dsl`

The tests SHALL assert that both files exist, are non-empty, and contain at least one non-comment line whose first token is `System`, `Container`, `Module`, or `Actor`. This verifies fixture path stability without implementing a parser.

## Git Hook

Phase 0 SHALL install or generate `.git/hooks/pre-commit` with:

```sh
#!/bin/sh
set -e
cargo fmt --check
cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery
cargo test
```

The hook file MUST be executable. Because `.git/hooks/` is not committed by Git, the repository SHALL contain `scripts/install-pre-commit-hook.sh`. Running that script from the repository root SHALL create or replace `.git/hooks/pre-commit` with the exact gate commands above and executable permissions.

## Conflux Archive Gate

Phase 0 SHALL add `scripts/pre-archive-rust-gates.sh` as the repository-local command for Conflux `pre_archive`. The script SHALL run:

```sh
cargo fmt --check
cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery
cargo test
```

If the installed Conflux version supports a machine-readable `pre_archive` configuration path, the implementation SHALL wire that configuration to `scripts/pre-archive-rust-gates.sh`. If the local Conflux version does not expose such a configuration path, the implementation SHALL document in `README.md` that `scripts/pre-archive-rust-gates.sh` is the command the orchestrator MUST wire into `pre_archive`.

## Verification Strategy

The final implementation evidence for this phase SHALL include:

- Output from `cargo build`.
- Output from `cargo fmt --check`.
- Output from strict `cargo clippy`.
- Output from `cargo test`.
- Output from `cargo test --locked`.
- A file mode or execution check proving the Git pre-commit hook is executable.
- Direct execution of `scripts/pre-archive-rust-gates.sh`.

## Dependency Boundary

Later phases MAY add dependencies for parsing, CLI argument handling, serialization, Tree-sitter, or testing when their scope requires them. Phase 0 SHOULD avoid those dependencies so the baseline remains easy to audit.
