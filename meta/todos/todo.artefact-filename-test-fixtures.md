---
node: cairn.tests
status: done
created: 2026-07-27
---

# Test fixtures still model the forbidden artefact filename shape

## Problem

`dec.artefact-filename-rule` settled that a decision, research, or source
filename is its `id` with the typed prefix stripped, and
`CAIRN_ARTEFACT_FILENAME_DRIFT` (CA038) enforces it.

Three fixtures were conformed while landing that rule, because they assert on
findings or on paths and so failed without it:

- `src/artefacts/registry/mod.rs` `write_project`
- `tests/kernel.rs` `write_phase_2_fixture`
- `tests/wire_format_snapshots.rs` `write_project`

The rest were left alone. They still write `meta/decisions/dec.<slug>.md` and
friends, and they pass only because nothing in them asserts that the finding
list is empty:

- `src/ui/mod.rs` (several fixtures and path assertions)
- `src/query_api/serialise.rs`, `src/query_api/tests.rs`
- `src/cli/render/artefacts/tests.rs`
- `src/hooks/architecture.rs`
- `tests/kernel.rs` (fixtures other than `write_phase_2_fixture`)
- `tests/mcp.rs`, `tests/graph_explorer.rs`

## Why it matters

Each of those fixtures now silently carries a CA038 warning inside its
`ArtefactSet`. Nothing breaks today, but the next test written against one of
them that asserts a clean finding list will fail for a reason unrelated to what
it is testing, and every one of these is a worked example of the shape the tool
now warns about.

Three fixtures are deliberate exceptions, because each one exists to exercise
the forbidden shape and must keep producing it:

- `src/cli/commands/decision.rs` creates `dec.my-rule.md` to exercise the
  legacy-collision guard.
- `src/artefacts/registry/validate/tests.rs` is CA038's own unit test, and
  `make_decision` derives the path from the id, so a `dec.`-prefixed id
  necessarily produces the legacy shape it asserts on.
- `tests/artefact_filename_remediation.rs` writes `dec.only-rule.md` to prove
  the finding fails a strict scan and yields a rename plan.

## Task

Rename the fixture paths to the conforming form, updating any path assertion
that names the old filename. Snapshot output will move with them; review the
diff to confirm only paths changed.

## Acceptance

- No fixture outside the three deliberate exceptions above writes a
  `meta/{decisions,research,sources}/{dec,res,src}.<slug>.md` path. The second
  and third exception were missed when this todo was written, which made the
  original single-exception wording unsatisfiable: a filename-drift detector
  cannot be tested without a drifted filename, so honouring it literally would
  have meant deleting CA038's own coverage.
- `cargo test` passes with snapshots reviewed, not blanket-accepted.

## Out of scope

`tests/fixtures/cairn-bootstrap/meta/sources/` holds nine `src.*.md` files that
carry the forbidden prefix, two of which also disagree with their own `id`
(`src.dlthub-ontology-first.md` declares `id: src.dlthub-map-first`,
`src.structurizr-dsl.md` declares `id: src.structurizr-blueprint`). They are
not fixtures in this todo's sense and they carry no CA038 warning: the bootstrap
blueprint declares no `sources` pointer, so nothing ever loads them. Measured on
2026-07-27, `cairn --file tests/fixtures/cairn-bootstrap/cairn.blueprint scan
--strict` reports 23 findings and not one is
`CAIRN_ARTEFACT_FILENAME_DRIFT`. Renaming them is real work with a real
judgment call in it (`src.review-adversarial-1.md` has `file:
./meta/sources/review-adversarial-1.md`, the exact path its conforming name
would occupy), so it is tracked as
`todo.bootstrap-fixture-artefact-filenames`.
