# Design: Phase 0 Foundation

## Cross-Cutting Conventions

Implementors MUST read `openspec/conventions.md` before starting work. That document defines cross-cutting conventions (error codes, module size limits, state versioning, naming rules, etc.) that apply to every phase. Phase 0 establishes the tooling that enforces many of those conventions; later phases inherit them.

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

## Workspace Lint Configuration

Lint policy SHALL be defined in `Cargo.toml` using `[workspace.lints]`, providing a single source of truth that applies to all workspace members. Crate roots (`src/lib.rs`, `src/main.rs`) SHALL NOT contain lint attributes — all lint configuration flows from the manifest.

`Cargo.toml` SHALL include:

```toml
[workspace.lints.rust]
missing_docs = "deny"
unsafe_code = "forbid"

[workspace.lints.clippy]
all = { level = "deny" }
pedantic = { level = "deny" }
cargo = { level = "deny" }
dbg_macro = { level = "deny" }
todo = { level = "deny" }
```

Every workspace member SHALL opt in with:

```toml
[lints]
workspace = true
```

- `missing_docs` — every public item MUST have a doc comment. This enforces documentation discipline from the first line of code.
- `unsafe_code = "forbid"` — no `unsafe` blocks unless explicitly justified and approved in a phase design document.
- `clippy::cargo` — validates `Cargo.toml` metadata quality (descriptions, categories, license).
- `clippy::dbg_macro` and `clippy::todo` — prevent debug macros and TODO placeholders from shipping.
- If a specific Clippy pedantic lint is genuinely unhelpful for a particular item, the implementation MAY add a targeted `#[allow(...)]` with a `// Reason: ...` comment explaining why the exception is necessary. Blanket `#[allow(clippy::pedantic)]` on modules or crates is forbidden.

Compiler warnings are caught by `RUSTFLAGS="-D warnings"` in quality gate scripts rather than `#![deny(warnings)]` in source. This avoids build breakage when upgrading the Rust compiler, which may introduce new warnings.

## Quality Infrastructure

### Formatting

The repository SHALL contain a `rustfmt.toml` at the workspace root with at minimum:

```toml
edition = "2021"
max_width = 100
use_field_init_shorthand = true
```

All Rust source files MUST conform to this configuration. The `cargo fmt --check` gate enforces this.

### Git Pre-Commit Hook

The repository SHALL provide a committed `scripts/install-pre-commit-hook.sh` that writes `.git/hooks/pre-commit`. The pre-commit hook SHALL run only fast checks to avoid punishing commit frequency:

```sh
#!/bin/sh
set -e
cargo fmt --check
```

The hook MUST be installed as part of project setup. The repository SHALL document the setup step in `README.md` and optionally provide a `just setup` or `make setup` target that runs the installer.

Hook failures SHALL block the commit and print a clear error message identifying which gate failed. Full lint and test enforcement runs in the local quality suite and the Conflux archive gate — not in the pre-commit hook.

### Local Quality Suite

The repository SHALL provide a single command that runs the full quality suite locally, equivalent to what CI would execute. This command SHALL be available as `just check` or `make check` (or both) and SHALL run:

1. `cargo fmt --check`
2. `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`
3. `cargo test`
4. `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`

`RUSTFLAGS="-D warnings"` ensures any compiler or Clippy warning is treated as an error, complementing the lint configuration in `Cargo.toml`. This avoids placing `#![deny(warnings)]` in source files, which can break builds when upgrading the Rust compiler.

This command sequence is the canonical CI-equivalent local check. Contributors SHOULD run it before pushing.

## Library and CLI Surface

The library SHALL expose only foundation-level metadata needed by smoke tests, such as a package name or version helper. This API exists solely to prove that `src/lib.rs` compiles under the strict lint policy.

The binary SHALL provide a deterministic foundation-level smoke behavior: `cairn --version` prints the package name and version. It MUST NOT parse `cairn.dsl`, build an ontology, or expose future query commands.

## Fixture Wiring

Rust tests SHALL read the existing checked-in fixtures:

- `test/fixtures/cairn.dsl`
- `test/fixtures/cairn-bootstrap/cairn.dsl`

The tests SHALL assert that both files exist, are non-empty, and contain at least one non-comment line whose first token is `System`, `Container`, `Module`, or `Actor`. This verifies fixture path stability without implementing a parser.

## Conflux Archive Gate

Phase 0 SHALL add `scripts/pre-archive-rust-gates.sh` as the repository-local command for Conflux `pre_archive`. The script SHALL run:

```sh
cargo fmt --check
RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features
cargo test
```

If the installed Conflux version supports a machine-readable `pre_archive` configuration path, the implementation SHALL wire that configuration to `scripts/pre-archive-rust-gates.sh`. If the local Conflux version does not expose such a configuration path, the implementation SHALL document in `README.md` that `scripts/pre-archive-rust-gates.sh` is the command the orchestrator MUST wire into `pre_archive`.

## Verification Strategy

The final implementation evidence for this phase SHALL include:

- Output from `cargo build`.
- Output from `cargo fmt --check`.
- Output from `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`.
- Output from `cargo test`.
- Output from `cargo test --locked`.
- A file mode or execution check proving the Git pre-commit hook is executable.
- Direct execution of `scripts/pre-archive-rust-gates.sh`.

## Dependency Boundary

Later phases SHALL add dependencies for parsing, CLI argument handling, serialization, Tree-sitter, or testing only when their phase scope requires them. Phase 0 SHALL avoid nonessential dependencies so the baseline remains easy to audit.
