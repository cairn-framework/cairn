# Proposal: driver-v2-selection

Successor to the static B-queue driver, the external supervisor at
`~/repos/cairn-missions/driver.sh` that runs outside this repository. Carries
the over-harness research inputs: `res.overharness-design-threads` (threads b,
c, and d) and `todo.review-gate-machine-check`.

## Motivation

The v1 driver consumes a hand-maintained queue file with an md5 line ledger.
Measured failure modes from the 2026-07-29 and 2026-07-30 batches: a false-halt
recovery required hand-editing the ledger (mission B3), the queue goes stale
the moment repository truth moves (it drained on 2026-07-30 and knows nothing
of work landed since), and every mission line restates what the graph already
publishes.

The repository now publishes machine-readable selection truth the driver
ignores: `cairn pending --json` (the signature queue), `cairn frontier --json`
(buildable ghosts), the lint wire's selectability fields (`strict_green`, and
per-finding `deferred_by` and `parked_by`), and the todo backlog through
`cairn next`. `dec.north-star-continuous-loop` fixes the boundary: per-repository
truth lives in the repository, scheduling lives outside it, and
`dec.product-perimeter` keeps cairn itself from scheduling. So the driver
should READ repository truth rather than carry a second copy of it.

## Outcome

Each driver session's mission is constructed at session start from repository
reads alone: the next selectable unit under the loop's own selection rules, or
a computed terminal state that ends the run. No static queue file exists. The
maintainer sees exactly what the driver acts on, because both read the same
commands.

## Acceptance boundary

A dry-run driver invocation against this repository at a known commit prints
the mission it WOULD dispatch (unit id, selection ground, evidence lines), and
that mission equals what a live loop session's Orient step selects at the same
commit. With nothing selectable it prints the terminal state and exits zero
without dispatching.

## Evidence

- A dry-run transcript on this repository, pinned in the implementing PR.
- The terminal state reproduces cairn's own stop evidence: strict scan green,
  no selectable finding, no ready todo.
- Ledger idempotence keyed on unit identity rather than queue-line hash:
  rerunning after a completed unit skips it without hand editing.

## Out of scope (exclusions)

- Moving the driver into this repository. Reconciled 2026-08-03: the
  future decision `res.overharness-design-threads` thread d was waiting
  for now exists as `dec.orchestration-placement`, but it is proposed, so
  `dec.product-perimeter` still binds and the external driver remains the
  live plan. If that record is signed, the in-repo successor is
  `todo.driver-in-repo`, and this change's task 1 audit is the direct
  input to its selector-wire task. Either way, moving the driver is out
  of scope here.
- Machine verification that the review gate ran: owned by
  `todo.review-gate-machine-check`. This change only records what the driver
  dispatched and why.
- Any new cairn write surface. The driver stays a read-only consumer; every
  repository-side gap this audit finds lands as its own reviewed unit.
- Multi-repository aggregation (thread c's shared index), parked with
  `dec.workspace-aggregation` as precedent. Version two stays single-repository.
