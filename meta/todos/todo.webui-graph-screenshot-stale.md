---
node: cairn.ui
status: open
created: 2026-07-03
---

# Webui Graph Screenshot Stale

`docs/assets/screenshots/webui-graph.png` (used as `docs/landing/index.html`'s
og:image/twitter:image social-preview meta tags and as the demo video's
`poster` attribute) still visually shows the SYSTEM node overlapping the
HUD legend that `39c1a4b` fixed (cairn-380): the file was last touched at
76892ba, before the fix landed. Confirmed via `inspect_image` and pixel
dimensions (1690x1768 physical / devicePixelRatio 2 = 845x884 logical,
matching the narrow-viewport repro case the fix targets).

Not fixed here: this needs a properly composed WIDE desktop screenshot
(1440x900-class, 2-column layout) for landing-page/social-preview quality,
which this session's sandboxed browser tool cannot produce (hard-capped
at 800x600 regardless of the requested viewport, confirmed by testing).
Recapture with a real desktop-width browser (or the repo's existing
`docs/assets/demo/record-setup.sh` throwaway-copy pattern plus a
puppeteer/playwright script with an actual controllable viewport) showing
the graph canvas with the SYSTEM node fully clear of all HUD chrome.
