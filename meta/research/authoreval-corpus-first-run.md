---
id: res.authoreval-corpus-first-run
nodes:
  - cairn.authoreval
  - cairn.root
date: 2026-08-10
method: primary
---

# First real-model run of the authorability corpus: what it showed

Recorded while attempting `todo.authorability-eval-prompt-corpus`. A six-prompt
authoring corpus was built, every shape was hand-verified satisfiable, and the
corpus was run against a real harness backend. The run did not satisfy the
todo's acceptance, so no corpus and no records landed. Four observations did,
and each is reproduced below with what produced it.

## Setup

Fixture: `tests/fixtures/cairn-bootstrap`, which scans clean before every
attempt (`cairn scan --strict` exit 0 on a fresh copy). Backend: the
`--backend command` seam, driven by an adapter that reads cairn's
`AuthorRequest` on stdin, calls `omp -p --mode json --no-tools --no-skills
--no-session` with `--model anthropic/claude-sonnet-4-5` in an empty working
directory, and writes cairn's `AuthorResponse` on stdout. Repair bound: the
default 3, so at most 4 attempts per prompt.

The adapter is not in this repository, by the instrument's own contract: "the
harness owns model execution", and the module integrates "through the
subprocess JSON protocol, not by implementing a Rust trait". Two adapter
choices shape the numbers below and are not cairn's:

- The model-facing envelope is a delimiter format (`<<<FILE path` ... `>>>FILE`),
  not raw JSON. Asking a model to JSON-escape a whole Rust file measures
  escaping, not cairn syntax.
- Reported `prompt` tokens sum input, cache reads, and cache writes; `completion`
  tokens sum output.

## 1. An unparseable blueprint aborts the run instead of scoring

The blueprint-authoring prompt asked for a module claiming named files. Its
`instruction` is reproduced here because the prompt file was reverted and
exists nowhere in the repository, and because whether the defect is a model
tendency or an artefact of a leading prompt cannot be judged without it. After
an embedded copy of the fixture's current `cairn.blueprint`, it read:

> Requirements:
> - Declare the module inside `Container Kernel` with the id
>   `cairn.kernel.watch`, claiming the path `./src/watch` and a contract at
>   `./meta/contracts/kernel/watch.md`.
> - Declare an edge from the new module to `cairn.kernel.scanner`.
> - Author the contract file it points at.
> - Author the two Rust files the module claims: `src/watch/mod.rs` and
>   `src/watch/debounce.rs`. Keep them small; `debounce.rs` exposes a function
>   that collapses a burst of event paths into the settled set.
> - The project must still reconcile cleanly afterwards.

The instruction names the path but says nothing about where a `path`
declaration goes, and the embedded blueprint shows the body form on no module,
because no fixture module declares a path. The model put `path` in the module
header rather than the module body:

```
Module Watch "..." id "cairn.kernel.watch" path "./src/watch" {
    contract "./meta/contracts/kernel/watch.md"
}
```

Scoring that workspace:

