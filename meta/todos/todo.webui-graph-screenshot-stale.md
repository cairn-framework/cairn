---
node: cairn.ui
status: done
created: 2026-07-03
---

# Webui Graph Screenshot Stale

`docs/assets/screenshots/webui-graph.png` (used as `docs/landing/index.html`'s
og:image/twitter:image social-preview meta tags and as the demo video's
`poster` attribute) still visually showed the SYSTEM node overlapping the
HUD legend that `39c1a4b` fixed (cairn-380): the file was last touched at
76892ba, before the fix landed. Confirmed via `inspect_image` and pixel
dimensions (1690x1768 physical / devicePixelRatio 2 = 845x884 logical,
matching the narrow-viewport repro case the fix targets).


## Resolution (2026-07-09)

Recaptured on the macOS workstation using Playwright (Chromium) in a throwaway
temp directory outside the worktree (`/tmp/cairn-screenshot-playwright`), with
its own `package.json`, so npm did not walk up into the repo.

Capture settings:
- Viewport: 1440x900 logical, deviceScaleFactor 2 (2880x1800 physical).
- Target: `target/release/cairn ui` serving the worktree repo on a free port
  (127.0.0.1:50648, 25 nodes, 27 edges, 1 info finding).
- Layout: 2-column desktop layout, graph canvas left, inspector right.
- Graph state: waited for `.graph-svg .canvas-node`, triggered the `fit` control
  to settle the default viewport, then captured after 800ms.

Verification:
- Pixel dimensions: 2880x1800 physical (1440x900 logical), as confirmed by
  `file` and `sips`.
- Visual inspection via `inspect_image`: 2-column desktop layout, SYSTEM node
  fully clear of the top chain banner and all HUD chrome, no node overlaps
  the zoom/legend/minimap docks, text legible. The previous narrow-viewport
  SYSTEM/legend overlap is gone.
- Landing-page references checked: `docs/landing/index.html` og:image and
  twitter:image both point to `https://cairn-framework.github.io/cairn/assets/screenshots/webui-graph.png`;
  no HTML edits were required.
- Asset validity confirmed by `cairn lint` and `cairn hook all` (only a
  pre-existing CAIRN_SPEC_RULE_UNIMPLEMENTED info finding, unrelated to this
  change).