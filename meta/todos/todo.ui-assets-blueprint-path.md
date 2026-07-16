---
node: cairn.ui
status: done
created: 2026-07-15
related: [todo.architecture-modularity-audit, todo.webui-feature-module-split, todo.size-gate-non-rust, dec.claim-only-assets-targets]
---

# Claim src/ui_assets on cairn.ui blueprint path


## Problem

Blueprint node `cairn.ui` claims only `path "./src/ui"`. The served frontend
lives in `src/ui_assets/` and is embedded via `include_str!` from
`src/ui/mod.rs:26-41` (`app.js`, `style.css`, `index.html`, vendor). Those
files are invisible to:

- `cairn files cairn.ui` (returns only `src/ui/{mod,server,wire}.rs`),
- path-ownership / orphan machinery (`cairn onboard` reports full coverage
  while the flagship assets are unclaimed),
- any future size or modularity gate that keys off blueprint paths.

## Evidence (res.architecture-modularity-audit, 2026-07-15)

- `cairn.blueprint` Module UI block: `path "./src/ui"` only.
- Embed sites: `src/ui/mod.rs` constants `INDEX_HTML`, `APP_JS`,
  `UI_STYLE_CSS`, vendor Preact/htm includes.
- `cairn files cairn.ui --json` lists three Rust files; no ui_assets entries.
- Non-Rust assets sit outside the code reconciler's orphan model, so missing
  path claims do not produce findings today.

## Approach (backlog only)

1. Add a blueprint `path` for `./src/ui_assets` (or explicit file paths) under
   `cairn.ui`, with a short decision if path semantics for non-code assets
   need ratifying.
2. Confirm `cairn files cairn.ui` lists the assets and that scan stays clean.
3. Coordinate with todo.size-gate-non-rust so the gate can optionally key off
   claimed paths rather than a hard-coded `ui_assets` glob.
4. Do not move files; ownership claim only.

## Priority

Small, enabling. Land before or with the size-gate extension and webui split
so ownership, gates, and modules agree on where the frontend lives.

## Resolution (2026-07-16)

Implemented claim-only assets targets. By setting `language: assets` on `cairn.ui` target `path: ./src/ui_assets` in `cairn.config.yaml`, the scanner claims the assets without parser warnings or symbol-extraction, resolving the blueprint coverage gap.
