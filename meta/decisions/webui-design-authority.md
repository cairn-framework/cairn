---
id: dec.webui-design-authority
nodes:
  - cairn.ui
status: accepted
date: 2026-07-28
informed_by:
  - res.design-studio-greenfield
  - res.webui-review-audit
  - res.webui-design-quality-review
supersedes:
  - dec.webui-ai-vision-loop-declined
  - dec.webui-design-quality-direction
  - dec.design-studio-exploration-method
  - dec.webui-design-direction
  - dec.webui-ux-first-redesign
related:
  - dec.webui-design-token-gate
  - dec.webui-a11y-static-audit-gate
  - dec.landing-design-token-conformance
  - dec.marketing-visual-world
revisit_triggers:
  - "the maintainer sanctions both a Node/Playwright toolchain and a vision provider, which reverses the vision-loop foreclosure and requires superseding this decision"
  - "the maintainer sanctions a paid or network vision provider on its own, which could replace the manual inspect_image review step"
  - "a design-quality proxy is shown to reward a change a visual review judges worse, requiring the proxy or its saturation to be retuned"
  - "the adopted state-vocabulary motifs fail the design-quality scorer or the visual harness"
  - "the UX-first result fails on real repos with graphs much larger than the dogfood 25 nodes"
  - "the two-way webui exploration is taken up, or the webui's scope otherwise grows beyond read-only exploration, changing the instrument framing"
  - "design-studio tooling becomes unavailable or its output quality is insufficient to compare tracks"
---
# Webui design authority: calibrated instrument, UX-first, deterministic quality

## Context

`cairn.ui` carried fourteen accepted decisions. Five of them formed one lineage
about how the webui's design is driven: how the aesthetic call was to be made,
what it resolved to, how design quality is measured, and what tooling is
forbidden inside that loop. Read as five independent authorities they obscured
the single contract they now express, and each restated the others to stay
coherent.

This decision consolidates that lineage without changing behaviour or reopening
anything. The deterministic surface gates stay separate and accepted:
`dec.webui-design-token-gate`, `dec.webui-a11y-static-audit-gate`,
`dec.landing-design-token-conformance`, and `dec.marketing-visual-world`.

## Decision

### 1. Aesthetic direction: Calibrated Instrument

The webui's aesthetic is the refined current direction expressed as "Calibrated
Instrument", not a geological metaphor rebuild. The geological vocabulary
survives as a state-clarity motif: Strata Survey's node-state treatments (ghost
and orphaned styling, selection emphasis) are adopted into the design system.

The full geological metaphor is declined on evidence, not taste: it lost narrowly
greenfield (zone-rubric 124/160 against 126/160) and its best contribution was
adopted anyway. "Refine, do not redesign" from the review lane is narrowed to the
product's identity: name, voice, and the instrument framing.

### 2. How that call was made, and how the next one is made

An aesthetic call is not made from taste alone. It is made by running two
parallel design-studio tracks and comparing them on a shared zone rubric: Track
A greenfield-simulated (a worktree with `src/ui_assets` and `docs/design-system`
removed, a project brief, frozen real `map.json` and API fixtures, design agents
running the full create loop under context denial so existing assets cannot
anchor the output) and
Track B iterate-current (a review-lane audit of the live webui proposing a
polish direction). The exploration runs on a throwaway fork and never touches
main. Its output enters the repo only through the existing token and component
gates. If the tracks converge the direction ships with evidence; if they diverge
the maintainer picks between two rendered artefacts rather than abstractions.

Cross-track numbers are directional, not cardinal: the scales and the evaluators
differ.

### 3. Design quality is a measurable axis

Design quality is a first-class measurable axis of the webui, pursued as four
bets in the order D, B, C, A:

- **D**: a deterministic design-quality scorer in the autoresearch benchmark.
  Measurable proxies (severity is colour-encoded, the graph uses two or more
  layout dimensions, dead-zone ratio, brand-tone lexicon in copy, motion-safe
  affordance density), each saturated so it cannot be gamed by stuffing, paired
  with a mandatory visual-verification step. Committing the scorer as the
  benchmark's new baseline goes through a segment bump
  (`init_experiment new_segment: true`), never an ordinary keep.
- **B**: severity and drift encoded visually. Colour-coded finding cards, an
  error-versus-warning escalation, drift felt on the node it points to.
- **C**: the two-chain hinge as a "trace the truth" surface, rendering a
  missing-proof state as a visible gap rather than the quietest pixel.
