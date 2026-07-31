---
id: dec.parked-deferral-composition
nodes:
  - cairn.kernel.scanner
status: accepted
ratification: binding
date: 2026-07-29
informed_by: [res.lint-selection-folding.parked-classification]
related: [dec.loop-selection-deferred-findings, dec.loop-selection-strict-green-fold]
revisit_triggers:
  - "a live artefact configuration produces a finding that is both decision-deferred and referenced by a blocked todo's defers:"
---
# At the deferred and parked intersection, deferral wins

Accepted 2026-07-31 by maintainer ratification, recorded in the pull request
that lands this flip. The implementation already ships the conservative
reading below, so acceptance changes no path and closes
`todo.parked-deferral-composition` with no code change.

## Context

`todo.lint-selection-folding` item 1a (ratified 2026-07-29, PR #528 sheet W2)
does not define the intersection case: an Info finding that both carries a
published `deferred_by` and matches a `blocked` todo's `defers:` reference.
Three ratified constraints meet there and cannot all hold for one finding:

1. Item 1a: an Info finding matching a `blocked` todo's reference on code and
   location is classified parked.
2. Item 1a acceptance: every parked finding still appears in `cairn lint`
   output naming its parking artefact, and the count a human sees does not
   change (no collapse).
3. Item 1b's carve-out: the decision-deferred case "keeps its inline
   annotation, untouched by this todo", and its collapsed rendering is owned by
   `dec.loop-selection-deferred-findings`.

A finding classified parked must render in full (2), and the same finding must
keep its deferred collapse (3). One constraint must yield at the intersection.

## Decision

Parking yields. `check_todo_defers` never sets `parked_by` on a finding whose
`deferred_by` is published: the finding stays wholly under the deferral regime,
its rendering and its wire publication unchanged. The `defers:` reference still
counts as matched, so it is not reported stale by `CAIRN_TODO_DEFERS_UNMATCHED`.
Selection outcome is identical under either reading, because a published
`deferred_by` and a published `parked_by` are independently non-selecting.

## Rationale

This is the only reading that satisfies constraints 2 and 3 simultaneously,
and it is fail-closed: the deferral regime, ratified first, is left exactly as
its decision defines it. The intersection is empty in this repository today
(the sole deferred finding, spec:634, is referenced by no `defers:` list), so
the choice binds behaviour only in a configuration no artefact currently
produces. Dual classification was rejected because it must either break the
acceptance's full-line rule or break the deferred collapse, and because a
per-finding double annotation invites double counting in the collapsed
summary lines.

## The rubric, applied to this decision

- **Tier**: `binding`. The mechanical span reads local: one node
  (`cairn.kernel.scanner`) inside the single container `cairn.kernel`, and it
  supersedes nothing. The ruled surface does not: the rule refines the
  semantics of the adopter-inherited `defers:` schema field, and the shipped
  pack reference states it with this decision as its authority
  (`tools/agent-pack/content/skills/cairn-dev/references/artefact-schemas.md`,
  inside the binding-surface allowlist `todo.decision-ratification-tiers`
  defines). Accepting as written changes no path at all.
- **Unblocks**: `todo.parked-deferral-composition`, blocked on this
  signature: it flips to `done` with no code change if accepted as written,
  or to the coded flip in its Scope if ruled otherwise. No finding moves:
  the intersection is empty today, and the sole deferred finding
  (`CAIRN_SPEC_RULE_UNIMPLEMENTED`, deferred by
  `dec.revisit-trigger-correlator-deferred`) appears in no `defers:` list.
- **Alignment**: against `dec.cairn-mission`, one regime per finding,
  traceable to one decision, keeps selection behaviour investigable and
  maintainable at zero code cost. Against `dec.north-star-continuous-loop`
  as its operational strategy: goal 1, the conservative reading already
  ships, so no agent waits on this signature. Goal 2, the pinned test and
  the published wire fields make the rule checkable rather than
  remembered. Goal 3, it queues instead of self-ratifying because it fixes
  semantics of a schema field every adopting repository inherits. Goal 4,
  the undefined intersection was enqueued as this proposed record while
  the conservative implementation shipped, and the loop continued. Goal 5,
  this entry now carries the rubric, so the queue triages in one read.
- **Options**: (a) deferral wins and parking never overwrites a published
  `deferred_by` (this decision, shipped); (b) parking wins, breaking the
  deferred collapse `dec.loop-selection-deferred-findings` owns; (c) dual
  classification, breaking constraint 2 or 3 above. Recommendation: (a).
  Cost of no: the guard flip, test rewrite, and rendering rule the
  Consequences section inventories, plus re-stating the shipped pack
  reference.

## Consequences

- Pinned by `test_todo_defers_deferred_finding_stays_deferred_not_parked`
  (src/scanner/tests.rs).
- The shipped todo schema reference states the rule with this decision as its
  authority (agent-pack `artefact-schemas.md`).
- If ratified otherwise, the change is one guard in
  `src/scanner/todo_defers.rs` plus that test, plus a rendering rule for the
  collapse conflict dual classification reopens.
- `todo.parked-deferral-composition` is blocked on this decision reaching
  `status: accepted` and closes with no code change if accepted as written.
