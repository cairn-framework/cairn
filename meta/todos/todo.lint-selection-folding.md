---
node: cairn.kernel.scanner
status: open
created: 2026-07-28
---

# Fold Parked Findings Out Of The Selecting Set

## Problem, as measured

Loop selection is lint-first: while any finding exists, no todo may be selected. An
audit of all 14 loop iterations run on 2026-07-28 (PRs #501, #503, #506, #508, #509,
#511, #513, #515 to #521) measured what that costs, and it corrects two guesses.

**What is real.** Every one of the 14 iterations re-decided whether the standing Info
findings block selection. Fourteen out of fourteen, in fresh sessions with no memory
of the last one. The standing set is two `CAIRN_SOURCE_UNVERIFIED`, whose bodies state
that hash-pinning a live source file would turn ordinary edits into structural errors
and whose fix is parked in `todo.source-tracked-verification-mode`, plus one
`CAIRN_SPEC_RULE_UNIMPLEMENTED` deferred by `dec.revisit-trigger-correlator-deferred`
(spec:634). Scan already prints "deferred by dec.X" for the third, so the annotation
exists and selection ignores it.

**What is smaller than assumed.** Direct time spent deciding selectability was about
56 of 1,042 minutes of loop elapsed time, roughly 5%. Review, rework, and CI
dominated. The case for folding is repeatability, not wall clock.

**What is actually expensive.** Which units got selected. The four iterations that
selected a finding rather than a todo (#513, #518, #519, #520) consumed about 228
minutes, 22% of the day, and all four landed provenance or bookkeeping diffs. Across
the day the merged split was 8 bookkeeping to 7 load-bearing, so "the day was
overwhelmingly bookkeeping" is not true either; the consecutive #518 to #520 stretch
was.

**What is false.** `CAIRN_DECISION_ACCUMULATION` did not regenerate node to node. PR
#517 added the check and its first post-merge lint reported `cairn.kernel.cli` 20,
`cairn.root` 23, and `cairn.ui` 14 simultaneously. #518 to #520 serialised an
already-existing three-node set, one per iteration, because one loop selects one unit.
There is no treadmill in the evidence.

A different fragility is real: after those consolidations `cairn.kernel.cli`,
`cairn.root`, and `cairn.ui` all sit at exactly 10 against a flat threshold of 10, and
the check fires above it. The next accepted decision touching any of the three
re-fires the finding. That is what a flat threshold buys.

**Evidence against weakening the rule.** Lint-first earned its keep once today. In
#513, selecting `CAIRN_SOURCE_UNVERIFIED` first led to running its own remediation:
`cairn remediate CAIRN_SOURCE_UNVERIFIED` prints `run: cairn sources`, and that
command exits with `CAIRN_CLI_MISSING_NODE`. The same bare-command defect exists for
the decisions and research actions in `src/query_api/handlers/remediate.rs`. So
folding must keep reporting, and must never fold a finding nobody has explicitly
parked.

## Scope

Three independent changes. Land them as separate units if either of the first two
grows.

Prior art this todo composes with rather than duplicates:
`dec.loop-selection-deferred-findings` (proposed 2026-07-27, accepted
2026-07-29) and its implementation todo carry the narrowest rule. The three
selection rules, once all land, are:

- a finding whose `deferred_by` names an accepted decision is not selectable
  (that decision, not this todo);
- an Info finding a `blocked` todo explicitly parks via `defers:` is not
  selectable (item 1 here);
- any Info finding is non-selecting while `scan --strict` is green (item 2
  here).

Signing that decision first neither conflicts with nor substitutes for items 1
and 2. Each unit lands its own selector-asset edit atomically with its scanner
half, per the two-halves rule below; combine the asset edits only when units
deliberately co-land.

1. **Fold parked findings, todo-side first.** A fold must rest on a typed link,
   never on prose or a keyword match: an unrelated `blocked` todo that merely
   mentions a code would otherwise remove a real finding from selection. So the
   parking is declared. Cut 1a: todos gain a `defers:` list, where a reference is a
   finding code plus the path or node it was raised against, and an Info finding is
   classified parked only when a `blocked` todo declares a reference matching both.
   Parking applies to Info alone: an Error or Warning stays selecting whatever any
   artefact declares about it, and a `defers:` reference aimed at one is itself a
   finding, so nothing can park a blocking finding by accident or on purpose.
   A reference matching no emitted finding is likewise a finding, so the links
   cannot rot silently. Cut 1b extends the same field to decisions, which is where
   a decision-deferred finding with no registry row would belong; the live
   registry-backed case (`spec:634`, deferred by
   `dec.revisit-trigger-correlator-deferred`) is already covered by
   `dec.loop-selection-deferred-findings` and keeps its inline annotation,
   untouched by this todo.
   Todo is a typed artefact in the same shared schema, so 1a is binding too, not a
   schema-free shortcut. It is smaller in surface and it clears the case that
   actually costs iterations today, which is the only sense in which it comes
   first. Either cut leaves reporting alone: `cairn lint` and `cairn scan` still
   print the parked findings, naming the parking artefact, and the count a human
   sees does not change. Parked is a report-level classification, never a silent
   suppression.
   Cut 1a landed 2026-07-29 (`res.lint-selection-folding.parked-classification`):
   todos parse `defers:` (code plus path-or-node; a malformed entry raises
   CA043 `CAIRN_TODO_DEFERS_INVALID` at Error), `check_todo_defers`
   (src/scanner/todo_defers.rs) sets `parked_by` on matching live Info findings
   while the todo is `blocked`, a stale reference raises CA041
   `CAIRN_TODO_DEFERS_UNMATCHED` and a blocking-aimed reference raises CA042
   `CAIRN_TODO_DEFERS_BLOCKING` (both Warning), the lint wire publishes
   per-finding `parked_by` (query and webui `schema_version` 7), renderers
   print parked findings in full naming the todo, and both loop-mode.md copies
   teach the composed skip in selection, MISSION, and stop evidence. A
   decision-deferred finding is never re-classified, composing with
   `dec.loop-selection-deferred-findings`; that intersection rule is carried by
   `dec.parked-deferral-composition` (proposed, maintainer ratification
   pending, tracked by `todo.parked-deferral-composition`).
   The live pair stays covered by the strict-green fold because
   `todo.source-tracked-verification-mode` is open since sheet W3, so no live
   `defers:` was added. Cut 1b and item 3 remain.
2. **Info is non-selecting while strict is green.** `cairn scan --strict` is the
   CI gate and it tolerates Info, so anything it tolerates is by definition not
   iteration-blocking. Make Info non-selecting whenever `--strict` would exit
   zero. Errors and Warnings are unaffected.
   Landed 2026-07-29 (`dec.loop-selection-strict-green-fold`): the lint/scan
   wire publishes `strict_green` from the shared strict predicate
   (`schema_version` 6), both loop-mode.md copies teach the fold in selection,
   MISSION, stop evidence, Verify, and guardrail, and under `--strict` the
   shared-JSON exit code reads the published field. Items 1b and 3 remain.
3. **Accumulation threshold.** `DEFAULT_DECISION_ACCUMULATION_THRESHOLD` in
   `src/scanner/config/mod.rs:13` is a flat 10 for every node, and the check counts
   only accepted decisions whose `nodes:` name the node directly
   (`src/scanner/checks.rs:120-136`), which is not what `cairn rationale <node>`
   prints. Measured on the merged tree after #520: `cairn.kernel.cli` 10,
   `cairn.root` 10, `cairn.ui` 10, `cairn.kernel.query` 6, `cairn.kernel.artefacts`
   6, `cairn.reconcile` 4, `cairn.brownfield` 3, `cairn.mcp` 2, `cairn.sse` 0. Three
   nodes sit exactly at the threshold, so the next accepted decision on any of them
   re-fires the finding, and a hub node legitimately carries more rulings than a
   leaf. What this todo asks for is the question answered, not a remedy chosen in
   advance. A per-node configurable threshold is the tempting answer and the weakest
   one: it turns the finding into a count each node can raise until it stops
   complaining. Prefer a rule with a reason behind it, for example counting only
   decisions that still bind (excluding those wholly superseded in substance) or
   distinguishing rulings by role, and adopt a threshold change only against a stated
   reader-cost signal such as how much a reader must load to learn a node's live
   rules. Withdrawing the finding is admissible if consolidation provably does not
   reduce that cost; that outcome needs a decision, not a deletion. Note what the
   evidence does not say: the three nodes did not accumulate because of each other,
   so do not design for a regeneration loop that was not observed.

Selection semantics live in the loop assets, not in the binary, so items 1 and 2
each have two halves: the classification the scanner emits, and the rule in
`.claude/skills/cairn-dev/references/loop-mode.md` plus its canonical copy under
`tools/agent-pack/content/` that consumes it. Both halves land together or the
assets keep teaching the old rule to adopting repositories.

## Depends on

Maintainer ratification for items 1 and 2, on different grounds. Item 1 adds a
field to typed artefacts every adopting repository inherits, for both cuts. Item 2
states what blocks an iteration, which lives in shipped pack content. Neither
depends on `todo.decision-ratification-tiers` landing first, and neither becomes
self-ratifiable after it does. Item 3 needs no ratification unless it withdraws the
finding, which does.

Ratify cut 1a alone if the decision-side field needs more thought: it is the half
that clears the two `CAIRN_SOURCE_UNVERIFIED` findings costing iterations today.

Cut 1a and item 2 were ratified 2026-07-29 (PR #528 sheet W2, which reads
"folding items 1a+2"), so cut 1a's implementation may proceed; item 2 landed
the same day (see its note above). Cut 1b, the decision-side `defers:` field,
is NOT ratified and still needs its own maintainer ruling.

## Acceptance

- Cut 1a: the two `CAIRN_SOURCE_UNVERIFIED` findings, declared in
  `todo.source-tracked-verification-mode`'s `defers:` list while that todo is
  `blocked`, are reported as parked and are not selectable. Covered by a scanner
  test over a fixture holding exactly that pair.
- A `defers:` reference whose code and location match no emitted finding raises a
  finding of its own, so a stale park cannot hide a real one.
- A `blocked` todo that mentions a finding code only in prose parks nothing.
- An Error or Warning finding still blocks selection, with or without a parking
  artefact. This is the regression that would quietly disable the gate, so it gets
  its own test.
- With the standing Info set parked or non-selecting and `scan --strict` green, a
  loop session's selection reaches the todo backlog, and every parked finding still
  appears in `cairn lint` output naming its parking artefact.
- The accumulation item lands either a threshold change with the measured per-node
  distribution recorded, or a decision withdrawing the finding. Not silence.
  `check_decision_accumulation` (`src/scanner/checks.rs:120-136`) counts only
  accepted decisions whose `nodes:` name the node directly, which is not what
  `cairn rationale <node>` prints, so measure with the check's own basis.

The audit trail behind the numbers above, including its window, sources, and limits,
is recorded at https://github.com/cairn-framework/cairn/pull/523#issuecomment-5105144953, with the per-iteration session ids and pinned counts at https://github.com/cairn-framework/cairn/pull/523#issuecomment-5105158502. The transcripts themselves are machine-local
and were not committable, so each claim is restated here as a PR number, a
file:line, or a measured count.

## Origin

Maintainer conversation, 2026-07-28. Both selection changes were called cheap and
independent, and the same exchange raised a treadmill hypothesis for
`CAIRN_DECISION_ACCUMULATION`. A transcript audit of the day's 14 loop iterations
then disproved the treadmill and shrank the direct cost of the selection argument to
about 5% of elapsed time, while confirming that all 14 iterations re-litigated it and
that finding-selected iterations spent 22% of the day on bookkeeping. The measured
numbers above supersede the original framing, and the accumulation item survives on
the at-threshold fragility rather than on regeneration.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It keeps backlog selection aligned with declared findings and mission work.

2026-08-07 audit (todo.roadmap-assumption-audit): landed except the `defers:` field ratification; narrow scope to that remainder.
