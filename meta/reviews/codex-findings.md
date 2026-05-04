# Codex Headless Review Findings

## Summary
Fail

The `cairn-macros` test crate fails compilation under the workspace lint configuration (`missing_docs`, `clippy::assertions_on_constants`, `dead_code`). This failure is hidden from the standard `cargo test` and `cargo clippy` commands because they run root-package-only by default; `--workspace` is required to surface it. Additionally, all three pre-phase proposals violate the updated `openspec/conventions.md` phase-encoding rule (76 vs 706) and the test-naming convention.

---

## Critical Issues

| # | File | Issue | Recommended Fix |
|---|---|---|---|
| 1 | `cairn-macros/tests/planned_attribute.rs` | Compilation fails under workspace lints: missing `//!` crate doc (`-D missing-docs`), `assert!(true)` triggers `clippy::assertions-on-constants`, and unused `planned_test_fixture` triggers `-D dead-code` via `-D warnings`. | Add `//! Proc-macro fixture test` (or similar), remove the tautological `assert!(true)`, and either use `#[test]` on `planned_test_fixture` or add `#[allow(dead_code)]` with a reason comment. |
| 2 | `openspec/changes/phase-7.6.0-tests/{design,tasks,specs}/*` | Uses `phase = 76`, but `openspec/conventions.md` §5 now mandates `phase = 706` for decimal phases (`major * 100 + minor`). Same for 7.7 (`77` → `707`) and 7.8 (`78` → `708`). | Update all `#[cflx_planned(phase = 76)]` references in the three pre-phase proposals to `phase = 706`, `707`, and `708` respectively. Update the spec scenario text that embeds the phase number. |
| 3 | `tests/phase_7_6_ai_provenance.rs` (et al.) | The three pre-phase proposals were committed, but the actual integration test files they specify were never created. `tasks.md` shows every task unchecked. | Apply the pre-phase proposals (or note that they are intentionally unapplied proposals). If this review is post-apply, write the three test files per their design.md. |

---

## Major Issues

| # | File | Issue | Recommended Fix |
|---|---|---|---|
| 1 | `openspec/changes/phase-7.6.0-tests/specs/ai-provenance-foundation-tests/spec.md` | Test names omit the `{expected_outcome}` component required by `conventions.md` §5 (e.g., `test_sidecar_is_state_versioned` should be `test_sidecar_is_state_versioned_returns_true` or similar). | Append an outcome segment to each test name, or update the convention if the three-part pattern is intentionally relaxed for pre-phase stubs. |
| 2 | `openspec/changes/phase-7.7.0-tests/{design,tasks,specs}/*` | Test names use double underscores and lack the mandatory `test_` prefix (e.g., `check__whole_map_inspection_without_arguments`). | Rename to `test_check_whole_map_inspection_without_arguments_fails` (or equivalent outcome) to comply with `test_{feature}_{scenario}_{outcome}`. |
| 3 | `openspec/changes/phase-7.8.0-tests/{design,tasks,specs}/*` | Test names lack the mandatory `test_` prefix (e.g., `default_format_is_json`). | Rename to `test_default_format_is_json` (plus outcome segment). |
| 4 | `openspec/changes/phase-7.6.0-tests/specs/ai-provenance-foundation-tests/spec.md` | References error code `CC002` in scenario 14 (`validate_strict_fails_cc002_on_pending`), but `openspec/registries/error-codes.md` contains only `CC001`. | Either register `CC002` now (if the pre-phase is meant to reference it) or change the pre-phase test name to a neutral placeholder and defer the `CC002` reference to phase-7.6 apply tasks. |

---

## Minor Issues / Notes

