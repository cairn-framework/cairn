# cairn-dev loop mode

Run ONE iteration of cairn development: select one unit, land it as one squash
commit, emit exactly one terminal token, end. Never select a second unit.

This file plus exactly the required assets listed below is the sole normative
procedure for one iteration (`dec.loop-command-harness-model` clause 8, as
relocated by `dec.unified-cairn-dev-entry`). Adapter-native invocations such as
`/cairn-loop` are transport: they resolve here and add nothing. The descriptive
overview in `docs/agent/cairn-dev-workflow.md` is never normative.

Cairn does not repeat, schedule, retry, or supervise. The invoking user or
harness owns iteration (`dec.no-orchestrator`).

## Entry

Loop mode runs only on explicit selection by the user or the harness. The
`cairn-dev` router must never enter it from broad matching or ordinary
development intent.

## Required asset closure

Load each asset at the step that needs it. Any required asset that cannot be
loaded (missing, unreadable, or the harness cannot invoke it) is `LOOP HALTED`:
touch nothing, report which asset failed, end. Never improvise a procedure that
lives in an asset.

This ordered list is the closure. Adapters and campaign locks consume this list
and no other.

```text
skills/cairn-dev/references/loop-mode.md
skills/cairn-loop-scope/SKILL.md
skills/cairn-loop-implement/SKILL.md
skills/cairn-loop-recovery/SKILL.md
skills/cairn-loop-landing/SKILL.md
```

| Asset | Step | Declared exits |
|---|---|---|
| `cairn-loop-scope` | Scope | `SCOPED`, `REROUTED`, `LOOP HALTED` |
| `cairn-loop-implement` | Implement and test | `IMPLEMENTED`, `LOOP HALTED` |
| `cairn-loop-recovery` | Any preflight recovery row | `RECOVERED`, `LOOP HALTED` |
| `cairn-loop-landing` | Land and Cleanup | `ITERATION COMPLETE`, `LOOP HALTED` |

`REROUTED` means Scope found a prerequisite that must land first; the tracker
edits it produced ARE this iteration, so go straight to Land.

## Input: MISSION

The harness re-injects one fixed user message per iteration. Any text beyond the
invocation binds MISSION. It is identical every iteration, so work that must
evolve across iterations belongs in the tracker, not in MISSION. Precedence:

1. The preflight verdict always wins; MISSION never builds on unresolved state.
2. MISSION names a unit (todo slug, node id, finding code): select exactly that. A
   node id selects its first lint finding, else its top open todo. Already done,
   blocked, or quarantined: report why and output `LOOP EXHAUSTED`.
3. MISSION is a scope filter: apply normal selection restricted to that scope.
   Nothing in scope: report, output `LOOP EXHAUSTED`. Never select outside it.
4. MISSION describes new work: derive the slug canonically so every session
   computes the same one. Lowercase the mission text, replace runs of
   non-alphanumerics with single hyphens, take the first 40 characters, append `-`
   plus the first 6 hex chars of the SHA-256 of the exact raw MISSION string. If
   `todo.<slug>` exists, select it rather than re-creating it. Otherwise create
   branch `loop/todo.<slug>` FIRST, then materialise the todo on it with
   `$CAIRN todo new <slug> --node <id>`; split per the sizing rule if large and
   select the first open child. Writing a todo while parked detached would strand
   the next session in the fail-closed row.
5. No MISSION: default selection.

When adopting any surviving branch for a MISSION, confirm its slug maps to this
MISSION's unit; a mismatched branch belongs to the preflight table's generic
surviving-branch row.

**Node resolution is fail-closed.** Exactly two forms resolve: an exact node id
from the blueprint, or a file path falling under exactly one node's `path`.
Nothing else. Never infer a node from meaning, and never accept a suffix alias
here; the interactive ladder in `graph-navigation.md` is for forming a corrected
mission, not for resolving one. Unresolved: report that the mission needs a node,
list candidates with a ready-to-paste corrected mission, output `LOOP EXHAUSTED`.
Node choice drives Scope, deps, and provenance for every later iteration, so a
wrong anchor compounds silently.

## Isolation

Work only in the persistent worktree `../cairn-loop` (create once if absent:
`git worktree add --detach ../cairn-loop origin/main`; never remove it). Every
branch is prefixed `loop/`. Never touch the main checkout, non-loop branches,
other sessions' dirty files, or PRs from non-loop heads. Branch names are derived,
never invented: `loop/todo.<slug>`, `loop/<finding-code>.<node>`,
`loop/split.<slug>`.

## Repo bindings

