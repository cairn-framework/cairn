---
node: cairn.ui
status: open
created: 2026-07-18
---

# Webui Instrument Layout

Adopt the Calibrated Instrument mock's layout architecture in the webui
graph canvas, replacing the single-column card list with the mock's
whole-graph-in-one-viewport deck.

## Problem

`dec.webui-design-direction` (2026-07-12) adopted the Calibrated
Instrument direction as language only: state vocabulary, counters,
legend, guidance (PRs #419, #421, #422). The mock's layout architecture
was demoted to reference material. The result: the current canvas
still renders every node as a large stacked card in one vertical
column, roughly 5 of 25 nodes visible per 1440x900 screen, the rest
behind endless scroll. The map is a list. The mock
(`harness-output/mocks/calibrated-instrument.html` on branch
`design-studio-greenfield`, worktree `cairn-ds-greenfield`) fits the
entire graph in one viewport with roughly 5x the node density.

Owner direction (2026-07-18): the mocks' layout and UX were the point,
not just their language; adopt them. Both mocks share the same layout
architecture (banded containment, compact chips, edges at rest, docked
findings strip, right readout); they differ only in skin. Strata
Survey (`harness-output/mocks/strata-survey.html`) remains a scored
reference too: its graph canvas took the best state-clarity score
(9/10). This extends the scope of `dec.webui-design-direction` (its
"reference material, not wholesale replacement" line applied to
tokens; the layout was never explicitly declined). If the executor
judges the scope shift big enough, pair this with a short amending
decision.

## Task

Restructure the graph canvas to the shared deck layout. Outcomes, not
a pixel copy of either mock:

1. Bounded desktop workspace: the app frame fits the viewport; only
   the canvas well and the inspector panel scroll internally.
2. Containment as horizontal bands (system, one per container, root
   modules), labelled with microlabel captions and separated by
   hairlines, not nested boxes or a card column.
3. Compact node chips (name, file/symbol counts, state keel) in a 2D
   flow inside each band, dense enough that the 25-node dogfood graph
   fits a 1440x900 viewport without scrolling; the well pans/zooms
   only when the graph genuinely outgrows it.
4. Dependency edges visible at rest as curves between chips, across
   bands.
5. Frame zones: counters/hash/sync status in the topbar, a rail with
   search, kind toggles, and zoom, the findings strip docked under the
   well, inspector as the right column. Keep the existing command
   palette and drawer behaviour.
6. Responsive narrow behaviour: single-column stack below the tablet
   breakpoint (both mocks collapse to one column at 900px), keeping
   the landed mobile-nav work.
7. Preserve everything already landed under
   `dec.webui-design-direction`: selection emphasis, pan-to-selection,
   state keels, legend, guidance empty states, topbar stat dedupe.

Reference material: both mock HTML files, `design-dna.md`, and
`tokens.css` on the exploration branch. Translate through the existing
design system: all colours/spacing/type through
`docs/design-system/tokens.css` per `dec.webui-design-token-gate`; new
components go through `components.css` with the live reference updated.

## Acceptance

- The full dogfood graph (25 nodes) is visible in one 1440x900
  viewport with zero canvas scroll; verified by headless screenshot.
- Containment reads as bands, not a card column; edges visible at rest.
- All existing webui interactions keep working: selection,
  pan-to-selection, palette, findings drawer, blueprint modal, kind
  toggles, mobile stack.
- Token and component gates pass (`scripts/check-design-tokens.sh`,
  biome); visual harness gates pass.
- No regression on the landed guidance/dedupe/mobile-nav work.

dec:dec.webui-design-direction
