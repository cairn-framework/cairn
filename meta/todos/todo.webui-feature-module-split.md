---
node: cairn.ui
status: open
created: 2026-07-15
related: [todo.architecture-modularity-audit, todo.webui-mobile-graph-nav, todo.webui-simplicity-review, todo.ui-assets-blueprint-path]
---

# WebUI feature-module split (Option A)


## Problem

Parallel agents cannot edit different webui features without colliding on one
file. `src/ui_assets/app.js` is a 2013-line Preact+htm monolith (80 963 bytes)
with 23 `useState` cells (15 in `App()`). `src/ui_assets/style.css` is 2729
lines and co-changes with `app.js` in 14 commits. Feature seams already exist
as section comments (canvas, inspector, findings, palette, blueprint modal,
top bar) but not as files.

## Evidence (res.architecture-modularity-audit, 2026-07-15)

- Line counts: `app.js` 2013, `style.css` 2729 (both outside
  `scripts/check-file-sizes.sh`, which only gates `src/**/*.rs`).
- Churn: `app.js` 27 commits, `style.css` 21; pair co-change count 14.
- State ownership: leaf components already hold local state
  (`GraphCanvas` viewport/pan, `FindingsPanel` scope/category,
  `CommandPalette` query/index, `Section` open, `CopyButton` copied). The
  residual problem is file-level ownership, not missing global control flow.
- A-vs-B verdict in the research artefact: **Option A (feature-local)** wins
  on parallel-editability, state-owner fan-in, and hotspot concentration. A
  single global TEA store would relocate the merge hotspot, not remove it.

## Approach (backlog only; do not implement here)

1. Extract feature modules under `src/ui_assets/` (e.g. `graph-canvas.js`,
   `inspector.js`, `findings-panel.js`, `command-palette.js`,
   `blueprint-modal.js`, `top-bar.js`, shared `utils.js` / `layout.js`).
2. Keep feature-local UI state in those modules; pass shared boot data
   (`graph`, `lint`) and selection as props or a thin context.
3. Do **not** introduce a global Model/Update store or a new state library.
4. Prefer serial extraction (one feature per PR) so each step stays reviewable.
5. CSS: either section-scoped discipline in `style.css` or per-feature CSS
   partials concatenated the same way `src/ui/mod.rs` already concatenates
   tokens + components + `style.css`.

## Sequencing

Required **before** parallel multi-agent webui work. Not required before
serial single-agent webui todos (`webui-mobile-graph-nav`,
`webui-simplicity-review`) if only one agent touches the tree at a time.
Not a blocker for CLI first-impression work.
