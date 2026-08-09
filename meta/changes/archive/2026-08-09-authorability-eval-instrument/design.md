# Design: authorability-eval-instrument

The five questions `todo.authorability-eval-instrument` requires be settled
before code, each answered with a concrete artefact.

## 1. Where the instrument lives, and which node claims it

The instrument is a new leaf Module in `cairn.blueprint`:

```
Module AuthorEval "Authorability eval instrument: prompt in, scored record out" id "cairn.authoreval" {
    path "./src/authoreval"
    path "./src/bin/cairn-authoreval.rs"
    path "./harness/authoreval"
    contract "./meta/contracts/authoreval.md"
}
```

No tag is applied: `cairn.config.yaml` declares the opt-in tag registry and
carries none that fits, and an unregistered tag is a
`CAIRN_TAG_UNREGISTERED` finding. `harness/authoreval` holds the prompt corpus,
which is data with no reconcilable language, so the same file declares it as an
`assets` target, the mechanism `src/ui_assets` and `tools/agent-pack` already
use.

The ruling is recorded as `dec.authoreval-instrument-placement`, which the new
module needs anyway: a blueprint shape change with no decision covering it is
`CAIRN_BLUEPRINT_CHANGE_NO_DECISION` at Error.

It declares no blueprint edge. Every reconciled subsystem, the scanner and the
query spine included, is reached only by invoking the `cairn` binary as an
external process, so the module introduces no dependency on another subsystem
and no cycle: `cairn deps cairn.authoreval` is empty by construction. It does
use the crate's shared substrate, `CairnError` and the panic hook that
`cairn.root` claims, exactly as `cairn.lsp`, `cairn.kernel.cli`, and
`cairn.brownfield` do; the blueprint edges none of those to `cairn.root`, which
carries no inbound or outbound edge at all.

Anchor question. The parent is anchored at `cairn.root` on the basis that it
writes only research artefacts and unowned scaffolding, and a declared surface
re-opens that. Resolution: the surface moves to `cairn.authoreval`, and the two
authorability todos stay at `cairn.root` because they are programme items that
predate the module and whose remaining output (a research artefact and a prompt
corpus) is still unowned scaffolding. Any later work on the instrument itself
anchors at `cairn.authoreval`.

Why a separate binary rather than a `cairn` subcommand: the shipped CLI is
already 51 commands over roughly seven operation families
(`dec.cli-agent-workflow-consolidation`), and an eval instrument is development
tooling, not a user surface. `cairn-authoreval` follows the existing
`cairn-mcp` and `cairn-lsp` precedent: its own `[[bin]]`, its own module, no
entry in the `cairn` command table.

## 2. The model-execution seam

The harness owns model execution. Cairn owns the prompt, the fixture, the
production validation, and the scoring. The seam between them is one trait:

```rust
pub(crate) trait AuthorevalBackend {
    fn identity(&self) -> BackendIdentity;
    fn invoke(&self, request: &AuthorRequest<'_>, timeout: Duration)
        -> Result<AuthorResponse, BackendError>;
}
```

The trait is crate-internal on purpose. An external harness does not implement
it; it speaks the subprocess JSON protocol below, which is the supported seam.
Keeping the trait private keeps the library contract to `RunConfig`,
`BackendSpec`, `run_prompt_file`, and the record types.

`timeout` is a per-call obligation rather than backend state, matching
`SummariserBackend`, so every implementation that can block honours the same
deadline contract. `ReplayBackend` is exempt: it answers from memory and cannot
exceed a deadline, and it uses `timeout` only to shape a scripted timeout
failure.

Request, written to the child's stdin as one JSON object:

```json
{
  "schema_version": 1,
  "prompt_id": "smoke.decision-authoring",
  "attempt": 1,
  "instruction": "...",
  "findings": [
    {"severity": "error", "code": "CAIRN_ARTEFACT_MISSING_FIELD",
     "message": "...", "node": null, "path": "meta/decisions/x.md",
     "deferred_by": null, "parked_by": null}
  ]
}
```

`findings` mirrors the `cairn lint --json` wire field for field. `deferred_by`
and `parked_by` are exactly what tells a model a finding is not its to fix, so
dropping them would make the feedback something other than the previous scan.

`attempt` is 1 on the first invocation and increments per repair. `findings` is
empty on attempt 1 and otherwise carries the previous failed scan verbatim.

Response, read from the child's stdout as one JSON object:

```json
{
  "files": [{"path": "meta/decisions/slug.md", "contents": "..."}],
  "tokens": {"prompt": 1200, "completion": 340}
}
```

