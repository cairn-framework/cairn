---
id: dec.gap-cairn-state-should-beads-jsonl-exports-stay-git-tracked-now
nodes: [cairn.state]
status: proposed
date: 2026-07-11
gap: true
informed_by: []
---

# Gap: Should .beads/*.jsonl exports stay git-tracked now that dec.native-todos-first retired bd for this repo's own work? Daemon writes to tracked files keep producing UU conflicts during background git operations (stash/pop collisions observed 2026-07-11).

## Question

Should .beads/*.jsonl exports stay git-tracked now that dec.native-todos-first retired bd for this repo's own work? Daemon writes to tracked files keep producing UU conflicts during background git operations (stash/pop collisions observed 2026-07-11).

## Context

Node: `cairn.state` (state: Synced)

Opened by `cairn gap cairn.state --question "Should .beads/*.jsonl exports stay git-tracked now that dec.native-todos-first retired bd for this repo's own work? Daemon writes to tracked files keep producing UU conflicts during background git operations (stash/pop collisions observed 2026-07-11)."`.

## Resolution

(Answer the question here, then flip `status` to `accepted` or delete this file.)

### Recommendation (pending ratification)

Stop tracking the export: gitignore `.beads/*.jsonl` and remove the files
from the index (history preserved; `bd show <id>` archaeology unaffected
since bd reads the dolt server, not these exports). Rationale: per
`dec.native-todos-first` this repo's own work no longer flows through bd,
so the export is a write-only mirror whose daemon writes collide with git
(stash/pop UU conflicts, near data-loss on 2026-07-11). Alternative if the
export must stay shareable via git: move exports to an untracked path and
commit snapshots deliberately. Revisit trigger: this repo resumes using bd
for its own task tracking.
