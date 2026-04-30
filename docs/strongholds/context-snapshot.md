# Context Snapshot: 2026-04-28 (post-research-net, pre-graphify)

Authoritative record of in-flight state. Read this **before** acting after resume. Supersedes all earlier 2026-04-XX snapshots.

## Branch / git state

- Branch: `dev` @ `3519f20` (synced with origin)
- Working copy: dirty with **untracked research net** at `docs/research/getcairn-dev/` (~213 files, 13.6K lines), plus untracked strongholds:
  - `docs/strongholds/cairn-domain-expandability.md`
  - `docs/strongholds/external-cairn-docs-research.md`
  - `docs/strongholds/image-batch-inventory.md`
  - `docs/strongholds/pr-bundle-recon.md`
  - `docs/strongholds/context-snapshot.md` (this file, modified)
- `.claude/worktrees/agent-a4895e9ba76616454/` leftover (failed Recovery Nazgul; safe to delete)
- Stash: `stash@{0}: webui-cmd-palette-and-changes-drawer` (untouched, predates session)
- Two abandoned local branches NOT pushed and harmless until cleaned:
  - `04-28-fix_makefile_match_uppercase_x_checkboxes_in_status-phases` (gt-create -a accident; commit `135c000` carries the entire research tree)
  - `temp_clean_fix` (intermediate cherry-pick branch)

## Closed campaigns this session

### Campaign 1: PR bundle merge ✅
All 5 PRs merged onto dev:
- #6 `chore: remove legacy agent scaffolding`
- #7 `chore: track .claude/ workflow skills and openspec project config`
- #8 `build: add make status target for phase and worktree visibility`
- #9 `chore: commit pending wip across settings, claude.md, landing`
- #10 `fix(makefile): match uppercase [X] checkboxes in status-phases` (follow-up for #8 review comment that slipped through)

### Campaign 2: getcairn.dev research net ✅
Doc set landed at `docs/research/getcairn-dev/` (untracked, awaiting commit decision):
- 10 numbered docs (`01-product-overview.md` through `10-source-attribution.md`)
- 64 named screenshots in `screenshots/` (skip 19, 57, 59-dup-of-58, 63-dup-of-62)
- 34 per-image analyses in `screenshots/_analysis/`
- 47 captured site pages in `site-pages/` (23 marketing + 24 docs)
- `_export-analysis.md` (15 sections from project + settings exports)
- `working-notes.md` extended with 4 new sections including software-domain PRD hypothesis (image #65)
- `offshore-survey-usv-rov-0.1.0.cairn/` (project export bundle)
- `export-from-settings/` (settings export: JSON, CSV, MD, dialog screenshot)
- 0 em-dashes in authored docs (sweep also cleaned 02, 04, 05, 06, working-notes pre-existing)

### Campaign 3: graphite-pr skill hardening ✅
Edited `~/.claude/skills/graphite-pr/`:
- `scripts/gt-merge-cascade.sh`: review-thread gate via `gh api graphql`. Counts unresolved + not-outdated reviewThreads per PR; fails with `unresolved-review-threads` before invoking `gt merge`. Low context cost (integers only).
- `SKILL.md`: updated "After submit: merge" with gate description; corrected single-PR pre-flight to GraphQL form (`gh pr view --json reviewThreads` does NOT exist; only GraphQL exposes that field).
- Validated end-to-end on PR #10 merge.

## Open items for next session

1. **Run `/graphify wiki`** on `docs/research/getcairn-dev/`. User queued this. Graphify may auto-index photos; descriptive PNG filenames + adjacent analysis MDs are the inputs.
2. **Decide whether to commit the research tree** (currently untracked). Options: commit as-is to dev, commit via a dedicated PR for review, or keep untracked.
3. **Cleanup**: delete the two abandoned local branches and the `.claude/worktrees/agent-a4895e9ba76616454/` leftover. Destructive-op guard blocked `git branch -D` earlier; needs explicit user approval.
4. **Optional**: enable GitHub branch-protection rule on `dev` ("Require conversations to be resolved before merging"). Belt-and-braces alongside the new skill gate.

## Key facts established this session

| Fact | Source |
|---|---|
| getcairn.dev's data model: only 2 node types (system, subsystem); no edges[] array; decisions implicit in `history[].pipelineTrace` | export-parse scout |
| `interfaceHash` is a pipe-joined ID string, not a content hash (substantive divergence from our model) | export-parse scout |
| `_narrativeAnalysis.mainstaySentence` + cards + verbPhrases is an AI-generated causality layer with no analogue in our framework | export-parse scout + image #56 |
| User fed software-domain PRD (OpenSpine) into getcairn.dev; platform handled it without complaint (image #65). Logged as **hypothesis**, sample size 1 | image-65 analysis + user direction |
| `gt merge` does NOT block on unresolved review comments by default. Branch protection or skill-side gate required | live failure on PR #8 + Graphite docs |

## Held / pending user input

- None blocking. Next-session start can go straight to graphify.

## Workflow protocol reminders

- **Em-dashes banned** in repo prose (CLAUDE.md).
- **Images stored in repo, sub-agents analyse**. Orchestrator does NOT view images directly.
- **Per-image analysis** = one image per scout call. Oversized images (>2000px) must be resized via `sips` before reading; sips temp-write may need `dangerouslyDisableSandbox: true`.
- **Skills live at** `~/.claude/skills/<name>/`. Edit there for cross-repo behaviour.
