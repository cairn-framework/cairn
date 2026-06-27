---
id: res.webui-design-quality-review
nodes:
  - cairn.ui
date: 2026-06-25
method: primary
---

# Webui design-quality review and the eval-coverage gap

## Setup

The autoresearch loop optimising the webui (`ux_defect_score`) had converged to
0 across an 11-scenario benchmark. A grounded design review was run to answer a
separate question the metric never asked: is the UI actually *good*, not just
*not broken*? The review read the 8 live webui states captured by benchmark run
#185 (overview, node inspector, decision detail, findings drawer, command
palette, blueprint modal, ghost/orphaned node, mobile portrait) through a vision
model, with a design-critique prompt per view (first-impression, visual
hierarchy and cognitive load, emotional tone, weak spots, one strength) and an
explicit flag for anything not judgeable from a static frame.

## The eval-coverage gap (the core finding)

The benchmark `WEIGHTS` in `harness/eval.mjs` are entirely conformance and
layout-integrity checks: HTML contrast (3), SVG-label contrast (3), overflow (5),
offscreen (2), clipped text (2), tap targets (1), design-token palette (1),
blank screen (50), missing landmark (40). Not one dimension measures visual
hierarchy, severity or state encoding, information density, cognitive load,
emotional tone, aesthetic distinctiveness, information architecture, or whether
the graph functions as a map.

This was a deliberate choice to keep the gate deterministic and un-gameable, but
it has a consequence: the loop can sit at `ux_defect_score = 0` while a design
review still finds real, repeated problems. The metric is colour-blind to taste.
It proves the UI is not broken; it never asks whether it is good. The
segment-4 functional-state coverage moved closer ("did the decision/finding/ghost
element render") but is still "did it render", not "is it well-designed".

## Findings (grounded, recurred across 3+ views)

1. **The map is a list cosplaying as a map.** Nodes are a uniform single-column
   stack of near-clones. The PROVENANCE / HINGE / AUTHORITY axis is labelled but
   empty: every node sits in lane one, leaving large dead zones left and right.
   (overview, inspector, ghost)

2. **Severity has no visual language.** ERROR and WARNING finding cards are
   identical: same chip, same weight, no colour-coded artery, no escalation. You
   cannot triage by scanning, and drift is not felt on the graph node it points
   to. A contract-hash mismatch whispers. (findings, ghost)

3. **The two-chain hinge, the headline promise, reads decorative.** It is
   expressed twice (a graph axis and a small widget), neither complete; no line
   is drawn from decision to proof; "no sources recorded", the most important
   provenance fact, is the quietest pixel on screen. (decision-detail)

4. **Mobile is a desktop in a trench coat.** Graph cards clip both edges and the
   header juggles five controls at 390px. Only the summary block reads
   mobile-native. (mobile-portrait)

5. **Overloaded amber and unexplained gauges.** Dependency bars, status dots, and
   the provenance/authority slider carry no legend; amber does double duty across
   the ghost pill, warning state, CLI badge, and axis ticks, diluting its
   meaning. Signals repeat (SYNCED appears three times; findings show in both the
   rail and the drawer).

## Consistent strength

The typographic system (Source Serif 4 for headings, IBM Plex Mono for technical
labels, on stone-dark surfaces) is genuinely premium and distinctive. The inline
blueprint-source view grounds abstract nodes in real, verifiable truth. These are
the assets to build on.

## Honesty caveat

Several "feels flat or static" reads depend on motion, hover, and selection
states that a static frame cannot show. The composition, encoding, and hierarchy
findings above are static-safe. The "alive versus dead" verdict is partly
contingent on interaction that was not exercised.

## Implication

Closing the gap is possible deterministically: measurable design proxies
(severity colour-encoded? graph uses two or more layout dimensions? dead-zone
ratio? brand-tone lexicon in copy? motion-safe affordance density?) can be scored
in the benchmark harness, each saturated so it cannot be stuffed, paired with a
mandatory visual-verification step. That is the unlock that lets the loop
optimise quality instead of plateauing at conformance.
