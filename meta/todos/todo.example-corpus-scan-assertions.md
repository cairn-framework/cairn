---
node: cairn.tests
status: open
created: 2026-07-16
---

# Example Corpus Scan Assertions

The example surfaces exist but have no anti-rot loop: `examples/demo/` is
asserted for existence and capability coverage
(`tests/phase_10_distribution.rs`) but not against an expected scan
finding set, and `test/fixtures/cairn-bootstrap/` is only smoke-parsed
(`tests/fixtures_smoke.rs`); CLAUDE.md already admits it "may lag behind
root".

Add a test (extend `tests/phase_10_distribution.rs` or new
`tests/examples_gate.rs`) that runs `cairn scan --json` against
`examples/demo` and `test/fixtures/cairn-bootstrap` and asserts each
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