`files` is the complete post-edit content of every path the model wrote,
relative to the scratch workspace root. A path that is absolute, empty,
directory-shaped, NUL-bearing, or that escapes the workspace is rejected, and
the whole batch is validated before a single byte is written, so no malformed
path can half-apply. An I/O failure still can, and that is deliberate: it fails
the run, emits no record, and drops the workspace, so nothing observes it.

The response must cover every path the prompt declares in `expects`. The
fixture already scans clean, so without that rule an answer that wrote nothing,
or wrote some unrelated valid file, would score `clean_first_shot` and the run
would report perfect authorability for no authoring at all. A response that
misses an expected path is a protocol violation, recorded as a
`backend_failure` whose error names the unauthored paths. This keeps the four
outcomes intact rather than inventing a fifth.

Command contract. `CommandBackend` spawns `program` with fixed `args`, writes
the request JSON to stdin, closes stdin, reads stdout to end, and kills the
child when the deadline elapses.

The deadline bounds the backend's execution rather than the wall time of the
whole call: nothing is spawned once it has passed, and once the child has
exited, collecting bytes it already wrote gets a short bounded grace, because
discarding a complete answer from a backend that finished near its budget would
be worse.

Three details make that deadline real, and they are why this is not a
line-for-line copy of `summariser::backend::LocalCommandBackend`, whose shape it
otherwise follows. The deadline starts at entry, not after the request is
written. The stdout and stderr readers start before the request is written, so a
backend that emits a pipe buffer's worth of output before draining stdin cannot
deadlock against a parent blocked in `write_all`; the write itself runs
off-thread for the same reason. Every error raised before the child is seen to
exit funnels through one helper that kills and reaps it, because dropping a
`Child` does not reap it. Errors raised after that point (a drain timeout, a
non-zero status, an unparseable answer) see an already-reaped child.

The plumbing is mirrored rather than shared: sharing it would mean either a new
blueprint edge from a dev-tool module into `cairn.summariser` or refactoring a
stable module for this tool's benefit, and neither is worth the coupling. The
summariser copy still has the defects this one fixed; correcting them there is
that module's unit, not this one.

Failure classification is fixed and total:

| `BackendError` | class | meaning |
|---|---|---|
| `Timeout` | `timeout` | the deadline elapsed. A spawned child is killed and reaped; an already-expired budget spawns nothing, and a replay script can serve this outcome with no child at all |
| `NonZeroExit` | `invocation` | the backend ran and failed |
| `Io` | `invocation` | the backend could not be spawned or spoken to |
| `ScriptExhausted` | `invocation` | a replay script ran out of turns |
| `Parse` | `protocol` | the backend answered, but not in the response shape |

`identity()` is what the record carries, so no record can be read without
knowing which backend and which model produced it.

## 3. The deterministic repair loop

One prompt, one workspace, at most `1 + max_repairs` model invocations.
`max_repairs` defaults to 3, so at most 4 invocations.

Each iteration: invoke the backend, check the response covers the prompt's
`expects`, apply it to the scratch workspace, then score. Scoring runs
`cairn scan --strict` for the verdict (its exit status is the gate: 0 is clean)
and `cairn lint --json` for the structured findings, both in the scratch
workspace, both the production surfaces. No finding logic is reimplemented.

A non-zero exit from `lint` is expected and ignored: blocking findings are the
measurement. A lint envelope with no `findings` key is not ignored; it is an
instrument fault, because reading it as "no findings" would erase the repair
feedback and could hand back a clean verdict the scan never gave. A relative
scoring-binary path is absolutised once per run, since every scoring call sets
the child's working directory and a relative program path would then resolve
platform-dependently. A bare program name is left alone: it means a `PATH`
lookup.

What is fed back after a failure: the previous failed scan's findings, verbatim
and sorted, in the `findings` field of the next request. Nothing else. The
instrument never rewrites, summarises, or ranks them, because the thing under
measurement is whether cairn's own output is enough to repair from.

Termination, checked in this order:

1. The backend does not deliver a usable answer, at any attempt: it fails to
   respond, times out, writes a path the workspace refuses, or leaves an
   expected path unauthored. Stop, outcome `backend_failure`. A protocol
   rejection can follow a response that did arrive, and even an attempt that
   already scanned.
2. The scan is clean: stop, outcome `clean_first_shot` when `attempt == 1`,
   otherwise `clean_after_repair`.
3. `attempt == 1 + max_repairs`: stop, outcome `repair_bound_exhausted`.

Determinism: no randomness, no wall-clock in the record, findings sorted by
severity rank, then code, then path, then node, then message, and hotspots
sorted by code.

## 4. The record schema

