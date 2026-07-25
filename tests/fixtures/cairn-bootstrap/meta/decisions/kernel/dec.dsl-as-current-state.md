---
id: dec.blueprint-as-current-state
nodes: [cairn.kernel.parser, cairn.kernel.scanner, cairn.kernel.query]
status: accepted
date: 2026-04-13
revisited: 2026-04-13
revisit_triggers:
  - "A workflow emerges where decisions legitimately outrank the blueprint in operational queries"
  - "Multiple users consistently misread the system because the blueprint and active ADRs disagree"
informed_by:
  - type: source
    id: src.review-adversarial-1
---

# dec.blueprint-as-current-state: The blueprint is current-state truth; decisions are rationale and commitments

## Context

v0.4 implicitly placed decisions above the blueprint in the authority hierarchy. This created a practical ambiguity: if an accepted ADR had not yet been reflected in the blueprint, was the current architecture what the ADR prescribed or what the blueprint described? v0.4 had no clean answer.

## Decision

The blueprint is the current architectural truth. An agent asking "what is the system today?" reads the blueprint. Decisions are rationale behind the blueprint plus commitments to change it.

An accepted decision that has not been reflected in the blueprint is a proposal pending implementation. Under the change-directory pattern (see dec.change-directories), such decisions live in change directories until merged. They do not appear in the active map.

## Consequences

- The "current state" question has one unambiguous answer.
- Decisions cannot drift silently ahead of the blueprint; if the blueprint has not been updated, the decision is not in effect.
- The ADR status lifecycle simplifies (see dec.change-directories).
- Agents proposing changes read `cairn rationale <node>` to see the decisions that shaped the current blueprint. They do not read active ADRs to infer what the system will become — they read change directories.
