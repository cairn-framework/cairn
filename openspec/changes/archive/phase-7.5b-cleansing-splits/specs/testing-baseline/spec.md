# Testing Baseline Capability Spec

## MODIFIED Requirements

### Requirement: God modules carry unit test coverage

After this phase, the five monolithic files no longer exist. Test coverage migrates to the directory module layout as follows:

| Coverage target | Host file after split |
|---|---|
| `parse_blueprint_delta`, `apply_blueprint_delta`, `validate_change` | `src/changes/mod.rs` |
| `run()`, command dispatch | `src/cli/mod.rs` |
| `registry()`, `visible_tools()`, `execute()`, `envelope_json()`, `error_json()` | `src/query_api/mod.rs` |
| `load_artefacts()`, `parse_*` status functions | `src/artefacts/registry/mod.rs` |
| `start_background()`, `request_path()`, route dispatch | `src/ui/mod.rs` |

Each `mod.rs` SHALL contain the `#[cfg(test)] mod tests` block that was in the original god module. No test assertion or helper function is modified during the split. Tests that call functions moved to submodules SHALL add the minimum necessary `use super::<submodule>::<fn>;` imports inside the test block to resolve the moved symbols.

#### Scenario: Refactor under tests

- **GIVEN** Phase 7.5b has split the five god modules into directory modules
- **WHEN** `cargo test` runs against the split tree
- **THEN** all inline unit tests previously in the god modules still pass without assertion changes
- **AND** all `/api/*` snapshot tests still pass without `cargo insta review`
- **AND** no `// cairn:allow-large-module` comment remains under `src/`

---

### Requirement: Rust source file size ceiling enforced by pre-archive gate

Cairn SHALL fail the pre-archive gate when any Rust source file under `src/` exceeds 500 lines, unless the file's first non-blank line is `// cairn:allow-large-module reason: <non-empty>`.

#### Scenario: Split submodules satisfy the ceiling

- **GIVEN** Phase 7.5b has been applied and all five god modules have been split
- **WHEN** `scripts/pre-archive-rust-gates.sh` runs
- **THEN** no file under `src/` triggers a size violation
- **AND** `grep -r "cairn:allow-large-module" src/` returns empty output

#### Scenario: New oversized file still blocks archive

- **GIVEN** a change introduces `src/foo.rs` at 600 lines without an allow-list comment
- **WHEN** `scripts/pre-archive-rust-gates.sh` runs
- **THEN** the script exits non-zero
- **AND** reports `foo.rs: 600 lines > 500`
