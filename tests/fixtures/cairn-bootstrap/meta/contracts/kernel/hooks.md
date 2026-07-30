---
node: cairn.kernel.hooks
informed_by:
  - type: decision
    id: dec.contradiction-classes
---

# Contract: cairn.kernel.hooks

Hooks gate commits and task boundaries on scan integrity: they classify findings into blocking and advisory channels and exit accordingly.

## Interface

- **Input.** The finding set produced by a scan of the current tree.
- **Output.** A pass or block verdict: exit 0 when no blocking finding stands, exit 1 with the blocking findings listed when one does.
- **Errors.** A scan that cannot run at all is itself a blocking condition; hooks never pass on missing evidence.

## Invariants

- Severity drives classification: mechanical contradictions block, rationale tensions advise, and the mapping is stable per dec.contradiction-classes.
- Hooks are deterministic over a given tree; two runs on identical state return the same verdict.
- A hook never mutates the tree it inspects.

## Out of scope

- Producing findings. The scanner and its reconcilers do; hooks consume them.
- Repairing violations. Hooks report; the operator or an agent fixes.
