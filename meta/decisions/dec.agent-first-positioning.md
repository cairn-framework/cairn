---
id: dec.agent-first-positioning
nodes:
  - cairn.root
status: accepted
date: 2026-07-10
informed_by:
  - res.messaging-workshop
---
# Agent First Positioning

## Context

Real user feedback (reddit DM, 2026-06-28, quoted in res.messaging-workshop)
showed the public messaging read as "you must hand-write and maintain
blueprints, contracts, and architecture", an ROI wall for the actual target
audience. Lived usage (hologlyph) shows the opposite model: the coding agent
authors and maintains the map conversationally; the developer never opens the
blueprint. Market research found the pain vocabulary ("goldfish memory",
"starts every session blind", "no system of record", "rules in prompts are
requests; hooks in code are laws") and an empty competitive slot: a free,
repo-native, bidirectionally reconciled architecture map that is itself the
enforceable truth.

## Decision

Cairn's public messaging positions the coding agent as the primary author and
maintainer of the map. The developer converses; the agent writes the
blueprint, contracts, and decisions; Cairn reconciles the map against real
code both ways and gates drift. Hand-authoring remains supported but is the
secondary path, never the pitch. The category-of-one framing is
falsifiability: the map can be proven wrong, which is why it can be trusted.

Supporting rulings, all shipped in the README and landing rewrite:

1. Named guarantees over generic reassurance: the Clean Exit Guarantee and
   the Nothing-Leaves-Your-Machine Guarantee. Terms must stay factually
   complete against what `cairn init` actually creates.
2. No scarcity or urgency devices in copy; fake scarcity destroys developer
   trust.
3. High-Value Leader stance: never lead with "free".
4. Copy must match binary behaviour exactly. Overclaims found in review are
   softened in copy and filed as product todos rather than shipped
   (todo.map-orphaned-section-severity-sort, todo.brownfield-one-step-first-map,
   todo.init-wire-agents-md-flag).

## Rationale

The agent-first model matches how the maintainer and the target market
actually use the tool. The value equation (res.messaging-workshop) scores
weakest on perceived likelihood; honest proof (dogfooding, real usage logs,
falsifiable claims) is the only likelihood lever available to a free tool
whose price is trust. Terminology and copy that contradict the binary spend
that trust.

## Consequences

- Every public surface (README, landing, future docs) frames the agent as
  author; wording that asks the developer to maintain artefacts by hand is a
  regression.
- Marketing claims are gated on tool behaviour; strengthening a claim
  requires shipping the matching capability first.
- The messaging research brief (res.messaging-workshop) is the living
  document for future copy rounds; personas evolve there after each
  adversarial panel.

revisit_triggers:
- A third-party case study or testimonial corpus materialises, changing the
  proof strategy.
- The brownfield first-run gap closes (one-step first map), allowing stronger
  time-to-value claims.
- A competitor occupies the free bidirectional-reconciliation slot.
