---
id: res.console-design-unit-closeout-audit
nodes:
  - cairn.ui
date: 2026-08-10
method: primary
---

# Audit: what `todo.console-orchestration-ux-design` had left

Recorded because the unit closed on 2026-08-10 with one criterion relocated
rather than met, and a session reading main afterwards sees only a `done` status.

Method: read each acceptance criterion in the unit's body against the artefact it
names, and check the artefact against the current source. No new measurement was
taken.

## Criterion by criterion

1. **State-source matrix exists, every state naming a query or a proposed
   versioned driver contract, plus at least two non-colour channels.**
   Discharged after a correction. `studio/orchestration-state-matrix.md`
   (authored 2026-08-05, aligned with the maintainer the same day) carries the
   2x2 legend, a non-colour channel inventory, and a pairwise proof section.
   As written on 2026-08-05 its `lease held` and `outcome recorded` rows named a
   proposed cairn query and said neither read surface existed yet; that is no
   longer true, and the rows have been corrected. Both surfaces
   shipped as `cairn lease list` and `cairn ruling list`
   (`src/cli/commands/coord.rs::run_lease_command`,
   `src/query_api/handlers/coordination.rs::coordination_leases_json`) on
   2026-08-07 under `dec.rung-three-coordination-substrate`, with
   `dec.coord-fact-write-once` (2026-08-09) later hardening the store behind
   them, all after the matrix was written. The
   rows now name the shipped verbs, and the same stale baseline in the unit's
   evidence item 2 is marked superseded. The console still renders none of these
   facts, which is `todo.console-signed-widening`'s work, not this unit's.
2. **Journeys and mockups render every required scenario, with driver-absent,
   driver-idle, and driver-crashed distinguishable.** Discharged only against the
   amended round-2 scope. Four screens exist under `studio/mocks/`
   (`orchestration-guided-journey.html`, `orchestration-return-orient.html`,
   `orchestration-plan-dispatch.html`, `orchestration-mixed-repository.html`),
   and the driver distinction rests on the lamp vocabulary strip present in the
   mixed and return screens, not on a dedicated four-up. Three scenarios
   (decision-to-consequence as its own screen, the driver-states four-up, the
   narrow layout) are deferred to design-against-prototype-feedback. That
   amendment is provisional under the ratification proviso in
   `studio/orchestration-grill-brief.md`, which is why the close-out is a
   maintainer act and not an iteration's to assume.
3. **Lease and granularity screens handed to
   `todo.parallel-dispatch-granularity`, with the answered questions recorded.**
   Discharged. That todo carries a "Mockup evidence received (2026-08-06, console
   unit round 3)" section naming the expired held claim `r-041`, the no-claim
   contrast, and the write-set overlap case in the dispatch preview. The todo is
   `done`.
4. **Webui write-authority decision enqueued in `cairn pending`.** Discharged.
   `dec.webui-write-authority` was authored by task 6 and accepted 2026-08-06;
   the criterion asked for enqueue, and acceptance exceeds it. The grill brief
   records both signatures from that session as closed.
5. **No file under `src/` or `harness/` changed in the unit.** Held throughout.

## What was left, and where it went

Only the maintainer's round-3 close-out: saying the amended round-2 scope and the
round-3 record are aligned. No iteration can produce that, and self-ratifying it
would have signed a scope amendment the maintainer reserved. It moved to
`todo.console-round-three-closeout` (`cairn.ui`, blocked, silent park), and
`todo.console-signed-widening` now declares that todo as its blocker, so the gate
on orchestration console implementation is unchanged in effect.

## Limits

This audit reads artefacts against criteria. It does not re-review the mockups on
their merits, and it takes no position on whether the amended round-2 scope is
the right scope. That judgement is the maintainer's and is the whole content of
the close-out.
