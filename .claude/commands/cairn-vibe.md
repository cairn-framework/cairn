---
name: "Cairn Vibe Session"
description: Director-driven cairn session that ships a themed release block approved up front, unlike the autonomous one-unit /cairn-loop
category: Workflow
tags: [workflow, cairn, vibe, dogfood]
---

Run a Cairn Vibe Session: the attended, director-style cousin of `/cairn-loop`.
The human is the director whose hands are off the keyboard, driving worker CLI
sessions (a `fast` low-latency model for mechanical work, a `good` strong model
for judgment) and verifying their output by reading. Never take a worker's word:
open the touched files and confirm the change meets the success criterion.

Whereas `/cairn-loop` is autonomous and works one unit at a time, `/cairn-vibe`
picks a release-sized themed block of work, gets the maintainer's approval of
the direction and the release plan together up front, then executes the whole
arc hands-off, landing each unit as an independently-reviewed PR, and finishes
by releasing per the pre-approved plan.

**Input**: the argument after `/cairn-vibe` is an optional theme hint, area, or
direction (e.g. `/cairn-vibe webui findings`). With no argument, derive a theme
from live graph state (`cairn status`, its open-todo list, `cairn next`, and
per-node `cairn todos <node>`). `cairn frontier` is an optional secondary signal
for ghost-node / architecture pressure only; it is not the todo source.

**Setup**

Cairn must reflect current source. Do all director build and query work in a
dedicated director worktree, never the main checkout. Running `cairn scan` or a
build in the main checkout rewrites the tracked, committed `map.json` snapshot
and dirties it. Create the worktree first. Each tool call is independent: the
cwd and environment do not carry across calls. A lone `cd` or `export PATH`
does not affect the next call, so bind every director command to its concrete
path in the same call.

Pick and record a unique concrete director worktree path, never a fixed shared
name. This example uses `/tmp/cairn-director-1699999999`; substitute a unique
suffix and reuse that exact literal in every Setup/Orient command and at
teardown.

(`<main-checkout>` is the path to your primary clone, e.g. the repo root;
substitute the real path in every command.)

```bash
git -C <main-checkout> fetch origin
git -C <main-checkout> worktree add --detach /tmp/cairn-director-1699999999 origin/main
( cd /tmp/cairn-director-1699999999 && cargo build --release )
```

Use `/tmp/cairn-director-1699999999` only for Setup and Orient. Run every
Setup/Orient cairn command via
`/tmp/cairn-director-1699999999/target/release/cairn <args>` (or `cd
/tmp/cairn-director-1699999999 && ./target/release/cairn <args>` in each call).
Do not rely on an exported PATH, a persisted cwd, or a `$DIRECTOR_WT` variable
across calls.

**Roles (RACI)**

- **Workers**: implement the unit, add or extend tests, push the feature branch,
  and open the unit PR (`gh pr create`). Mechanical units go to `fast`; design or
  judgment units go to `good`.
- **Director**: runs Setup and Orient (read-only cairn queries plus the local
  `cargo build --release`) in the dedicated director worktree, never the main
  checkout, and runs a fresh local `cargo build --release` in each post-merge
  verification worktree. The director commissions the two reviewer subagents,
  adjudicates, and squash-merges when CI is green and review is satisfied. For
  release, the director (directly or via a dedicated release worker) opens the
  release-prep PR and creates the tag, only after the phase-2 disposition and
  after confirming no material divergence. The director MUST NOT implement unit
  or product code.

**Steps**

