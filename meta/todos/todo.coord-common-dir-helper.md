---
node: cairn.coord
status: open
created: 2026-08-07
blocked_by: [todo.parallel-dispatch-granularity]
related: [dec.rung-three-coordination-substrate]
---

# Coordination store: git common dir helper

Rung 3 (`res.parallel-dispatch-rung-3`, Part 2) places the coordination store at
`<git-common-dir>/cairn/coord/`, which is the only location shared by every
worktree of a checkout family. Nothing in cairn resolves the common dir today.

`git_path` at `src/cli/commands/hook.rs:181` must NOT be reused: `git rev-parse
--git-path cairn/coord` resolves an unrecognised path against the per-worktree
gitdir, verified in a secondary worktree:

```
$ cd /Users/george/repos/cairn-vibe-edit
$ git rev-parse --git-path cairn/coord
/Users/george/repos/cairn/.git/worktrees/cairn-vibe-edit/cairn/coord
$ git rev-parse --git-common-dir
/Users/george/repos/cairn/.git
```

Reusing it would give every worktree a private store and silently defeat the one
seam rung 3 exists to provide.

## Task

Add a `git_common_dir(root)` helper running `git rev-parse --git-common-dir`,
resolving a relative result against the repository root, following the existing
read-only subprocess discipline in `src/cli/commands/hook.rs:166-197`.

Repoint `src/hooks/architecture.rs:172-177`, which hardcodes `root/.git/HEAD`
and `root/.git/<ref_path>` and is already wrong in a secondary worktree (there
`.git` is a file, not a directory).

## Acceptance

- A test asserts the helper returns the shared common dir when run from a
  secondary worktree fixture, and equals `<root>/.git` in a primary checkout.
- `current_head_hash` resolves HEAD correctly from a secondary worktree.
- Existing ratification hooks still pass.
