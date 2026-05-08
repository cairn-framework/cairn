---
id: dec.module-path-mapping
nodes: [cairn.kernel, cairn.kernel.reconciler, cairn.kernel.scanner, cairn.code-reconciler]
status: accepted
date: 2026-05-08
revisited: 2026-05-08
revisit_triggers:
  - "src/ layout changes (a kernel/ subdirectory is reintroduced, or a top-level kernel module moves)"
  - "A second concrete reconciler (deploy, infra, schema) is added and forces a reconsideration of the reconciler/scanner split"
  - "CAIRN spec replaces the reconciler/scanner/scan taxonomy with different terms"
---

# dec.module-path-mapping: Bootstrap blueprint paths and reconciler taxonomy

## Context

The bootstrap fixture is "Cairn describing Cairn." Its blueprint must mirror the actual `src/` layout, otherwise the fixture cannot reconcile against itself and cannot serve as a reference example.

The original blueprint declared paths under `./src/kernel/*` and a `CodeReconciler` under `./src/reconcilers/code`. Neither directory exists. The kernel modules sit at `src/blueprint/`, `src/artefacts/`, `src/scanner/`, `src/changes/`, `src/hooks/`, `src/query_api/`, `src/cli/`. The code reconciler lives at `src/reconcile/`. The blueprint also used "Reconciliation" for the join engine, which collides with the "Reconciler" stem and contradicts the canonical taxonomy in `CLAUDE.md` and `openspec/specs/terminology-rename`.

## Decision

1. **Container `Kernel` keeps its grouping but drops its `path` declaration.** The Kernel is a logical label that says "these eight modules form the domain-agnostic core." It is not a directory. Modules under it carry their own paths to actual `src/` subdirectories.

2. **Three module names are aligned to the canonical taxonomy** (reconciler = pluggable interface, scanner = engine, scan = verb):
   - `cairn.kernel.reconciler` ("Reconciler") is the abstract contract a reconciler must satisfy. No on-disk path. The trait file lives in the directory owned by the concrete CodeReconciler module.
   - `cairn.kernel.scanner` ("Scanner", renamed from "Reconciliation") is the engine that runs all reconcilers and assembles their output into the map. Path: `./src/scanner`.
   - `cairn.code-reconciler` ("CodeReconciler") is the concrete reconciler for source code. Path: `./src/reconcile` (the directory containing the trait in `mod.rs` and per-language reconcilers in `code.rs`, `go.rs`, `python.rs`, `typescript.rs`).

3. **`Reconciliation` was renamed to `Scanner`** to remove the shared "reconcil-" stem with `Reconciler` and `CodeReconciler`. Three distinct stems read more clearly to humans and to AI agents reading the blueprint.

4. **`ReconcilerInterface` was renamed to `Reconciler`.** The original carried an "Interface" suffix to distinguish from "CodeReconciler"; once `Reconciliation` was renamed to `Scanner` the collision was resolved and the canonical short name suffices.

5. **A future reconciler** (deploy, infra, schema) registers as a sibling of `cairn.code-reconciler`, all implementing the path-less `cairn.kernel.reconciler` interface.

## Consequences

- The `Kernel` container has no `path` declaration. The bootstrap parser must permit path-less containers (this matches CAIRN's container semantics: a container is a grouping, not necessarily a directory).
- Path strings on every kernel Module match real `src/` directories, so the scanner produces no orphaned-file findings against kernel code.
- Existing decision artefacts that cited `cairn.kernel.reconciliation` (`dec.change-directories`, `dec.contradiction-classes`, `dec.two-chain-authority`, `dec.dsl-as-current-state`) updated their `nodes:` list to `cairn.kernel.scanner`.
- Contract pointers on the kernel modules still point at non-existent files. That gap is the subject of issue #52 (contract artefacts missing) and is intentionally out of scope here.