A run that completes produces exactly one record per prompt, so a live run's
records can be counted against its corpus. An instrument fault produces none: it
fails the run, per "Exit status of the runner" below. Records are emitted as
JSON Lines.

```json
{
  "schema_version": 1,
  "prompt_id": "smoke.decision-authoring",
  "outcome": "clean_first_shot",
  "backend": {"kind": "replay", "model": "offline-replay/v1"},
  "iterations": 1,
  "tokens": {"prompt": 1200, "completion": 340, "total": 1540},
  "first_shot_valid": true,
  "hotspots": [],
  "error": null
}
```

The four outcomes, and what each is required to carry:

| `outcome` | `iterations` | `first_shot_valid` | `hotspots` | `error` |
|---|---|---|---|---|
| `clean_first_shot` | 1 | `true` | empty | `null` |
| `clean_after_repair` | 2 or more | `false` | the last failed scan's | `null` |
| `repair_bound_exhausted` | `1 + max_repairs` | `false` | the last failed scan's | `null` |
| `backend_failure` | attempts made, including the failed one | `false`, always: a first shot that scanned clean would have stopped the loop | empty | the classified error |

`hotspots` is empty on `backend_failure` even when an earlier scan failed: the
record's error is the finding, and mixing stale scan hotspots into it would
misattribute an infrastructure failure to authoring quality.

`tokens` accumulates every response received, so an exhausted run reports the
full cost it spent. Primary metric is `iterations` and `tokens`; secondary are
`first_shot_valid` and the `hotspots`.

A hotspot aggregates the findings sharing one code:

```json
{"class": "syntax", "subclass": "artefact",
 "code": "CAIRN_ARTEFACT_MISSING_FIELD", "severity": "error", "count": 1,
 "node": null, "path": "meta/decisions/cli-json-default.md"}
```

`node` and `path` are the location fields the lint wire actually publishes.

## 5. The failure taxonomy

The parent's third acceptance bullet requires a failure be attributable to
syntax, to generated guidance, or to a missing repair affordance. `class` is
exactly those three. `subclass` keeps the finer attribution this unit needs
without inventing a fourth peer class.

| `class` | `subclass` | assigned when |
|---|---|---|
| `syntax` | `blueprint` | the code names a defect in the `.blueprint` text the model authored |
| `syntax` | `artefact` | the code names malformed or incomplete artefact frontmatter |
| `generated_guidance` | `graph` | the text is well formed but the declared graph does not match the tree, which is what cairn's generated guidance exists to prevent |
| `generated_guidance` | `unknown` | the code is outside the classification table, so the table's own coverage gap stays visible in the data |
| `missing_repair_affordance` | the originating subclass | the same code was present in the immediately preceding failed scan and survived a repair attempt, so the feedback offered nothing that cleared it |

Precedence is fixed: `missing_repair_affordance` first, then the code table.
Persistence is only assignable from attempt 2 onward, because a first-shot
failure has no preceding scan and calling it a missing affordance would be
false history.

The code table is prefix-driven, evaluated in declaration order, so a new
finding code lands in `generated_guidance`/`unknown` rather than silently
mimicking an unrelated class:

| prefix | class | subclass |
|---|---|---|
| `CAIRN_PARSE_`, `CAIRN_BLUEPRINT_`, `CAIRN_INTEGRITY_`, `CAIRN_TAG_UNREGISTERED`, `CAIRN_ORDER_CYCLE`, `CAIRN_NO_BLUEPRINT`, `CAIRN_IO_READ_BLUEPRINT` | `syntax` | `blueprint` |
| `CAIRN_ARTEFACT_`, `CAIRN_DECISION_`, `CAIRN_TODO_`, `CAIRN_RESEARCH_`, `CAIRN_SOURCE_`, `CAIRN_REVIEW_`, `CAIRN_CONTRACT_`, `CAIRN_CHANGE_` | `syntax` | `artefact` |
| `CAIRN_RECONCILE_`, `CAIRN_PROVENANCE_`, `CAIRN_TEST_COVERAGE_`, `CAIRN_MODULE_OVERSIZED`, `CAIRN_INTERFACE_HASH_CHANGED`, `CAIRN_PATH_GITIGNORED`, `CAIRN_SPEC_RULE_`, `CAIRN_HOOK_` | `generated_guidance` | `graph` |
| anything else | `generated_guidance` | `unknown` |

## Fixture safety

