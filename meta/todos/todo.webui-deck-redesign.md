---
node: cairn.ui
status: open
created: 2026-07-18
---

# Webui Deck Redesign

Use the design-studio methodology to sort the webui properly: codify a
real design system from the greenfield exploration's output, and
implement it in the webui, replacing the single-column card canvas
with the mocks' whole-graph-in-one-viewport deck layout.

## Problem

`dec.webui-design-direction` (2026-07-12) adopted the Calibrated
Instrument direction as language only: state vocabulary, counters,
legend, guidance (PRs #419, #421, #422). The mocks' layout
architecture and codified system were demoted to reference material.
The result: the current canvas still renders every node as a large
stacked card in one vertical column, roughly 5 of 25 dogfood nodes
visible per 1440x900 screen, the rest behind endless scroll. The map
is a list. Both mocks (branch `design-studio-greenfield`, worktree
`cairn-ds-greenfield`, under `harness-output/mocks/`) fit the entire
graph in one viewport at roughly 5x the density.

Owner direction (2026-07-18): the mocks' layout and UX were the point,
not just their language; use design-studio to sort the webui, create a
proper design system, and get it implemented. Both mocks share the
same layout architecture (banded containment, compact chips, edges at
rest, docked findings strip, right readout); they differ only in skin.
Strata Survey remains a scored reference too: its graph canvas took
the best state-clarity score (9/10). Authority:
`dec.webui-deck-layout` (2026-07-18), refining
`dec.webui-design-direction` (whose "refine, do not redesign" line
governed identity, not the canvas layout or system depth).

## Task

Three stages, one todo. Run via the design-studio implementation lane
(github.com/george-rd/design-studio) where it helps; its greenfield
codified output is the starting corpus, not a suggestion.

### 1. Codify the design system

Rebuild `docs/design-system/` as a proper system in the Calibrated
Instrument direction, seeded from the exploration branch's codified
output (`harness-output/design-dna.md`, `harness-output/tokens.css`,
the skill under `harness-output/design-system/skill/`):

- `tokens.css`: reconcile the current token set with the greenfield
  tokens (chassis steps, signal accent, instrument amber, hairlines);
  one authoritative set, no fork.
- `components.css`: real components for the deck vocabulary (bezel
  counters, rail, node chips, state keels, bands, channel rows,
  readout sections), each demonstrated in the live reference
  `index.html`.
- `README.md` and the live reference updated in the same commit as
  every token/component addition, per the design-system contract.
- Existing gates keep passing throughout
  (`scripts/check-design-tokens.sh`, biome); the landing page and any
  other token consumers must stay conformant with the reconciled set.
- Resolve the stale `docs/design-system/NEXT_SESSION.md` handoff:
  fold its still-valid items (hinge layout ideas, chain rails, decision
  chips) into this todo's stages or discard them explicitly, then
  retire the file.

### 2. Implement the deck layout in the webui

Restructure `src/ui_assets/` to the shared deck layout. Outcomes, not
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

### 3. Verify with the eval loop

Use the design-studio evaluate lane (or the existing visual harness
under `harness/`) against the implemented webui, not just the mocks:
adversarial gate on interactions (selection, edge navigation, search,
toggles), zero console errors, no page-frame overflow at 390px and
683px.

## Downstream, out of scope here

The webui's look is baked into public assets that go stale after this
redesign: README (`webui.gif`, `design-system.png`,
`landing-full.png`), landing hero (`webui.mp4`, poster and og:image
`webui-graph.png`), `docs/assets/screenshots/`, and visual harness
baselines. Tracked separately in `todo.ui-asset-refresh`, which this
todo unblocks.

## Acceptance

- `docs/design-system/` presents the reconciled Calibrated Instrument
  system: tokens, components, fonts, live reference, README, all
  consistent; token gate and biome pass; landing stays conformant.
- The full dogfood graph (25 nodes) is visible in one 1440x900
  viewport with zero canvas scroll; verified by headless screenshot.
- Containment reads as bands, not a card column; edges visible at rest.
- All existing webui interactions keep working: selection,
  pan-to-selection, palette, findings drawer, blueprint modal, kind
  toggles, mobile stack.
- Visual harness gates pass; adversarial interaction gate clean.
- No regression on the landed guidance/dedupe/mobile-nav work.

dec:dec.webui-deck-layout
dec:dec.webui-design-direction
