# Design: driver-v2-selection

> Scope reduced 2026-08-04: the external driver implementation
> described below is retired (`dec.orchestration-placement` accepted;
> `todo.driver-in-repo` is the in-repo successor). Kept as context for
> the surviving read-surface audit.

## Approach

The driver becomes a thin read-and-dispatch loop with the fail-closed checks
of version one carried over verbatim:

1. Refresh the checkout at `origin/main`.
2. Ask the repository what is selectable, in the loop's own precedence order:
   lint first (honouring the ratified folds, so a finding that is
   decision-deferred, todo-parked, or Info-while-strict-is-green does not
   select), then the todo backlog. The signature queue is REPORTED, never
   dispatched: a proposed decision waits for a human and the driver moves on.
3. Dispatch one session with the constructed mission line.
4. Verify the session, failing closed and STOPPING on any of the version-one
   conditions, which are requirements rather than a summary: a dirty park,
   HEAD not at `origin/main`, a surviving loop branch or an open loop PR, a
   nonzero session exit, a final non-blank line that is not exactly a
   terminal token, `LOOP EXHAUSTED` without the unit's todo `done` on main,
   and a completion where the todo went `blocked` (a split or defer means
   replanning, not progress).
5. Append the unit id to the ledger.

Nothing in that loop needs a queue file, and nothing in it lets cairn schedule:
the reads are ordinary queries, and the scheduling stays in the shell script
outside the repository.

The repository-side question this change must settle first is whether those
reads are already sufficient. `cairn next` ranks work, `cairn pending` lists
signatures, `cairn frontier` lists buildable ghosts, and the lint wire carries
selectability. What is unverified is whether a mission line can be built from
their JSON alone: unit id, node, selection ground, and the evidence a session
needs to reproduce the choice. Task 1 answers that against a pinned commit and
files any gap as its own todo rather than widening this change.

## Changes

ADDED:
- `meta/research/driver-v2-read-surface.md`: the audit result from task 1, the
  evidence that the read surface is or is not sufficient.

MODIFIED:
- Nothing in `src/` by this change. Any gap the audit finds becomes its own
  todo and its own reviewed unit, so this change stays a proposal plus an
  evidence artefact.

REMOVED:
- Nothing.

RENAMED:
- Nothing.

## Driver-side work, retired 2026-08-04

The external driver (`~/repos/cairn-missions`) was to gain a mission
constructor, a unit-identity ledger, and a `--dry-run` flag. That
implementation is retired with the external plan
(`dec.orchestration-placement` accepted); the equivalent contracts,
including dry-run equivalence and the drained-backlog zero-exit case,
now live in `todo.driver-in-repo`. Recorded for history only; nothing
lands outside this repository under this change.
