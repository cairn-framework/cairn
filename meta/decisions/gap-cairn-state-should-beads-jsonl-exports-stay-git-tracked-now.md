---
id: dec.gap-cairn-state-should-beads-jsonl-exports-stay-git-tracked-now
nodes: [cairn.state]
status: accepted
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

No. Untracked as of 2026-07-11 (recommended option auto-accepted on ask
timeout, unattended run; reversible, revisit note below): `.beads/.gitignore`
now ignores `*.jsonl` and `issues.jsonl` / `interactions.jsonl` were removed
from the index (`git rm --cached`; history preserved).

Rationale: per `dec.native-todos-first` this repo's own work no longer flows
through bd, so the export is a write-only mirror whose daemon writes collide
with git (stash/pop UU conflicts, near data-loss on 2026-07-11).

Consequences, verified against source:

- `cairn backlog` (`src/state/backlog.rs:112`) reads `.beads/issues.jsonl`
  from disk, not the dolt server. The file stays on disk locally, so the
  read-view keeps working here; fresh clones/worktrees lose it unless bd
  re-exports (acceptable: bd is retired for this repo's own tracking).
- `bd show <id>` archaeology is unaffected (dolt-server-backed).

Alternatives considered: keep tracked (recurring UU conflicts), stop the
daemon auto-export (config lives outside the repo, regresses silently),
move exports to an untracked path with deliberate snapshots (more moving
parts for a retired tracker).

Revisit trigger: this repo resumes using bd for its own task tracking.
