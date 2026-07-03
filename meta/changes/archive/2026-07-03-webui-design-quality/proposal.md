# Proposal: Webui design-quality

## Motivation

The autoresearch loop optimising the webui converged to `ux_defect_score = 0`,
but a grounded design review (`res.webui-design-quality-review`) found real,
repeated defects the benchmark cannot see. Every benchmark weight is a
conformance or layout-integrity check; none measures visual hierarchy, severity
encoding, information density, emotional tone, or whether the graph functions as
a map. The loop has hit the ceiling of what its metric can drive: the UI is not
broken, but it is flat and list-shaped, and its headline "two-chain, trace the
proof" promise reads decorative.

`dec.webui-design-quality-direction` rules to treat design quality as a
measurable axis and pursue four bets in a recommended order. This change tracks
that work.

## Scope

In recommended order:

- **D. Design-quality scorer (first).** Extend the autoresearch benchmark
  (`harness/`) with deterministic, saturated design proxies plus mandatory visual
  verification, so the loop can optimise quality, not just conformance.
- **B. Severity and drift encoding (second).** Colour-code finding cards by
  severity, escalate error versus warning, and surface drift on the graph node.
- **C. Trace-the-truth hinge (third).** Draw the decision-to-proof relationship
  in the decision detail; render missing proof as a visible gap.
- **A. Map becomes a map (fourth).** Place nodes on the declared
  PROVENANCE / HINGE / AUTHORITY axis with size and weight encoding and visible
  edges, replacing the single-column list.

## Out of scope

- Light theme (net-new feature, fixes no reviewed defect).
- Repointing the webui to the landing page or design-system surfaces (scope
  pivot; can be reviewed separately).
- Any paid or network vision model inside a commit gate (precluded by
  `dec.webui-ai-vision-loop-declined`; the D scorer stays deterministic and
  benchmark-scoped).
- Blueprint structural changes: `cairn.ui` already owns the webui, so these are
  modifications, not new nodes.
- The A aesthetic direction (refined-current versus full geological metaphor):
  the maintainer's call, recorded as a revisit trigger on the decision.

## Acceptance criteria

- The benchmark reports a design-quality score with at least the proxies in
  `dec.webui-design-quality-direction` item 2, each saturated.
- A manual visual verification step is recorded separately in the loop (not
  machine-testable; it is a human checkpoint, not an automated gate).
- Finding cards are visually distinguishable by severity at a glance, and a node
  with a finding is distinguishable on the graph.
- The decision detail draws a decision-to-proof relationship and shows a distinct
  missing-proof state.
- Nodes render on two or more layout dimensions with visible edges (A), once the
  aesthetic direction is chosen.
- The existing conformance metric stays at `ux_defect_score = 0` (no visual
  regression), and `scripts/check-design-tokens.sh` and `scripts/check-a11y.sh`
  stay green.
