# Tasks: authorability-eval-instrument

- [x] 1. Declare the `cairn.authoreval` module in `cairn.blueprint` with its
  three paths and its contract, author `meta/contracts/authoreval.md`, declare
  `harness/authoreval` as an `assets` target in `cairn.config.yaml`, and record
  the placement ruling as `dec.authoreval-instrument-placement`.
- [x] 2. Implement the backend seam: the `AuthorevalBackend` trait, the request
  and response shapes, `BackendError` with its total failure classification,
  the deterministic `ReplayBackend`, and the `CommandBackend` that speaks JSON
  over stdin and stdout under a per-call timeout.
- [x] 3. Implement the scratch workspace: recursive copy of the fixture, and
  response application that rejects absolute paths and any component escaping
  the workspace root.
- [x] 4. Implement scoring over the production surfaces only: `cairn scan
  --strict` for the verdict and `cairn lint --json` for the findings, both run
  in the scratch workspace.
- [x] 5. Implement the failure taxonomy: `class`, `subclass`, the prefix table,
  and the persistence rule that only applies from attempt 2 onward.
- [x] 6. Implement the record schema and the bounded repair loop, with the four
  outcomes and their required record contents.
- [x] 7. Add the `cairn-authoreval` binary and its `[[bin]]` entry, and the
  smoke prompt at `harness/authoreval/prompts/smoke.decision-authoring.json`.
- [x] 8. Tests: every outcome with its full record contract, the taxonomy
  including persistence and the unknown-code fallback, workspace path
  rejection, and the fixture staying byte-identical after a full run.