`$CAIRN` is a textual placeholder, not a live shell variable: substitute the bound
value when composing every command, and give every executable shell block its own
assignment line. Nothing set in one tool call survives into the next.

- `CAIRN`: THIS repo (cairn develops itself) builds from source in Setup and binds
  `CAIRN=./target/debug/cairn`, run from the loop worktree. Shipped default:
  `CAIRN="$(command -v cairn)"`, then `[ -x "$CAIRN" ]`, then verify
  `"$CAIRN" --version` succeeds. Absent or failing at any step: touch nothing,
  report, output `LOOP HALTED`.
- Language gates: THIS repo runs `cargo build`, `cargo clippy --all-targets
  --all-features -- -D warnings`, and `cargo test` when Rust changed. Shipped
  default: none beyond `$CAIRN hook all`; the target repo declares its gates in
  its hook config and its own instructions.
- User-facing copy: THIS repo centralises every user-facing CLI string in
  `docs/design-system/copy.toml`; nothing is hardcoded in Rust source. Shipped
  default: wherever the target repo centralises its strings, per its own
  instructions. `cairn-loop-implement` resolves this binding rather than naming a
  path itself.

## Setup

Runs AFTER the preflight verdict and before the first `$CAIRN` command; preflight
needs only git and gh, and a fail-closed verdict builds nothing. If the verdict
adopted a surviving `loop/*` branch, check it out now (`git checkout
loop/<slug>`; clean tree, pure ref move) so Setup, Scope, and everything after
run against the recovered state, not origin/main. Then resolve the `CAIRN`
binding: in this repo, `cargo build` in `../cairn-loop` (incremental,
near-instant when no Rust changed; one profile throughout), then
`CAIRN=./target/debug/cairn`.

## Preflight: observe read-only, act on the FIRST matching row

No checkout, stash, clean, add, or commit during observation.

```bash
git fetch origin main
git status --porcelain
git branch --show-current
git for-each-ref 'refs/heads/loop/*' --format='%(refname:short) %(objectname)'
gh pr list --state open --json number,headRefName \
  --jq '.[] | select(.headRefName | startswith("loop/"))'
```

| State | Action |
|---|---|
| Dirty tree, on a `loop/*` branch whose slug maps to a known unit | Finishing that unit IS this iteration. Load `cairn-loop-recovery` section 1. No checkout until the tree is clean. On `RECOVERED`, continue at Verify. |
| Dirty tree, anything else (detached, unknown branch, unexplained files, intent unclear) | FAIL CLOSED. Touch nothing: no stage, stash, clean, commit, checkout, and no file writes either; the dirty tree is evidence. Report the state (`git status --short`, branch/HEAD, why unclassifiable) and output `LOOP HALTED`. Next sessions land in this row and re-report until the maintainer resolves it; that repeating halt IS the durable signal. |
| Clean; exactly ONE open `loop/*` PR | Recovery unit. Load `cairn-loop-recovery` section 2, then `cairn-loop-landing` at Cleanup. On `ITERATION COMPLETE` from landing, end. |
| Clean; MORE than one open `loop/*` PR | FAIL CLOSED. A prior run violated one-PR-per-iteration and the right merge order is a judgment call. Report all of them, output `LOOP HALTED`. |
| Clean; a `loop/*` branch whose tip matches a MERGED PR's headRefOid (`gh pr list --state merged --head <branch> --json headRefOid,mergedAt`) | Interrupted cleanup, not work. Load `cairn-loop-recovery` section 3. Preflight deletes branches ONLY here and in the discard-note case (section 4). On `RECOVERED`, continue preflight. |
| Clean; a `loop/*` branch covered by an existing `todo.recover-<slug>` on main | Load `cairn-loop-recovery` section 4 (status-branched: discard-authorized cleanup, ambiguous, or quarantined). Never delete or commit to a quarantined branch; if the worktree has it checked out, park off it (clean tree, pure ref move). On `RECOVERED`, continue preflight as if the branch were absent. |
| Clean; any other surviving `loop/*` branch (closed PR, no PR, or tip differs from merged PR) | Load `cairn-loop-recovery` section 5. Adopt (open todo or finding, then `RECOVERED`, continue at Scope) or quarantine (author `todo.recover-<slug>`, hand off to `cairn-loop-landing`, which emits the terminal token). A local branch ref is the only thing keeping those commits alive: never delete without a MERGED PR at the same tip or an explicit maintainer discard note in the todo. |
| Clean, parked detached, no unquarantined `loop/*` branches or PRs | Select fresh work. |

## Select ONE unit

