---
node: cairn.ui
status: open
created: 2026-08-10
parent: todo.console-state-legibility
---

# Make the contrast audit honest, then clear what it exposes

Implementation unit split out of `todo.console-state-legibility` under the
sizing rule. This unit owns clause 3 of that todo (the audit measurement) plus
the one clause 2 item that is its direct cause (the `opacity` dimming). Nothing
else from the parent belongs here: the state grammar and the wire rendering are
separate sub-todos.

This unit goes first because the measurement is currently wrong in the
permissive direction. Any later console work is graded against a checker that
reports better contrast than the screen shows, so a fix landed after this one is
measured honestly and a fix landed before it is not.

## Task

1. **Composite ancestor opacity in the audit.** `harness/lib/audit.mjs` reads
   uncomposited `getComputedStyle` colour. Text under a partly transparent
   ancestor is measured, not skipped: `isVisible` rejects only an element whose
   own opacity is exactly 0 (`harness/lib/audit.mjs:113-119`), and the DOM loop
   then calls `contrastRatio(fg, bg)` (`harness/lib/audit.mjs:173`) with `fg`
   from `cs.color` and `bg` from `effectiveBg(el)`, neither of which knows the
   ancestor faded it. The SVG text pass repeats both mistakes
   (`harness/lib/audit.mjs:214`, `:236`). Walk the ancestor chain, multiply the
   opacity values, and composite the foreground over the resolved backdrop
   before computing the ratio, in both passes. The parent measured 10.86:1
   where the composited value is 4.42:1.

2. **Fix the cause, in both layers.** `.node-shell.dimmed { opacity: 0.55 }`
   dims a whole node module, text included. It is declared twice: canonically
   at `docs/design-system/components.css:414-416` and again as a local override
   at `src/ui_assets/style.css:147`, with `.node-shell.focused.dimmed` restoring
   `opacity: 1` at `docs/design-system/components.css:418-420` and
   `src/ui_assets/style.css:151`. `src/ui/mod.rs:67-71` concatenates tokens,
   then components, then the local overrides into one served stylesheet, so
   deleting the local pair alone leaves the canonical rule in force and changes
   nothing on screen. In both places, stop fading node text with ancestor
   `opacity` and recess it with ink token steps from
   `docs/design-system/tokens.css` instead, so dimmed text stays above 4.5:1
   while still reading as secondary.

   The stroke opacities on `.dependency-link path`
   (`src/ui_assets/style.css:92`, `:100`, `:109`, `:118`) are SVG strokes, not
   text, and are out of scope; leave them.

## Acceptance

- The audit composites ancestor opacity before computing contrast in both the
  DOM pass and the SVG text pass, asserted by a harness test that fails against
  the current uncomposited code.
- `ux_defect_score` is zero with the composited measurement in force, so the
  zero is honest rather than an artefact of the skipped multiplication.
- No node text is dimmed by ancestor `opacity` in the stylesheet the server
  actually returns, checked against the concatenated output of
  `src/ui/mod.rs`, not against `src/ui_assets/style.css` alone.
- A dimmed node still reads as secondary: its rendered text colour differs from
  a normal and a focused node's and resolves to a `tokens.css` ink step, while
  measuring at least 4.5:1. `ux_defect_score` does not cover this
  (`harness/eval.mjs:333-344` scores contrast, overflow, clipping, tap size,
  palette, blankness, and landmarks), so deleting both `opacity` rules and
  putting nothing back would otherwise pass every other criterion here.
- `scripts/check-design-tokens.sh` and `scripts/check-a11y.sh` exit 0, and no
  hardcoded hex or rem is introduced.
