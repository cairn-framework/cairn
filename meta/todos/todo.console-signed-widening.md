---
node: cairn.ui
status: blocked
created: 2026-08-03
related: [dec.control-plane-programme, todo.overharness-console-ux, dec.orchestration-placement]
blocked_by: [todo.console-round-three-closeout]
---

# Console Signed Widening

Extracted from `todo.overharness-console-ux` when its Task 1 fallback
completed: the read-only three-lane console shipped in PR #572 while
`dec.control-plane-programme` awaited signature, and that todo's own
rule is to stop there. This unit owns everything the signed decision
must authorise before it ships.

This unit owns every line of orchestration console implementation. Its
current gate is in the last section.

## Task

1. Widen the console's write surface exactly as
   `dec.control-plane-programme` clause 3 assigns: writes only through
   the sanctioned todo verbs and the feedback intake seam. The console
   never dispatches.
2. Intake lane (`todo.overharness-console-ux` Task 4): surface the
   `.cairn/feedback.md` queue beside pending; triage promotes entries
   through the sanctioned write verbs.
3. Implement the state grammar `todo.console-orchestration-ux-design`
   specifies, to its state-source matrix: declared intent, observed
   actuality, and execution state each carried by at least two
   non-colour channels, with the vocabulary that unit settled.
4. Implement the execution-state surface those mockups specify: lease
   facts read from the sanctioned cairn lease-facts query, live
   execution state (session liveness, driver presence, run activity)
   read from the versioned driver contract, the dispatch preview
   rendered as a readable declaration rather than a form, and the three
   distinct empty states (no driver attached, attached and idle,
   crashed). The preview is a document; nothing in this console fires
   it.
5. Implement the decision-to-consequence path: opening a ruling shows
   what signing it moves, stated as a sentence rather than a badge.

## Acceptance

- The signed ownership split is quotable and the console's affordances
  match it exactly: no dispatch affordance exists anywhere in the UI.
- The intake lane renders feedback entries read-only with the exact
  promoting commands.
- Every state in the state-source matrix is distinguishable in greyscale
  and under reduced motion, and reaches a screen reader.
- A harness scenario covers each of the three driver empty states and
  asserts a landmark unique to that state, so a regression fails loudly
  instead of rendering a blank lane.
- Visual harness gates pass with `ux_defect_score` zero.

## Active gate (updated 2026-08-04)

`dec.control-plane-programme` is signed and
`dec.orchestration-placement` was accepted on 2026-08-04 (superseding
`dec.product-perimeter`), so this console is the driver's steering
surface. Two prerequisites remain:

1. `todo.console-round-three-closeout`, recorded in `blocked_by:`: the
   maintainer's round-3 close-out on the design unit's journeys,
   state-source matrix, and mockups, which tasks 3 to 5 above implement.
   `todo.console-orchestration-ux-design` completed its agent-actionable
   scope on 2026-08-10 and handed that one criterion to this todo, so
   the blocker moved with the criterion and the gate is unchanged.
2. `dec.webui-write-authority`, accepted 2026-08-06, authored by task 6
   of that design unit. It retired the read-only rule and named the
   sanctioned verbs this unit implements: `cairn ruling retry`,
   `release`, `park`, `unpark`, `budget`, and `run`, each an append-only
   recorded fact the driver observes. What still gates the write surface
   is not another signature but the fact store those verbs write to,
   which is rung 3 design under `todo.parallel-dispatch-granularity`,
   and the plan identity `cairn ruling run` takes as its argument, which
   clause 2 deliberately left to that design.

2026-08-07 audit (todo.roadmap-assumption-audit): correctly blocked on todo.console-orchestration-ux-design; child of the driver programme. Blocker moved to todo.console-round-three-closeout on 2026-08-10 when that unit's maintainer criterion was extracted; the gate is unchanged in effect.
