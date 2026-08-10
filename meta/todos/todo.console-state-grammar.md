---
node: cairn.ui
status: blocked
created: 2026-08-10
blocked_by:
  - todo.console-contrast-honesty
parent: todo.console-state-legibility
---

# Give node state a grammar that survives greyscale, stillness, and a screen reader

Implementation unit split out of `todo.console-state-legibility` under the
sizing rule. This unit owns the rest of clause 2 of that todo: the three state
channels that currently carry no information for a reader who cannot see
colour, cannot see motion, or cannot see the screen at all. The contrast work
is `todo.console-contrast-honesty` and the wire rendering is
`todo.console-wire-legibility`.

Blocked on `todo.console-contrast-honesty` because that unit makes the contrast
audit composite ancestor opacity. Landing colour and token changes before the
measurement is honest grades them against a checker that overstates contrast.

## Task

1. **Split `orphaned` from `drift`, in the token and in the selector.**
   `docs/design-system/tokens.css:150` sets `--orphaned: var(--drift)` and
   `:151` sets `--orphan-wash: var(--drift-wash)`; the light theme
   (`[data-theme="light"]`, `docs/design-system/tokens.css:229`) repeats both
   at `:302` and `:303`. Retargeting the token is necessary but not sufficient:
   `docs/design-system/components.css:1094-1097` paints the orphaned key and
   the drift key from one selector with `background: var(--orphaned)`, so a new
   orphaned hue would simply move both keys onto it. Split that selector too.
   `src/ui_assets/style.css:367-369` additionally gives both keys the same
   tilt. The labels already differ (`src/ui_assets/evidence-rail.js:347-348`);
   it is the keys that are indistinguishable.

2. **Stop leaning on motion for drift, and make the reduced-motion escape
   actually win the cascade.** `docs/design-system/components.css:530` drives
   `driftPulse` on `.node-module.drift` (specificity 0,2,0) and `:546` drives
   `driftBlink` on `.node-module.drift .state-dot` (0,3,0), with the keyframes
   at `:1135` and `:1146`. The reduced-motion block at
   `docs/design-system/components.css:1268-1282` lists `.node-module` (0,1,0)
   and `.node-shell .node-module` (0,2,0), and nothing matching the dot.
   `animation` does not inherit, so the dot keeps blinking for a reader who
   asked for reduced motion. `driftPulse` is stopped only in the webui, where
   `src/ui_assets/graph-workspace.js:123` wraps each module in `.node-shell`
   and the 0,2,0 reduced-motion rule wins on source order; the design-system
   live reference has no wrapper (`docs/design-system/index.html:975`), so
   there the 0,1,0 rule loses and `driftPulse` runs too. Adding a bare
   `.state-dot` would not help either: 0,1,0 cannot beat 0,3,0. Put
   `.node-module.drift` and `.node-module.drift .state-dot`, or equivalently
   specific selectors, into the reduced-motion block. The harness cannot catch
   any of this, because `harness/eval.mjs:77-78` injects its own global
   `animation: none !important` before measuring. Then add a static channel
   (shape, outline, or a printed marker) so drift survives stillness and
   greyscale rather than falling back to the border and dot colour at
   `docs/design-system/components.css:527-531` and `:546`. Motion stays
   decoration.

3. **Put node state into the accessible name.** `src/ui_assets/node-module.js:56`
   renders `<span class="state-dot" aria-hidden="true">`, and the module's
   accessible name is the node id alone, so no state reaches a screen reader.
   Name the state in the accessible name, not only in a decorative dot.

`--orphaned`, `--orphan-wash`, and `.node-module.drift` all already exist, so
retargeting them is a modification, not an addition, and `AGENTS.md`'s
four-file closure does not fire on its own. It fires only if the implementation
introduces a new named token or component, for instance a distinct static drift
marker: then `docs/design-system/tokens.css` or `components.css`, the live
reference at `docs/design-system/index.html`, and
`docs/design-system/README.md` move in the same commit. Either way the live
reference is updated wherever it displays a state whose appearance changed.

## Acceptance

- Every state pair on the canvas (`synced`, `ghost`, `orphaned`, `drift`, the
  four in `src/ui_assets/utils.js:27`) is distinguishable in greyscale and under
  `prefers-reduced-motion`, verified by a capture of each rather than by
  inspection of the stylesheet.
- Under `prefers-reduced-motion` and WITHOUT the harness's injected
  `animation: none !important`, the computed `animation-name` is `none` on both
  `.node-module.drift` and its `.state-dot`, checked in the webui (wrapped in
  `.node-shell`) and in `docs/design-system/index.html` (not wrapped), since
  the two differ in which reduced-motion selector applies.
- Node state reaches a screen reader through the accessible name, asserted by
  the harness.
- `--orphaned` and `--drift` resolve to different colours in both themes, and
  the two rendered legend keys differ from each other, not merely the tokens.
- `scripts/check-design-tokens.sh` and `scripts/check-a11y.sh` exit 0, and no
  hardcoded hex or rem is introduced outside `tokens.css`.