After MISSION precedence: the first `$CAIRN lint --json` finding; else the top
open todo from `$CAIRN status` / `$CAIRN todos <node>`, skipping `blocked`; else
verify the stop evidence (lint clean, no open todos, no unquarantined `loop/*`
branches or PRs, clean park; quarantined branches and their blocked recover-todos
do not block exhaustion) and output `LOOP EXHAUSTED`.

"First" and "top" are defined, not incidental: sort findings by severity (errors
first), then file path, then line, then code; sort todos by slug. Tool output
order is not a contract.

This section owns selection. `$CAIRN brief` (no argument) is a context helper,
not a selector. When the selected unit is the top open todo, brief's fused output
(decisions, contract) feeds Scope. Treat any gate list brief prints as advisory;
Verify below is the gate authority. Never let brief's pick override lint-first
order, and never pass brief an argument to target a todo: arguments resolve bead
ids, not todos or nodes.

**Sizing rule.** The unit must fit one small reviewable PR. Too big: this
iteration IS the decomposition. Create `loop/split.<slug>` first, create sub-todos
on it with `$CAIRN todo new`, set the parent `blocked` with body line
"blocked on sub-todos: <ids>" (the iteration completing the last child flips the
parent to `done`), and land that decomposition as the single commit.

**Artefact rule.** Every todo or `meta/` artefact this loop creates is written ON
the unit's `loop/*` branch, never while parked detached, and reaches main only
through Land. A state that forbids landing also forbids writing: report instead.
Nothing is ever left uncommitted in the loop worktree.

## Load the unit body

Before Scope, read the selected unit's todo body at `meta/todos/todo.<slug>.md`
and validate it: the file exists, is readable, parses as frontmatter plus body,
and its `node:` matches the node you resolved.

Its Scope, Depends on, and Acceptance sections BIND this iteration. Selection
yields a slug and a status; without the body, the unit's actual contract never
reaches the session and the iteration optimises against a guess.

If the body is missing, unreadable, or its `node:` disagrees with the resolved
node, that is `LOOP HALTED`: report the path and the mismatch, write nothing.

For a lint-finding unit there is no todo body; use the finding and its
`cairn remediate <code>` plan instead, and skip this step.

## Scope

Load `cairn-loop-scope`. It owns orientation queries, decision compliance, the
success criterion, and the reroute rule. On `SCOPED`, continue. On `REROUTED`, the
tracker edits it produced are this iteration: go to Land.

## Implement and test

Load `cairn-loop-implement`. It owns branch derivation and creation, the smallest
change satisfying the criterion, blueprint upkeep, and the test rule. On
`IMPLEMENTED`, continue.

## Verify: the gate

Run the bound language gates. Always `$CAIRN scan` (zero findings) and
`$CAIRN hook all` (exit 0). Fix the cause of any failure. Never bypass hooks.

## Record

If structure changed or a non-obvious tradeoff was made, write a decision artefact
in `meta/decisions/`.

## Land and Cleanup

Load `cairn-loop-landing`. It owns explicit-path staging, tracker completion, one
logical commit, push, one PR, the two-lens pre-submit review, and the fail-closed
squash-merge with re-verification. Always pass `slug` and `CAIRN`. On the normal
Land path do not pass `pr`. On the open-PR recovery row, pass the existing `pr` and
enter at Cleanup. Pass its returned token through as this iteration's final line.

## End

Summarise the unit, success criterion, nodes touched, test added, final scan
finding count, and PR and merge status. Then output exactly one token:

- `ITERATION COMPLETE`: unit landed, or safely deferred with a blocked todo.
- `LOOP EXHAUSTED`: no selectable work remains, or the immutable MISSION can never
  progress in this run.
- `LOOP HALTED`: fail-closed state needs the maintainer.

The token is the FINAL line, alone, verbatim, with the summary before it.
Adapters pass it through unchanged and append nothing.

If blocked on a decision only the maintainer can make: author the researched
recommendation as a `meta/` artefact plus a blocked todo, land them through Land
as the single commit, report, output `ITERATION COMPLETE`. Never wait mid-loop.

## Guardrails

- Zero `cairn scan` findings is the target; a finding blocks the iteration.
- Behaviour without a test is not done.
- Never contradict an accepted decision without writing a superseding one.
- One iteration, one unit, one squash commit on main. A growing PR means split.
- Branch deletion requires merged evidence: a MERGED PR at the same tip, or an
  explicit maintainer discard note. Nothing else, nowhere else.
- Fail closed: any state you cannot classify is preserved untouched and reported,
  never staged, cleaned, pushed, or "fixed" by heuristic.
- A required asset that fails to load is `LOOP HALTED`, never a free hand.
