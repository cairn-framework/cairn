# Proposal: Phase 7.5a Test Fortification

## Dependencies

- Requires: `phase-7-mcp` (archived).
- Execution: MUST run before Phase 8 Summariser and before the paired Phase 7.5b Cleansing Splits.

## Problem/Context

Five modules exceed the 500-line ceiling defined in `openspec/conventions.md:55`:

| File | Lines | Multiple of limit |
|---|---|---|
| `src/changes.rs` | 1286 | 2.57× |
| `src/cli/mod.rs` | 1258 | 2.52× |
| `src/query_api.rs` | 1170 | 2.34× |
| `src/artefacts/registry.rs` | 1101 | 2.20× |
| `src/ui.rs` | 862 | 1.72× |

None of them carry an inline `#[cfg(test)] mod tests` block. The 48 integration tests under `tests/` exercise end-to-end paths but assert on substrings, not snapshots, and none pin the webui JSON wire format stabilised in Phase 7.

A structural split of these modules (Phase 7.5b) would proceed without a regression wall. Phase 7.5a establishes that wall so the split can be executed by a codex agent whose only correctness signal is the gate battery.

The project also has no gate enforcing the 500-line convention. Drift between convention and practice is itself a CAIRN anti-pattern: the framework that catches drift should catch its own.

## Proposed Solution

1. Add `insta` as a dev-dependency and lock every `/api/*` JSON response as a reviewed snapshot.
2. Author inline unit test modules for the five god modules, each pinning current observable behaviour (route dispatch, command dispatch, delta parse/serialise, lookup).
3. Add `scripts/check-file-sizes.sh` and wire it into `scripts/pre-archive-rust-gates.sh` with an opt-out mechanism for the currently-oversized files so future phases cannot re-exceed the ceiling without a deliberate, annotated choice.
4. Extend `scripts/cflx-analyze-cairn-phases.py` to accept phase ids of the form `phase-<major>[.<minor>][<suffix>]-<name>` so test-first pre-phases (`phase-8.0-tests`, etc.) can be interleaved with feature phases.
5. Update `openspec/conventions.md` and `AGENTS.md` to mandate test-first pre-phases for future feature phases and snapshot tests for any public JSON wire format. Define the `#[ignore = "awaits phase-N"]` pattern so pre-phases archive cleanly.

## Acceptance Criteria

- `insta` is a dev-dependency; `docs/testing.md` documents the `cargo insta review` workflow.
- Every `/api/*` endpoint has at least one snapshot test covering its normal response shape.
- Each of the five god modules has a `#[cfg(test)] mod tests` block with at least one test per public function or dispatch branch.
- `scripts/check-file-sizes.sh` exits non-zero when any `src/**/*.rs` file exceeds 500 lines without a `// cairn:allow-large-module reason: <non-empty>` comment on the first non-blank line.
- `scripts/pre-archive-rust-gates.sh` fails when `check-file-sizes.sh` fails.
- The five existing god modules carry the allow-list comment with reason `scheduled-for-phase-7.5b-split` so this phase archives green. Phase 7.5b removes those comments as files drop below the ceiling.
- `scripts/cflx-analyze-cairn-phases.py` accepts decimal + suffix phase ids and orders them correctly; unit-tested.
- `openspec/conventions.md` adds a "Test-first pre-phase" section; `AGENTS.md` references it.
- All strict Rust gates pass.

## Out of Scope

- Splitting any god module. That work is Phase 7.5b.
- Adding new features to any subsystem.
- Changing public wire formats. Snapshots pin current behaviour; any deliberate change is reviewed via `cargo insta review`.
- Brownfield or self-declaration of CAIRN's own subsystems in the blueprint. That is deferred to a post-phase-10 proposal.
