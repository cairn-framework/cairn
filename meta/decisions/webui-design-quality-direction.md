---
id: dec.webui-design-quality-direction
nodes:
  - cairn.ui
status: accepted
date: 2026-06-25
informed_by: [res.webui-design-quality-review]
related: [dec.webui-ai-vision-loop-declined]
revisit_triggers:
  - "The maintainer chooses the A aesthetic direction (refined-current versus full geological metaphor), unblocking the map-layout rebuild"
  - "A design-quality proxy is shown to reward a change a visual review judges worse, requiring the proxy or its saturation to be retuned"
  - "The maintainer sanctions a paid or network vision provider, which would supersede dec.webui-ai-vision-loop-declined and could replace the manual inspect_image step"
---

# Webui design quality: a measurable axis, deterministic and benchmark-scoped

## Context

The autoresearch loop optimising the webui converged to `ux_defect_score = 0`
across an 11-scenario benchmark, yet a grounded design review
(`res.webui-design-quality-review`) found real, repeated defects the metric
cannot see. The cause is structural: every benchmark `WEIGHTS` dimension in
`harness/eval.mjs` is a conformance or layout-integrity check (contrast,
overflow, offscreen, clipped, tap, palette, blank, landmark). None measures
visual hierarchy, severity or state encoding, information density, emotional
tone, or whether the graph functions as a map. The metric proves the UI is not
broken; it never asks whether it is good.

The open question this decision answers: how does cairn improve subjective design
quality without (a) reverting the deterministic-gates posture recorded in
`dec.webui-ai-vision-loop-declined`, or (b) chasing taste with a metric that is
blind to it?

## Decision

1. **Treat design quality as a first-class, measurable axis of the webui, and
   pursue four bets in this recommended order: D, then B, then C, then A.**

2. **D (first): extend the autoresearch benchmark with a deterministic
   design-quality scorer.** Add measurable design proxies (severity is
   colour-encoded; the graph uses two or more layout dimensions; dead-zone ratio;
   brand-tone lexicon in copy; motion-safe affordance density), each saturated so
   it cannot be gamed by stuffing, paired with a mandatory visual-verification
   step. This gives the loop a number for taste so B, C, and A can be optimised
   against it rather than landed on vibes.

3. **B (second, lowest-risk large win): encode severity and drift visually.**
   Colour-coded finding cards, an error-versus-warning escalation, and drift felt
   on the graph node it points to. This is the first defect the new scorer should
   catch and clear.

4. **C (third): make the two-chain hinge a real "trace the truth" surface.** Draw
   the decision-to-proof relationship, and render a missing-proof state ("no
   sources recorded") as a visible gap rather than the quietest pixel.

5. **A (fourth, biggest prize): make the map a real map.** Place nodes on the
   declared PROVENANCE / HINGE / AUTHORITY axis with size and weight encoding and
   visible edges, replacing the single-column list. A carries an unresolved
   aesthetic sub-decision (refined-current versus a full geological "cairn"
   metaphor) that is the maintainer's to make; it is sequenced last so the scorer
   from D can judge whether a new layout is actually better.

## Relationship to the declined vision-loop decision

This does not reverse `dec.webui-ai-vision-loop-declined`. That decision declined
a non-deterministic, paid, network vision model inside a commit gate, and
explicitly did not preclude "a future deterministic visual-regression approach
... a new unit of work". D sits inside that carve-out: the scored metric is
deterministic DOM and pixel proxies computed in the autoresearch benchmark (not a
repo commit gate), driven by headless Chrome (the harness lives on the
autoresearch branch and is not yet merged to main; this decision rules the
approach, not a shipped runtime), with no paid provider and no network. The
vision model (`inspect_image`) stays a manual review aid in the loop, never an
automated gate.

## Risks

- A design-quality proxy can drift from real quality. Mitigation: every proxy is
  saturated (diminishing returns past a threshold) and gated behind mandatory
  visual verification, so the loop cannot win by stuffing one signal.
- Scope creep into a webui rebuild. Mitigation: B and C are bounded, token-based,
  and reversible; A is gated on both the D scorer and an explicit maintainer
  aesthetic call.

## Consequences

- The benchmark gains a design-quality dimension; a future segment bump
  (`init_experiment new_segment: true`) commits the scorer as the new baseline.
- The four bets are tracked in change `webui-design-quality`
  (`meta/changes/webui-design-quality/`).
- Light theme, repointing the webui to the landing or design-system surfaces, and
  any paid or network vision gate remain out of scope.
