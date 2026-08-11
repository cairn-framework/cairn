---
name: cairn-loop-recovery
description: "State-recovery procedure for one cairn-dev loop iteration: resume a dirty surviving loop branch, recover an open loop PR, clear interrupted cleanup, adopt or quarantine an orphaned loop branch, author a recover-todo. Loaded by cairn-dev loop mode from its preflight verdict table; not for ordinary development sessions."
---

# cairn-loop-recovery

Recover loop state for the preflight verdict loop mode's table selected. Loop
mode owns the verdict table (state classification and the fail-closed
backstops); this skill owns the **procedure** each recovery row expands into.
Run the section matching the verdict and return the token it declares.

## Exit tokens

This skill declares the mid-iteration tokens loop mode routes on. Return
exactly one, as the final line, then let loop mode's flow continue:

- `RECOVERED` - the state was recovered; continue the iteration at the phase
  the calling verdict names (Verify for in-place recovery, Scope for an
  adopted branch, or continue preflight after cleanup/quarantine-park).
- `LOOP HALTED` - the state is unclassifiable or unrecoverable by this skill
  (intent unclear, conflicting evidence, a violation the maintainer must
  judge). Touch nothing; report; halt.

Paths that finish the iteration (open-PR recovery after merge, quarantine
via a recover-todo) **hand off** to `cairn-loop-landing`; that skill emits
the terminal token (`ITERATION COMPLETE` or `LOOP HALTED`). This skill does
not re-emit a terminal token after a successful hand-off.

## 1. Dirty tree on a `loop/*` branch whose slug maps to a known unit

Finishing that unit IS this iteration. Recover in place:

1. Read the **full** diff against the unit's intent (the todo body or finding
   code): `git diff`, `git log origin/main..HEAD`, and the unit's artefacts.
2. Decide coherence: complete the unfinished work, or trim incomplete work
   back to a single coherent landed unit. Prefer trimming over expanding when
   the diff has sprawled beyond one reviewable PR.
3. Return `RECOVERED`. Loop mode continues at Verify, then Land via
   `cairn-loop-landing`. Do not invoke landing from this skill.

Invariant (a fail-closed backstop, owned by loop mode): **no checkout of
any kind until the tree is clean.** If intent is unclear, do not guess; emit
`LOOP HALTED`.

## 2. Clean tree, exactly one open `loop/*` PR

Recovery unit. The diff is already published, so skip the Land publish steps
and enter the `cairn-loop-landing` skill at its Pre-submit review / Cleanup:

1. Understand the open PR's diff and its CI/review state
   (`gh pr view <n>`, `gh pr checks <n>`, `gh pr view <n> --comments`).
2. Fix what CI or review requires. Re-push to the same `loop/*` branch.
3. Hand off to `cairn-loop-landing` at Cleanup, passing the existing `pr`,
   plus `slug` and `CAIRN`. The landing skill's terminal token is this
   iteration's final line; do not emit a recovery token after the hand-off.

## 3. Clean tree, a `loop/*` branch whose tip matches a MERGED PR

This is interrupted cleanup, not work. The PR already merged; only the branch
ref and the park remain:

1. `git checkout --detach origin/main` if the worktree still has the branch
   checked out (clean tree, so a pure ref move).
2. `git branch -D <branch>` (the merged PR at the same tip is the deletion
   evidence).
3. Return `RECOVERED`; loop mode continues preflight toward fresh-work
   selection.

## 4. Clean tree, a surviving `loop/*` branch covered by `todo.recover-<slug>`

Branch on the todo's status:

- Status `done` **with** an explicit maintainer discard note in the body:
  cleanup is authorized. Park off the branch if checked out
  (`git checkout --detach origin/main`), then `git branch -D <branch>`,
  continue preflight. Return `RECOVERED`.
- Status `done` **without** a discard note: ambiguous; treat as quarantined
  (below) and report it.
- Any other status (authored `blocked`): QUARANTINED (below).

For QUARANTINED: never delete or commit to the branch. If the worktree has it
checked out, park off it the same way (clean tree, pure ref move; the branch
keeps its commits). Continue preflight as if the branch were absent; it is the
maintainer's via the todo. Return `RECOVERED` (the branch is parked, not this
iteration's unit); the maintainer resolves it through the todo.

## 5. Clean tree, any other surviving `loop/*` branch

(Closed PR with no merge, no PR, or a tip that differs from the merged PR.)

- If its slug maps to an **open todo or live finding**: adopt it as this
  iteration's unit. `git checkout loop/<slug>` (clean tree, pure ref move) and
  return `RECOVERED`; loop mode continues at Scope.
- Otherwise **preserving it IS this iteration's unit**: author
  `todo.recover-<slug>` (status `blocked`) so a local branch ref is not the
  only thing keeping those commits alive. Body must record:
  - the branch name and tip SHA (`git rev-parse loop/<slug>`),
  - PR state (`gh pr list --state all --head <branch>`, or "no PR"),
  - a one-paragraph diff summary (`git diff origin/main...loop/<slug>`
    `--stat` plus prose).
  Then hand off to `cairn-loop-landing` (normal Land path: stage the
  recover-todo, commit, PR, merge). The landing skill's terminal token is
  this iteration's final line; do not emit a recovery token after the
  hand-off.

## Guardrails

- Never stash, clean, reset, or delete unmerged work. A dirty tree is
  evidence; a surviving branch is someone's commits.
- Branch deletion requires merged evidence (MERGED PR at the same tip) or an
  explicit maintainer discard note in a `done` recover-todo. Nothing else.
- When intent is unclear or evidence conflicts, do not improvise: emit
  `LOOP HALTED` and report. The repeating halt is the durable signal.
- Everything this skill writes (a recover-todo) is committed on a `loop/*`
  branch and reaches main only through the Land path, inside one commit.