| # | File | Issue | Recommended Fix |
|---|---|---|---|
| 1 | `src/cli/accept.rs` | `let _planned_tests = read_planned_tests();` reads the sidecar but never consumes the result. The parsing logic is effectively dead code. | Either wire the planned-test count into the gate output (e.g., report "X planned tests detected") or remove the call until phase-7.5c's sidecar integration is completed. |
| 2 | `cairn-macros/src/lib.rs` | The sidecar writer hardcodes `file: "<unknown>"` and `line: 0` despite `proc-macro2` having `span-locations` enabled. | Use `input_fn.sig.ident.span().start()` from `proc_macro2` to capture real file/line data, or remove the unused fields from the sidecar schema. |
| 3 | `cairn-macros/tests/planned_attribute.rs` | `planned_test_fixture` is not marked `#[test]`, so the fixture never exercises the ignored-test path in a test runner. | Add `#[test]` to the fixture so `cargo test -- --ignored` can verify the macro emits `#[ignore]` correctly. |
| 4 | `.pre-commit-config.yaml` | The new `openspec-validate-specs` hook uses bare `openspec validate --specs --strict`, while `accept.rs` uses `cflx openspec validate <id> --strict`. The CLI prefix inconsistency is benign but untidy. | Align on one prefix; both binaries appear to exist in the environment, so this is cosmetic. |
| 5 | Workspace test coverage | `cargo test` and `cargo clippy --all-targets` (without `--workspace`) do not build `cairn-macros` integration tests, hiding the compilation failure from the pre-push battery. | Add `--workspace` to the pre-push `cargo test` and `cargo clippy` hooks, or move the macro test to a unit test inside `cairn-macros/src/lib.rs` where it is always compiled. |
| 6 | `openspec/conventions.md` | The coverage-requirements paragraph was strengthened but still says "A future gate may automate public-function coverage checking in `scripts/pre-archive-rust-gates.sh`." That script does not yet implement this check. | No action required; this is a forward-compatible note. |

---

## Tool Output

### `cargo check`
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
```
*Passes (root package only).*

### `cargo clippy --all-targets --all-features -- -D warnings`
```
   Compiling cairn-macros v0.1.0 (/Users/george/repos/cairn/cairn-macros)
    Checking cairn v0.1.0 (/Users/george/repos/cairn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.74s
```
*Passes (root package only). `cairn-macros` tests are not checked without `--workspace`.*

### `cargo test --no-run`
```
    Blocking waiting for file lock on artifact directory
   Compiling cairn v0.1.0 (/Users/george/repos/cairn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.26s
  Executable unittests src/lib.rs (target/debug/deps/cairn-393e080a271436c3)
  ... (8 test executables for cairn package only)
```
*Passes (root package only).*

### `cargo test --no-run --workspace`
```
   Compiling cairn-macros v0.1.0 (/Users/george/repos/cairn/cairn-macros)
warning: function `planned_test_fixture` is never used
error: missing documentation for the crate
error: this assertion is always `true`
error: could not compile `cairn-macros` (test "planned_attribute") due to 3 previous errors
```
*Fails. This is the hidden compilation breakage.*

### `openspec validate --changes --strict`
```
- Validating...
✓ change/phase-10-distribution
✓ change/phase-10.0-tests
✓ change/phase-7.5d-graph-explorer-followups
✓ change/phase-7.6-ai-provenance-foundation
✓ change/phase-7.6.0-tests
✓ change/phase-7.7-ux-foundation
✓ change/phase-7.7.0-tests
✓ change/phase-7.8-cairn-export
✓ change/phase-7.8.0-tests
✓ change/phase-8-summariser
✓ change/phase-8.0-tests
✓ change/phase-9-brownfield
✓ change/phase-9.0-tests
Totals: 13 passed, 0 failed (13 items)
```
*Passes.*

### `openspec validate --specs --strict`
```
- Validating...
✓ spec/artefacts
✓ spec/changes
✓ spec/cli
✓ spec/foundation
✓ spec/graph-explorer
✓ spec/hooks
✓ spec/kernel
✓ spec/mcp
✓ spec/multi-target
✓ spec/parser
✓ spec/query
✓ spec/reconciliation
✓ spec/terminology-rename
✓ spec/testing-baseline
✓ spec/verification-states
Totals: 15 passed, 0 failed (15 items)
```
*Passes.*

---

## Consensus Recommendation

**Do not proceed to archive or merge additional work until the `cairn-macros/tests/planned_attribute.rs` compilation failures are fixed.** That file is the only committed Rust code that actually breaks the workspace lint contract.

After the macro test is fixed, address the phase-encoding inconsistency (76/77/78 → 706/707/708) across all three pre-phase proposals so that future apply agents do not write tests that violate the convention on first commit.

The `src/cli/accept.rs` reversion is correct: removing the spec-validation step from the per-change accept gate is the right separation of concerns because `.pre-commit-config.yaml` now runs `openspec validate --specs --strict` at pre-push time.

No Cargo.toml changes are required for the new test files; `cairn-macros` is already a workspace member and a dependency of the root crate, so `use cairn::cflx_planned;` will work in integration tests.
