---
node: cairn.kernel.query
status: open
created: 2026-07-29
---

# Maintainer Pending Queue

## Problem

Nothing surfaces what is waiting on the maintainer. Measured on 2026-07-29:
six decisions sat at `status: proposed`, blocking at least seven todos, and
the maintainer had been told about two of them. Reconstructing the queue took
a grep session across `meta/decisions/` and `meta/todos/`; the day before, a
whole session argued about standing findings whose fixes were parked behind
two of exactly these signatures. The maintainer's words: "I don't actually
know right now what's waiting."

`cairn status` shows todos, `cairn change list` shows changes, `cairn frontier`
shows buildable ghosts. No surface shows proposed decisions.

## Scope

First cut, typed data only. Todos carry no machine-readable relationship
edges yet (`todo.todo-relationship-schema-decision` exists precisely because
of that), and parsing Depends-on prose would introduce the second, untyped
convention the folding todo rejects. So:

- `cairn pending` and `cairn pending --json` list every decision at
  `status: proposed`, with its age in days, `nodes:`, and its `ratification:`
  tier once that field exists (absent renders as `binding`, that todo's
  default). Sort: age descending, oldest first. Nothing else is listed,
  parsed, or inferred.
- The webui gains a read-only Pending panel consuming the same JSON. Read-only
  is load-bearing: `cairn.ui` stays an explorer, and any future orchestrator
  console consumes the same response rather than growing a second source of
  truth.

Follow-ups that are NOT this unit:

- Ruling-blocked todo rows and unblock-count sorting need typed
  decision-to-todo edges, so they arrive only after
  `dec.todo-relationship-model` is ratified and implemented.
- Printing the pending count from the loop's End step is a shipped-pack edit,
  which is binding; it belongs to the `todo.decision-ratification-tiers`
  ratification or its own explicit word, never to this query unit.

## Depends on

Nothing.

## Acceptance

- On a fixture holding decisions in `proposed`, `accepted`, `superseded`, and
  `deprecated` states, `cairn pending --json` lists every proposed decision
  and no other, oldest first.
- Flipping the fixture decision to `accepted` empties its row on the next run
  with no other edit.
- The command passes the `command_reference_consistency` battery (dispatch,
  `--json`, help, docs row), per the `cairn-add-cli-command` recipe.
- The webui panel renders from the same response the CLI prints, verified the
  way existing webui surfaces are.

## Origin

Maintainer conversation, 2026-07-29: going in circles, told things wait on a
signature without a view of what is waiting, needing load-bearing items put in
front of them. Goal 5 of `dec.north-star-continuous-loop`.
