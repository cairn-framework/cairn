# Graphite workflow (gt)

Load this file when: `gt` is invoked, PR work begins, or a stack merge is in progress.

This repo uses Graphite (`gt` CLI) for stacked PRs. `gt` owns branch state.
Every branch, commit, and push goes through `gt create` / `gt modify` / `gt submit`.
Plain `git status`, `git log`, `git diff`, `git add`, `git reset`, `git stash` stay fine.
Raw `git commit`, `git push`, `git checkout -b`, `git branch -D` bypass Graphite's
metadata and corrupt the stack.

## The 90% loop

```bash
gt sync --no-interactive --force                     # Sync trunk
git add <files-for-this-unit>                        # Stage selectively
gt create -m "<type>(<scope>): <subject>"            # New branch + commit
gt submit --stack --publish --no-interactive         # Publish (auto-review fires)
```

Amend on review: `git add <files>; gt modify -a; gt submit --publish --no-interactive`.
New scope on top: `git add <files>; gt create -m "..."`.
After submit and review, run `~/.claude/skills/graphite-pr/scripts/gt-merge-cascade.sh`
to merge with review-thread gating.

## Sizing

One commit equals one logical unit. Target under 250 lines added+removed, hard cap 400.
See `~/.claude/skills/graphite-pr/SKILL.md` for full rules.
