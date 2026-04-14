---
id: dec.dsl-as-current-state
nodes: [cairn.kernel.parser, cairn.kernel.reconciliation, cairn.kernel.query]
status: accepted
date: 2026-04-13
revisited: 2026-04-13
revisit_triggers:
  - "A workflow emerges where decisions legitimately outrank the DSL in operational queries"
  - "Multiple users consistently misread the system because the DSL and active ADRs disagree"
informed_by:
  - type: source
    id: src.review-adversarial-1
---

# dec.dsl-as-current-state: The DSL is current-state truth; decisions are rationale and commitments

## Context

v0.4 implicitly placed decisions above the DSL in the authority hierarchy. This created a practical ambiguity: if an accepted ADR had not yet been reflected in the DSL, was the current architecture what the ADR prescribed or what the DSL described? v0.4 had no clean answer.

## Decision

The DSL is the current architectural truth. An agent asking "what is the system today?" reads the DSL. Decisions are rationale behind the DSL plus commitments to change it.

An accepted decision that has not been reflected in the DSL is a proposal pending implementation. Under the change-directory pattern (see dec.change-directories), such decisions live in change directories until merged. They do not appear in the active ontology.

## Consequences

- The "current state" question has one unambiguous answer.
- Decisions cannot drift silently ahead of the DSL; if the DSL has not been updated, the decision is not in effect.
- The ADR status lifecycle simplifies (see dec.change-directories).
- Agents proposing changes read `cairn rationale <node>` to see the decisions that shaped the current DSL. They do not read active ADRs to infer what the system will become — they read change directories.
