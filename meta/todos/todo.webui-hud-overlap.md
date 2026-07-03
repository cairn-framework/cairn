---
node: cairn.ui
status: open
created: 2026-07-03
---

# Webui Hud Overlap

Pre-existing cosmetic issue (confirmed present in the previously-committed
docs/assets/screenshots/webui-graph.png, not introduced by any recent
change): on small graphs the auto-laid-out SYSTEM root node box renders
under the PROVENANCE/HINGE/AUTHORITY legend bar in the graph canvas
overlay at 1440x900. Needs a GraphCanvas layout fix (either push default
node placement below the fixed HUD row, or raise the HUD's z-index/
background so it fully occludes rather than blends).

bd:cairn-380
