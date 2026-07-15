---
node: cairn.root
status: open
created: 2026-07-15
---

# Architecture modularity audit

## Problem

Parallel vibe-mode agents collide on god-modules and monoliths. Unclear coupling makes concurrent edits risky. The flagship instance is the 2,013-line `src/ui_assets/app.js` (Preact + htm + Hooks) with roughly fifteen scattered `useState` cells in one `App()` and no central store or reducer. Similar concentration may exist across Rust `src/`. Without measured boundaries, two agents editing different features still touch the same files and state.

## Approach: measure first, then prescribe

Do not adopt a framework or centralise state up front. Measure the codebase, then choose architecture from evidence. Explicit principle: **avoid premature centralisation**. Consolidating many local cells into one global store can just relocate the merge hotspot.

### 1. Measure

Across Rust `src/` and `src/ui_assets/`:

- Coupling (who imports whom, fan-in/fan-out)
- Change-hotspots (git churn: which files change most often, and together)
- File sizes and god-module candidates
- Parallel-edit boundaries (can two agents edit different features without touching the same file or shared state?)

### 2. Check blueprint conformance

Cairn already models module boundaries as blueprint nodes with `path` ownership and directed edges. Check whether real code conforms:

- Cross-module reach-arounds (code that ignores declared ownership)
- Dependency cycles
- Effect scatter (fs/git/gh/db side effects mixed through pure logic)
- God-modules that claim more than one node's worth of responsibility

### 3. Evaluate against generalisable principles

Judge candidates against clean-architecture / Elm-Architecture **principles** (not a prescribed store shape):

- Explicit state and transitions
- Functional core + effects at the boundary (imperative shell for fs/git/gh/db)
- Unidirectional module dependencies (no cycles)
- Owned modules with clear contracts
- Clear state ownership boundaries (feature-local or global)

### 4. WebUI state architecture: explicit comparison

For the webui flagship, run a head-to-head comparison and pick from measurement. Do **not** pre-commit to either side.

| Option | Shape |
|---|---|
| **(A) Feature-local** | Per-view / per-feature reducers or small state machines, explicit effect boundaries, modules owned by feature |
| **(B) Global TEA-style** | Single app-level Model/Update store |

Judge both on measurable criteria:

1. **Parallel-editability**: can two agents edit different features without touching the same file or shared state cell?
2. **Coupling**: fan-in of the state owner; how many call sites break when one feature changes?
3. **Change-hotspot risk**: does the chosen shape concentrate git churn into one file?

Do not prescribe "consolidate the ~15 `useState` into one reducer" or "one app-level store" as the answer. The audit may conclude that feature-local ownership is safer for parallel agents, or that a carefully scoped global store wins on some criteria; the evidence decides.

## Root cause and self-guardrail

Meta-question: cairn dogfoods cairn, so why did this architectural drift happen?

Answer:

1. **Cairn's map validation is incomplete relative to modularity.** It checks implemented claims: node/path existence, file claims, and that edge *endpoints* exist (`src/map/build.rs::validate_edges` only verifies endpoints exist). It does **not** prove that a declared edge is *realised* in code (actual import/call dependency), and it has **no signal for intra-module health** (size, coupling, god-modules). A node can own a 2,000-line file and still scan clean; a declared edge can be an unrealised aspiration and still scan clean. That edge-realisability validation gap is an additional reason architectural drift went uncaught (alongside the missing intra-module health signal).
2. **The file-size gate is incomplete.** `scripts/check-file-sizes.sh` only checks `src/**/*.rs`, so the webui JS (`app.js` at 2,013 lines) was exempt and grew unchecked.

### Audit outcome: make cairn catch this itself

Do not treat this as a one-off cleanup. As an explicit audit outcome, evaluate whether cairn itself should emit a modularity / size signal so this class of drift is caught automatically in future. Candidates to measure and decide on (outcome-neutral; evidence picks):

- Extend the size gate beyond `.rs` to cover the webui (and any other non-Rust owned paths).
- Add a coupling / oversized-module finding (or similar scan signal) that fires when a node-owned module exceeds measured health thresholds.

Frame the recommendation as "make cairn catch this itself", with a concrete proposal only if measurement supports it.

## Sequencing

The audit's **measurement phase** (coupling, git change-hotspots, file sizes, parallel-edit boundaries) is cheap and can run **early**. One of its jobs is to answer whether any module that the near-term first-impression work touches is a parallelism bottleneck worth a small targeted split **before** parallelising that work.

Near-term touchpoints to check early (non-exhaustive):

- `cairn.kernel.cli` (several first-batch todos hit this node)
- The webui flagship (`src/ui_assets/app.js`) if any first-impression work lands there

**Caution:** do not let a big-bang architecture refactor block first-impression work. Refactor surgically only where measurement proves a split unblocks concrete near-term parallel work. If measurement shows a module is fine for the planned parallel slices, leave it and continue.

## Output / acceptance

1. A coupling and hotspot report covering Rust `src/` and the webui.
2. A prioritised modularisation backlog (each item becomes its own follow-on todo).
3. An evidence-based recommendation for webui state architecture, documenting the A-vs-B comparison and the measured criteria that decided it. Explicitly note the flagship `app.js` instance and that a single global store is not automatically more parallel-friendly.
4. A self-guardrail recommendation: whether (and how) cairn should emit modularity / size signals so this class of drift is caught automatically (extend size gate, coupling/oversized-module finding, or neither if measurement says the cost is not worth it).
5. A short sequencing note: which near-term modules (if any) need a surgical split before parallel first-impression work, and which do not.

## Non-goals

- Do not perform the refactors here (audit and backlog only).
- Do not pre-adopt any framework.
- Do not prescribe premature centralisation of state.
