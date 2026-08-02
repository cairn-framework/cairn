---
node: cairn.root
status: open
created: 2026-07-28
---

# Greenfield Completion Eval

## Problem

Cairn's value claim is that an agent building a project with cairn ends up with a
better result than one building it without. Nothing in the repository measures
that. The archived navigation baseline measured something narrower and, on its own
terms, did not clear its threshold: quality means were 4.94 target-only, 5.13 bare
cairn, 5.19 cairn plus pack, against a preregistered 1-point minimum effect, and
bare cairn was never invoked in 16 of 16 runs
(`meta/research/agent-experiment-linklint.md:106-127`). Its primary cohort, all 36
intention-to-treat assignments, produced no outcome at all because the runner
omitted authentication state (`meta/research/agent-experiment-linklint.md:68-76`).
The earlier greenfield Linklint experiment is the right shape but ran one trial per
arm, with ambient parent guidance and a hand-authored matching blueprint, and its
own caveats say so (`meta/research/agent-experiment-linklint.md:40-48`).

So the claim is unproven, not disproven, and the instrument that could prove it does
not exist yet.

## Scope

Reuse the apparatus, add only the missing lifecycle. What already exists and must
not be rebuilt:

- The three-arm runner with isolation, pinned repo SHAs, fixed model, tools, time,
  and seed, randomised paired order, intention-to-treat preservation, raw
  stdout/stderr bytes with SHA-256, and monotonic timings
  (`archive/strongholds/agent-guidance-baseline/evidence.tar.gz:run_baseline.py:20-69,380-478`).
- The stronghold receipt pattern: a small checked-in manifest pinning versions and
  hashes, pointing at a hashed evidence archive
  (`archive/strongholds/agent-guidance-baseline/manifest.json:1-31`).
- Pinned tokenisation and accounting, including the exact `o200k_base` version and
  vocabulary hash, from the context-bundle evaluation
  (`archive/strongholds/agent-context-bundle-evaluation/manifest.json:26-33`).
- Deterministic outcome sensors already shipped: `cairn scan --json` and
  `cairn lint --json` findings by severity, `cairn health` (clean flag plus
  synced/ghost/orphaned counts, `docs/commands.md:193-195`), `cairn frontier --json`
  (ready and blocked work, empty on a complete graph), `cargo test --locked
  --workspace`, and `scripts/check-file-sizes.sh`.

What to add:

1. Two or three frozen product briefs, committed as artefacts so runs are
   comparable across months, each with task-specific behavioural acceptance tests
   that are ground truth independent of either arm's code.
2. A build lifecycle. The archived runner is read-only navigation; this needs
   agents that modify a tree until a completion condition or a timeout.
3. Arm definitions at identical model, tool authority, and prompt bytes, differing
   only in whether `cairn init` output and an initial ghost blueprint are present,
   which is the Linklint shape (`meta/research/agent-experiment-linklint.md:10-19`).
4. A completion rubric aggregating the sensors above plus tokens, tool calls, and
   elapsed time, scored as a **delta against the substrate's committed baseline**,
   never against the word "clean". Scoring against clean is what blocked
   `todo.blueprint-authorability-eval`: its substrate starts with 22 warnings, so
   iterations-to-clean is unmeasurable there
   (`meta/todos/todo.blueprint-authorability-eval.md:45-62`).
5. A scripted persona with a deterministic answer corpus and a fixed clarification
   budget. There is no precedent for this in the repository, and without a budget
   the human channel silently becomes the variable that decides the outcome.
6. Repeated paired trials sized for whole-project variance, not one run per arm.
7. A release-time frozen result bundle following the stronghold manifest pattern.

## Substrate

Fresh isolated repositories generated per run from a frozen brief. Not
`tests/fixtures/cairn-bootstrap`, which is invalid as a clean start until its
verdict lands (`meta/todos/todo.bootstrap-fixture-repair-or-delete.md:9-25`). Not
`examples/demo` as the task either: it is already complete and pinned empty
(`examples/demo/expected-findings.json:1`), which makes it a scorer-calibration
fixture rather than a greenfield state.

**The brief carries an oracle, or deletion wins.** Every graph sensor improves when
an arm removes declared work: `cairn frontier` empties if the ghost nodes are
deleted, `cairn health` counts improve if nodes are dropped, and `scan` findings fall
if the artefacts that raised them are gone. So each brief freezes a manifest of
required outcomes: the behavioural acceptance tests that must pass, and the node ids,
contracts, and artefacts that must still exist and still be claimed at the end.
Completion is scored only after that manifest verifies. An arm that deletes a
required target scores as incomplete regardless of how clean its scan is.

## Cadence

Release-time, on a tag, gating the release. Not per commit. The per-commit loop
keeps the fast gates.

## Depends on

Nothing blocking. Item 4's baseline-delta rule and the release cadence are rules
about how cairn is evaluated, not schema, so they can be ratified with the unit
that lands them.

## Post-hoc tech debt: how it is measured

"Tech debt" is the metric most likely to become a vibe, so it is split in two.

Primary, mechanical, part of the headline score: `scripts/check-file-sizes.sh`
oversize count and exception count, `cairn scan --json` warning and info counts by
code, `cairn health` synced against ghost and orphaned counts, `cairn frontier
--json` remaining ready and blocked work, test count and pass rate, and clippy
warning count at default levels. Every one of these is a shipped deterministic
sensor, so both arms are scored by the same program.

Secondary, exploratory, never in the headline number: a blinded rubric grade of the
final tree. If it is run at all it follows the archived protocol shape, two graders
who cannot see which arm produced the tree, an adjudicator for disagreements, and a
preregistered minimum effect. Reported separately and labelled exploratory, because
the navigation baseline showed how easily a sub-threshold quality delta gets read as
a result.

## Acceptance

- One brief runs unattended, both arms, repeated trials, from a single command.
- Every score in the headline metric is computed from a shipped deterministic sensor
  or from the run record. Human or model grading appears only in the secondary
  exploratory block, labelled as such.
- Results are reported as a delta against the substrate baseline, and a run on a
  deliberately dirty substrate still produces a meaningful score.
- The clarification budget is enforced mechanically, and exhausting it is recorded
  rather than silently extended.
- The result bundle pins model, harness, cairn commit, brief hash, and tokenizer
  version, so a later run is comparable or provably not.
- No CI scheduling is added before the instrument has produced one usable result.

The apparatus inventory behind this todo was produced by a same-day audit whose
window, sources, and limits are recorded at https://github.com/cairn-framework/cairn/pull/523#issuecomment-5105144953. Every claim it
rests on cites a committed artefact, so this todo stands without the transcripts.

## Origin

Maintainer conversation, 2026-07-28: prove the value claim with a
greenfield-to-completion A/B, agent as the human, scored on end state, tokens,
iterations, and post-hoc tech debt, run before release rather than per commit. The
apparatus inventory above was gathered the same day from the archived strongholds
and research artefacts it cites, which are the durable record.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It demonstrates that a new repository can reach a clean completion state.
