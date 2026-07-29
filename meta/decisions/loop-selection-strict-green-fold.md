---
id: dec.loop-selection-strict-green-fold
nodes:
  - cairn.kernel.scanner
  - cairn.kernel.map
  - cairn.kernel.query
  - cairn.kernel.cli
status: accepted
date: 2026-07-29
revisited: 2026-07-29
informed_by: [res.shared-json-strict-flag-gap]
related: [dec.loop-selection-deferred-findings]
revisit_triggers:
  - "the strict gate's severity boundary changes (e.g. a severity between Warning and Info appears)"
  - "item 1a's parked classification lands and the three composed selection rules need a single home"
---
# An Info finding is non-selecting while the lint wire is strict-green

**Accepted 2026-07-29 by maintainer ratification** (sheet of record: PR #528,
row W2, which reads "folding items 1a+2"). The ratified text is
`todo.lint-selection-folding` item 2; this artefact records that ruling in the
graph so `cairn rationale` can answer for it. It adds no rule beyond the
ratified item.

## Context

`cairn scan --strict` is the CI gate and it tolerates Info findings, so
anything it tolerates is by definition not iteration-blocking. Loop selection
nevertheless read "the first lint finding" without exception, and the audit in
`todo.lint-selection-folding` measured the cost: all 14 loop iterations run on
2026-07-28 re-decided, in fresh sessions, whether the standing Info set blocks
selection. The case for folding is repeatability, not wall clock.

## Decision

Every Info finding is non-selecting while the lint/scan JSON wire publishes
`"strict_green": true`. Error and Warning findings are unaffected.

Three obligations make the rule mechanical rather than judged:

1. The verdict is published, never inferred. The lint/scan `data` payload
   carries a top-level `strict_green` boolean, true exactly when
   `scan --strict` would exit zero over the emitted finding set (no Error and
   no Warning, whatever their deferral state). One shared predicate
   (`map::graph::strict_green`) feeds the published field and every strict
   exit path, so the classification cannot drift from the gate it represents.
2. Selection trusts only the published field, never a verdict a session
   recomputes. A wire that does not publish the field folds nothing, so an
   older binary fails closed to the previous rule.
3. The iteration gate agrees with the selector. Verify's blocking bar is
   `scan --strict` exit 0; zero findings remains the target, and a standing
   Info the strict gate tolerates does not block landing. Without this clause
   the fold would let selection reach the todo backlog and then strand the
   iteration at Verify against the same standing set.

## Rationale

Publishing the verdict on the envelope was chosen over stamping each Info
finding because strict-greenness is a property of the whole finding set: a
per-finding copy would be all-or-none by construction and could only invite
inconsistency. Recomputing the verdict in the selector was rejected because it
reintroduces the per-session judgement the fold exists to remove, and because
the deferral precedent (`dec.loop-selection-deferred-findings`) already
establishes that selection trusts published wire fields only. Suppressing the
Info findings instead was rejected for the same reason it was rejected there:
reporting is untouched, and `cairn lint`/`cairn scan` still print the full
standing set.

## Relation to dec.loop-selection-deferred-findings

Composition, not supersession. That decision's sentence "a finding with no
such deferral remains selectable regardless of severity" bounds the deferral
rule's own scope: deferral is never inferred from severity. This rule folds
Info findings on a different machine-visible fact, the published strict
verdict, and leaves every deferral obligation standing (accepted-only
publication, per-instance resolution, the deferred work represented by a
todo). Both texts were ratified on the same sheet (PR #528, rows W1 and W2),
so they were accepted as composing. The loop asset now states the composed
rule: an Error or Warning finding with no published `deferred_by` is always
selectable.

## Consequences

- Query JSON payloads and the webui `/api/*` envelope bump to
  `schema_version` 6 for the added field.
- Fixing obligation 1 surfaced and closed a shared-JSON gap: `lint --json` /
  `scan --json` previously ignored `--strict` and exited 0 on warnings. Under
  `--strict` the CLI exit code now reads the published field itself.
- With the standing set all Info and strict green, a loop session's default
  selection reaches the todo backlog. The two `CAIRN_SOURCE_UNVERIFIED`
  findings keep standing and keep printing; their artefact-declared parking is
  item 1a, a separate ratified unit that composes with this rule.
- The loop-mode selection, MISSION, stop-evidence, Verify, and guardrail
  sentences are pinned by pack phrase assertions
  (`tools/agent-pack/tests/selection_strict_green_tests.rs`).
