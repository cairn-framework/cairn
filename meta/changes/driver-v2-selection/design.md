# Design: driver-v2-selection

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
4. Verify the session's terminal token and park state exactly as version one
   does.
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

## Driver-side work, recorded for completeness

The driver lives outside this repository (`~/repos/cairn-missions`), so the
following is NOT part of this repository's delta and lands there:

- A mission constructor replacing the `b-queue.txt` read.
- A ledger keyed on unit identity rather than queue-line hash.
- A `--dry-run` flag printing the constructed mission without dispatching,
  which is what the acceptance boundary exercises.
