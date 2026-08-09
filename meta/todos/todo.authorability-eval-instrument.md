---
node: cairn.root
status: open
created: 2026-08-09
parent: todo.blueprint-authorability-eval
---

# Authorability Eval Instrument

## Scope
Deliver the authoring-eval instrument in one accepted and applied change.

The change artefacts settle five questions before the code is written, and the
parent depends on all five:

1. Where the instrument lives, and which node's `path` in `cairn.blueprint`
   claims it. The parent is anchored at `cairn.root` on the basis that it writes
   only research artefacts and unowned scaffolding, so a declared surface
   re-opens the anchor question.
2. The model-execution seam. The harness owns model execution; Cairn owns
   prompts, fixtures, production validation, and deterministic scoring. Fix the
   invocation contract (command, arguments, request and response shape, timeout,
   failure classification) so an offline backend and a real one are
   interchangeable.
3. The deterministic repair loop: what is fed back after a failed scan, how many
   iterations are allowed, and what terminates the loop.
4. The record schema: one shape for every run, carrying an outcome state, the
   iterations and tokens spent as the primary metric, first-shot validity and
   per-format failure hotspots as secondary, and the backend and model identity
   the run used, so a record can never be read without knowing what produced
   it. Four outcomes, and every one emits a record: clean at first shot; clean
   after repair; the repair bound exhausted, carrying the hotspots from the
   last failed scan; and the backend failing to answer
   (timeout or invocation error, per question 2's failure classification),
   carrying that error and no scan hotspots, since no scan ran. A prompt always
   produces exactly one record, so a live run's records can be counted against
   its corpus.
5. The failure taxonomy the parent's third acceptance bullet needs, so a failure
   is attributable to blueprint or artefact syntax, to generated guidance, or to
   a missing repair affordance.

The implementation is one prompt in, one record out:

- copy `tests/fixtures/cairn-bootstrap` into a scratch workspace so the
  checked-in fixture is never mutated;
- invoke the model through the backend contract above, so an offline backend and
  a real harness backend are interchangeable;
- apply the response to the workspace copy;
- score it with the production surfaces, `cairn scan --strict` and
  `cairn lint --json`, never a reimplementation of either;
- run the bounded deterministic repair loop and emit one record.

Ship a deterministic offline backend and one smoke prompt so the whole path is
exercised by a test with no network and no model.

## Parent constraints
The parent todo is `todo.blueprint-authorability-eval`. Its `## Depends on`
section is why this unit carries a change:

> `todo.bootstrap-fixture-repair-or-delete`, so the fixture substrate is both
> trustworthy and clean. This needs a change proposal because it adds a declared
> harness or scripts surface.

That fixture dependency is discharged: the repair landed and
`tests/examples_gate.rs::test_bootstrap_fixture_scans_clean` holds the fixture
clean, so iterations to a clean scan is measurable from zero.

Its second acceptance bullet binds this child whole, and the scorer half of the
first binds it too. The authoring corpus half belongs to
`todo.authorability-eval-prompt-corpus`:

> - The authoring corpus and scorer run unattended on demand.
> - The production parser, scanner, and lint surfaces grade outputs.

Its reuse constraint also binds:

> Reuse the agent-guidance evaluation runner or the summariser's
> `LocalCommandBackend` pattern rather than building another orchestrator. The
> oh-my-pi harness owns model execution. Cairn owns prompts, fixtures, production
> validation, and deterministic scoring.

The archived runner is `run_baseline.py` inside
`archive/strongholds/agent-guidance-baseline/evidence.tar.gz`. Read it before
writing a new orchestrator.

The parent's substrate constraint bounds what a prompt may touch:

> Verified failure mode: adding a `research` pointer while `meta/sources/` stays
> unreached breaks the clean baseline (`CAIRN_RESEARCH_MISSING_SOURCES` at Error,
> or `CAIRN_RESEARCH_UNKNOWN_SOURCE` at Warning when the research cites the
> unloaded sources). Keep eval prompts inside the loaded authority corpus
> (modules, contracts, decisions, todos, reviews).

## Dependencies
None. This unit is the prerequisite for
`todo.authorability-eval-prompt-corpus`, represented by that todo's typed
`blocked_by` edge. Unblocking it is this unit's own work, at Reconcile: see the
last Acceptance bullet.

## Acceptance
- The change is created, its tasks are ticked, and `cairn change accept`
  passes. Land's `cairn change apply` archives it inside this unit's single
  commit, so no active change directory survives the iteration.
- All five questions in `## Scope` are answered in the change's `design.md`,
  each with a concrete artefact: a node id and `path` line, a request and
  response shape, an iteration bound and termination rule, a record schema, and
  the failure taxonomy's classes.
- One command runs the smoke prompt end to end against the offline backend and
  exits 0 with no network, no API key, and no installed harness.
- The run emits exactly one record per prompt, carrying the outcome state, the
  backend and model identity, iterations, tokens, first-shot validity, and zero
  or more failure hotspots. A first-shot-clean run carries none; every hotspot
  a failed scan produces carries a class from the taxonomy.
- The checked-in `tests/fixtures/cairn-bootstrap` is byte-identical after a run.
- Tests exercise every outcome and assert the full record contract for each,
  including deterministic backend and model identities, the last failed scan's
  hotspots on exhaustion, and the classified error with no scan hotspots on a
  backend that fails to answer.
- Scoring calls `cairn scan --strict` and `cairn lint --json`; no finding logic
  is reimplemented inside the instrument.
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D
  warnings`, `cargo test`, `cairn scan --strict`, and `cairn hook all` all pass.
- At Reconcile, in this order, leaving both edits staged for Land:
  `cairn todo set authorability-eval-instrument done`, then, with that blocker
  now done, `cairn todo set authorability-eval-prompt-corpus open`. Both flips
  reach main in this unit's single commit. The parent stays `blocked`; it flips
  to `done` only once the prompt corpus is done too.

## Sizing
M. Change artefacts, one runner, one scorer, one offline backend, one smoke
prompt, and the tests that defend the record contract.

## Non-goals
Do not author the real prompt corpus or run a real model; that is
`todo.authorability-eval-prompt-corpus`. Do not add a CI job, and do not make
the smoke test depend on a network, an API key, or an installed harness.
