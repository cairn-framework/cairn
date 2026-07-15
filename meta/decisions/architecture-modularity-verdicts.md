---
id: dec.architecture-modularity-verdicts
nodes:
  - cairn.root
status: accepted
date: 2026-07-15
informed_by:
  - res.architecture-modularity-audit
---

# Architecture modularity audit verdicts

## Context

Parallel vibe-mode agents collide on god-modules. The flagship instance is
`src/ui_assets/app.js` (2013 lines, ~15 `useState` cells in one `App()`).
Similar concentration exists in Rust hubs (`src/cli/mod.rs` 2205 lines,
81 commits). Cairn's own map validation and size gate did not catch the
drift: edge checks are endpoint-only, the size gate is `.rs`-only, and
`src/ui_assets` is not a blueprint path. The audit
(res.architecture-modularity-audit, todo.architecture-modularity-audit)
measured coupling, churn, sizes, and parallel-edit boundaries before any
refactor.

## Decision

1. **WebUI state architecture: Option A (feature-local).** Split `app.js` by
   feature module; keep leaf-local state; pass shared boot/selection as props
   or thin context. Do **not** adopt a single global TEA-style Model/Update
   store: measurement shows that consolidating ownership relocates the merge
   hotspot rather than removing it.
2. **Self-guardrails: yes.** Extend the file-size gate beyond `.rs` to cover
   webui paths, and add an oversized-module scan finding so agent-native
   loops see the same class of drift. Full import-graph edge-realisation
   validation is deferred (cost / false-positive risk).
3. **Claim `src/ui_assets` on `cairn.ui`.** Path ownership must include the
   embedded frontend assets.
4. **Sequencing: no pre-split of `cairn.kernel.cli` before first-impression
   work.** Near-term todos (`accept-language-aware-gates`, `per-command-help`,
   `todo-listing`, `neighbourhood-edges`) touch mostly disjoint files. Do not
   start a big-bang architecture refactor that blocks first-impression
   delivery. WebUI multi-agent parallel work should wait on the feature-module
   split; serial single-agent webui work need not.

## Rationale

From res.architecture-modularity-audit measured criteria:

- Parallel-editability favours feature-owned files over a global store.
- `app.js` (27 commits) + `style.css` (21) co-change 14 times; one store file
  would concentrate that further.
- Leaf components already use local `useState`; the missing step is file
  extraction, not centralisation.
- CLI hub is a hotspot (81 commits) but first-impression touchpoints are
  already extracted (`accept.rs`, `render/artefacts.rs`, query handlers).
- Size-gate extension is the highest-ROI self-guardrail (would have failed on
  the flagships today).

## Consequences

- Follow-on todos (not done in the audit): `webui-feature-module-split`,
  `size-gate-non-rust`, `modularity-scan-finding`, `ui-assets-blueprint-path`.
- No framework adoption, no premature state centralisation, no production
  rewrite of `cli/mod.rs` as a precondition.
- Optional later: extract `cli/mod.rs` tests to shrink hub merge surface;
  break local cycles (`artefacts`/`map`, `query_api`/`summariser`,
  `reconcile`/`scanner`) when those modules next change.
