# Tasks: Phase 7.5c Verification States

## 1. Workspace and Macro Crate

- [ ] 1.1 Add `cairn-macros/` as a Cargo workspace member with `proc-macro = true` and `[lints] workspace = true`.
- [ ] 1.2 Add `syn` and `quote` to the workspace dependency list, and `cairn-macros` to the cairn crate's `[dependencies]`.
- [ ] 1.3 Implement the `#[cflx_planned(phase = N)]` attribute proc-macro in `cairn-macros/src/lib.rs`, parsing the `phase` named argument as a non-negative integer and rejecting `phase = 0` or negative values at expansion time.
- [ ] 1.4 Make the macro emit `#[ignore = "cflx_planned: phase-<N>"]` on the attached function.
- [ ] 1.5 Make the macro emit a registration entry into `target/cflx/planned.json` with `version = 1`, `test_path`, `phase`, `file`, and `line` fields, following the state-versioning convention in `openspec/conventions.md` section 3.
- [ ] 1.6 Reject macro expansion when the attached function already carries an explicit `#[ignore]` attribute, with a clear compile error directing the author to choose one mechanism.
- [ ] 1.7 Re-export the attribute from the cairn crate via `pub use cairn_macros::cflx_planned;` in `src/lib.rs`.
- [ ] 1.8 Add a fixture-based unit test under `cairn-macros/tests/` that compiles a function marked `#[cflx_planned(phase = 8)]` and asserts the macro output and sidecar entry.

## 2. Verification State Enum and Error Code

- [ ] 2.1 Define `VerificationState` in `src/verification.rs` with variants `Draft`, `Planned`, `Passed`, `Failed`, `Blocked`, deriving `Debug`, `Clone`, `PartialEq`, `Eq`, `serde::Serialize`, `serde::Deserialize`.
- [ ] 2.2 Re-export `VerificationState` from `src/lib.rs`.
- [ ] 2.3 Add or extend a `CairnError` variant for blocked verifications carrying an upstream-cause field, ensuring `.code()` returns `"CC001"`.
- [ ] 2.4 Allocate `CC001 -- verification blocked by upstream dependency -- phase-7.5c` under the `CC -- Changes` heading in `openspec/registries/error-codes.md`.
- [ ] 2.5 Add a unit test that round-trips each `VerificationState` variant through `serde_json`.
- [ ] 2.6 Add a unit test that constructs a blocked-verification error and asserts `.code() == "CC001"`.

## 3. cflx accept Gate Integration

- [ ] 3.1 Update `cflx accept` gate logic to read `target/cflx/planned.json` (when present) and label matching test outcomes as `Planned`.
- [ ] 3.2 Update `cflx accept` gate logic to label test outcomes carrying error code `CC001` as `Blocked` rather than `Failed`, and to surface the upstream cause in human-readable and JSON output.
- [ ] 3.3 Confirm that a `Blocked` outcome does NOT fail `cflx accept` by default in this phase; document the deferral in the gate's help text.
- [ ] 3.4 Add an integration test running `cflx accept` over a fixture phase with one passing, one `#[cflx_planned]`, and one blocked test, asserting all three classifications.

## 4. Conventions and Spec Updates

- [ ] 4.1 Rewrite the "Test-First Pre-Phase" subsection of `openspec/conventions.md` section 5 so it references `#[cflx_planned(phase = <N>)]`, with a short note that the macro expands to `#[ignore]` underneath.
- [ ] 4.2 Add a paragraph in the same section introducing the five-state `VerificationState` enum and pointing readers at `openspec/specs/testing-baseline/spec.md` for canonical scenarios and `openspec/registries/error-codes.md` for `CC001`.
- [ ] 4.3 Update the requirement "Test-first pre-phase convention" in `openspec/specs/testing-baseline/spec.md` so prose, scenarios, and example reference the new attribute.
- [ ] 4.4 Add a new requirement "Verification states attached to test attributes" to `openspec/specs/testing-baseline/spec.md` with scenarios for planned classification, blocked classification, and `CC001` surfacing.
- [ ] 4.5 Update `AGENTS.md` line 25 so the agent-facing instruction names `#[cflx_planned(phase = <N>)]`, with a second sentence noting the attribute is structured and the `#[ignore]` reason string MUST NOT be parsed.

## 5. Required Verification

- [ ] 5.1 `cargo build` passes with zero warnings.
- [ ] 5.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 5.3 `cargo fmt --check` passes.
- [ ] 5.4 `cargo test` passes.
- [ ] 5.5 `cargo test --locked` passes.
- [ ] 5.6 `cflx openspec validate phase-7.5c-verification-states --strict` passes.
