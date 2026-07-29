---
node: cairn.brownfield
status: done
created: 2026-07-27
---

# Maintainer ruling on AutoDocs-on-itself (Arm B)

Not agent-actionable. This exists so `todo.autodocs-head-to-head` has a real
dependency instead of an indefinite deferral.

That todo binds two arms: Arm A is Cairn brownfield over the AutoDocs repo, free
and runnable today; Arm B is AutoDocs over its own repo, which an agent cannot
reach alone.

Why, and what the options are: `res.autodocs-head-to-head-feasibility` (source
`src.autodocs`). The recommendation is `dec.autodocs-head-to-head-arm-b`,
accepted 2026-07-29, which is the single source of truth for the branches. Do
not restate them here.

## Acceptance

The maintainer accepts `dec.autodocs-head-to-head-arm-b`, or replaces it with
another decision, and states which branch was chosen. Then apply that ruling's
parent transition to `todo.autodocs-head-to-head` and set this todo `done`.

## Blocked (2026-07-27)

blocked on maintainer ruling: acceptance or replacement of
`dec.autodocs-head-to-head-arm-b`.

## Resolved (2026-07-29)

Resolved by acceptance of `dec.autodocs-head-to-head-arm-b` (maintainer
ratification, sheet of record PR #528, row W6): the recommended branch was
chosen, Arm B dropped, and `todo.autodocs-head-to-head` rewritten one-sided
and reopened.
