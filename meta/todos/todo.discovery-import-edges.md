---
node: cairn.brownfield
status: done
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

## Done (2026-07-12)

Implemented in `src/brownfield/imports.rs` (tree-sitter import extraction
for Rust/TS/JS/Python/Go plus edge derivation) and wired into
`discover()`. Edges are emitted only between co-discovered candidates,
in import direction, with confidence scaled by import count; relative
JS/TS references resolve by path, name matches skip ambiguous directory
names rather than guess. Verified: the brownfield fixture yields exactly
`src.api -> src.auth` and `src.auth -> src.db`, no cycle, `cairn scan`
exit 0 after `change apply`. Unit tests cover each language's extractor;
discovery tests mirror the fixture, relative-import resolution, and
ambiguity skipping.
