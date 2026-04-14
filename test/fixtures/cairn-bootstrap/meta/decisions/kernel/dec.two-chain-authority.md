---
id: dec.two-chain-authority
nodes: [cairn.kernel.parser, cairn.kernel.artefacts, cairn.kernel.reconciliation]
status: accepted
date: 2026-04-13
revisited: 2026-04-13
revisit_triggers:
  - "Evidence that humans or agents consistently confuse provenance with authority findings in lint output"
  - "A third chain emerges that is neither pure evidence nor pure enforcement"
informed_by:
  - type: research
    id: res.related-work-survey
  - type: source
    id: src.review-adversarial-1
---

# dec.two-chain-authority: Split the authority hierarchy into two chains

## Context

v0.4 presented a single six-layer hierarchy: source → research → decision → DSL → contract → code. Elegant as a diagram but structurally wrong. A source cannot "win" over a decision in the same way a contract should win over code — they are different kinds of truth. The framework can mechanically verify that code matches a contract; it cannot mechanically verify that research faithfully reflects its sources. Presenting both under one word overclaimed enforcement.

## Decision

Split the stack into two linked chains:

- **Provenance chain:** source → research → decision. Traceability of reasoning. Advisory only.
- **Authority chain:** decision → DSL → contract → code. Enforcement. Mechanical at every link.

The decision layer is the hinge: output of provenance, input to authority.

Findings from each chain surface differently. Provenance issues are rationale tensions (warn, never block). Authority issues are structural errors or interface contradictions (block).

## Consequences

- The framework's enforcement claims now match its actual capability. "A fence, not a proof" maps onto the authority chain only; the provenance chain gets surfacing, not enforcement.
- Artefact types divide cleanly: source, research, decision are provenance artefacts; contract, todo, review are authority artefacts. Decision belongs to both.
- The ADR schema must carry both `informed_by` (provenance obligation, upward) and `nodes` (authority obligation, downward). This is why the decision layer is the hinge.
- The phrase "authority hierarchy" survives only for the authority chain. The provenance chain is a traceability chain.