- **A**: the map becomes a real map, placed on the declared PROVENANCE / HINGE /
  AUTHORITY axis with size, weight, and visible edges.

Bets B and C stay bounded, token-based, and reversible. Bet D is unbuilt and
gated on the visual harness; `todo.webui-design-quality` is its tracker and
records that gate.

### 4. The webui is redesigned UX-first

The webui is redesigned from what cairn is supposed to do (orient in a codebase,
inspect a node, trace provenance into a decision hinge and authority out of it,
review findings and drift, follow changes), not by iterating on the current UI.

- The two greenfield mocks and the landed card-column canvas are scored
  evidence, not the spec. No layout structure is prescribed in advance; the UX
  phase chooses it.
- Two outcomes are settled regardless of the information architecture that
  emerges: a bounded desktop workspace (only designated panels scroll
  internally, no endless page scroll) and whole-graph legibility (the dogfood
  graph readable in one viewport without scrolling).
- The greenfield codified output (`design-dna.md`, `tokens.css`, the skill) is
  seed corpus, reconciled with `docs/design-system/` into one canonical system.
  Token conformance is enforced by the existing token gate
  (`dec.webui-design-token-gate`, `scripts/check-design-tokens.sh`), extended to
  the rebuilt `docs/design-system/components.css` through that gate's own
  parameterised-target mechanism, with no new mechanism. Consumers (webui,
  landing, live reference) follow the reconciled set.
- The implementation is component-based and modular as an outcome, not a mapping
  rule: cohesive responsibilities, explicit data and event interfaces,
  independently testable rendering, built on reusable primitives. It is not
  required to mirror the existing ES-module split.
- The webui stays read-only. Two-way interaction (a person leaving feedback or
  notes for the harness, landing in the `cairn feedback` seam) is an explicit
  future exploration with no acceptance criterion attached, and taking it up is a
  revisit trigger of this decision.
- Public-asset staleness the redesign creates is tracked in
  `todo.ui-asset-refresh`, downstream of the redesign.

### 5. The AI-vision iteration loop stays foreclosed

A browser-to-vision-critique-to-patch-to-reload loop over `src/ui_assets/` is
declined and will not be built. It needs two prerequisites that are the
maintainer's to grant, not the loop's to guess: sanction to add a Node plus
Playwright/puppeteer toolchain to this deliberately `package.json`-less
single-binary Rust repo, and an AI vision provider with its config. A
non-deterministic, paid, network-dependent model inside a gate is the opposite of
how every other cairn gate works (`dec.toolchain-lint-strictness`, and the
local-hooks-over-paid-CI posture).

Reopening it requires a decision superseding this one that records the
maintainer's sanction for both prerequisites. A vision model stays a manual
review aid, never an automated gate. A future deterministic visual-regression
approach (for example a pixel-diff against a checked-in baseline rendered by an
already-present tool) is not precluded; it is a new unit of work.

Clause 5 is why the vision question is not re-derived each session: it was
raised, escalated, and answered.

The five decisions named in `supersedes` are historical detail after this
consolidation. This decision carries their live obligations; their frontmatter is
set to `superseded` so they stay available through history without inflating the
node's binding authority set.

## Rationale

This is the smallest consolidation that reaches the configured accumulation
threshold for `cairn.ui`: five decisions of one lineage become one, leaving ten
accepted. The four surface gates are kept separate because they bind scripts and
non-webui surfaces, and superseding them would move enforcement authority for
work that is not this lineage.

Status-only cleanup was rejected: each superseded decision carried live
obligations, and dropping them to satisfy a counter would have traded a
readability finding for semantic loss.

## Consequences

- `cairn.ui` has ten accepted decisions at the current threshold, so
  `CAIRN_DECISION_ACCUMULATION` no longer fires for it.
- Queries show this decision as the binding summary for webui design and retain
  the five superseded decisions for provenance.
- Historical narrative in archived changes, research, done todos, and the
  changelog keeps naming the superseded ids. Those records describe what was
  authoritative when they were written and are left intact. Live pointers name
  this decision: the `docs/design-system/` README lane table and showcase
  lineage, the `related` edge in `dec.marketing-visual-world`, the
  standing-trigger citation in `dec.revisit-trigger-correlator-deferred`, and the
  `todo.ui-asset-refresh`, `todo.webui-design-quality`, and
  `todo.revisit-trigger-correlator` trackers.
- Future changes to webui design amend or supersede this summary instead of
  adding another narrow accepted record beside it.
