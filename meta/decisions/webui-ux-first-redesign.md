---
id: dec.webui-ux-first-redesign
nodes:
  - cairn.ui
status: accepted
date: 2026-07-18
informed_by: [res.design-studio-greenfield, res.webui-review-audit]
refines: [dec.webui-design-direction]
---

# Webui is redesigned UX-first from cairn's purpose, via design-studio

## Context

`dec.webui-design-direction` (2026-07-12) ratified the Calibrated
Instrument aesthetic and adopted the greenfield exploration's design
language: state vocabulary, counters, legend, guidance. Its Track B
evidence read "refine, do not redesign", and the codified greenfield
output was scoped as reference material. The three implementation
priorities landed (PRs #419, #421, #422).

What that scope left in place is the interaction architecture: the
canvas renders every node as a large card in a single vertical column,
roughly 5 of 25 dogfood nodes visible per 1440x900 screen, the rest
behind endless scroll. The map is a list. Both greenfield mocks
(`calibrated-instrument.html`, `strata-survey.html`, branch
`design-studio-greenfield`) demonstrate that a bounded, whole-graph
workspace is achievable at roughly 5x the density; headless
screenshots of the live webui against both mocks (2026-07-18) confirm
the gap.

## Decision

Owner direction (2026-07-18): the webui is redesigned UX-first, from
what cairn is supposed to do and how it could be used, not by
iterating on the current UI. The design-studio loop runs a greenfield
UX phase that derives the information architecture and layout from
cairn's real jobs (orient in a codebase, inspect a node, trace the
two-chain topology of provenance evidence into a decision hinge and
authority rules out of it, review findings and drift, follow changes),
then produces and implements the canonical design system.

Bindings and non-bindings, stated exactly:

- The two mocks and the landed card-column canvas are scored evidence
  for the UX phase, not the spec. No layout structure (bands, chips,
  columns) is prescribed in advance; the UX phase chooses it.
- Two outcomes are settled regardless of what IA emerges: a bounded
  desktop workspace (only designated panels scroll internally; no
  endless page scroll), and whole-graph legibility (the dogfood graph
  readable in one viewport without scrolling).
- The Calibrated Instrument aesthetic ruling stands; Strata Survey's
  state vocabulary stays adopted as motif. "Refine, do not redesign"
  is narrowed to the product's identity (name, voice, the instrument
  framing).
- The greenfield codified output (`design-dna.md`, `tokens.css`, the
  skill) is promoted from reference material to seed corpus: the loop
  reconciles it with the current `docs/design-system/` into one
  canonical system. The token and component gates
  (`dec.webui-design-token-gate`) govern the result unchanged;
  consumers (webui, landing, live reference) follow the reconciled
  set.
- The implementation is component-based and modular as an outcome, not
  a mapping rule: components have cohesive responsibilities, explicit
  data and event interfaces, and independently testable rendering,
  built on reusable design-system primitives. The component design
  chooses the boundaries; it is not required to mirror the existing
  ES-module split.
- Two-way interaction (a person leaving feedback or notes in the webui
  for the harness or agents to pick up, e.g. landing in the
  `cairn feedback` / `.cairn/feedback.md` seam) is an explicit future
  exploration, not an obligation of this implementation. The webui
  stays read-only; the redesign must merely avoid architectural
  choices that would foreclose a write seam later.

Execution is specified in `todo.webui-ux-redesign`.

## Consequences

- `todo.webui-ux-redesign` is the implementation home.
- The redesign staleness it creates in public assets (README webui gif
  and screenshots, landing hero video/poster and og:image, harness
  baselines, the `docs/design-system/NEXT_SESSION.md` handoff) is
  tracked in `todo.ui-asset-refresh`, downstream of the redesign.

revisit_triggers:
  - the UX-first result fails the visual harness or design-quality
    scorer on real repos with graphs much larger than the dogfood 25
    nodes
  - the two-way webui exploration is taken up, growing the webui
    beyond read-only exploration
