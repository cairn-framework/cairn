---
node: cairn.kernel.query
status: open
created: 2026-07-29
---

# Maintainer Pending Queue

## Problem

Nothing surfaces what is waiting on the maintainer. Measured on 2026-07-29:
six decisions sat at `status: proposed` (`dec.source-tracked-verification`,
`dec.source-file-never-self`, `dec.loop-selection-deferred-findings`,
`dec.brownfield-discovery-cycle-severity`, `dec.autodocs-head-to-head-arm-b`,
`dec.contract-node-shape-drift-deferred`), blocking at least seven todos, and
the maintainer had been told about two of them. Reconstructing the queue took
a grep session across `meta/decisions/` and `meta/todos/`; the day before, a
whole session argued about standing findings whose fixes were parked behind
two of exactly these signatures. The maintainer's words: "I don't actually
know right now what's waiting."

`cairn status` shows todos, `cairn change list` shows changes, `cairn frontier`
shows buildable ghosts. No surface shows proposed decisions or the
ruling-blocked todos behind them.

## Scope

- `cairn pending` and `cairn pending --json` list, in one view:
  - every decision at `status: proposed`, with age, `nodes:`, its
    `ratification:` tier once that field exists, and the count of artefacts
    it blocks;
  - every `blocked` todo whose Depends-on names a non-accepted `dec.*` or an
    explicit maintainer verdict, with the blocking id;
  - open todos that exist only to draft a ruling (today:
    `todo.todo-relationship-schema-decision`), since they become signatures.
- Sort: transitive unblock count descending, then age descending. The top row
  is always the signature that moves the most work.
- Blocked-on-capability and blocked-on-other-todo rows are excluded: this is
  the human queue, not the dependency graph. `todo.revisit-trigger-correlator`
  (waits on a capability) and `todo.herdr-cairn-tool-attribution` (waits on
  another todo) are the regression cases.
- Wire shape follows the query-api spine: typed response struct, schema
  version, snapshot test, per `src/query_api/` conventions.
- The webui gains a read-only Pending panel fed by the same handler. Read-only
  is load-bearing: `cairn.ui` stays an explorer, and any future orchestrator
  console consumes the same JSON rather than growing a second source of truth.
- The loop's End step prints the pending count with the summary, so every
  session ends by surfacing the queue instead of narrating fragments of it.
  That line is a pack-content edit, which is binding; it rides the same
  ratification as `todo.decision-ratification-tiers` or its own explicit word.

## Depends on

Nothing for the command and panel. The End-step line needs the binding word
above. The `ratification:` column degrades gracefully while the tiers unit has
not landed: absent field renders as `binding` per that todo's default.

## Acceptance

- On a fixture holding one proposed decision blocking two todos and one
  independent blocked todo, `cairn pending --json` lists the decision with
  unblock count 2 and the ruling-blocked todos, excludes the
  capability-blocked todo, and sorts the decision first.
- Accepting the fixture decision empties its row on the next run without any
  other edit.
- `cairn pending` on this repository lists the six decisions named above (or
  their successors) the day it lands, and the count matches
  `grep -l "^status: proposed" meta/decisions/ | wc -l`.
- The command passes the `command_reference_consistency` battery (dispatch,
  `--json`, help, docs row), per the `cairn-add-cli-command` recipe.
- A wire snapshot pins the response shape.

## Origin

Maintainer conversation, 2026-07-29: going in circles, told things wait on a
signature without a view of what is waiting, needing load-bearing items put in
front of them. Goal 5 of `dec.north-star-continuous-loop`.