Work the six phases in order. Do not advance until a phase's exit criterion is
met. Ephemeral in-session phase tracking is fine for this session's own
progress. Durable cross-session work items go through `cairn todo new <slug>
--node <id>` and status edits via `cairn todo set` on the resulting
`meta/todos/todo.<slug>.md` (this repo's native tracker), not bd.

1. **Orient** - in the dedicated director worktree from Setup, never the main
   checkout, run `/tmp/cairn-director-1699999999/target/release/cairn status`,
   `/tmp/cairn-director-1699999999/target/release/cairn change list`,
   `/tmp/cairn-director-1699999999/target/release/cairn next`,
   `/tmp/cairn-director-1699999999/target/release/cairn lint --json`, and (per
   candidate node) `/tmp/cairn-director-1699999999/target/release/cairn todos
   <node>`. Optionally glance at
   `/tmp/cairn-director-1699999999/target/release/cairn frontier` as a secondary
   signal for ghost-node / architecture pressure; do not treat it as a todo
   backlog. Record the baseline:
   `/tmp/cairn-director-1699999999/target/release/cairn lint` has no new findings
   (the one accepted deferred Info is fine) and exits 0;
   `/tmp/cairn-director-1699999999/target/release/cairn hook all` exits 0; and
   `/tmp/cairn-director-1699999999/target/release/cairn scan` must be zero
   findings (run it now if not already clean). Triage any pre-existing scan
   finding or new lint finding before proposing a block. Name the open unblocked
   todos and which todos are already blocked on maintainer authorisation or an
   external dependency. At Orient's end, immediately remove the director
   worktree:
   `git -C <main-checkout> worktree remove --force /tmp/cairn-director-1699999999`.
   It exists only for Setup and Orient. Exit criterion: if there are no open
   unblocked todos and no actionable next unit, report the state and stop (do not
   manufacture work); otherwise proceed to phase 2 with a written backlog
   snapshot.

2. **Recommend block and release, then ask** - this is the defining gate. From
   live `cairn status`, `cairn next`, and per-node `cairn todos <node>`, group
   open todos into coherent clusters. Drop any todo blocked on maintainer
   authorisation or an external dependency. Rank the unblocked clusters by value
   (user-visible impact, closes prior dogfood friction, unblocks downstream work,
   fits one release). Select the single highest-value coherent cluster of roughly
   three to six todos (or fewer if the backlog is thin). If after dropping
   blocked items no unblocked cluster remains, report the state and stop; do not
   invent a block. Then present ONE recommendation to the maintainer using the
   **AskUserQuestion tool**, bundling:

   - (a) the recommended theme,
   - (b) the exact todo list (slugs and one-line summaries),
   - (c) the rationale and the ranked runner-up themes,
   - (d) the proposed semver bump (`patch` for fixes and ergonomics, `minor`
     for new surfaces, `major` for a breaking public surface),
   - (e) the RELEASE DISPOSITION as selectable options:
     - **Full release** - on completion: bump version, update CHANGELOG, tag;
       release.yml + cargo-publish.yml publish the GitHub Release, crates.io
       crate, and Homebrew formula.
     - **Release-candidate only** - prepare and merge the release-prep PR, then
       stop before the tag.
     - **Land block, no release** - merge every unit and stop; no version bump
       and no tag.

   WAIT for the maintainer's answer. This single gate approves both the work
   direction and how it will be released, so there is no second confirmation
   later. Re-ask only if their answer opens a genuinely new ambiguity (e.g.
   they pick a runner-up theme that needs its own disposition choice, or they
   ask to reshape the todo list). Exit criterion: the theme, the todo list,
   the semver intent, and the release disposition are all ratified.

3. **Execute (director fan-out)** - work each todo in the approved cluster as
   its own unit. Assign each unit to a worker session: `fast` for mechanical or
   well-specified work, `good` for design or judgment. Parallelise independent
   units in separate git worktrees branching from `origin/main` so they never
   share a working tree. No agent, worker or director, builds, scans, or edits in
   the main checkout. Fetch `origin` immediately before fan-out so worker
   worktrees branch from the current `origin/main`. Each unit follows the
   `/cairn-loop` discipline (workers implement, test, push, and open the PR):

   - feature branch from `origin/main` (NEVER commit to main);
   - smallest change that meets a written success criterion;
   - a test for the changed behaviour (test-first for a bugfix);
   - gates green: when Rust changed, `cargo fmt --check`,
     `cargo clippy --all-targets --all-features -- -D warnings`,
     `cargo test`; always `cairn scan` (zero findings) and
     `cairn hook all` (exit 0);
   - keep each todo's frontmatter status in sync ONLY via `cairn todo set`
     (native tracker; never bd/beads);
   - keep PRs small, one per logical unit.

   The director verifies each worker's output by reading the touched files
   before trusting it. Kill finished worker sessions so the roster stays
   legible. Exit criterion: every approved unit has a pushed branch and an
   open PR, or is marked blocked with a written reason.

4. **Review every PR (mandatory pre-submit gate)** - for each unit's diff, the
   director commissions two parallel independent reviewer subagents: one on a
   correctness / contract lens and one on a simplicity / convention lens.
   Adjudicate their findings as For / Against / Verdict. Send blocking findings
   back to the owning worker to fix, then re-review. Squash-merge with
   `--delete-branch` once CI is green and review is satisfied. CodeRabbit is
   advisory only: address its comments if it posts in time, never block on it.
   After each merge, if the unit had a `meta/changes/<id>/` change directory,
   archive it after the merge in a fresh branch worktree. Immediately before
   creating it, run `git -C <main-checkout> fetch origin` in the same call; create
   it from the updated `origin/main`, never the main checkout or a throwaway
   verification worktree.
   Run `cairn change apply <id>`, commit the archive move, and land it via a
   small PR that passes the same mandatory review gate as any unit: CI green,
   review satisfied. Then re-run `cairn scan` against the resulting merged main
   in a fresh verification worktree. A merged change left active misleads the
   next iteration; the `CAIRN_CHANGE_TASKS_COMPLETE` scan finding flags this.
   For each post-merge verification, choose and record a fresh unique concrete
   path containing the merge SHA, such as `/tmp/cairn-verify-a1b2c3d`, and
   substitute the actual merge SHA consistently in every command in one call:

   ```bash
   git -C <main-checkout> fetch origin
   git -C <main-checkout> worktree add --detach /tmp/cairn-verify-a1b2c3d origin/main
   ( cd /tmp/cairn-verify-a1b2c3d && cargo build --release && ./target/release/cairn scan )
   git -C <main-checkout> worktree remove --force /tmp/cairn-verify-a1b2c3d
   ```

   Do not reuse a stale director worktree or stale binary.
   Exit criterion: every unit PR is merged (or explicitly dropped with
   maintainer notice), CI is green on main, and `cairn scan` is clean
   (zero findings).

5. **Release per the approved plan** - when every unit is merged and gates are
   green on merged main, carry out the release disposition chosen in phase 2
   with no further confirmation, because it was approved up front. The director
   (directly or via a dedicated release worker) owns the release-prep PR and
   tag actions.

   - **Full release**: open and merge one release-prep PR that bumps the
     version in `Cargo.toml` and adds a `CHANGELOG.md` section summarising the
     shipped todos; then create the git tag and verify that release.yml and
     cargo-publish.yml published the GitHub Release, crates.io crate, and
     Homebrew formula.
   - **Release-candidate only**: open and merge the release-prep PR, then stop
     and hand a release-candidate report without tagging.
   - **Land block, no release**: stop after landing and summarise what shipped.

   If the delivered block materially diverged from what was approved (scope
   shrank, a unit was dropped, a success criterion was re-scoped), surface that
   to the maintainer before tagging rather than publishing silently. Exit
   criterion: the disposition is done, or a divergence has been raised and
   answered.

6. **Honesty and stop conditions** - if at any point there are no open unblocked
   todos and no actionable next unit, or every ready todo is blocked on the
   maintainer, report the state and stop; do not manufacture work. (The empty
   or all-blocked case should already have stopped in phase 1 or 2; restate it
   here only as a mid-session escape hatch.) End the session with the Output
   report below.

**Output**

Per session, summarise: the ratified theme and disposition; each unit's success
criterion, nodes touched, test added, final `cairn scan` finding count (target
zero), and CI / merge status; and the release outcome (tag URL, crates.io
version, Homebrew formula, or the reason no release ran). If a gate or CI
blocked a unit, report the finding and where it stuck rather than waving it
through. Name the next natural open-todo cluster if one is visible.

**Guardrails**

- Never commit to main. Every unit lands via a feature branch and a reviewed
  PR.
- The main checkout is never an agent working directory: no builds, scans, or
  edits there. Worker worktrees branch from `origin/main`. At session end, the
  main checkout's `git status` must equal its status at session start. If it
  began clean, fetch origin and fast-forward it once (`git -C <main-checkout>
  fetch origin && git -C <main-checkout> merge --ff-only origin/main`) at the
  end; if it drifted, investigate the leak before finishing. Do not
  fast-forward after every PR: workers branch from `origin/main`, so one
  fast-forward at the end suffices and avoids churn. Prune any finished worker
  worktrees at session end, so no worktrees leak between sessions.
- Worker file edits must use absolute worktree paths. The edit and write tools
  resolve relative paths against the session root (typically the main checkout),
  not the shell working directory, so `cd` into a worktree does not isolate
  edits; a relative path can silently write outside the worktree.
- Native-todos-first: never create new bd issues for this repo's own work
  (`dec.native-todos-first`). Status edits go through `cairn todo set`.
- Do not contradict an accepted decision without writing a superseding one.
- Director scope: MAY run Setup/Orient read-only commands, the local `cargo
  build --release` there, and fresh post-merge verification builds, and owns
  orchestration, verification-by-reading, PR merges, and the pre-approved
  release-prep + tag. MUST NOT implement unit or product code (that is worker
  work).
- Kill finished worker sessions so the roster stays legible.
- A clean `cairn scan` (zero findings) is the target state. A scan finding is a
  blocked unit, not a formality. `cairn lint` may carry the one accepted deferred
  Info and still exit 0; new lint findings are not acceptable. `cairn hook all`
  must exit 0.
- Behaviour without a test is not done. Reconcile drift by fixing the code or
  the blueprint, never by ignoring a file.
- Keep PRs small and independently reviewable. One logical unit per PR.
- The single phase-2 gate is the only maintainer ratification for direction and
  release. Do not re-ask for confirmation at release time unless the delivered
  block materially diverged from what was approved.
