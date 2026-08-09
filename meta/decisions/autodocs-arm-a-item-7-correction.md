---
id: dec.autodocs-arm-a-item-7-correction
nodes:
  - cairn.brownfield
status: proposed
date: 2026-08-09
informed_by: [res.autodocs-arm-a-brownfield-run]
related:
  - dec.autodocs-head-to-head-arm-b
revisit_triggers:
  - "If AutoDocs supports polyglot repositories AND drops the repository-root layout requirement"
---

# Withdraw the item 7 claim from the Arm B drop, preserving its ruling

## Context

`dec.autodocs-head-to-head-arm-b` is accepted by maintainer ratification
(PR #528 sheet W6). Its ruling drops Arm B and rewrites
`todo.autodocs-head-to-head` as a one-sided Arm A stress test. That ruling is
sound and has now been fully discharged.

One claim inside its Rationale is not. It states that the cheap arm "also
unblocks the large-brownfield measurement `res.codeatlas-analysis` item 7
deferred". `res.autodocs-head-to-head-feasibility` option 2 repeats it, and
`todo.autodocs-head-to-head` carried it into its Scope.

Running Arm A falsified it (`res.autodocs-arm-a-brownfield-run`). Item 7 defers
JSON-surface token budgets "until a large brownfield dogfood repo exists to
measure against". AutoDocs holds 284 tracked files, which is medium, not the
large target item 7 names. The run also measured nothing item 7 actually asks
about: no JSON output token counts were taken.

This decision exists because loop reconciliation may not leave an accepted
decision asserting something the evidence disproves.

## Decision

The Arm B drop stands, unchanged and unreopened. Every operative obligation of
`dec.autodocs-head-to-head-arm-b` survives:

- Arm B stays dropped.
- Its revisit trigger stays live, and is carried on this decision's own
  `revisit_triggers` so it survives the target's transition to `superseded`:
  Arm B returns when AutoDocs supports polyglot repositories AND drops the
  repository-root layout requirement, at which point running it needs only a
  spend ruling.
- The refusal to substitute a local model for the quality axis stands.

Only the item 7 claim is withdrawn. Arm A does not unblock
`res.codeatlas-analysis` item 7. That deferral remains in force, and the trigger
for lifting it is still the arrival of a genuinely large brownfield repository
to measure against.

On acceptance this supersedes `dec.autodocs-head-to-head-arm-b`: the target
takes `status: superseded`, this decision takes
`supersedes: [dec.autodocs-head-to-head-arm-b]`, and every obligation restated
above continues to bind through this decision. Supersession is the mechanism
because `refines` is informational and cannot override a claim inside an
accepted ruling.

Those frontmatter edits are deliberately NOT made yet. Cairn enforces
`CAIRN_DECISION_SUPERSEDES_STATUS`: a structured `supersedes:` pointer requires
its target to already be `superseded`, so writing the link now would force the
loop to demote a ruling it may not touch. The link is therefore an effect of
acceptance, and `todo.autodocs-arm-a-item-7-ratification` carries the edits as
its Acceptance. Until then `related:` records the association without asserting
authority.

## Rationale

Supersession that subtracts one sentence, rather than a fresh ruling on Arm B,
because exactly one sentence of the target's Rationale is wrong and the ruling
it supported does not depend on it. Option 2 was chosen primarily because the
comparison Arm B asks for is unavailable at any price, upstream not supporting
the target on two independent counts. That reason is untouched by this
evidence, so it is restated intact above rather than re-argued.

Left `proposed` rather than accepted because `cairn-loop-reconcile` section 4
forbids the loop from self-ratifying a binding decision outright, and this
target was in addition signed by the maintainer personally. Acceptance is
theirs to give even for a correction that only subtracts a claim.

The alternative considered and rejected was recording the correction only in
`res.codeatlas-analysis` item 7, which is where a future session would look. It
is cheaper, and it was done anyway, but on its own it leaves two artefacts of
different authority disagreeing, with the accepted one winning by rank while
being wrong.

## Consequences

- Until this is accepted, `dec.autodocs-head-to-head-arm-b` remains the accepted
  authority and still contains the falsified sentence. The contradiction is
  recorded rather than resolved.
- `todo.autodocs-arm-a-item-7-ratification` is blocked on the maintainer's
  ruling and carries the forced choice.
- If accepted, no downstream work changes: item 7 was deferred before Arm A and
  stays deferred after it. What changes is that the record stops claiming
  otherwise.
- If rejected, the maintainer is asserting that Arm A did satisfy item 7, which
  would need the 284-file measurement in `res.autodocs-arm-a-brownfield-run`
  reinterpreted as large enough, and would still leave item 7's own quantity
  unmeasured.
