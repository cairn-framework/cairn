---
id: res.vision-refactor-audit
nodes: [cairn.kernel]
date: 2026-07-02
method: primary
---

# Vision refactor audit

## Context

The maintainer's verdict: cairn was repeatedly made too small to live up to its
vision, and the shrinking is why development keeps stalling. Five facts,
gathered directly from the codebase, ground the seven-workstream refactor this
research informs.

## Findings

1. **Structured symbol data currently dies at extraction.** `collect_public_symbols`
   (`src/reconcile/code.rs:282`) walks Tree-sitter nodes and immediately flattens
   each symbol into a single normalised string via `interface_symbol`
   (`src/reconcile/code.rs:300`), fed only into the interface fingerprint hash.
   The identifier name, kind, and source location are read during the walk but
   never survive past that string. `cairn files <node>` returns file paths, not
   symbols.
2. **Reconciliation output is not persisted.** The scanner writes `map.md`
   (prose) and `.cairn/log.md` (append-only event log) on every scan, but the
   full `ReconcileReport`/`ScanResult` (including whatever structured data it
   held) lives only in the gitignored `.cairn/state/reconciler-cache.json`
   cache, which is an optimisation artefact (version-gated, silently
   recomputed on mismatch), not a committed measurement record. There is no
   machine-readable, versioned, committed snapshot of the map.
3. **Contracts are opaque prose.** `Contract` (`src/artefacts/contract.rs:18`)
   has no structured interface field; the reconciler cannot verify a contract's
   claimed public interface against what the code actually exports. Drift
   between "what the contract says a module exposes" and "what the module
   exposes" is undetectable.
4. **`StateBackend` is production-dead.** `storage_backend()`
   (referenced from `src/state/mod.rs`) is called only from
   `src/state/tests.rs`; grep confirms no production call site. The workflow
   logic that is live — `create_change_epic`, `create_task_beads`,
   `list_child_tasks`, `claim_change` on `BeadsStateBackend`
   (`src/state/beads.rs:202,225,265,301`) and their call sites in
   `src/cli/commands/change.rs` — schedules and claims work items, which is
   workflow, not architecture-truth reconciliation.
5. **Direction is check-only.** Every existing query (`get`, `neighbourhood`,
   `files`, `contract`, `order`, ...) answers "does this match declared intent",
   never "here is everything needed to build the declared-but-unbuilt node."
   There is no query that composes contract + decisions + rationale +
   dependency interfaces into a single generation-ready bundle, and no query
   that reports which ghost nodes are buildable now given the state of their
   dependencies.

## Implication

Each finding maps to one ratified workstream: (1) → symbol-level reality layer,
(2) → persistent `map.json`, (3) → structured `interface:` contracts, (5) →
generative direction (`cairn bundle`/`cairn gap`) and the orchestrator surface
(`cairn frontier`), (4) → change-system trim. `dec.no-orchestrator` is not
implicated: `cairn frontier` exposes a traversal query, it does not run
anything, and the trim in (4) removes scheduling machinery rather than adding
it.