```
$ cairn lint --json
{"error":{"code":"CAIRN_COMMAND_FAILED","message":"cairn.blueprint:57:101: expected `{`, encountered word `path`","remediation":null,"source_span":"cairn.blueprint"}}
$ cairn-authoreval run ... --backend command ...
cairn-authoreval: authorability eval error: `.../cairn lint --json` published no `findings` key; refusing to read that as a clean scan
```

`src/authoreval/scorer.rs` requires a `findings` key and fails closed when it is
absent. That is right for a truncated envelope and wrong for this one: the
envelope is well formed and is reporting the model's defect. The run raises a
`CairnError` and emits no record for any prompt in the invocation.

Reproduced five times out of five, from five independent invocations of the
module prompt. Every answer put `path` in the module header, and every run
aborted the same way. Three of the five were run specifically to test whether
the abort is a coin flip; the instruction was unchanged between them, and the
model's own wording of the module description varied while the misplacement did
not. Rerunning the other five prompts as separate invocations scored all five
normally, which isolates the fault to the error envelope rather than to the
corpus or the backend. Per-prompt invocations are a diagnostic, not a
substitute: the acceptance clause reads "one unattended run over the whole
corpus".

### The cheap alternative works, and that is the problem

A reference module answer exists and was hand-verified to scan clean, so the
cheapest route around the reroute is real: reword the instruction to show where
a `path` declaration goes, and the abort stops. That was run as a sixth probe
and it worked. One added sentence ("A `path` declaration goes inside the module
body, on its own line, in the same place the existing modules put `contract`.
It is not part of the module header.") put `path` in the body on all four
attempts, the run completed, and it produced a record:
`repair_bound_exhausted`, 4 iterations, 28187 tokens, hotspots
`CAIRN_ARTEFACT_MISSING_FIELD` on the decision it authored and
`CAIRN_PROVENANCE_NO_DECISION` on the new node.

So the acceptance clause is reachable without the scorer fix. The reroute does
not rest on the alternative failing. It rests on what the alternative costs.

The original instruction names the path and never shows the body form, and the
fixture it embeds declares no module path anywhere, so the model had no
in-context example. Under that instruction the model misplaced `path` five
times out of five. That is the finding: without an example, this model does not
know where a `path` goes. A corpus that must hand the model the grammar in
order to keep the harness from crashing cannot report that finding, because the
prompt has already answered the question it was asking. Buying a completed run
by teaching the answer deletes the measurement the parent commissioned.

Two facts about the code decide the rest. Scoring runs inside the attempt loop
in `src/authoreval/runner/mod.rs`, so an unparseable first answer raises before
the loop can feed anything back: the repair bound cannot rescue this failure.
And `src/bin/cairn-authoreval.rs` buffers every record and writes them only
after the last prompt, so one unparseable answer from one prompt discards the
records of every prompt that already succeeded. The blast radius of a single
syntax slip is the whole corpus, which is why prompt wording is the wrong place
to absorb it.

Consequence: the failure class the instrument most wants to measure, blueprint
syntax, is the one class it cannot record. Filed as
`todo.authoreval-lint-error-envelope` against `cairn.authoreval`, with three
options and a recommendation.

## 2. A `blueprint.delta` is not graded by the surfaces the scorer runs

The parent's authoring family requires a `blueprint.delta` prompt. The scorer
cannot grade one. A delta renaming a node that does not exist:

```
## RENAMED Nodes
- cairn.kernel.nonexistent -> cairn.kernel.blueprint
```

placed at `meta/changes/rename-parser-to-blueprint/blueprint.delta` beside a
`proposal.md` gives `cairn scan --strict` exit 0 and `cairn lint --json` zero
findings. `cairn change list` parses it and reports `1 renamed_nodes` without
validating the id. The reference answer and the deliberately broken answer score
identically.

A validator does exist, on a path the scorer never takes: `cairn change apply`
calls `validate_change` (`src/changes/mod.rs`), and
`src/changes/validate/mod.rs` rejects a rename whose source node is unknown.
The scorer runs `scan --strict` and `lint --json` only, so it never reaches it.

Consequence: a delta prompt scores `clean_first_shot` whatever the model writes.
The corpus must still carry one, because the parent's family names it, but its
result is unmeasured and must be reported as such rather than counted in
first-shot validity. Grading it would mean the scorer invoking change
validation, which is a change to shipped scoring behaviour and outside the
corpus unit's non-goals. That is a choice for a later unit with its own
evidence, not a blocker on the corpus: an unmeasured record is still a record.

Deliberately not filed as a todo. The gap is in shipped `scan`/`lint` coverage
of a staged delta, not in the corpus, and whether cairn should validate node
ids in an unapplied change directory is a scope call on `cairn.kernel.changes`
that belongs to the maintainer, not to a loop iteration reroute. It is recorded
here and named in the corpus todo's amended acceptance so neither is lost.

## 3. One missing field at a time makes a model oscillate

The todo-authoring prompt is the only prompt that both ran to completion and
failed. Outcome `repair_bound_exhausted` over 4 iterations, 12945 prompt tokens
and 2074 completion tokens, one hotspot: `CAIRN_ARTEFACT_MISSING_FIELD` at Error
on `meta/todos/todo.debounce-window-tuning.md`, classified
`missing_repair_affordance`.

The model wrote `state:` for `status:` and `raised:` for `created:`. The
validator reports the first missing field only, so the feedback named one field
per round, and the answers alternated:

| Attempt | Frontmatter | Finding fed back |
|---|---|---|
| 1 | `state`, `raised` | lacks required `status` |
| 2 | `status`, `raised` | lacks required `created` |
| 3 | `state`, `created` | lacks required `status` |
| 4 | `status`, `raised` | bound exhausted |

Each answer fixed the field the finding named and reverted the other. The
taxonomy's persistence rule keys on the finding code, not the field, so the
oscillation reads as one persistent code and earns
`missing_repair_affordance`. That attribution is right about the outcome: the
feedback was not sufficient to converge. It is coarse about the cause, which is
one-field-at-a-time reporting rather than an unrepairable finding.

This is a result the corpus unit exists to publish, not a defect to fix in it.

## 4. All six prompt shapes are satisfiable, and three of the five that produced a record scored clean first shot

Every shape was hand-verified before the run: a reference answer for each was
applied to a fixture copy and scored `cairn scan --strict` exit 0. Two shapes
need more than the obvious file:

- A module claiming named files needs `#[cfg(test)]` coverage in the claimed
  source and a decision artefact for the new node, or
  `CAIRN_TEST_COVERAGE_MISSING` and `CAIRN_PROVENANCE_NO_DECISION` stand at
  Warning and the scan is dirty.
- A contract on a container needs the blueprint pointer and the contract file in
  the same answer.

Diagnostic per-prompt results, at `anthropic/claude-sonnet-4-5`:

| Prompt shape | Outcome | Iterations | Total tokens |
|---|---|---|---|
| module claiming named files | run aborted, no record, 5 of 5 | 1 attempted each | 6356 first, 33 216 over five |
| `blueprint.delta` for a rename | `clean_first_shot` (unmeasured, see 2) | 1 | 5314 |
| decision covering two nodes | `clean_first_shot` | 1 | 3687 |
| todo | `repair_bound_exhausted` | 4 | 15019 |
| review | `clean_first_shot` | 1 | 4615 |
| contract on a container | `clean_first_shot` | 1 | 6517 |
| module, prompt pre-teaching the body form (probe) | `repair_bound_exhausted` | 4 | 28187 |

These numbers are diagnostic only. They come from eleven invocations rather than
the one the acceptance requires, and one prompt produced no record at all, so
they are not the parent's result and no aggregate validity rate is computed from
them.

The aborted prompt's row comes from the adapter's own log, not from a record:
the run raised before the record was written, so the instrument published
nothing for it. Prompt tokens were 3967 every time; completion tokens were
2389, 2601, 2817, 3161 and 2413. Reporting those as spent rather than as
unknown follows the corpus todo's own rule that a prompt which never reached a
clean scan reports what it spent under its outcome state.

The five surviving records, verbatim, so the table is checkable after the
prompts and the run output are gone:

```json
{"schema_version":1,"prompt_id":"corpus.blueprint-delta-rename","outcome":"clean_first_shot","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":1,"tokens":{"prompt":3835,"completion":1479,"total":5314},"first_shot_valid":true,"hotspots":[],"error":null}
{"schema_version":1,"prompt_id":"corpus.decision-multi-node","outcome":"clean_first_shot","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":1,"tokens":{"prompt":3041,"completion":646,"total":3687},"first_shot_valid":true,"hotspots":[],"error":null}
{"schema_version":1,"prompt_id":"corpus.todo-open-work","outcome":"repair_bound_exhausted","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":4,"tokens":{"prompt":12945,"completion":2074,"total":15019},"first_shot_valid":false,"hotspots":[{"class":"missing_repair_affordance","subclass":"artefact","code":"CAIRN_ARTEFACT_MISSING_FIELD","severity":"error","count":1,"node":null,"path":"meta/todos/todo.debounce-window-tuning.md"}],"error":null}
{"schema_version":1,"prompt_id":"corpus.review-attestation","outcome":"clean_first_shot","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":1,"tokens":{"prompt":3065,"completion":1550,"total":4615},"first_shot_valid":true,"hotspots":[],"error":null}
{"schema_version":1,"prompt_id":"corpus.contract-container","outcome":"clean_first_shot","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":1,"tokens":{"prompt":3898,"completion":2619,"total":6517},"first_shot_valid":true,"hotspots":[],"error":null}
```

## What this does not show

Nothing here is the parent's answer. The corpus, the single unattended run, and
the published metrics remain `todo.authorability-eval-prompt-corpus`, which is
now blocked behind `todo.authoreval-lint-error-envelope`.
