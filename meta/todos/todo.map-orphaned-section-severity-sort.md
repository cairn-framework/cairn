---
node: cairn.kernel.scanner
status: done
created: 2026-07-10
---

# Map Orphaned Section Severity Sort

`write_map` (src/scanner/outputs.rs) emits only `## Synced`, `## Ghost`, and `## Findings`;
orphaned files appear only as flat findings in graph order. Marketing copy and the value
of the first-run experience want: an `## Orphaned` section (NodeState::Orphaned exists in
src/map/graph.rs but is never rendered as a bucket), and findings sorted by severity so
the list reads ranked. Surfaced by an adversarial offer review: shipped copy claimed
"every part marked synced, ghost, or orphaned, plus a ranked list of findings" and had to
be softened to match the binary. Fix the tool, then restore the stronger copy.
