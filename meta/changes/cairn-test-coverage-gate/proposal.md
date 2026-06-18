# Proposal: Test-coverage integrity gate

## Dependencies

- Requires: `phase-1-kernel` (graph build, node states, claimed files).
- Builds on: `phase-4-hooks` (the existing `--strict` exit-code promotion).
- Resolves spikes: `cairn-a8z` (state-dependent test-coverage gate design),
  `cairn-87n` (implement the gate).

## Problem/Context

Cairn reconciles a declared architecture map against shipped code, but it has no
signal for a third reality-layer property: whether a module that exists is
actually tested. A synced module (code present) with no linked tests is drift
the same way a missing contract is drift. The recurring user wish, refined in
spike `cairn-a8z`, is for cairn to flag, and optionally gate, modules whose code
exists but whose test surface is absent.

Cairn never runs tests (non-goal, spec.md:837). It only checks existence and
linkage of the verification surface. This change adds that check: a
deterministic, convention-based scan-time finding.

## Proposed Solution

Add a graph-build validation, `validate_test_coverage`, that emits
`CAIRN_TEST_COVERAGE_MISSING` for synced leaf modules whose reconciled source
files contain no test marker. The check is:

- **Rust-first**: scans claimed `.rs` files for `#[cfg(test)]`. Other languages
  follow the same shape (a per-language marker set); Rust ships first because
  cairn dogfoods itself in Rust.
- **State-dependent**: ghost nodes (declared, no code) are exempt, mirroring the
  contract-pointer rule exactly (`build.rs:194-198`: ghost -> Warning-exempt,
  else required). Synced nodes (code exists) are checked.
- **Tag-exempt**: a node tagged `no-test-coverage` is skipped. This is the
  "skip integration-only modules by convention" escape hatch (e.g. the `tests/`
  crate, proc-macro crates).
- **Loose by default, strict on demand**: the finding is a Warning (advisory;
  surfaces in `cairn lint` and the tension hook, never blocks). The existing
  `cairn scan --strict` / `cairn lint --strict` flag already exits 1 on
  warnings, so the gate needs no new config knob.

## Acceptance Criteria

- A synced leaf module whose `.rs` files lack `#[cfg(test)]` produces exactly one
  `CAIRN_TEST_COVERAGE_MISSING` Warning finding naming the node.
- A ghost node produces no finding.
- A node tagged `no-test-coverage` produces no finding.
- A module whose files contain `#[cfg(test)]` produces no finding.
- `cairn scan` on the cairn repo reports zero such findings after exempting
  legitimately-testless modules (tag), and `cairn scan --strict` is clean.
- All strict Rust gates pass.

## Out of Scope

- Contract `## Tests` section verification (mechanism c in the spike). Follow-up.
- Import-edge test detection via LSP/tree-sitter (mechanism b). Phase 5 territory.
- Languages beyond Rust. Same shape, added later.
- A config toggle to disable the check. The tag is the per-node opt-out; if a
  project-wide toggle is needed, it is a trivial follow-up.
