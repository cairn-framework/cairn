---
node: cairn.authoreval
status: done
created: 2026-08-10
---

# Authoreval Lint Error Envelope

## Scope
A model answer that leaves `cairn.blueprint` unparseable currently aborts the
whole authoreval run instead of scoring. Make that answer a scored attempt, so a
corpus containing any blueprint-authoring prompt can run unattended.

## Evidence
Observed on 2026-08-10 while attempting
`todo.authorability-eval-prompt-corpus`: five times out of five, from five
independent invocations of the module prompt against
`--backend command --model anthropic/claude-sonnet-4-5`.

The model placed `path` in the module header rather than the module body:

```
Module Watch "..." id "cairn.kernel.watch" path "./src/watch" {
    contract "./meta/contracts/kernel/watch.md"
}
```

Scoring that workspace gives:

```
$ cairn lint --json
{"error":{"code":"CAIRN_COMMAND_FAILED","message":"cairn.blueprint:57:101: expected `{`, encountered word `path`","remediation":null,"source_span":"cairn.blueprint"}}
```

`src/authoreval/scorer.rs` requires a `findings` key and fails closed when it is
absent, which is correct for a truncated envelope and wrong for this one: the
envelope is well formed, and it is reporting the model's defect. The run exits
with `authorability eval error: ... published no findings key` and emits no
record at all, for any prompt in the invocation.

The interaction is deterministic given an unparseable blueprint, so the failure
class the instrument most wants to measure (blueprint syntax) is exactly the
class it cannot record. Full measurement context, including the five prompts
that scored normally and the two further blind spots the same run exposed, is
in `res.authoreval-corpus-first-run`.

## Options
1. Treat an `error` envelope from `lint --json` as a dirty scan carrying one
   synthesised finding built from the envelope's `code` and `message`, and feed
   that finding back as repair feedback unchanged. Keeps the "no finding logic
   here" invariant: the code and message are cairn's own words. Note the
   classification trap below: the envelope's code is `CAIRN_COMMAND_FAILED`,
   which the taxonomy does not map.
2. Keep the envelope an instrument fault, and have the scorer fall back to the
   `scan --strict` exit status with an empty finding list. Records the outcome
   but feeds the model nothing to repair from, so a parse error can never be
   repaired.
3. Fix nothing here and reword the module instruction so the model stops
   putting `path` in the header. This works: adding one sentence that shows the
   body form put `path` in the body on all four attempts and the run completed
   with a scored record. It is rejected on cost, not on feasibility. Under the
   original instruction the model misplaced `path` five times out of five, and
   that is the measurement the parent commissioned; a prompt that hands over
   the grammar to keep the harness from crashing has already answered its own
   question. The blast radius decides the rest: an unparseable answer raises
   inside the attempt loop in `src/authoreval/runner/mod.rs` before any
   feedback, and `src/bin/cairn-authoreval.rs` buffers records and discards all
   of them on exit, so one syntax slip in one prompt costs the whole corpus's
   output. Prompt wording is the wrong place to absorb that.

Option 1 is the recommendation. It preserves the fail-closed rule for a
genuinely unreadable envelope (no `findings` key and no `error` key), and it
turns the parse error into the repair signal a model can act on.

## Acceptance
- A response that leaves the blueprint unparseable produces a record, not a
  `CairnError`, and the record's outcome distinguishes it from a clean scan.
- The repair feedback for that attempt carries cairn's own parse message.
- The resulting hotspot classifies as `syntax` / `blueprint`, not
  `generated_guidance` / `unknown`. The observed envelope carries
  `CAIRN_COMMAND_FAILED`, which `SUBCLASS_TABLE` in
  `src/authoreval/taxonomy.rs` does not map, so `classify` takes the `Unknown`
  branch and attributes a blueprint syntax failure to generated guidance. The
  parent and the corpus both require those three classes to stay
  distinguishable, so a record that misattributes here defeats the fix.
  Classify from the envelope's `source_span`, or from a parse-specific code, in
  either case keeping cairn's original code and message in the feedback. Pin it
  with a test asserting the hotspot's class and subclass.
- An envelope carrying neither `findings` nor `error` still fails closed.
- The existing offline smoke prompt still scores `clean_first_shot`.

## Sizing
S. One scorer branch, its record path, and the tests that pin both.

## Non-goals
Do not change the blueprint grammar, the parse message, or the corpus. This todo
makes the instrument record what the grammar already rejects.
