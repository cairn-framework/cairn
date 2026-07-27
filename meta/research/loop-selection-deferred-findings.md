---
id: res.loop-selection-deferred-findings
nodes:
  - cairn.kernel.cli
  - cairn.kernel.map
  - cairn.kernel.query
  - cairn.kernel.scanner
  - cairn.root
date: 2026-07-27
method: primary
tags: [loop-mode, selection, findings, spec-634]
---
# Can loop selection tell a fixable finding from a deliberately deferred one?

## Why this was probed

A dev-loop iteration reached `## Select ONE unit` with a clean tree and exactly
one lint finding outstanding:

```
CAIRN_SPEC_RULE_UNIMPLEMENTED [info] docs/registries/spec-rules.md
spec rule `ADR revisit_triggers appear relevant to recent changes` (spec:634)
is pending but names no enforcer
(deferred by dec.revisit-trigger-correlator-deferred)
```

Selection says "the first `$CAIRN lint --json` finding; else the top open todo",
with no exception. `dec.revisit-trigger-correlator-deferred` (accepted) rules
that this finding must keep standing as "the honest living tracker of a
Designed-but-unbuilt rule". The procedure therefore points every iteration at a
unit the graph's own authority forbids completing.

## Method

Original observation against this repository at `692e957`. No external sources.

1. Ran `cairn lint --json` and `cairn remediate CAIRN_SPEC_RULE_UNIMPLEMENTED`.
2. Read the accepted decision and its probe
   (`res.revisit-trigger-correlator-probe`) to establish whether the finding is
   fixable at all.
3. Read the emission site and the finding wire shape to establish what a
   selector could key on deterministically.
4. Read the loop-mode selection paragraph and the pack tests that pin loop-mode
   prose, to establish the cost of each candidate rule.

## Findings

- **The finding is unfixable by design, and the tooling already says so.**
  `cairn remediate CAIRN_SPEC_RULE_UNIMPLEMENTED` returns "No remediation
  actions are required. The project is in good shape." The accepted decision
  forecloses every deterministic matcher (verbatim: 0 recall, node-id: 0 recall,
  term coverage: a structural tautology, git-log: foreclosed by that decision as
  breaking scan purity).
- **The deferral is structured, but neither published nor strong enough yet.**
  `src/map/graph.rs` carries `Finding::deferred_by`, set at the emission site in
  `src/map/spec_rule_coverage.rs`, but it is `#[serde(skip)]` with the comment
  "Skipped in `--json` so the wire format is unchanged". The only trace a JSON
  consumer sees is the message suffix `(deferred by dec.<slug>)`. The companion
  check `validate_deferred_decision_targets` only rejects a target that names no
  decision or a `Superseded` one, so a `Proposed` or `Deprecated` decision
  currently passes as a live deferral.
- **The human surface already demotes deferrals; only the machine surface does
  not.** `cairn scan` collapses them (`src/cli/format/render.rs`), printing
  "1 deferred finding deferred by dec.revisit-trigger-correlator-deferred"
  instead of a finding row. A selector reading `lint --json` sees a full-weight
  finding where a human reader sees a footnote.
- **The projections are not one surface.** `lint --json` is serde-derived from
  `map::query::lint`, under the `query_api::SCHEMA_VERSION` envelope;
  `scan --json` is hand-written in `render_findings`
  (`src/cli/format/render.rs`) and emits only `code`, `severity`, and `message`;
  `map.json` embeds findings through `src/scanner/snapshot.rs`, which carries
  its own schema version. Dropping `serde(skip)` alone would reach the first and
  the third, not the second.
- **No open tracker held the deferred work.** Completed todos reference
  `spec:634` (`todo.deferred-finding-cites-decision`, status `done`, made the
  deferral visible in output), but no open or blocked todo tracked building the
  enforcer, although the decision closes with "re-file an implementation bead
  when such a capability lands".
- **Loop-mode prose is testable.** `tools/agent-pack/tests/reconcile_step_tests.rs`
  already pins loop-mode obligations: it asserts on the raw loop-mode body with
  heading offsets and `contains` checks, and applies whitespace flattening to
  the reconcile recipe it also reads. `determinism_drift_tests.rs` pins the
  `.claude/` and pack copies as a byte-identical pair. A new selection
  obligation can be pinned in the same file, flattening the loop-mode body if a
  phrase spans the hard wrap.

## Options

1. **Status quo.** Each iteration re-derives the exception by hand, or follows
   the letter and stalls. Silent, recurring, and not deterministic across
   sessions.
2. **Stop emitting the finding when a decision defers it.** Contradicts the
   accepted decision, which wants the Info standing. Deleting a signal to
   satisfy a selector is the wrong direction.
3. **Publish the deferral on the wire, then key selection on it.** Drop the
   `serde(skip)` on `deferred_by`, bump the finding wire `schema_version`,
   record the wire delta, rebase snapshots, and amend selection to skip findings
   that carry it.
4. **Prose-only rule keyed on the message substring `(deferred by dec.`.** No
   wire change, but it makes a selector depend on message prose that a copy edit
   may reword, and it inherits the same missing accepted-status check.

## Recommendation

Option 3, extended to enforce accepted status. Options 1, 2 and 4 are rejected:
1 leaves the defect recurring, 2 deletes a signal an accepted decision wants
standing, and 4 makes a selector depend on message prose.

Selection should skip a finding that an **accepted** decision defers, because
such a finding is standing evidence, not a task, and the deferred work belongs
to a todo instead. Publishing today's `deferred_by` alone would be too weak: it
would let a merely proposed decision remove a finding from selection. Tightening
that check is part of the implementation unit, not of this recommendation.
`todo.loop-selection-deferred-findings` carries the concrete edit list;
`todo.revisit-trigger-correlator` is the tracker item the rule assumes exists.

This recommendation is not self-ratified. It changes the sole normative loop
procedure and its tracker obligation sits against a line in an accepted
decision, so `dec.loop-selection-deferred-findings` is filed `proposed` and the
implementation todo stays `blocked` until a maintainer rules.
