---
node: cairn.kernel.map
status: done
created: 2026-07-14
---

# Spec Rule Deferred Cell Check

## Problem
Reviewer NB1 on PR #320: `docs/registries/spec-rules.md` rows carry a
`Deferred-by` cell. A non-empty `Deferred-by` value that does not resolve
to an existing decision artefact under `meta/decisions/` is a dangling
deferral - it claims a decision gates the rule while none does. `cairn`
already models `DecisionStatus::Superseded`, so a deferral pointing at a
superseded decision is also stale.

## Acceptance
- `cairn scan` reports a finding when a spec-rule row has a non-empty
  `Deferred-by` cell whose target is not an existing decision whose
  status is not `Superseded`.
- A row whose `Deferred-by` cell is empty, or points at a live
  (non-superseded) decision, is not flagged.
- Covered by a behavioural test reading a tiny registry fixture with one
  dangling and one valid deferral.

Informed by: reviewer NB1 on PR #320.
