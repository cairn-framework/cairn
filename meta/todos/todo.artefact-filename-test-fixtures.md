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

`src/cli/commands/decision.rs` is the deliberate exception: its fixture creates
`dec.my-rule.md` precisely to exercise the legacy-collision guard, and it must
keep doing so.

## Task

Rename the fixture paths to the conforming form, updating any path assertion
that names the old filename. Snapshot output will move with them; review the
diff to confirm only paths changed.

## Acceptance

- No fixture outside `src/cli/commands/decision.rs` writes a
  `meta/{decisions,research,sources}/{dec,res,src}.<slug>.md` path.
- `cargo test` passes with snapshots reviewed, not blanket-accepted.
