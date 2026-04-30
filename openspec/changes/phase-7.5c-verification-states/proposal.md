# Proposal: Phase 7.5c Verification States

**Change Type**: hybrid

## Dependencies

- `phase-7.5b-cleansing-splits` (required dependency, archived).

Execution: MUST run BEFORE `phase-8.0-tests` (and therefore before `phase-9.0-tests` and `phase-10.0-tests`). Archives when the new attribute, the state enum, the registry update, and the convention rewrites are in place and the strict Rust gate battery passes.

## Sequencing

This phase has a hard ordering constraint. Three test-first pre-phases are queued in `openspec/changes/` (`phase-8.0-tests`, `phase-9.0-tests`, `phase-10.0-tests`) and each one is scoped to write `#[ignore = "awaits phase-N"]` test stubs. Today, no `#[ignore]` attribute exists in `src/` or `tests/`. If `phase-7.5c` lands first, those three pre-phases pick up the new `#[cflx_planned(phase = N)]` attribute from the start with zero retroactive rewrite. If `phase-7.5c` lands after, every applied stub must migrate. The retroactive cost is sharp; ship `phase-7.5c` first.

The lifecycle order is therefore:

1. `phase-7.5b-cleansing-splits` (archived).
2. `phase-7.5c-verification-states` (this phase).
3. `phase-8.0-tests` (uses `#[cflx_planned(phase = 8)]` from the start).
4. `phase-8-summariser` (removes the attribute as features land).
5. `phase-9.0-tests`, `phase-9-brownfield`, `phase-10.0-tests`, `phase-10-distribution` (same pattern).

## Problem/Context

Cairn's verification battery is binary at gate time: a test either passes or fails under `cargo test`. Test stubs that target a future phase use `#[ignore = "awaits phase-N"]`, a comment-string convention defined in `openspec/conventions.md` section 5 and `openspec/specs/testing-baseline/spec.md` requirement "Test-first pre-phase convention". Two problems follow.

1. The phase target is encoded as freeform prose inside an `#[ignore]` reason string. Tooling that wants to surface "which tests are awaiting which phase" has to parse a comment, not query a structured attribute. Agents removing attributes when features land are instructed by `AGENTS.md` line 25 to grep for the comment string, which is fragile.
2. The battery conflates a test that asserted false (`Failed`) with a test that could not execute because of an upstream missing piece (`Blocked`). The two outcomes have different fix paths. Conflating them sends agents and humans down the wrong investigation path when the gate goes red.

This phase introduces a structured replacement that resolves both problems without promoting verification to a kernel artefact type.

## Proposed Solution

Add five things.

1. A new `cairn-macros/` Cargo workspace member declared `proc-macro = true`. This crate is the home for cflx-side proc macros, starting with `#[cflx_planned]`.
2. A `#[cflx_planned(phase = N)]` attribute proc-macro that emits `#[ignore = "cflx_planned: phase-N"]` plus a build-time registration of the attached test name and target phase number into a sidecar registry under `target/cflx/planned.json`.
3. A five-state verification enum (`Draft`, `Planned`, `Passed`, `Failed`, `Blocked`) attached to test attributes via `#[cflx_planned]` and the existing `cargo test` pass/fail signal. The enum is a logical contract; the implementation surfaces `Planned` (via the proc-macro) and `Blocked` (via the new error code), and treats `Draft`, `Passed`, `Failed` as formalisations of states already implicit in the existing battery.
4. A new error code `CC001` for the `Blocked` state, allocated in the existing `C` (Changes) category of `openspec/registries/error-codes.md` and consulted by the `cflx accept` gate to distinguish a blocked verification from a failed one.
5. Convention and agent-orientation rewrites: `openspec/conventions.md` section 5 ("Test-First Pre-Phase") replaces the `#[ignore = "awaits phase-<N>"]` reference with the new attribute; `openspec/specs/testing-baseline/spec.md` requirement "Test-first pre-phase convention" is updated in lockstep; `AGENTS.md` line 25 is updated to instruct agents to remove the structured attribute, not parse the comment string.

The phase does NOT introduce a `verification` artefact type to `openspec/specs/artefacts/spec.md`. The state enum lives on test attributes and in a build-derived sidecar; promoting verification to a kernel artefact is deferred until a downstream consumer demands a richer schema.

## Acceptance Criteria

- The `cairn-macros/` crate is a member of the Cargo workspace, declares `proc-macro = true`, and has its `[lints]` section set to `workspace = true`.
- The attribute `#[cflx_planned(phase = N)]` accepts a single integer named argument `phase` and emits `#[ignore = "cflx_planned: phase-N"]` plus a sidecar registration entry under `target/cflx/planned.json`.
- A test marked `#[cflx_planned(phase = N)]` is reported as ignored under `cargo test`.
- A test marked `#[cflx_planned(phase = N)]` is reported as failed under `cargo test -- --ignored` if its body returns false or panics.
- The five-state enum `VerificationState` with variants `Draft`, `Planned`, `Passed`, `Failed`, `Blocked` is defined in the cairn crate and is consulted by `cflx accept` gate logic when a test surfaces a `Blocked` outcome.
- The error code `CC001` is allocated in `openspec/registries/error-codes.md` under the `CC -- Changes` heading with the description `verification blocked by upstream dependency` and the introducing phase `phase-7.5c`.
- A `Blocked` outcome surfaced through the cairn error pipeline carries the code `CC001` via `CairnError.code() -> "CC001"`.
- `openspec/conventions.md` section 5 references `#[cflx_planned(phase = <N>)]` in place of `#[ignore = "awaits phase-<N>"]` for the test-first pre-phase pattern.
- `openspec/specs/testing-baseline/spec.md` requirement "Test-first pre-phase convention" references the new attribute.
- `AGENTS.md` instructs agents to remove the `#[cflx_planned(phase = <N>)]` attribute (not parse the `#[ignore]` comment) when the corresponding feature lands, and notes that the attribute is structured.
- All strict Rust gates pass: `cargo build` (zero warnings), `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`, `cargo fmt --check`, `cargo test`, `cargo test --locked`, `cflx openspec validate phase-7.5c-verification-states --strict`.

## Out of Scope

- Promoting verification to a kernel artefact type in `openspec/specs/artefacts/spec.md`.
- A native `cargo test` skip mechanism that replaces `#[ignore]`. The proc-macro emits `#[ignore]` underneath so `cargo test` continues to work without runner changes.
- Additional `#[cflx_planned]` argument variants beyond `phase = N` (for example `infra = "..."` or `artefact = "..."`). The attribute syntax is forward-compatible with named arguments, but no other variant is implemented in this phase.
- A `cairn verifications --status planned` query command. Sidecar data exists; query commands over it are deferred.
- Splitting `Blocked` into multiple sub-codes (`CC002`, `CC003`) for "missing upstream phase", "missing infrastructure", "missing artefact reference". One code (`CC001`) ships now; sub-codes wait on operational evidence.
- Modifying the `Makefile` `status-phases` target. That target parses `tasks.md` checkboxes and is unrelated to the attribute migration.
- Migrating any existing `#[ignore]` site. There are no live `#[ignore]` sites in `src/` or `tests/` at the time this phase is authored. The three queued pre-phase proposals continue to declare `#[ignore]` in their task text; their apply step will use `#[cflx_planned]` because this phase ships first.
