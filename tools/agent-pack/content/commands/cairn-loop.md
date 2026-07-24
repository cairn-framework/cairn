---
name: "Cairn Dev Loop"
description: Run one iteration of cairn development, one unit landed as one squash commit, designed for harness loop mode (/loop N)
category: Workflow
tags: [workflow, cairn, dogfood]
---

Run ONE iteration of the Cairn Dev Loop: a workflow that develops cairn using
cairn. You are a fresh session inside a harness loop; the harness re-injects
this same message each iteration, so do exactly one unit of work, land it as
one commit on main, and end. Never select a second unit. This file (plus the
skills it loads) is the sole normative orchestrator; a short descriptive
overview lives in `docs/agent/cairn-dev-workflow.md` and is never normative.

**Required skills.** Load these before the step that needs them. A required
skill that fails to load (missing file, unreadable, or the Skill tool cannot
invoke it) is LOOP HALTED: touch nothing, report which skill failed, and end.
Never improvise a procedure that lives in a skill.

| Skill | When | Declared tokens |
|---|---|---|
| `cairn-loop-recovery` | Any preflight recovery row (dirty-tree recovery, open-PR recovery, interrupted cleanup, recover-todo branch, surviving-branch adopt/quarantine) | `RECOVERED`, `LOOP HALTED` (terminal tokens come from landing after a hand-off) |
| `cairn-loop-landing` | Land (publish) and Cleanup (merge); also the open-PR recovery row after CI/review is green, and the quarantine hand-off | `ITERATION COMPLETE`, `LOOP HALTED` |

**Input: MISSION.** The harness re-injects one fixed user message per
iteration (the message sent after `/loop N`). Any text in that message beyond
the command itself binds MISSION. It is identical every iteration; there is
no per-iteration channel, so work that must evolve across iterations belongs
in the tracker, not in MISSION. Precedence:

1. The preflight verdict always wins; MISSION never builds on unresolved
   state. (Deferring costs nothing: the next session receives the same
   MISSION.)
2. MISSION names a unit (todo slug, node id, finding code): select exactly
   that. A node id selects within the node deterministically: its first lint
   finding, else its top open todo. Already done, or blocked/quarantined:
   report why and output LOOP EXHAUSTED; the message is immutable, so no
   later iteration of this run can progress either.
3. MISSION is a scope filter (e.g. "webui only"): apply normal selection
   restricted to that scope. Nothing in scope: report it, output LOOP
   EXHAUSTED. Never select outside the scope.
4. MISSION describes new work: derive the slug canonically so every session
   computes the same one: lowercase the mission text, replace runs of
   non-alphanumerics with single hyphens, take the first 40 characters, then
   append `-` plus the first 6 hex chars of the SHA-256 of the exact raw
   MISSION string (before any canonicalisation), which makes cross-mission
   collisions vanishingly unlikely. If `todo.<slug>` already exists, do not
   re-create it; select it. Otherwise create the branch `loop/todo.<slug>`
   FIRST, then materialise the todo on it with `$CAIRN todo new <slug> --node
   <id>` (`$CAIRN`: the bound cairn binary, defined under Repo bindings
   below); split into sub-todos per the sizing rule if large, and select the
   first open one. Writing a todo while parked detached would strand the
   next session in the fail-closed row. Node id: resolution accepts exactly
   two forms, an exact node id from the blueprint, or a file path that falls
   under exactly one node's `path`. Nothing else resolves; never infer a
   node from meaning. Unresolved: report that the mission needs a node,
   list candidate nodes with a ready-to-paste corrected mission, and output
   LOOP EXHAUSTED. Node choice drives Scope, deps, and provenance for every
   later iteration, so a wrong anchor compounds silently.
   When adopting any surviving branch for a
   MISSION, confirm its slug maps to this MISSION's unit; a mismatched
   branch belongs to the table's generic surviving-branch row.
5. No MISSION: default selection.

**Isolation.** Work only in the persistent worktree `../cairn-loop` (create
once if absent: `git worktree add --detach ../cairn-loop origin/main`; never
remove it). Every branch you create is prefixed `loop/`. Never touch the main
checkout, non-loop branches, other sessions' dirty files, or PRs from
non-loop heads. Branch names are derived, never invented: `loop/todo.<slug>`
for a todo, `loop/<finding-code>.<node>` for a lint finding,
`loop/split.<slug>` for a decomposition.

**Repo bindings.** One seam; everything else in this file is generic.
`$CAIRN` is a textual placeholder, not a live shell variable: substitute the
bound value when composing every command, and give every executable shell
block its own assignment line; nothing set in one tool call survives into
the next. Two bindings:
- `CAIRN` - the cairn binary. THIS repo (cairn develops itself): build from
  source in Setup, bind `CAIRN=./target/debug/cairn`, run from the loop
  worktree. Shipped default: `CAIRN="$(command -v cairn)"` then
  `[ -x "$CAIRN" ]` (POSIX-portable; the -x test rules out alias and
  function resolution), then verify `"$CAIRN" --version` succeeds; absent
  or failing at any step: touch nothing, report, output LOOP HALTED.
