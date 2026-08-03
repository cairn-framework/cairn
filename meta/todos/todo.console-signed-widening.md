---
node: cairn.ui
status: open
created: 2026-08-03
related: [dec.control-plane-programme, todo.overharness-console-ux]
---

# Console Signed Widening

Extracted from `todo.overharness-console-ux` when its Task 1 fallback
completed: the read-only three-lane console shipped in PR #572 while
`dec.control-plane-programme` awaited signature, and that todo's own
rule is to stop there. This unit owns everything the signed decision
must authorise before it ships.

Blocked on the maintainer signing `dec.control-plane-programme`; that
gate is external, so no `blocked_by:` entry is declarable (ruling 4 of
`dec.todo-relationship-model`).

## Task

1. Widen the console's write surface exactly as the signed decision
   assigns: writes only through the sanctioned todo verbs and the
   feedback intake seam; the driver dispatches, the console never does.
2. Intake lane (`todo.overharness-console-ux` Task 4): surface the
   `.cairn/feedback.md` queue beside pending; triage promotes entries
   through the sanctioned write verbs.

## Acceptance

- The signed decision's ownership split is quotable and the console's
  affordances match it exactly.
- The intake lane renders feedback entries read-only with the exact
  promoting commands; visual harness gates pass.

## Scope update (2026-08-03, dec.control-plane-programme signed)

The programme decision is accepted, so this unit is unblocked. Its scope
now follows `dec.orchestration-placement` (proposed): the console is the
driver's steering surface, not an independent write surface. Widening
means: sanctioned todo verbs and the feedback intake seam under the
signed ownership split, wired so the driver (todo.driver-in-repo)
dispatches and the console shows and records. Build after the placement
decision is signed, alongside the driver unit.
