---
node: cairn.kernel.map
status: open
created: 2026-07-12
---

# Ghost Module Signal

gh:#238

No signal distinguishes implemented modules from empty-directory ghosts.

## Evidence (verified on main, 2026-07-12)
- `src/map/build.rs:54-55` marks a node `Synced` when any declared path exists
  on disk, even an empty directory.
- Scratch probe: node with empty declared dir shows `"state":"Synced"`,
  `"files":[]` in `cairn get --json`; `cairn context` shows no `[Ghost]` suffix;
  `cairn health --json` counts it under `synced`; `cairn frontier` omits it.

## Task
Treat a declared path with no source files as Ghost (or emit a distinct finding,
e.g. CAIRN_GHOST_MODULE) so `get`, `context`, `frontier`, and `health` stop
reporting empty scaffolding as implemented. Relates to dec.ghost-rule-tracking.
