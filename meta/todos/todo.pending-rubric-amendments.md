---
node: cairn.root
status: done
created: 2026-07-30
---

# Pending Rubric Amendments

## Problem

`dec.north-star-continuous-loop` (accepted, goal 4 and "The rubric a queued
decision must carry") requires every queued decision to carry Tier, Unblocks,
Alignment, and Options. Both entries in the pending queue predate that
enforcement and carry none of them: `dec.parked-deferral-composition`
(proposed, 2026-07-29) and `dec.bootstrap-fixture-corpus-split` (proposed,
2026-07-30). The maintainer triages the queue against prose that never states
tier, blast radius, or what a signature moves.

## Scope

Amend the two proposed decisions in place with a `## The rubric, applied to
this decision` section matching the accepted precedents
(`dec.north-star-continuous-loop`, `dec.cairn-mission`):

- **Tier**: `local` or `binding`, with the mechanical facts behind the claim
  (node span, supersession, affected paths against the binding-surface
  allowlist).
- **Unblocks**: the todos and findings that move on signature, by id.
- **Alignment**: one sentence per goal, stated against `dec.cairn-mission`
  first, then `dec.north-star-continuous-loop` as its operational strategy.
- **Options**: the alternatives each record already argues, the
  recommendation, and what saying no costs.

Both decisions REMAIN `status: proposed`; the maintainer signs separately.
Neither `## Decision` section changes.

## Depends on

Nothing. The rubric authority is accepted; the amendment is prose the loop
may author because it changes no status and ratifies nothing.

## Acceptance

- Both decision files contain the rubric section with all four bullets.
- `cairn pending` still lists exactly these two decisions.
- `git diff origin/main` shows no hunk inside either `## Decision` section.
- `cairn scan --strict` exits 0.