- Language gates - THIS repo: `cargo build`, `cargo clippy --all-targets
  --all-features -- -D warnings`, `cargo test` when Rust changed. Shipped
  default: none beyond `$CAIRN hook all`; the target repo declares its gates
  in its hook config and AGENTS.md.

**Setup.** Runs AFTER the preflight verdict and before the first `$CAIRN`
command (preflight needs only git and gh; on a fail-closed verdict nothing
is built at all). If the verdict adopted a surviving `loop/*` branch, check
it out NOW (`git checkout loop/<slug>`; clean tree, pure ref move) so Setup,
Scope, and everything after run against the recovered state, not origin/main.
Then resolve the `CAIRN` binding: in this repo, `cargo build` in
`../cairn-loop` (incremental, near-instant when no Rust changed; one profile
throughout), then `CAIRN=./target/debug/cairn`.

**Preflight: observe read-only, then act on the FIRST matching row.**
No checkout, stash, clean, add, or commit during observation. Recovery
procedures live in the `cairn-loop-recovery` skill; the table classifies and
points. Fail-closed backstops stay here and never move.

```bash
git fetch origin main
git status --porcelain                                    # dirty?
git branch --show-current                                 # branch, or empty = detached
git for-each-ref 'refs/heads/loop/*' --format='%(refname:short) %(objectname)'
gh pr list --state open --json number,headRefName \
  --jq '.[] | select(.headRefName | startswith("loop/"))' # open loop PRs only
```

| State | Action |
|---|---|
| Dirty tree, on a `loop/*` branch whose slug maps to a known unit (todo or finding) | Finishing that unit IS this iteration. Load `cairn-loop-recovery` §1 (recover in place). No checkout of any kind until the tree is clean. On `RECOVERED`, continue at Verify. |
| Dirty tree, anything else (detached, unknown branch, unexplained files, intent unclear) | FAIL CLOSED. Touch nothing: no stage, stash, clean, commit, checkout, and no file writes either, the dirty tree is evidence. Report the state (`git status --short`, branch/HEAD, why unclassifiable) and output LOOP HALTED. Next sessions land in this row and re-report until the maintainer resolves it; that repeating halt IS the durable signal. |
| Clean; exactly ONE open `loop/*` PR exists | Recovery unit. Load `cairn-loop-recovery` §2, then `cairn-loop-landing` at Cleanup. On `ITERATION COMPLETE` from landing, end. |
| Clean; MORE than one open `loop/*` PR | FAIL CLOSED. A prior run violated one-PR-per-iteration and the right merge order is a judgment call. Report all of them, output LOOP HALTED. |
| Clean; a `loop/*` branch whose tip matches a MERGED PR's headRefOid (`gh pr list --state merged --head <branch> --json headRefOid,mergedAt`) | Interrupted cleanup, not work. Load `cairn-loop-recovery` §3. Preflight deletes branches ONLY here and in the discard-note case (§4). On `RECOVERED`, continue preflight. |
| Clean; a `loop/*` branch covered by an existing `todo.recover-<slug>` on main | Load `cairn-loop-recovery` §4 (status-branched: discard-authorized cleanup, ambiguous, or quarantined). Never delete or commit to a quarantined branch; if the worktree has it checked out, park off it (clean tree, pure ref move). On `RECOVERED`, continue preflight as if the branch were absent. |
| Clean; any other surviving `loop/*` branch (closed PR, no PR, or tip differs from merged PR) | Load `cairn-loop-recovery` §5. Adopt (open todo/finding → `RECOVERED`, continue at Scope) or quarantine (author `todo.recover-<slug>`, hand off to `cairn-loop-landing`; landing emits the terminal token). A local branch ref is the only thing keeping those commits alive: never delete without a MERGED PR at the same tip or an explicit maintainer discard note in the todo. |
| Clean, parked detached, no unquarantined `loop/*` branches or PRs | Select fresh work (below). |

**Select ONE unit** (after MISSION precedence): the first `$CAIRN lint --json`
finding; else the top open todo from `$CAIRN status` / `$CAIRN todos <node>`
(skip todos with status `blocked`); else verify the stop evidence (lint
clean, no open todos, no unquarantined `loop/*` branches or PRs, clean park;
quarantined branches and their blocked recover-todos do not block exhaustion)
and output LOOP EXHAUSTED. "First" and "top" are defined, not incidental:
sort findings by severity (errors first), then file path, then line, then
code; sort todos by slug. Output order from the tools is not a contract.
This table owns selection; `$CAIRN brief` (no argument) is a context helper,
not a selector. When the selected unit is the top open todo, brief's fused
output (decisions, contract) feeds Scope. Treat any gate list brief prints
as advisory; Verify in this file is the gate authority. Never let brief's
pick override lint-first order, and never pass brief an argument to target
a todo: arguments resolve bead ids, not todos or nodes.

