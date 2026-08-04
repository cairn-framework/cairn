---
node: cairn.ui
status: blocked
created: 2026-08-03
related: [dec.control-plane-programme, todo.overharness-console-ux, dec.orchestration-placement]
blocked_by: [todo.console-orchestration-ux-design]
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
4. Implement the execution-state surface those mockups specify: runs and
   leases against the driver contract, the dispatch preview rendered as a
   readable declaration rather than a form, and the three distinct empty
   states (no driver attached, attached and idle, crashed). The preview
   is a document; nothing in this console fires it.
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

## Active gate (2026-08-03)

`dec.control-plane-programme` is signed, so the original write-authority
gate cleared. Three prerequisites remain:

1. `dec.orchestration-placement` (proposed, binding) is what makes this
   console the driver's steering surface at all. It supersedes
   `dec.product-perimeter` on acceptance. External signature, so no
   `blocked_by:` entry is declarable (ruling 4 of
   `dec.todo-relationship-model`).
2. `todo.console-orchestration-ux-design`, recorded in `blocked_by:`: the
   journeys, the state-source matrix, and the evaluated mockups that
   tasks 3 to 5 above implement.
3. The decision resolving `dec.webui-design-authority` clause 4's
   read-only rule, authored by task 6 of that design unit. Its line 28
   revisit trigger has fired and the resolution must be signed before
   this unit's write surface ships.
