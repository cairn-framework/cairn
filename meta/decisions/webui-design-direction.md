---
id: dec.webui-design-direction
nodes:
  - cairn.ui
status: superseded
date: 2026-07-12
informed_by: [res.design-studio-greenfield, res.webui-review-audit]
---

# Webui bet A direction: calibrated instrument, geological state vocabulary as motif

## Context

`dec.webui-design-quality-direction` sequenced bet A ("make the map a real
map") last, gated on an explicit aesthetic call between a refined current
direction and a full geological "cairn" metaphor.
`dec.design-studio-exploration-method` ratified how to make that call: two
parallel design-studio tracks, compared on a shared zone rubric. Both tracks
have now run (2026-07-12).

## Evidence

Track A (greenfield, context-denied, branch `design-studio-greenfield` in the
`cairn-ds-greenfield` worktree, commit 30fcd7f) built both poles as static
HTML mocks rendering the real frozen `map.json`, evaluated in a headless
browser over three iterations:

- Calibrated Instrument (refined technical pole): weighted average 8.0,
  zone-rubric total 126/160.
- Strata Survey (geological pole): weighted average 7.8, zone-rubric total
  124/160, but best-in-class canvas state clarity (9/10) from its state
  vocabulary (dashed "keel" ghosts, tilted ochre-underlined orphans).

Track B (review lane against the current live webui,
`res.webui-review-audit`): verdict ready_with_nits, direction "refine, do not
redesign". Weakest zone is the graph canvas (density 2/5, state clarity 2/5:
near-invisible selection, undifferentiated module cards, unexplained
micro-affordances). Header, inspector, and command surface score 3 to 5.

Scales differ (Track A 1 to 10 per dimension, Track B 1 to 5) and the
evaluators differ, so cross-track numbers are directional, not cardinal. The
directional signal is consistent: the refined-instrument pole wins even in a
greenfield run where the geological metaphor had no incumbency disadvantage,
and the current webui's identity is worth keeping while its canvas state
legibility is the deficit both tracks flag.

## Decision

Bet A's aesthetic direction is **refined current, expressed as the
"Calibrated Instrument" direction**, not a full geological metaphor
rebuild. The geological vocabulary survives as a state-clarity motif, not as
the organising metaphor: adopt Strata Survey's node-state treatments (ghost
and orphaned styling, selection emphasis) into the existing design system.

Implementation priorities, from the shared rubric's weakest zones:

1. Graph canvas state legibility: stronger selection treatment plus
   pan-to-selection, differentiated module cards, an explained affordance
   legend.
2. Dead ends become guidance: findings empty state gets a last-reconciled
   timestamp and a CTA; command palette gets enter hints.
3. Deduplicate the topbar/inspector stat readouts; tidy the mobile topbar.

All changes enter through the existing token and component gates
(`dec.webui-design-token-gate`); the codified greenfield output
(`harness-output/design-dna.md`, `tokens.css` on the exploration branch) is
reference material, not a wholesale token replacement.

## Consequences

- The blocked bet A line in `todo.webui-design-quality.md` is superseded by
  this decision; bets B, C, D tracking stays in that todo.
- `todo.design-studio-exploration` completes on ratification.
- The full geological metaphor is declined with evidence, not taste: it lost
  narrowly (124 vs 126) even greenfield, and its best contribution (state
  vocabulary) is adopted anyway.

revisit_triggers:
  - the adopted state-vocabulary motifs fail the design-quality scorer or
    visual harness once implemented
  - the webui's scope grows beyond read-only exploration, changing the
    instrument framing
