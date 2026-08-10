---
id: res.console-state-legibility-decomposition
nodes:
  - cairn.ui
date: 2026-08-10
method: primary
---

# Sizing evidence for `todo.console-state-legibility`

Recorded because the unit was decomposed on 2026-08-10 under the loop sizing
rule, and because four of the claims its 2026-08-03 body used to locate the work
were wrong when checked against the source. A session reading only the `blocked`
parent would have inherited all four.

Method: read each of the nine sub-items in the unit body against the file it
names, at `origin/main` `32e462ee`. No new measurement was taken; the 10.86:1
and 4.42:1 contrast figures are the parent's, not remeasured here.

## Confirmed as written

- `status.next_recommended` is produced by the query API
  (`src/query_api/handlers/project.rs:57`, `:90`, `:104`), served on
  `/api/status` (`src/ui/server.rs:152`), and reaches the browser inside the
  `fetchStatus()` payload (`src/ui_assets/app-data.js:11`). No client code
  reads it: the field name appears nowhere under `src/ui_assets/`. The other
  consumer is the CLI renderer (`src/cli/render/project.rs`), not the console.
- The backlog lane renders the stem and path, not the title
  (`src/ui_assets/channel-bar.js:55-61`).
- `DriftIndicator` counts errors plus warnings only
  (`src/ui_assets/status-bezel.js:32-46`), so the clean copy prints beside a
  non-zero findings count whenever only Info findings exist. That is this
  repository's normal state.
- `--orphaned` resolves to `--drift` and `--orphan-wash` to `--drift-wash` in
  both themes (`docs/design-system/tokens.css:150`, `:151`, `:302`, `:303`).
- `.state-dot` is `aria-hidden` and the module's accessible name is the node id
  alone (`src/ui_assets/node-module.js:56`).

## Corrected

1. **The pending lane is half built already.** The body says the lane "renders
   `item.id` truncated to a few characters" and asks for the summary and tier.
   `PendingDetail` (`src/ui_assets/channel-bar.js:116-161`) already renders
   `ruling_summary`, `rubric.tier`, `rubric.unblocks`, `rubric.alignment`,
   `rubric.options`, and the receipt evidence. The gap is confined to the
   collapsed row built by `itemLabel` (`src/ui_assets/channel-bar.js:63-81`).
   Scoped as written, the item would have rebuilt a component that exists.

2. **The audit does not skip the dimmed text; it mismeasures it.** The body says
   the audit "skips only `opacity === 0`", implying dimmed text goes unchecked.
   `isVisible` (`harness/lib/audit.mjs:113-119`) does reject only an element
   whose own opacity is exactly 0, but the DOM loop then calls
   `contrastRatio(fg, bg)` (`:173`) on that text anyway. The defect is that `fg`
   comes from `cs.color` and `bg` from `effectiveBg(el)`, and neither accounts
   for a faded ancestor. The SVG text pass repeats it (`:214`, `:236`). The fix
   is compositing the ancestor chain into both operands, not loosening a skip.

3. **Two more affected rules live in the canonical layer, not only in the webui
   overrides.** `.node-shell.dimmed { opacity: 0.55 }` and
   `.node-shell.focused.dimmed { opacity: 1 }` are declared in
   `docs/design-system/components.css:414-420`
   as well as `src/ui_assets/style.css:147` and `:151`, and `src/ui/mod.rs:67-71`
   concatenates tokens, then components, then the local overrides, so editing
   only `style.css` leaves the canonical rule in force. Likewise
   `docs/design-system/components.css:1094-1097` paints the orphaned key and the
   drift key from a single selector with `background: var(--orphaned)`, so
   retargeting the `--orphaned` token would move both keys onto the new hue and
   leave the legend as indistinguishable as before.

4. **Reduced motion does not hide drift, and it does not fully stop it
   either.** The body implies the drift state is invisible under
   `prefers-reduced-motion`. `.node-module.drift` still paints a drift border
   colour and the state dot still takes `var(--drift)`
   (`docs/design-system/components.css:527-531`, `:546`). What reduced motion
   is *meant* to remove is the motion cue, leaving colour alone, which
   greyscale then removes. It does not even manage that. The reduced-motion
   block at
   `docs/design-system/components.css:1268-1282` lists `.node-module` (0,1,0)
   and `.node-shell .node-module` (0,2,0), and nothing matching the dot, while
   `driftPulse` sits on `.node-module.drift` (0,2,0) and `driftBlink` on
   `.node-module.drift .state-dot` (0,3,0). `animation` does not inherit, so
   the dot keeps blinking in both consumers. `driftPulse` differs: it is
   stopped only where the module is wrapped in `.node-shell`, which the webui
   does (`src/ui_assets/graph-workspace.js:123`) and the live reference does
   not (`docs/design-system/index.html:975`). The harness cannot
   detect any of this, because `harness/eval.mjs:77-78` injects a global
   `animation: none !important` before it measures.

Note on the closure rule: `--orphaned`, `--orphan-wash`, and
`.node-module.drift` all already exist, so retargeting them is a modification.
`AGENTS.md`'s four-file design-system closure is written for *adding* a token
or component and fires only if the implementation introduces a new named
primitive.

## Consequence for the plan

Nine changes across the webui, the shared design system, and the visual harness
exceed one small reviewable PR, so the unit was split into
`todo.console-contrast-honesty`, `todo.console-state-grammar`, and
`todo.console-wire-legibility`. The latter two each depend on the first and not
on each other: until the audit composites ancestor opacity, any console change
is graded by a checker that overstates contrast, but grammar and wire legibility
touch disjoint components (node module, tokens, legend versus bootstrap data,
channel bar, status bezel) and impose no order between themselves.

## Limits

Whether the composited audit surfaces defects beyond `.node-shell.dimmed` is
unknown until it runs; if it does, `todo.console-contrast-honesty` absorbs them
or splits again.