The checked-in `tests/fixtures/cairn-bootstrap` is never the workspace. Every
run copies it into a fresh scratch directory owned by a `TempDir`, created
uniquely and removed on drop, and mutates only the copy. Nothing deletes a
guessed path. The copy rejects symlinked entries rather than following them, so
a fixture cannot pull content in from outside itself or recurse through a cycle.
A test hashes the fixture tree before and after a full run and asserts it is
byte-identical.

## Smoke prompt

`harness/authoreval/prompts/smoke.decision-authoring.json` asks for a decision
artefact covering a named node, which is inside the fixture's loaded authority
corpus and therefore respects the parent's substrate constraint: the fixture's
`meta/sources/` and `meta/research/` stay unreached, so no prompt may add a
`research` pointer. Its replay script returns a valid decision on the first
turn, so the smoke command exercises the whole path and lands on
`clean_first_shot`.

## How the todo's one-record acceptance is read

The todo says "one prompt in, one record out" and "The run emits exactly one
record per prompt". That is read as covering every measured outcome, and it is
satisfied: a timeout, an invocation error, and a protocol violation each emit a
record, including a protocol rejection of a response that arrived after an
attempt had already scanned.

It is not read as covering an instrument fault. An unreadable prompt, a missing
fixture, a production surface that will not run, or a failed write are the
instrument being unable to measure, not a measurement whose record went missing,
and `run_prompt_file` returns `Err` for them so the runner exits non-zero and
writes nothing. Emitting a record there would be worse: it would put a row in
the corpus asserting an authoring outcome that was never observed, and a live
run counts its records against its prompts.

The behavioural surfaces state this distinction directly: the contract's Purpose
and its one-record invariant, `run_prompt_file`'s `# Errors`, the record module
documentation, and the exit-status section below.

## Exit status of the runner

The runner exits 0 when every prompt produced a record, including records whose
outcome is a failure. A failed authoring attempt is a successful measurement.
Non-zero is reserved for instrument faults: an unreadable prompt, a missing
fixture, a missing `cairn` binary, or an unwritable output path.

## Changes

ADDED:
- `src/authoreval/{mod,cli,prompt,record,scorer,taxonomy,workspace}.rs`,
  `src/authoreval/backend/{mod,command}.rs`, and
  `src/authoreval/runner/{mod,progress}.rs`: the instrument, its
  model-execution seam, its repair loop, and its invocation parsing.
- `src/authoreval/tests/{mod,taxonomy,workspace,cli,prompt}.rs`: taxonomy,
  workspace containment and the path validator, invocation parsing and binary
  resolution, and prompt validation.
- `src/authoreval/loop_tests.rs`: the repair-feedback contract, the scoring
  envelope, and the command backend's subprocess lifecycle, driven against stub
  executables.
- `src/bin/cairn-authoreval.rs`: the runner binary.
- `tests/authoreval_instrument.rs`: every record outcome driven through the
  real scorer.
- `tests/authoreval_protocol.rs`: protocol refusals, fixture safety, hotspot
  location, and the command-level smoke run.
- `tests/authoreval_support/mod.rs`: the fixtures and helpers both share.
- `harness/authoreval/prompts/smoke.decision-authoring.json`: the smoke prompt
  and its replay script.
- `meta/contracts/authoreval.md`: the module contract.
- `meta/decisions/authoreval-instrument-placement.md`: the placement ruling the
  new module requires.
- `meta/research/authoreval-instrument-evidence.md`: what building the
  instrument showed.
- `meta/todos/todo.remediate-code-filter-ignored.md`,
  `meta/todos/todo.accept-gate-stale-path-binary.md`,
  `meta/todos/todo.conventions-thiserror-divergence.md`: the defects that
  evidence names.

MODIFIED:
- `cairn.blueprint`: declares the `cairn.authoreval` module.
- `cairn.config.yaml`: declares `./harness/authoreval` as an `assets` target.
- `Cargo.toml` and `Cargo.lock`: the `cairn-authoreval` binary, and camino's
  `serde1` feature so the public config type can carry `Utf8PathBuf` and still
  derive serde.
- `src/lib.rs`: declares the module.
- `src/error.rs`: adds `CairnError::AuthorEval`.
- `docs/registries/error-codes.md`: allocates `CO008`.
- `tests/finding_code_coverage.rs`: delists `CAIRN_ARTEFACT_MISSING_FIELD`,
  which this unit's tests now genuinely trigger.
- `meta/todos/todo.authorability-eval-instrument.md` to `done` and
  `meta/todos/todo.authorability-eval-prompt-corpus.md` to `open`, with the
  dependency prose in that todo and in the parent
  `meta/todos/todo.blueprint-authorability-eval.md` corrected to match.
- `map.json`: regenerated.

REMOVED:
- Nothing.

RENAMED:
- Nothing.
