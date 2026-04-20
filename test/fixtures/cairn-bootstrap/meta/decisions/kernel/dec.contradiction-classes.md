---
id: dec.contradiction-classes
nodes: [cairn.kernel.reconciliation, cairn.kernel.hooks]
status: accepted
date: 2026-04-13
revisited: 2026-04-13
revisit_triggers:
  - "A real-world pattern emerges that is neither clearly mechanical nor clearly advisory"
  - "Projects consistently disagree with the default severity of a specific finding type"
informed_by:
  - type: source
    id: src.review-adversarial-1
---

# dec.contradiction-classes: Split contradiction into three severity classes

## Context

v0.4 used "contradiction" as a single overloaded term covering orphaned files, broken artefact pointers, interface drift, and citation issues. This conflated findings with different enforcement characteristics. Some are mechanically unambiguous (file not claimed by any module). Some are mechanical but resolvable in two ways (interface hash drift: update contract or revert code). Some are not mechanical at all (research not linked from any decision).

Presenting them uniformly meant either blocking too aggressively or warning too quietly.

## Decision

Three classes:

- **Structural errors.** Unambiguous mechanical violations (duplicate IDs, path ties, broken pointers, invalid ID references, source checksum mismatch, orphaned files under claimed containers). Block commits unconditionally.
- **Interface contradictions.** Mechanical but resolvable (module interface hash differs from recorded hash). Block commits until explicitly resolved by updating the contract or reverting the change.
- **Rationale tensions.** Advisory findings in the provenance chain (orphan research, decision citing deleted research, revisit trigger matched by recent changes). Surface in `cairn lint` and `map.md`, but never block.

Only the first two are called "contradictions." The third is a "tension" — the framework draws attention without making a correctness claim.

## Consequences

- Enforcement strictness matches the framework's actual capability per class.
- Agents receive clearer signal about what must be fixed versus what warrants attention.
- `cairn lint` groups findings by class for scannability.
- The word "contradiction" becomes precise rather than umbrella.
