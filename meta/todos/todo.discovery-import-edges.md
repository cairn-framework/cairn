---
node: cairn.brownfield
status: open
created: 2026-07-10
---

# Discovery Import Edges

`cairn init --from-code` discovery now proposes nodes only. The previous
behaviour fabricated all-pairs bidirectional "sibling module" dependency
edges between co-located directories, which guaranteed a false
CAIRN_ORDER_CYCLE error on the first scan of any repo with two or more
modules under one parent. Removed 2026-07-10.

The real feature this leaves open: derive directed dependency edges from
observed imports during discovery. The tree-sitter extractors already
parse the source; walk `use crate::x` (Rust), import statements (TS/JS,
Python, Go), map them to discovered candidate paths, and emit only edges
with actual code evidence, in import direction. Confidence can reflect
import count. Suggested (unverified) relations stay in
`suggested-edges.json` per docs/brownfield.md; the blueprint delta only
gets observed edges.

Acceptance: the brownfield fixture in docs/assets/demo/brownfield-setup.sh
(api imports auth, auth imports db) yields api -> auth and auth -> db
edges, no cycle, `cairn scan` exit 0.
