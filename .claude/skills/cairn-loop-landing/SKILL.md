---
name: cairn-loop-landing
description: "The landing and merge procedure for one /cairn-loop iteration: stage explicit paths, land one commit, open one PR, run the two-lens pre-submit review, then fail-closed squash-merge with re-verification. Loaded by the cairn-loop command at its Land and Cleanup steps; declares the terminal exit tokens the router keys on."
---

# cairn-loop-landing

Land one loop unit as exactly one squash commit on main, then merge it with
fail-closed re-verification. This skill is the **sole** landing procedure for
the `/cairn-loop` command. The command owns selection and the verdict table;
this skill owns the publish-and-merge procedure and the terminal tokens a
successful or failed landing returns.

## Exit tokens

This skill declares the terminal tokens the router's End step keys on. It
returns exactly one, as its final line:

- `ITERATION COMPLETE` - the unit merged (or was safely deferred earlier; not
  this skill's path). On a clean merge, emit this.
- `LOOP HALTED` - a fail-closed merge state the session cannot classify or
  clear: CLOSED-unmerged PR, unreadable PR state, a dirty tree at merge time,
  or the post-merge `cairn scan --strict` is non-zero. Touch nothing; report
  and halt.

The command's End step passes the emitted token through verbatim.

## Inputs

Always required (the command passes these in):

- `slug` - the unit's branch tail: `todo.<slug>`, `<finding-code>.<node>`, or
  `split.<slug>` (the exact name Isolation derived).
- `CAIRN` - the bound cairn binary per the command's Repo bindings.

`pr` is path-dependent:

- **Normal Land** (this skill creates the PR): do **not** pass `pr`. After
  `gh pr create`, capture the new PR number (`gh pr view --json number
  --jq .number` against the pushed head) and bind it for Cleanup.
- **Open-PR recovery** (enters at Cleanup; the PR already exists): the
  command passes `pr`. Skip Land publish; go straight to Pre-submit review /
  Cleanup with the given `pr`.

## Land: exactly one commit

Skip this section when entering at Cleanup (open-PR recovery).

If this unit had a change directory, run `$CAIRN change apply <id>` on the
branch as its final task, then re-run `$CAIRN scan --strict` and `$CAIRN hook all` so
the archived state is what gets verified; completion and archival land in the
same commit.

Stage only the files the unit touched, by explicit path: source edits, tests,
artefacts, and the paths apply moved or generated (read them from
`git status --short` after apply; they are part of the unit). `git add -A` and
`git add .` are banned everywhere in this loop.

Tracker completion is part of the unit: before committing, set the selected
todo's status to `done` (`$CAIRN todo set <slug> done`) and, when it was the
last open child, flip the blocked parent to `done`; stage those frontmatter
edits with the unit. A landed unit whose todo stays open gets re-selected by
every later session.

Commit the staged unit as one logical commit whose message names the unit
(todo id or finding code) and the success criterion. Then:

```bash
git push -u origin loop/<slug>
gh pr create
# Capture the new PR number for Cleanup (normal Land only):
pr=$(gh pr view --json number --jq .number)
[ -n "$pr" ] || { echo "LOOP HALTED: gh pr create produced no PR number"; exit 1; }
```

## Pre-submit review (mandatory before merge)

Run two independent read-only reviews of the diff: a **correctness** lens
(state machine, fail-closed paths, edge cases, contract changes) and a
**simplicity** lens (dead code, duplication, naming, over-explanation).
Adjudicate; fix what stands; drop the rest with a one-line reason. Then run
Cleanup. (A single-line documentation change may skip this.)

## Cleanup: the merge, fail-closed

Owns the merge and everything after it, for both the normal path and the
open-PR recovery row (which enters here directly: the diff is already
published). Get CI green and review threads resolved first. Bind the three
variables on the first line to this iteration's real values; everything else
runs verbatim:

```bash
set -euo pipefail   # fail closed: any failing command below stops the script;
                    # a nonzero exit here means report and output LOOP HALTED
pr=""; slug=""; CAIRN=""                # BIND: PR number, the unit's branch
                                        # tail (todo.<slug>, <finding-code>.<node>,
                                        # or split.<slug>), and the cairn binary
                                        # per Repo bindings; the ONLY substitution point
[ -n "$pr" ] && [ -n "$slug" ] && [ -n "$CAIRN" ] || exit 1  # refuses to run until bound
state=$(gh pr view "$pr" --json state --jq .state)
case "$state" in
  OPEN)   gh pr merge "$pr" --squash --delete-branch || true ;;
          # '|| true': after a successful server-side merge gh tries to check
          # out main, which fails inside a worktree; that nonzero exit is
          # expected and must not kill the script. The MERGED re-verify below
          # is the real gate; if the merge itself failed, that test halts us.
  MERGED) ;;  # a crashed prior run already merged; continue to verification
  *)      # CLOSED-unmerged or unreadable: FAIL CLOSED, touch nothing,
          # report, output LOOP HALTED
          exit 1 ;;
esac
git fetch origin main
test -z "$(git status --porcelain)"       # must be clean; non-empty = halt
test "$(gh pr view "$pr" --json state --jq .state)" = MERGED   # re-verify before deletion
merged_tip=$(gh pr view "$pr" --json headRefOid --jq .headRefOid)
git checkout --detach origin/main
if git show-ref --verify -q "refs/heads/loop/$slug"; then
  if [ "$(git rev-parse "loop/$slug")" = "$merged_tip" ]; then
    git branch -D "loop/$slug"
  fi  # tip differs: leave the branch, the next preflight owns it
fi    # branch absent (PR recovery): nothing to delete
"$CAIRN" scan --strict                    # against merged main; must exit 0
```

End state of every successful iteration: `../cairn-loop` detached at
origin/main, zero unquarantined `loop/*` branches, zero `loop/*` PRs. That
clean park is what lets the next session select fresh work. On reaching it,
emit `ITERATION COMPLETE`.

## Guardrails

- One unit, one branch, one PR, one squash commit on main. A growing PR means
  stop and split, not a bigger commit.
- `git add -A` and `git add .` are banned; stage explicit paths only.
- Branch deletion requires merged evidence: a MERGED PR at the same tip, or
  an explicit maintainer discard note. Nothing else, nowhere else.
- The post-merge `cairn scan --strict` is part of the gate; a non-zero result
  halts.
- Never bypass hooks (`--no-verify`, `SKIP=` are forbidden). Fix the cause.
