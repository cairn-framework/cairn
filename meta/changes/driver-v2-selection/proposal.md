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

The repository's read surface is audited for driver sufficiency. At a
pinned commit, the audit answers whether a mission line (unit id, node,
selection ground, reproducible evidence) can be built from
`cairn next --json`, `cairn pending --json`, `cairn frontier --json`,
and `cairn lint --json` alone, landing as
`meta/research/driver-v2-read-surface.md`; every gap is filed as its
own todo against the owning node. The audit is the direct input to the
in-repo successor's selector-wire task (`todo.driver-in-repo` task 4).

## Acceptance boundary

The research artefact exists with per-command sufficiency verdicts at a
pinned commit, and every gap it names has a filed todo. Nothing outside
`meta/` changes under this change.

## Evidence

- `meta/research/driver-v2-read-surface.md`, pinned to the audited
  commit.
- The gap todos it filed.

The former driver-side evidence (dry-run transcript, drained-backlog
terminal run, ledger idempotence) was retired with the external
implementation on 2026-08-04 (`dec.orchestration-placement` accepted);
the dry-run equivalence and drained-backlog contracts now live in
`todo.driver-in-repo`'s acceptance.

## Out of scope (exclusions)

- Moving the driver into this repository. Reconciled 2026-08-04:
  `dec.orchestration-placement` is accepted and supersedes
  `dec.product-perimeter`, so the in-repo successor is the open
  `todo.driver-in-repo`. This change stays what it is: the record of
  the external v1 supervisor, whose read-surface audit (task 1, still
  to run) is the direct input to that successor's selector-wire task.
  Moving the driver is out of scope here.
- Machine verification that the review gate ran: owned by
  `todo.review-gate-machine-check`. This change only records what the driver
  dispatched and why.
- Any new cairn write surface. The driver stays a read-only consumer; every
  repository-side gap this audit finds lands as its own reviewed unit.
- Multi-repository aggregation (thread c's shared index), parked with
  `dec.workspace-aggregation` as precedent. Version two stays single-repository.
