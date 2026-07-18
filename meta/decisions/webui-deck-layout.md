---
id: dec.webui-deck-layout
nodes:
  - cairn.ui
status: accepted
date: 2026-07-18
informed_by: [res.design-studio-greenfield, res.webui-review-audit]
refines: [dec.webui-design-direction]
---

# Webui adopts the design-studio deck layout for the graph canvas

## Context

`dec.webui-design-direction` (2026-07-12) ratified the Calibrated
Instrument aesthetic and adopted the greenfield exploration's design
language: state vocabulary, counters, legend, guidance. Its Track B
evidence read "refine, do not redesign", and the codified greenfield
output was scoped as reference material. The three implementation
priorities landed (PRs #419, #421, #422).

What that scope left in place is the canvas layout: every node renders
as a large card in a single vertical column, roughly 5 of 25 dogfood
nodes visible per 1440x900 screen, the rest behind vertical scroll.
Both greenfield mocks (`calibrated-instrument.html`,
`strata-survey.html`, branch `design-studio-greenfield`) share one
layout architecture that fits the entire graph in a single viewport:
banded containment strata, compact node chips, dependency edges
visible at rest, a bounded app frame where only the canvas well and
inspector scroll internally. Headless screenshots of the live webui
against both mocks (2026-07-18) confirm the density gap is roughly 5x.

## Decision

Owner direction (2026-07-18): the mocks' layout and UX architecture is
adopted, not just their language. The webui graph canvas moves from
the single-column card list to the shared deck layout: bounded desktop
workspace, containment as labelled horizontal bands, compact chips in
2D flow, edges at rest, findings docked under the well, inspector as
the right column, single-column stack below the tablet breakpoint.

This refines `dec.webui-design-direction`, it does not replace it:

- The Calibrated Instrument aesthetic ruling stands; Strata Survey's
  state vocabulary stays adopted as motif.
- "Refine, do not redesign" is narrowed: it governed identity (tokens,
  type, components), which is kept; it no longer shields the canvas
  layout, which both tracks scored as the weakest zone.
- The token and component gates (`dec.webui-design-token-gate`) govern
  the implementation unchanged; the greenfield `tokens.css` remains
  reference material, not a token replacement.

Execution is specified in `todo.webui-instrument-layout`.

## Consequences

- `todo.webui-instrument-layout` is the implementation home; its
  amending-decision contingency is resolved by this decision.
- Any future canvas work targets the deck layout, not the card column.

revisit_triggers:
  - the deck layout fails the visual harness or design-quality scorer
    on real repos with graphs much larger than the dogfood 25 nodes
  - the webui's scope grows beyond read-only exploration
