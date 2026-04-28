# Session Handoff — 2026-04-28

**last_updated:** 2026-04-28T12:35:00Z
**session_sha:** c98d506

## What Was Done

- Ran `/graphify . --wiki` on full repo. 1,618 nodes, 3,182 edges, 51 communities, 61 wiki articles. Outputs in `graphify-out/` (now gitignored).
- Audited graphify findings. Identified phantom-node extraction artefacts (8 duplicate spec.md section nodes attached to phase design.md files instead of the canonical spec). Documented merge-vs-keep policy: 8 mergeable phantoms, 5 historical god-module records to keep, 4 future-test-file references to keep.
- Cleanup of legacy agent scaffolding: removed `.codex/`, `.opencode/`, `.orchestrator/`, stray `.pyc`. Extended `.gitignore` to cover `.codex/`, `.opencode/`, `.sauron/`, `graphify-out/`.
- Established `.claude/` as tracked operational surface: committed `.claude/skills/openspec-*` (vendored, openspec CLI 1.3.1), `.claude/commands/opsx/`, plus a new `.claude/skills/README.md` documenting the authored-vs-vendored boundary.
- Populated `openspec/config.yaml` with CAIRN-specific context (Rust stack, two-chain architecture, terminology, em-dash ban, verification gate). Replaced default TypeScript example boilerplate.
- Added `make status` target with sub-targets `status-phases`, `status-worktrees`, `status-untracked` so failures isolate. Verified working.
- Stashed and removed `.worktrees/ui` after confirming all 3 committed commits (b3a802d, ab5f23c, 5291360) are already on dev. Salvaged 475 lines of uncommitted WIP (command palette + changes drawer) to `stash@{0}`. Stash message references `docs/design-system/NEXT_SESSION.md` items 6 and 7.
- Investigated cflx upgrade. Currently on v0.6.7, latest v0.6.20. Decision: WAIT. Sequential workflow untouched in any release; v0.6.9 retires `cflx.py validate` which CLAUDE.md still references, so upgrade risks breaking the verification gate.
- Confirmed CFLX runs sequentially by default. The `--parallel` flag is opt-in. Analyze script `scripts/cflx-analyze-cairn-phases.py` emits proper per-phase dependency chain; cflx honours it.
- Restructured 3 cleanup commits into a Graphite stack and submitted as PRs #6, #7, #8. Added wip mods as standalone PR #9.

## What Remains

- **Graphify Category-A phantom merge** — 8 duplicate spec.md section nodes still in `graphify-out/graph.json`. Post-process pass would redirect inbound edges to canonical spec nodes and delete the phantoms. Detailed in transcript.
- **Phase work** — All 6 active phases drafted, 0 of 132 tasks complete. Next phase per analyze script: `phase-8.0-tests` (test-first pre-phase for summariser).
- **`docs/design-system/NEXT_SESSION.md`** — webui v2 follow-up plan. Items 6 (command palette) and 7 (changes drawer) are partially implemented in `stash@{0}`. Items 1-5 (hinge layout, hinge diagram in inspector, chain bars, chain rails, decision chips on leader lines) are unstarted.
- **Issue #1** — Open: "Research: diff view UI for desired vs actual graph" (Apr 24). Different scope from the stash; not blocked.

## Current State

- Branch: dev (= origin/dev = c98d506)
- Open PRs: #6, #7, #8 (cleanup stack), #9 (wip catch-up). All published, awaiting auto-review.
- Working copy: clean.
- Worktrees: 1 (main only).
- Stashes: stash@{0} = webui-cmd-palette-and-changes-drawer (recovery: `git stash apply stash@{0}`).
- Local-only excludes added to `.git/info/exclude`: `.sauron/`, `graphify-out/` (redundant once PR #6 merges).

## Next Steps

1. **Review and merge the cleanup stack.** Bottom-up: PR #6 → PR #7 → PR #8. PR #9 is independent of the stack and can merge any time.
2. **Decide on the phantom merge.** If yes: write the post-process script described in the transcript that scans graph.json for nodes whose label matches `docs/spec\.md|openspec/specs/.+\.md` and source_file differs, redirects inbound edges to the canonical node by label match, deletes the phantom. ~50 lines Python. Re-cluster after.
3. **Start phase-8.0-tests if continuing the test-first chain.** `cflx run` (no --parallel flag, sequential is default). Analyze script will pick phase-8.0-tests as the head of the queue.
4. **Optional: open a draft PR for stash@{0}** if you want the webui WIP backed up to remote. Otherwise the stash is local-only and would be lost if the clone is destroyed.
5. **Optional: revisit cflx upgrade after PR #6 merges.** v0.6.9's retirement of `cflx.py validate` needs verification against `CLAUDE.md`'s gate battery before upgrading.
