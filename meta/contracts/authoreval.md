---
node: cairn.authoreval
---

# Contract: cairn.authoreval

## Purpose

Measures whether a model authors valid cairn blueprint and artefact syntax. One
prompt goes in, and a run that completes gives one record back. Every applicable
response is scored by cairn's own production surfaces, so the instrument
measures the shipped validators rather than a copy of them. An attempt that
never produced an applicable response, because the backend failed, timed out, or
answered outside the protocol, is recorded without a score. An instrument fault
fails the run and produces no record at all.

The harness owns model execution. This module owns the prompt, the scratch
fixture, the invocation contract, the repair loop, the scoring, and the record.

## Public interface

- `RunConfig`: fixture, scoring binary, backend, repair bound, and per-call
  deadline in milliseconds.
- `BackendSpec`: `Replay` for the offline deterministic backend, or `Command`
  for a program speaking the JSON contract on stdin and stdout.
- `run_prompt_file(config, prompt_path)`: runs one prompt end to end and
  returns its `Record`.
- `Record`, `Outcome`, `Hotspot`, `RecordError`, `TokenTotals`,
  `BackendIdentity`, `BackendErrorClass`, `FailureClass`, `FailureSubclass`:
  the record schema and its taxonomy.
- `Invocation`, `help_text()`, `sibling_cairn_bin()`, `RECORD_SCHEMA_VERSION`:
  the `cairn-authoreval` binary's surface.

The backend trait is crate-internal. An external harness integrates through the
subprocess JSON protocol, not by implementing a Rust trait.

## Invariants

- The checked-in fixture is never mutated. Every run works on a scratch copy
  owned by a `TempDir` and removed on drop.
- A response path that is absolute, empty, directory-shaped, NUL-bearing, or
  that escapes the workspace root, or whose ancestors are not all directories,
  is rejected. The whole batch is validated before any byte is written, so no
  malformed path can leave a partial edit behind. A later I/O failure can, but
  it fails the run, emits no record, and drops the workspace, so that state is
  never observed.
- The fixture copy rejects symlinked entries rather than following them.
- A response must author every path its prompt declares in `expects`. The
  fixture scans clean, so a response that authors nothing, or something
  unrelated, would otherwise earn `clean_first_shot` for no authoring.
- Scoring calls `cairn scan --strict` for the verdict and `cairn lint --json`
  for the findings. No finding logic is reimplemented here. A lint envelope with
  no `findings` key fails closed rather than reading as a clean scan.
- A relative scoring-binary path is absolutised before any spawn, because every
  scoring call sets the child's working directory. A bare program name is left
  alone: it means a `PATH` lookup.
- The command backend's deadline bounds the backend's execution: it starts
  before the spawn, and nothing is spawned once it has passed. Collecting output
  an exited child already wrote may take a short bounded grace beyond it. Output
  readers start before the request is written, and every error raised before the
  child is seen to exit kills and reaps it; errors after that point see an
  already-reaped child. A backend that forks a descendant inheriting its
  standard streams is out of contract: only the direct child is reaped.
- Repair feedback is the previous failed scan's findings verbatim, sorted, and
  nothing else, mirroring the lint wire field for field including `deferred_by`
  and `parked_by`. Rewriting them would measure the instrument, not cairn.
- A run that completes emits exactly one record per prompt, whatever the
  outcome. An instrument fault emits none: it fails the run.
- A first-shot-clean run carries no hotspots. A backend failure carries the
  classified error and no hotspots.
- `missing_repair_affordance` is only assignable from attempt 2 onward, because
  a first-shot failure has no preceding scan to have survived.
- The module declares no blueprint edge: every reconciled subsystem is reached
  only by invoking the `cairn` binary as an external process. Rationale in
  `dec.authoreval-instrument-placement`.

## Failure modes

- Unreadable or invalid prompt file: `CairnError::AuthorEval`, no record.
- Missing fixture, or a production surface that will not run:
  `CairnError::AuthorEval`, no record.
- A backend that fails to answer, answers unusably, or answers without authoring
  the prompt's expected paths: a `backend_failure` record, not an error. Timeout,
  invocation, and protocol are the three classes; an unusable path and an
  unauthored expected path are both protocol, and both can follow a response
  that arrived and even a scan that already ran.
