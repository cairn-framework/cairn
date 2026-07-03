---
node: cairn.root
status: open
created: 2026-07-03
---

# Status Active Changes Bug

`cairn status` always reports `active_changes: []` / "Active changes: None",
even when `meta/changes/` has real active change directories. Two call
sites hardcode the empty value instead of calling `changes::discover`:
`src/query_api/handlers/project.rs:31` (`status_json`, the `--json` path)
and `src/cli/render/project.rs` `render_status`'s human branch (the literal
"Active changes:\nNone" string). `cairn changes` itself is correct (uses
`changes::discover`); only `cairn status`'s summary is stale. Found while
investigating why `cairn status` and `cairn changes` disagreed (Phase 0 of
`dec.native-todos-first`); out of scope for that change, tracked here.

Fix: call `changes::discover(root)` in both call sites (mirroring
`render_changes` in `src/cli/render/changes_view.rs`) and surface the
real list (or at least a count) instead of the hardcoded empty array.
