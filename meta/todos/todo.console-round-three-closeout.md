---
node: cairn.ui
status: blocked
created: 2026-08-10
related: [dec.orchestration-placement, dec.webui-write-authority, todo.console-orchestration-ux-design, todo.console-signed-widening]
---

# Round three close-out: the maintainer says aligned

Not agent-actionable. This todo holds the one acceptance criterion of
`todo.console-orchestration-ux-design` whose completion authority is the
maintainer, extracted on 2026-08-10 so that unit could close its agent-actionable
work without ratifying the maintainer's provisional scope.

It is a silent park: `blocked` with no declared blocker
(`dec.todo-relationship-model` clause 4), and no `parent:`, so a closed unit does
not own an open obligation. The ordering that matters is the `blocked_by:` edge
on `todo.console-signed-widening`, which points here. Per-criterion evidence that
the rest of the design unit was discharged: `res.console-design-unit-closeout-audit`.

## What the maintainer is being asked

Round 2 and round 3 were worked through in session on 2026-08-06 but never
closed. Under the ratification proviso in `studio/orchestration-grill-brief.md`,
a ruling stands as direction until a round confirms it, and the round protocol
says a round is done when the maintainer says it is aligned. Two records are
still provisional (both in `todo.console-orchestration-ux-design`, under "Round 2
ruling: prototype first" and "Round 3 record"):

1. **The amended round-2 scope.** Three of task 3's required scenarios
   (decision-to-consequence as its own screen, the driver-states four-up, the
   narrow layout) are deferred to design-against-prototype-feedback, and the
   driver-absent, driver-idle, driver-crashed distinction counts as demonstrated
   by the lamp strip on the mixed and return screens rather than by a dedicated
   four-up.
2. **The round-3 record.** Task 6 discharged by enqueue
   (`dec.webui-write-authority`, accepted 2026-08-06), the task-5 write-set
   overlap residue discharged into the dispatch preview, task 3 routed to
   `todo.guided-console-prototype`.

## Recommendation

Ratify both. The deferred scenarios moved behind a prototype the maintainer runs
as a user, which tests them against real use rather than against a static screen.

Accepting releases `todo.console-signed-widening`, which owns every line of
orchestration console implementation. Rejecting costs nothing already built: the
rejected scope reopens as new todos on `cairn.ui`.

## Acceptance

- The maintainer records, in session, that the amended round-2 scope and the
  round-3 record are aligned, or names which part is rejected.
- The outcome is written into `todo.console-orchestration-ux-design` and
  `studio/orchestration-console-brief.md` as the round-3 close-out, dated.
- On alignment, `todo.console-signed-widening` moves to `open`. On rejection, the
  reopened scope exists as todos on `cairn.ui` before this todo closes.
- If the outcome is written as a decision, it cites
  `res.console-design-unit-closeout-audit` in `informed_by`. That audit is
  deliberately unlinked until then: the ruling that consumes it is the one this
  todo is waiting for, which is why it currently stands as a
  `CAIRN_RESEARCH_ORPHAN` info finding.
