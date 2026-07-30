---
node: cairn.tests
status: done
created: 2026-07-28
---

# Bootstrap Fixture Repair Or Delete

## Problem

`tests/fixtures/cairn-bootstrap` scans with 22 warnings, now pinned as a
committed burn-down in `tests/fixtures/cairn-bootstrap/expected-findings.json`
and gated by `tests/examples_gate.rs`. The baseline stops the fixture rotting
further; it does not decide whether the fixture should exist.

Measured on 2026-07-28 with `cairn --file
tests/fixtures/cairn-bootstrap/cairn.blueprint scan --json`:

- 7 `CAIRN_ARTEFACT_POINTER_MISSING`: nested pointer directories
  (`./meta/decisions/kernel/parser/` and similar) that the flat artefact layout
  ruled out (`dec.artefact-filename-rule`).
- 9 `CAIRN_RECONCILE_LANGUAGE_UNKNOWN`: the fixture claims `src/*` paths that do
  not exist inside the fixture, so no language can be inferred.
- 6 `CAIRN_CONTRACT_MISSING`: contract pointers with no file behind them.

All three groups are the fixture describing a repository it is not. `AGENTS.md`
states it "may lag behind the root blueprint" and it is only smoke-parsed
(`tests/fixtures_smoke.rs`).

## Scope

Take one verdict and land it:

- **Repair**: flatten the pointer directories, drop or populate the `src/*`
  targets and the contract pointers, and reduce the baseline to `[]`. Then fold
  the fixture into a clean-scan assertion in `tests/examples_gate.rs` and delete
  its `expected-findings.json`.
- **Delete**: remove the fixture, its baseline, its case in
  `tests/examples_gate.rs`, and the smoke tests that read it
  (`test_bootstrap_fixture_readable_contains_declared_node`,
  `test_bootstrap_fixture_sources_are_named_for_their_ids`,
  `test_bootstrap_fixture_sources_do_not_cite_themselves`), then name the
  surviving corpus wherever the deleted one is cited.

Check the two readers of the fixture before deciding:
`todo.blueprint-authorability-eval` scores model output against a temporary copy
of it, and `todo.artefact-filename-test-fixtures` uses its sources directory as a
corpus no reconciler reaches. Repair keeps both. Delete has to relocate both.

## Verdict

The maintainer ratified REPAIR on 2026-07-29 (PR #528 sheet W10). The repair
itself is a later loop unit, not part of the ratification batch, so this todo
stays open until it lands. Dependants (`todo.blueprint-authorability-eval`,
`todo.example-corpus-scan-assertions`) unblock when the repair lands, not at
ratification.

## Depends on

Nothing. The verdict was given 2026-07-29; see Verdict.

## Acceptance

- `cargo test --test examples_gate` and `cargo test --test fixtures_smoke` pass
  under the chosen verdict.
- No `expected-findings.json` survives holding a non-empty set for a fixture the
  verdict repaired.
- `cairn scan` on the root repository reports no new finding.

## Dependants

- `todo.blueprint-authorability-eval` (node `cairn.root`) was blocked on this
  verdict, then on the repair landing; it moved to `open` in the landing
  commit. Its primary metric (iterations to a clean scan of this fixture) is
  measurable now that the fixture starts clean.
- `todo.example-corpus-scan-assertions` (node `cairn.tests`) closed `done` in
  the landing commit: the verdict is reflected in the bootstrap case in
  `tests/examples_gate.rs` (`test_bootstrap_fixture_scans_clean`).

## Landed

The repair executed on 2026-07-30: pointer directories flattened into
`meta/decisions/` and `meta/research/` (slug-only filenames), the nine
phantom `src/*` paths dropped (declaration-only corpus), the six missing
contracts populated, `expected-findings.json` deleted, and the bootstrap
case in `tests/examples_gate.rs` folded into a direct clean-scan assertion.
The evidence corpus (`meta/sources/`, `meta/research/`) stays deliberately
unclaimed so the smoke-test corpus premise holds; the two `scan --strict`
exit-code tests in `tests/phase_7_7_ux_foundation.rs` own an inline
warning-only project instead of borrowing the fixture's dirt. Evidence:
`res.bootstrap-fixture-repair`.
