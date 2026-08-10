---
id: res.authoreval-corpus-baseline
nodes:
  - cairn.authoreval
  - cairn.root
date: 2026-08-10
method: primary
---

# Authorability baseline: what the corpus runs showed

The first published measurement from `todo.blueprint-authorability-eval`. A
six-prompt authoring corpus was authored, every shape was proved satisfiable
against the real scorer before any model saw it, and the corpus was run
unattended against a real harness backend. The published run is the one whose
prompts are the prompts in the repository. A second, earlier run over five
byte-identical prompts is reported beside it, because the two disagree and the
disagreement is the most useful thing in this artefact.

The predecessor, `res.authoreval-corpus-first-run`, is the record of an attempt
that produced no publishable result. Its four observations shaped this corpus
and three of them are re-tested below.

## Setup

Fixture: `tests/fixtures/cairn-bootstrap`, whose checked-in state scans clean,
asserted by `tests/examples_gate.rs::test_bootstrap_fixture_scans_clean`. Each
prompt starts from its own scratch copy of it, so no prompt sees another's edit.
A prompt's repair attempts then reuse that one copy, which is what makes
iterations to a clean scan measurable from a known-clean start.

Backend: the `--backend command` seam, driven by a harness-owned adapter. Model:
`anthropic/claude-sonnet-4-5`, carried in every record's `backend` field. Repair
bound: the default 3, so at most 4 attempts per prompt.

**Record scope, which bounds every claim below.** A record keeps the last
failed scan's findings and nothing else: intermediate attempts are discarded,
and a backend failure keeps no findings at all. The
`missing_repair_affordance` class means one specific thing, that the code was
also present in the immediately preceding failed scan, which is itself not
retained. So no claim here reaches an attempt the instrument did not keep, and
"retained" below always carries that meaning.

The invocation, with the local adapter and output paths elided:

```
cairn-authoreval run harness/authoreval/prompts/corpus.*.json \
  --backend command --command bun --command-arg <adapter> \
  --model anthropic/claude-sonnet-4-5 --timeout-ms 300000 \
  --out <records.jsonl>
```

Exit 0, six records, one per prompt, from one invocation.

### The adapter, and the choices that are not cairn's

The adapter is not in this repository, by the instrument's own contract: the
harness owns model execution, and an external harness integrates "through the
subprocess JSON protocol, not by implementing a Rust trait". It reads cairn's
`AuthorRequest` on stdin and writes cairn's `AuthorResponse` on stdout. Between
those two points it runs:

```
omp -p --mode json --no-tools --no-skills --no-rules --no-session --no-title \
  --model anthropic/claude-sonnet-4-5 <prompt>
```

in a fresh empty working directory. No tools, no skills, no rules, no session:
the model sees the prompt text and nothing else about cairn.

Three adapter choices shape the numbers and are worth naming, because a run that
changes any of them is not comparable:

- The model-facing envelope is a delimiter format, not raw JSON. The adapter
  appends to cairn's instruction: reply with nothing but files, each wrapped as
  `<<<FILE relative/path` ... `>>>FILE`, one envelope per file, no prose and no
  code fences. Asking a model to JSON-escape a whole Rust file would measure
  escaping, not cairn syntax.
