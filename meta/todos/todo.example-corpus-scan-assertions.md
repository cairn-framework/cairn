---
node: cairn.tests
status: blocked
created: 2026-07-16
---

# Example Corpus Scan Assertions

The example surfaces exist but have no anti-rot loop: `examples/demo/` is
asserted for existence and capability coverage
(`tests/phase_10_distribution.rs`) but not against an expected scan
finding set, and `tests/fixtures/cairn-bootstrap/` is only smoke-parsed
(`tests/fixtures_smoke.rs`); CLAUDE.md already admits it "may lag behind
root".

Add a test (extend `tests/phase_10_distribution.rs` or new
`tests/examples_gate.rs`) that runs `cairn scan --json` against
`examples/demo` and `tests/fixtures/cairn-bootstrap` and asserts each
against a committed expected-findings JSON next to the fixture (demo
expected clean). This lands the anti-rot loop on the corpus that already
exists before any new fixture is added, and either fixes or forces
deletion of the lagging bootstrap fixture. Optionally add one
deliberately finding-bearing brownfield example later; brownfield and
orphan scenarios are today only exercised in tempdir-programmatic tests.

Motivation: `res.a2ui-analysis` finding 5 (a2ui ports one numbered
example corpus through every spec version as docs, CI fixture, and prompt
material; a new parallel corpus for cairn was refuted, closing the loop
on existing surfaces survived). No change proposal needed.

## Landed

`tests/examples_gate.rs` (PR #521). It copies each corpus to a temporary
directory, runs `cairn --file <copy>/cairn.blueprint scan --json`, and compares
the finding objects against `expected-findings.json` committed inside the corpus.

- `examples/demo` was repaired to a clean scan (8 findings to 0) and its baseline
  is `[]`, so any new finding fails the gate.
- The optional finding-bearing brownfield example was not added. Adding a corpus
  was out of scope until the existing ones were gated.

## Depends on

`todo.bootstrap-fixture-repair-or-delete` (node `cairn.tests`).

## Status note

BLOCKED, not done. `tests/fixtures/cairn-bootstrap` was neither repaired nor
deleted: its 22 warnings are pinned as a committed burn-down, which stops further
rot but settles nothing, and this todo's own scope asks for the fix-or-delete.
That verdict decides a substrate question `todo.blueprint-authorability-eval` and
`todo.artefact-filename-test-fixtures` both depend on, so the sizing rule sends it
to its own unit, `todo.bootstrap-fixture-repair-or-delete`, which carries the 22
findings grouped by cause. Close this todo when that verdict lands and the
bootstrap case in `tests/examples_gate.rs` reflects it.
