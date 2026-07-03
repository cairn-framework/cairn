---
node: cairn.ui
status: done
created: 2026-07-03
---

# Webui Hud Overlap

Confirmed root cause: the docs/assets/screenshots/webui-graph.png capture
is ~845 logical px wide (1690x1768 physical, devicePixelRatio 2), just
under the `@media (max-width: 860px)` breakpoint where `.main` switches
to a single-column, `grid-template-rows: auto auto` stack. `.graph-canvas`
has no in-flow children (graph-bg, chain-banner, graph-svg, graph-zoom,
graph-minimap, graph-legend are all `position: absolute`), so against an
`auto` row it collapsed toward 0px height, squeezing the fixed-position
SYSTEM node and the bottom-anchored zoom/legend docks into the same few
pixels. Not reproducible at 1440x900 (2-column layout, no row-stacking;
the original bead's "at 1440x900" was inaccurate). Fixed with a
`min-height: 40vh` floor on `.graph-canvas` inside that media query,
mirroring `.inspector-wrap`'s existing `max-height: 60vh` cap in the same
block. Verified: canvas height 92px -> 240px at the reproducing width, no
overlap between the SYSTEM node (y 123-189) and the zoom/legend docks
(top >= 234) at zoom=1 after `fit()`; screenshot confirms the SYSTEM node
box fully clear and legible. The >860px desktop layout is untouched
(media-query-scoped change; no `.graph-canvas` height rule exists outside
it).

bd:cairn-380