Sizing rule: the unit must fit one small reviewable PR. Too big: this
iteration IS the decomposition. Create the branch `loop/split.<slug>` first,
then on it create sub-todos with `$CAIRN todo new` and set the parent to
`blocked` with body line "blocked on sub-todos: <ids>" (the iteration
completing the last child flips the parent to `done`), and land that
decomposition as this iteration's single commit.

Artefact rule: every todo or `meta/` artefact this loop creates is written
ON the unit's `loop/*` branch (never while parked detached) and reaches main
only through the Land path, inside the iteration's single commit. If a state
forbids landing (the fail-closed row), it also forbids writing: report
instead. Nothing is ever left uncommitted in the loop worktree.

**Scope.** For the unit's node: `$CAIRN neighbourhood <node> --include-todos
--include-changes`, `$CAIRN rationale <node>`, `$CAIRN deps <node> --direction
in --transitive`. Respect accepted decisions. Write one verifiable success
criterion. Scope may reroute, never expand: if orientation reveals a
prerequisite that must land first, stop before touching code; author the
prerequisite todo, set this unit's todo `blocked` on it (the body names the
prerequisite todo slug, `todo.<slug>`, and node id where relevant), land
those tracker edits as this iteration's single commit, and end.
The prerequisite is then an open todo, eligible for normal selection while
the blocked unit is skipped; selection order is unchanged.

**Implement + test.** The unit's branch is `loop/<tail>` where `<tail>` is
the derived form from Isolation (`todo.<slug>`, `<finding-code>.<node>`, or
`split.<slug>`); every later step (push, PR, Cleanup) uses this exact name.
If it is already checked out, adopted at verdict time or created earlier
this session during MISSION materialisation or decomposition, continue on
it. Otherwise create it, always from fresh origin/main:
`git checkout --detach origin/main && git checkout -b loop/<tail>`. (If the
derived name exists but was NOT adopted by the verdict and NOT created this
session, you missed a preflight row; go back, the table owns it.) Make the
smallest change satisfying the
criterion. New files fall under
a node `path` or get a new Module; new cross-module calls get a blueprint
edge; CLI strings go in `docs/design-system/copy.toml`. Changed behaviour
gets a test; for a bug fix write the test first, red then green. Substantial
work goes through `cairn-propose` / `cairn-apply`.

**Verify: the gate.** Run the bound language gates (this repo: `cargo build`,
`cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`
when Rust changed). Always: `$CAIRN scan` (zero findings) and
`$CAIRN hook all` (exit 0). Fix the cause of any
failure. Never bypass hooks.

**Record.** If structure changed or a non-obvious tradeoff was made, write a
decision artefact in `meta/decisions/`.

**Land + Cleanup.** Load the `cairn-loop-landing` skill. It owns: stage
explicit paths (no `git add -A` / `git add .`), tracker completion
(`$CAIRN todo set <slug> done`), one logical commit, push, one PR, the
two-lens pre-submit review, and the fail-closed squash-merge with
re-verification (the Cleanup script). Always pass `slug` and `CAIRN`. On
the normal Land path do **not** pass `pr`: the skill creates the PR and
binds `pr` itself before Cleanup. On the open-PR recovery row, pass the
existing `pr` and enter the skill at Cleanup (the diff is already
published). The skill returns exactly one of its declared tokens; pass it
through as this iteration's final line. If the skill fails to load, output
LOOP HALTED.

**End.** Summarize: the unit, success criterion, nodes touched, test added,
final scan finding count, PR and merge status. Output exactly one token
(from this file's fail-closed rows, from selection exhaustion, or from a
loaded skill):

- ITERATION COMPLETE: unit landed, or safely deferred with a blocked todo.
- LOOP EXHAUSTED: no selectable work remains, or the immutable MISSION can
  never progress in this run (named unit done or blocked, scope empty).
- LOOP HALTED: fail-closed state needs the maintainer; do not continue.

The token is the FINAL line of output, alone, verbatim; the summary comes
before it. Tooling and the maintainer read loop health from that line.

If blocked on a decision only the maintainer can make: author the researched
recommendation as a `meta/` artefact plus a blocked todo, land them through
the Land path (`cairn-loop-landing`) as this iteration's single commit,
report, output ITERATION COMPLETE. Never wait for an answer mid-loop.

**Guardrails.**

- Zero `cairn scan` findings is the target state; a finding blocks the
  iteration, it is not a formality.
- Behaviour without a test is not done.
- Never contradict an accepted decision without writing a superseding one.
- One iteration, one unit, one squash commit on main. A growing PR means
  stop and split.
- Branch deletion requires merged evidence: a MERGED PR at the same tip, or
  an explicit maintainer discard note. Nothing else, nowhere else.
- Fail closed: any state you cannot classify is preserved untouched and
  reported, never staged, cleaned, pushed, or "fixed" by heuristic.
- A required skill that fails to load is LOOP HALTED, never a free hand to
  improvise the procedure.
