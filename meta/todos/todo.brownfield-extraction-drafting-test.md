---
node: cairn.brownfield
status: done
created: 2026-08-10
parent: todo.brownfield-extraction-external-validation
---

# Exercise the drafting entry point end to end on a fixture repository

Validation unit split out of `todo.brownfield-extraction-external-validation`
under the sizing rule. It carries the deterministic half of that unit: the
end-to-end drafted-artefact assertion, running entirely in-repo. The
external-repository run belongs to `todo.brownfield-extraction-external-run`,
and the maintainer's ruling on a draft to
`todo.brownfield-extraction-maintainer-ruling`.

## Task

Exercise the drafting entry point end to end against a fixture repository that
contains ADR-like material. Build the fixture the way the existing onboard
coverage in `tests/kernel.rs` does (a temp root, not a checked-in tree), give it
a loadable `cairn.blueprint` whose System block declares both
`decisions "./meta/decisions"` and `research "./meta/research"`, and give it
ordinary project material the closed evidence set already reads: a `docs/adr/`
document, a README `Decision` section, and an `// invariant:` source comment.

Then run the flow in the order the shipped reference names
(`.claude/skills/cairn-dev/references/task-brownfield-decision-extraction.md`):
`cairn onboard decisions --json`, hand-author the fixture's own
`method: primary` research artefact under its research directory, take the
`node` field of a bound entry verbatim, and run
`cairn decision new <slug> --node <id> --informed-by <research-id>`. The
research artefact is part of the flow, not an extra: without it the draft's
`informed_by` dangles and the test asserts on provenance the reference forbids.

Assert on the resulting artefact, not on the report. The report is already
covered by `todo.brownfield-onboard-decisions-index`; what is uncovered is that
a bound entry carries through the writer into a decision artefact with a real
binding and a non-accepted status.

## Non-goals

No change to the command surface, the shipped reference, or the pack wiring.
No artefact in this repository's own `meta/` tree: every artefact this test
writes lives under its temp fixture root. No external-repository run and no
`meta/sources/` artefact: those are `todo.brownfield-extraction-external-run`.
No maintainer ruling: that is `todo.brownfield-extraction-maintainer-ruling`.

## Acceptance

- A test exercises the drafting entry point against a fixture repository with
  ADR-like material and asserts that the drafted decision artefact's `nodes:`
  binding names a node that exists in the fixture's blueprint and that its
  status is exactly `proposed`.
- The fixture is not the dogfood repo, carries no cairn artefact directory
  before the run, and carries no cairn-specific annotation beyond the
  `cairn.blueprint` the flow requires. The `// invariant:` comment is ordinary
  source prose the closed evidence set reads, not a cairn marker added for the
  test.
- The test is deterministic and safe in the full suite: it asserts on the
  artefact it wrote under its own temp root and leaves nothing behind in the
  repository.
- If this landing is the one that completes
  `todo.brownfield-extraction-external-validation` (that is, the external run
  already landed), it also performs that parent's handoff: remove the resolved
  `blocked_by` edge `todo.brownfield-extraction-maintainer-ruling` declares on
  the parent, leave that todo `blocked` with the body line the run wrote, and
  give the maintainer the same two steps in order, record the outcome and any
  rejection reason in the run's research artefact, then
  `cairn todo set brownfield-extraction-maintainer-ruling open`. Left in place,
  that edge stands `CAIRN_TODO_STATUS_CONTRADICTION` until someone clears it.
- `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cairn scan --strict` all pass.