- On a repair attempt the adapter appends cairn's findings verbatim as JSON
  under one sentence of framing ("Your previous answer did not reconcile
  cleanly ... Fix them and reply with the complete content of every file
  again"). It rewrites nothing: the repair signal under test is cairn's.
- Reported `prompt` tokens sum input, cache reads, and cache writes; `completion`
  tokens sum output. That definition is the adapter's, and it is the only part of
  this bullet the records can show. The rest is an unverified harness-side
  observation, recorded because it bears on how the figures read: the harness
  caches its own system prompt, so the first attempt of each prompt carries a
  large cache-write component that the aggregate totals in a record do not break
  out. Treat the absolute figures as harness-inflated rather than as a model-cost
  estimate.

## The corpus and its conditions

Six prompts under `harness/authoreval/prompts/corpus.*.json`, covering the
parent's authoring family: a module claiming named files, a `blueprint.delta`
for a refactor, a decision covering two nodes, a todo, a review, and a contract
on a container. `tests/authoreval_corpus.rs` asserts the shape coverage, the
size bound, and the substrate compliance against the corpus files themselves.

Three conditions are uniform across all six, and they decide how the numbers
read:

- **The blueprint is in context, the schemas are not.** Every prompt embeds the
  fixture's complete `cairn.blueprint`, because a real authoring agent can see
  the project it is editing. No prompt names a frontmatter field, and the module
  prompt never shows the body form of a `path` declaration, which no fixture
  module carries. The model authors artefact frontmatter from its own knowledge
  of cairn plus whatever cairn's findings tell it.
- **No prompt reaches the unloaded evidence corpus.** The fixture deliberately
  leaves `meta/research/` and `meta/sources/` unclaimed, so each prompt carries
  the sentinel forbidding a `research` or `sources` pointer, and no expected
  path lands there. Asserted in `tests/authoreval_corpus.rs`, not inferred from
  a clean scan.
- **Every shape is proved satisfiable before a model runs.** Each prompt carries
  a hand-written reference answer as its replay script.
  `test_every_corpus_prompt_is_satisfiable` applies all six offline and requires
  `clean_first_shot` from the real scorer, so a dirty result is never the
  corpus's own defect.

## Results

The published run, over the corpus exactly as it stands in the repository:

| Prompt | Outcome | Iterations | Prompt tokens | Completion tokens | Total |
|---|---|---|---|---|---|
| `corpus.blueprint-delta-rename` | `clean_first_shot` (unmeasured) | 1 | 6086 | 4053 | 10 139 |
| `corpus.contract-container` | `clean_after_repair` | 2 | 12 188 | 4283 | 16 471 |
| `corpus.decision-multi-node` | `clean_after_repair` | 2 | 12 410 | 1152 | 13 562 |
| `corpus.module-claiming-files` | `backend_failure` (protocol) | 4 | 25 837 | 11 732 | 37 569 |
| `corpus.review-attestation` | `clean_after_repair` | 2 | 12 314 | 1085 | 13 399 |
| `corpus.todo-open-work` | `repair_bound_exhausted` | 4 | 24 717 | 1958 | 26 675 |

The whole run cost 117 815 tokens.

**Primary metric, iterations and tokens to a clean scan.** Over the five
measured prompts, three reached a clean scan, each in exactly 2 iterations, for
43 432 tokens between them, a mean of 14 477. One, `corpus.todo-open-work`,
exhausted the repair bound: 4 iterations and 26 675 tokens spent, no clean scan
earned. One, `corpus.module-claiming-files`, produced no authoring score at all:
its final response left `meta/decisions/watch-module.md`, which the prompt
requires, unauthored after 4 iterations and 37 569 tokens, so the instrument
recorded a `protocol` backend failure rather than a score. Those are what each
spent, not clean-scan figures they never earned.

**Secondary metric, first-shot validity.** Zero of five. Not one measured prompt
produced an answer that reconciled on the first attempt, under the stated
condition of blueprint in context and schemas withheld. Every clean result in
this run arrived on a later invocation, after cairn's findings were fed back.

**Secondary metric, per-format hotspots.** Five hotspots across the four records
that carry any:

| Code | Record | Class | Subclass | Severity |
|---|---|---|---|---|
| `CAIRN_CONTRACT_MISSING_NODE` | contract | `syntax` | artefact | error |
| `CAIRN_DECISION_MISSING_NODES` | decision | `syntax` | artefact | error |
| `CAIRN_ARTEFACT_FILENAME_DRIFT` | decision | `syntax` | artefact | warning |
| `CAIRN_ARTEFACT_MISSING_FIELD` | review | `syntax` | artefact | error |
| `CAIRN_ARTEFACT_MISSING_FIELD` | todo | `missing_repair_affordance` | artefact | error |

Every hotspot is artefact frontmatter. **Zero are blueprint text**, and zero are
graph.

## The second run, and what it costs the headline

An earlier run used the same corpus with one difference: the
`corpus.blueprint-delta-rename` instruction and its reference proposal were
reworded, for a terminology reason unrelated to measurement. The other five
prompts were byte-identical. Those five therefore have two repeated samples
each, which is the run-to-run comparison `res.authoreval-corpus-first-run` said
the programme did not have.

| Prompt | Earlier run | Published run |
|---|---|---|
| `corpus.contract-container` | `clean_after_repair`, 2, 17 520 | `clean_after_repair`, 2, 16 471 |
| `corpus.decision-multi-node` | `clean_after_repair`, 3, 20 085 | `clean_after_repair`, 2, 13 562 |
| `corpus.module-claiming-files` | `repair_bound_exhausted`, 4, 37 642 | `backend_failure`, 4, 37 569 |
| `corpus.review-attestation` | `clean_after_repair`, 2, 13 992 | `clean_after_repair`, 2, 13 399 |
| `corpus.todo-open-work` | `clean_after_repair`, 2, 13 094 | `repair_bound_exhausted`, 4, 26 675 |

Three of the five repeat their outcome exactly. Two do not, and they move in
opposite directions: the todo prompt converged in one run and exhausted the
bound in the other, and the module prompt failed both times but under two
different failure kinds. Iterations did not simply track the outcome either:
contract and review took 2 in both runs, while the decision prompt took 3 and
then 2 with its outcome unchanged.

What survives both runs, and is therefore the durable finding:

- **First-shot validity is 0 of 5 in both runs.** Ten measured first attempts,
  no clean first shot. This is the number to quote.
- **No record in either run carries a blueprint-subclass hotspot.** Under the
  record scope above, that means every retained failed scan retained a
  parseable blueprint and no blueprint-text finding, and the published run's
  module record retained no scan at all.
- **Every hotspot in either run is artefact frontmatter**, apart from one
  `CAIRN_PROVENANCE_NO_DECISION` in the earlier run that stood downstream of an
  artefact failure rather than independently of it.

What does not survive is the aggregate: one run reports 4 of 5 measured prompts
converging and the other 3 of 5, from the same prompts and the same model, and
two individual prompts changed outcome between them. Do not quote a convergence
rate from a single run of this size, this artefact included.

## Attribution

The parent asks whether failures belong to syntax, generated guidance, or
missing repair affordances. Across both runs:

- **Syntax, in artefact frontmatter, is the dominant retained class and it is
  often repairable.** No measured prompt reconciled on its first attempt in
  either run, and every retained failed scan that carried a hotspot carried an
  artefact one. Six records across the two runs took exactly two iterations
  (contract, decision and review in the published run; contract, review and todo
  in the earlier one). For those the retained hotspot is by construction the
  first attempt's failure, and the next attempt scanned clean. This is the
  failure the instrument was built to find.
- **Generated guidance: zero retained hotspots in either run.** No retained
  failed scan shows a well-formed answer producing an inconsistent graph.
  Unretained attempts could have; neither run says anything about that class in
  either direction.
- **Missing repair affordance appears in both runs, on a different prompt each
  time.** `CAIRN_ARTEFACT_MISSING_FIELD` earned it on the todo prompt in the
  published run and on the module prompt's decision artefact in the earlier one,
  and `res.authoreval-corpus-first-run` section 3 recorded the same code earning
  it on its todo prompt. Three runs, three prompts, one code persisting from one
  failed scan into the next: that much is corroborated.
  The *mechanism* is not. The predecessor published an answer sequence showing
  two field names alternating under one-field-at-a-time reporting, and neither
  run here retained answers, so for these two the oscillation stays a hypothesis
  imported from the predecessor rather than an observation.
- **One failure is not an authoring result at all.** The module prompt's
  published record is a `protocol` backend failure: its final response left
  `meta/decisions/watch-module.md`, which the prompt requires, unauthored. The
  instrument is right to refuse to score it. Across the two runs the
  hardest prompt therefore produced one scored dirty outcome
  (`repair_bound_exhausted`, earlier run) and one unscored protocol failure
  (published run). Neither reached a clean scan.

The pairing worth carrying forward is 0 of 5 first-shot validity against three
measured prompts that did reach a clean scan, each on a later invocation that
carried cairn's findings. The chronology is all the records establish.
Causation is not measured here: there is no no-feedback control, the same
prompt and model changed outcome between the two runs, and a later invocation
resamples the model as well as adding feedback. What can be said is that no
measured prompt got there unaided on its first attempt, twice over.

## Three prior observations, re-tested

**Section 1: the abort did not recur, and no retained scan shows the grammar
failure.** The scorer fix from `todo.authoreval-lint-error-envelope` never
showed up in a record: no record in either run carries the synthesised envelope
finding, and no run aborted. The predecessor measured this same model misplacing
`path` into the module header five times out of five, under an instruction that
named the path and never showed the body form. This corpus's module prompt also
names the path and never shows the body form, and no retained failed scan in
either run carries a blueprint-text finding.

That is narrower than "did not recur". Under the record scope, an intermediate
attempt could have misplaced `path` and been repaired without leaving a trace.
In the earlier run the module record's retained scan reports artefact and graph
findings, which means that attempt parsed; in the published run the module
record is a protocol failure retaining no scan, so it evidences nothing about
the blueprint at all. Whether an unretained attempt misplaced `path` is unknown
and will stay unknown for as long as a record keeps one scan.

The instructions are also not identical: this one names the path and the
contract pointer in a single clause ("claiming the path `./src/watch` and a
contract at `./meta/contracts/kernel/watch.md`"), and every fixture module
declares a `contract` in its body, so the pairing may supply by analogy the
example the predecessor found missing. That is a hypothesis neither run settles.
What the retained evidence settles is only that 5 of 5 was not reproduced under
a different instruction against the same model and fixture. Any claim about
blueprint-grammar authorability must quote the instruction it came from.

**Section 2, the ungraded delta, stands.** The scorer runs `scan --strict` and
`lint --json`, neither of which validates a staged `blueprint.delta`, so a delta
naming a node that does not exist scores exactly as a correct one does.
`corpus.blueprint-delta-rename` therefore scored `clean_first_shot` in both runs
and its number means nothing. It is excluded from first-shot validity and from
every hotspot count above, per the corpus todo's amended acceptance. The prompt
stays in the corpus because the parent's authoring family names it. Grading it
would mean the scorer invoking change validation, which is a change to shipped
scoring behaviour and outside this unit's non-goals.

**Section 3, one missing field at a time: the persistence is corroborated, the
mechanism is not.** See the attribution above.

## What this does not show

- Nothing here measures navigation or comprehension. That family belongs to
  `todo.agent-guidance-baseline`.
- Two runs, one model, one fixture, one prompt each per run. That is enough to
  show the per-prompt outcomes are unstable and not enough to quantify how
  unstable. A programme that wants a convergence rate needs repetitions per
  prompt, which this corpus supports and this unit did not commission.
- No result here licenses a change to shipped guidance, the blueprint grammar,
  or any repair affordance. Naming the failure classes was the commissioned
  output. Acting on the `missing_repair_affordance` class, in particular the
  one-field-at-a-time hypothesis that now has three observations behind it,
  needs its own unit and its own evidence.

## The published records, verbatim

```json
{"schema_version":1,"prompt_id":"corpus.blueprint-delta-rename","outcome":"clean_first_shot","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":1,"tokens":{"prompt":6086,"completion":4053,"total":10139},"first_shot_valid":true,"hotspots":[],"error":null}
{"schema_version":1,"prompt_id":"corpus.contract-container","outcome":"clean_after_repair","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":2,"tokens":{"prompt":12188,"completion":4283,"total":16471},"first_shot_valid":false,"hotspots":[{"class":"syntax","subclass":"artefact","code":"CAIRN_CONTRACT_MISSING_NODE","severity":"error","count":1,"node":"cairn.kernel","path":"./meta/contracts/kernel.md"}],"error":null}
{"schema_version":1,"prompt_id":"corpus.decision-multi-node","outcome":"clean_after_repair","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":2,"tokens":{"prompt":12410,"completion":1152,"total":13562},"first_shot_valid":false,"hotspots":[{"class":"syntax","subclass":"artefact","code":"CAIRN_ARTEFACT_FILENAME_DRIFT","severity":"warning","count":1,"node":null,"path":"meta/decisions/reconciler-registration.md"},{"class":"syntax","subclass":"artefact","code":"CAIRN_DECISION_MISSING_NODES","severity":"error","count":1,"node":null,"path":"meta/decisions/reconciler-registration.md"}],"error":null}
{"schema_version":1,"prompt_id":"corpus.module-claiming-files","outcome":"backend_failure","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":4,"tokens":{"prompt":25837,"completion":11732,"total":37569},"first_shot_valid":false,"hotspots":[],"error":{"class":"protocol","detail":"response left the prompt's expected paths unauthored: meta/decisions/watch-module.md"}}
{"schema_version":1,"prompt_id":"corpus.review-attestation","outcome":"clean_after_repair","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":2,"tokens":{"prompt":12314,"completion":1085,"total":13399},"first_shot_valid":false,"hotspots":[{"class":"syntax","subclass":"artefact","code":"CAIRN_ARTEFACT_MISSING_FIELD","severity":"error","count":1,"node":null,"path":"meta/reviews/rev.stable-ids.md"}],"error":null}
{"schema_version":1,"prompt_id":"corpus.todo-open-work","outcome":"repair_bound_exhausted","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":4,"tokens":{"prompt":24717,"completion":1958,"total":26675},"first_shot_valid":false,"hotspots":[{"class":"missing_repair_affordance","subclass":"artefact","code":"CAIRN_ARTEFACT_MISSING_FIELD","severity":"error","count":1,"node":null,"path":"meta/todos/todo.scanner-cache-invalidation.md"}],"error":null}
```

## The earlier run's records, verbatim

Kept because the five unchanged prompts are the second sample above. Its
`corpus.blueprint-delta-rename` record measures a prompt that is not the shipped
one and is reported here for completeness only.

```json
{"schema_version":1,"prompt_id":"corpus.blueprint-delta-rename","outcome":"clean_first_shot","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":1,"tokens":{"prompt":6078,"completion":948,"total":7026},"first_shot_valid":true,"hotspots":[],"error":null}
{"schema_version":1,"prompt_id":"corpus.contract-container","outcome":"clean_after_repair","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":2,"tokens":{"prompt":12188,"completion":5332,"total":17520},"first_shot_valid":false,"hotspots":[{"class":"syntax","subclass":"artefact","code":"CAIRN_CONTRACT_MISSING_NODE","severity":"error","count":1,"node":"cairn.kernel","path":"./meta/contracts/kernel.md"}],"error":null}
{"schema_version":1,"prompt_id":"corpus.decision-multi-node","outcome":"clean_after_repair","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":3,"tokens":{"prompt":18525,"completion":1560,"total":20085},"first_shot_valid":false,"hotspots":[{"class":"syntax","subclass":"artefact","code":"CAIRN_DECISION_MISSING_NODES","severity":"error","count":1,"node":null,"path":"meta/decisions/reconciler-registration.md"}],"error":null}
{"schema_version":1,"prompt_id":"corpus.module-claiming-files","outcome":"repair_bound_exhausted","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":4,"tokens":{"prompt":25627,"completion":12015,"total":37642},"first_shot_valid":false,"hotspots":[{"class":"missing_repair_affordance","subclass":"artefact","code":"CAIRN_ARTEFACT_MISSING_FIELD","severity":"error","count":1,"node":null,"path":"meta/decisions/watch-module.md"},{"class":"missing_repair_affordance","subclass":"graph","code":"CAIRN_PROVENANCE_NO_DECISION","severity":"warning","count":1,"node":"cairn.kernel.watch","path":null}],"error":null}
{"schema_version":1,"prompt_id":"corpus.review-attestation","outcome":"clean_after_repair","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":2,"tokens":{"prompt":12315,"completion":1677,"total":13992},"first_shot_valid":false,"hotspots":[{"class":"syntax","subclass":"artefact","code":"CAIRN_ARTEFACT_MISSING_FIELD","severity":"error","count":1,"node":null,"path":"meta/reviews/rev.stable-ids.md"}],"error":null}
{"schema_version":1,"prompt_id":"corpus.todo-open-work","outcome":"clean_after_repair","backend":{"kind":"command","model":"anthropic/claude-sonnet-4-5"},"iterations":2,"tokens":{"prompt":12277,"completion":817,"total":13094},"first_shot_valid":false,"hotspots":[{"class":"syntax","subclass":"artefact","code":"CAIRN_ARTEFACT_MISSING_FIELD","severity":"error","count":1,"node":null,"path":"meta/todos/todo.scanner-cache-invalidation.md"}],"error":null}
```
