---
node: cairn.ui
status: blocked
created: 2026-07-18
---

# UI Asset Refresh

Refresh every public asset that bakes in the webui's appearance, after
`todo.webui-deck-redesign` lands. Blocked on that todo; doing this
first would capture the old design twice.

## Problem

The webui's look is embedded in assets across the README, the landing
page, and the harness. After the deck redesign they all show a UI that
no longer exists, on surfaces that are the project's first impression
(the og:image is what every link unfurl shows).

## Inventory

- `README.md`: `docs/assets/demo/webui.gif` (webui demo),
  `docs/assets/screenshots/design-system.png` (showcase),
  `docs/assets/screenshots/landing-full.png` (landing screenshot).
- `docs/landing/index.html`: hero video `docs/assets/demo/webui.mp4`
  with poster `docs/assets/screenshots/webui-graph.png`; `og:image`
  and `twitter:image` meta both point at the GitHub Pages URL of
  `webui-graph.png`.
- `docs/assets/screenshots/`: `webui-graph.png`, `landing-full.png`,
  `landing-hero.png`, `design-system.png`.
- `docs/assets/demo/`: `webui.mp4` and `webui.gif` are screen
  recordings with no `.tape` source; re-record manually against the
  redesigned webui. Terminal demos (`tour`, `drift`, `install`,
  `brownfield`) are unaffected.
- Visual harness (`harness/`): re-baseline whatever `eval.mjs` gates
  against the old rendering.
- Landing page styling: verify it still reads coherently against the
  reconciled token set (the redesign todo keeps it conformant; this
  pass judges coherence, not just gate passage).

## Acceptance

- No public surface (README, landing, og/twitter meta, Pages site)
  shows the pre-redesign webui or design system.
- Landing hero video and poster show the deck layout; og:image
  refreshed at the same URL path so existing unfurls update.
- Visual harness green against the new baselines.

dec:dec.webui-deck-layout
