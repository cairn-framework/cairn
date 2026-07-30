---
node: cairn.kernel.changes
informed_by:
  - type: decision
    id: dec.change-directories
---

# Contract: cairn.kernel.changes

The Changes module owns change directories: proposal-scoped deltas staged against the current blueprint, validated before archive.

## Interface

- **Input.** A change directory holding a proposal and per-node delta files (ADDED, MODIFIED, REMOVED, RENAMED semantics).
- **Output.** A validated delta set, and on archive a permanent record of the applied change.
- **Errors.** A delta referencing an unknown node, or an archive attempted against a failing scan, surfaces as a blocking finding.

## Invariants

- A change is validated against the scanner's view before it may archive.
- Archived changes are immutable history; the module never rewrites them.
- Delta semantics are explicit: every touched node states which of the four verbs applies.

## Out of scope

- Running the scan itself. Changes calls into the scanner and consumes its verdict.
- Editing the blueprint. Applying a change is the operator's act; the module validates and records it.
