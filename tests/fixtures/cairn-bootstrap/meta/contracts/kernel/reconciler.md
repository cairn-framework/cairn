---
node: cairn.kernel.reconciler
informed_by:
  - type: decision
    id: dec.module-path-mapping
  - type: decision
    id: dec.two-chain-authority
---

# Contract: cairn.kernel.reconciler

The Reconciler is the abstract contract that any concrete reconciler (`code`, future `deploy`, future `infra`, future `schema`) must satisfy. The kernel does not implement it; the kernel registers and invokes implementations via the scanner.

## Interface (Rust trait, defined in `src/reconcile/mod.rs`)

A reconciler MUST:

- Expose a unique `ReconcilerId` so the scanner can register and dispatch to it.
- Implement `reconcile`, accepting the relevant slice of the parsed blueprint plus the targets it claims, and producing a list of reconciliation outcomes per target, classifying each as `synced`, `ghost`, or `orphaned`.
- Surface contradictions and tensions as `Finding` values with a stable error code drawn from the error-code registry.
- Be idempotent: running twice over unchanged input produces identical output.

## Invariants

- A reconciler MUST NOT modify the blueprint or any authority artefact. Read-only input, finding-only output.
- A reconciler MUST emit deterministic output (sorted, no embedded timestamps) so the scanner can hash-compare runs.
- A reconciler MUST surface its own classification per target. Cross-target severity escalation is the scanner's responsibility, not the reconciler's.
- Each `ReconcilerId` MUST be unique within a registry.

## Out of scope

- Map assembly. The scanner aggregates reconciler outputs into the final map.
- Schema validation of the incoming blueprint. The parser owns syntax; the scanner owns reference integrity.
- Error-code allocation. Codes are assigned through the registry; reconcilers select existing codes or propose new ones via spec amendment.
