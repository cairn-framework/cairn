---
id: dec.webui-deck-layout
nodes:
  - cairn.ui
status: accepted
date: 2026-07-18
informed_by: [res.design-studio-greenfield, res.webui-review-audit]
refines: [dec.webui-design-direction]
---

# Webui adopts the design-studio deck layout and its codified design system

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
adopted, not just their language, and the design-studio loop produces
and implements the canonical design system. The webui graph canvas
moves from the single-column card list to the shared deck layout:
bounded desktop workspace, containment as labelled horizontal bands,
compact chips in 2D flow, edges at rest, findings docked under the
well, inspector as the right column, single-column stack below the
tablet breakpoint.

This refines `dec.webui-design-direction`, it does not replace it:

- The Calibrated Instrument aesthetic ruling stands; Strata Survey's
  state vocabulary stays adopted as motif.
- "Refine, do not redesign" is narrowed to the product's identity
  (name, voice, the instrument framing). It no longer shields the
  canvas layout or the depth of the design system, both of which the
  tracks scored as the deficit.
- The greenfield codified output (`design-dna.md`, `tokens.css`, the
  skill) is promoted from reference material to seed corpus: the
  design-studio loop reconciles it with the current
  `docs/design-system/` into one canonical system. The token and
  component gates (`dec.webui-design-token-gate`) govern the result
  unchanged; consumers (webui, landing, live reference) follow the
  reconciled set.

Execution is specified in `todo.webui-deck-redesign`.

## Consequences

- `todo.webui-deck-redesign` is the implementation home; its
  amending-decision contingency is resolved by this decision.
- Any future canvas work targets the deck layout, not the card column.
- The redesign staleness it creates in public assets (README webui gif
  and screenshots, landing hero video/poster and og:image, harness
  baselines, the `docs/design-system/NEXT_SESSION.md` handoff) is
  tracked in `todo.ui-asset-refresh`, downstream of the redesign.

revisit_triggers:
  - the deck layout fails the visual harness or design-quality scorer
    on real repos with graphs much larger than the dogfood 25 nodes
  - the webui's scope grows beyond read-only exploration
