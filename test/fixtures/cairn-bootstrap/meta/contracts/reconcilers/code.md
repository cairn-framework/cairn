---
node: cairn.code-reconciler
informed_by:
  - type: decision
    id: dec.module-path-mapping
  - type: decision
    id: dec.contradiction-classes
---

# Contract: cairn.code-reconciler

The CodeReconciler is the concrete reconciler for source-code targets. It implements the Reconciler interface (see `cairn.kernel.reconciler`) for languages currently supported via tree-sitter.

## Interface

- Accepts a path that the blueprint claims a Module owns and a language tag (`rust`, `go`, `python`, `typescript`).
- Walks the path with the appropriate tree-sitter grammar, extracts top-level declarations (functions, types, modules), and emits a fingerprint plus an interface hash.
- Compares the fingerprint to the blueprint-declared module's expectations and emits `synced`, `ghost`, or `orphaned` per file plus a per-module rollup.

## Invariants

- Output is deterministic for fixed input. Symbol names are sorted; the interface hash is stable across runs.
- Files outside the declared path are not considered. Cross-module reachability is the scanner's concern, not the reconciler's.
- Interface hashes are recorded under `.cairn/state/interface-hashes.json`. A mismatch between the recorded hash and the freshly computed hash surfaces as an interface-contradiction finding (the scanner classifies it).
- A path-tie between two modules (overlapping `path` declarations) is reported by the scanner, not by this reconciler. The reconciler treats each path independently.

## Out of scope

- Languages without a registered tree-sitter grammar. Adding a new language requires its own reconciler module under `src/reconcile/<lang>.rs` and a registration line in `src/reconcile/mod.rs`.
- Symbol-level review (semantic equivalence). The reconciler reports interface shape only; the summariser proposes contract updates on contradictions.
- Build-system integration. Cargo manifests, package.json, and equivalents are out of scope for the code reconciler. A future `deploy` or `build` reconciler may consume them.
