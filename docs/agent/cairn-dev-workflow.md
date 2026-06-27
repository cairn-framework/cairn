# The Cairn Dev Loop

How to iterate on cairn, using cairn. This is the canonical development loop for
this repo: a repeatable sequence that drives every code change through cairn's
own graph queries and gates. Load this file when starting a development
iteration, or invoke `/cairn-loop` to run it.

The premise is dogfooding. Cairn's job is to keep a declared architecture map in
sync with real code and to carry provenance for why the structure is shaped the
way it is. So the loop uses cairn to orient before touching code, to scope the
blast radius, and to gate the result. The framework verifies its own
development.

## Prerequisites

Cairn must reflect the current source, so build before you query:

```bash
cargo build --release
export PATH="$PWD/target/release:$PATH"   # so `cairn` resolves to this build
cairn --version
```

Rebuild whenever you change cairn's own reconcile or scan logic, otherwise the
graph you query is stale relative to the code you just wrote.

## The loop at a glance

| Phase | Question it answers | Primary tool |
|---|---|---|
| 1. Orient | What state is the graph in right now? | `cairn context`, `cairn lint` |
| 2. Scope | What does this change touch, and why is it shaped this way? | `cairn neighbourhood`, `cairn rationale`, `cairn dependents` |
| 3. Propose | What am I building, and what counts as done? | `cairn-propose` skill, or a decision artefact |
| 4. Implement | Make the change, keep the map honest | edit code + `cairn.blueprint` |
| 5. Test | Does new behaviour have a failing-then-passing test? | `cargo test`, fixtures under `tests/` |
| 6. Verify | Did I introduce drift or break the build? | `cargo` gates, `cairn scan`, `cairn hook all` |
| 7. Record | Why did this change happen? | decision artefact, `cairn rationale` |
| 8. PR | Is the change reviewable and on a path to merge? | GitHub PR, CI |
| 9. Merge | Is CI green and review satisfied? | CI gate, review threads, merge |
| 10. Continue | What is the next iteration? | backlog, `cairn lint`, loop to phase 1 |

Each phase has an exit criterion. Do not advance until it is met. The loop does
not stop at one change: phase 10 selects the next unit of work and returns to
phase 1. A clean iteration is code merged, CI green, `cairn scan` clean, and the
next task identified.

## Phase 1: Orient

Understand the graph before adding to it. Triage existing findings first so you
never confuse pre-existing drift with damage your change caused.

```bash
cairn context              # nodes, edges, artefacts, finding counts
cairn lint --json          # structured findings; errors block, warnings advise
cairn status               # active changes, snapshot state
```

Exit criterion: you know the current finding count (the target baseline is
zero), and you can name the node or nodes you are about to touch.

## Phase 2: Scope

Pull the neighbourhood around the change so you size the blast radius and respect
prior decisions.

```bash
cairn neighbourhood <node> --include-todos --include-changes
cairn rationale <node>                 # decisions, research, sources behind it
cairn dependents <node> --transitive   # who breaks if this node's interface moves
cairn depends <node> --transitive      # what this node leans on (cycle check)
```

If `cairn rationale` surfaces an accepted decision that constrains the area, work
within it or write a superseding decision in phase 6. Do not silently contradict
the provenance chain.

Exit criterion: you have a written, verifiable success criterion and you know
which modules and edges the change affects.

## Phase 3: Propose

Capture intent before code. Match the proposal weight to the change.

- Substantial work (new module, cross-cutting behaviour, interface change): use
  the `cairn-propose` skill to scaffold a change under `meta/changes/` with
  proposal, design, and tasks. Then drive it with `cairn-apply`.
- Small surgical work (a bugfix, a doc, a contained refactor): skip the change
  directory. State the success criterion inline and, if it alters structure,
  plan the decision artefact you will write in phase 6.

Exit criterion: the intent is recorded somewhere durable, not only in your head.

## Phase 4: Implement

Make the smallest change that satisfies the criterion (see the
`karpathy-guidelines` skill). Keep the map honest as you go:

- New source file: confirm it falls under an existing node's `path` in
  `cairn.blueprint`. If not, extend a node's `path` or declare a new Module.
- New cross-module call: add the edge in the blueprint,
  `from.id -> to.id "relationship label"`, and check `cairn depends` for cycles.
- User-facing CLI string: add it to `docs/design-system/copy.toml` and wire it
  via `copy::lookup(...)`. Do not hardcode messages in Rust.

Run `cairn scan` mid-flight whenever you add or move files, so orphans surface
immediately rather than at the gate.

Exit criterion: the change compiles and the blueprint still describes the tree.

## Phase 5: Test

Behaviour is not done until a test pins it. This is a coding workflow, so every
iteration that changes behaviour adds or extends a test, and a bugfix starts from
a test that reproduces the bug.

- Write the test first when fixing a bug: it should fail against the current
  code, then pass once the fix lands. That proves the test exercises the bug.
- Co-locate unit tests with the module under `#[cfg(test)]`; put cross-module and
  CLI-surface tests under `tests/` (the `cairn.tests` node).
- Run the focused test while iterating, then the full suite before moving on:

```bash
cargo test <name>          # fast loop on the test you are writing
cargo test                 # full suite must be green before phase 6
```

Exit criterion: new or changed behaviour has a test that fails without the change
and passes with it, and `cargo test` is green.

## Phase 6: Verify

This is the gate. Run the language gates and the cairn gates together.

```bash
# Code gates (run when Rust changed)
cargo build                                          # zero warnings
cargo clippy --all-targets --all-features -- -D warnings
cargo test

# Graph gates (run for every change)
cairn scan                 # zero findings is the target
cairn hook all             # exit 0 means the commit is safe
```

`cairn scan` clean means no orphaned files, no duplicate IDs, no dangling edges,
no interface drift. `cairn hook all` is the strictest gate and matches what CI
enforces. If either reports an error finding, read it and fix the underlying
cause. Do not bypass hooks (`--no-verify` and `SKIP=` are forbidden in this
repo).

Exit criterion: all gates green. A failing gate is a blocked iteration, not a
formality to wave through.

## Phase 7: Record

If the change altered structure or made a non-obvious tradeoff, write a decision
artefact so the next iteration can read why.

```yaml
---
id: dec.<short-name>
nodes: [<node.id>]
status: accepted
date: <YYYY-MM-DD>
---

Context, the decision, and the rationale.
```

Provenance in cairn is all-or-nothing once any node declares a `decisions`
pointer: the moment one decision exists, every leaf node without a covering
decision raises `CAIRN_PROVENANCE_NO_DECISION` (a non-blocking warning). So
wiring decisions into the blueprint is a deliberate, repo-wide commitment, not
an incremental one. Until the repo makes that commitment, keep decision records
in `meta/decisions/` as durable prose and confirm with `cairn rationale <node>`
once a `decisions` pointer is added.

Exit criterion: a reader can reconstruct why this iteration happened.

## Phase 8: PR

Commit, push, and open a reviewable pull request with plain git plus the GitHub
CLI (`gh`) or the GitHub MCP tools.

```bash
git checkout -b <type>/<scope>-<subject>
git add <files-for-this-unit>
git commit -m "<type>(<scope>): <subject>"
git push -u origin HEAD
gh pr create --fill                             # opens the PR
```

Keep one PR to one logical unit (target under 250 lines changed, hard cap 400).
Before submitting, run `/reforge` then `/debate` on the diff per `CLAUDE.md`. Skip
that only for a single-line documentation change.

Exit criterion: the branch is pushed, a PR is open, and CI has started.

## Phase 9: Merge

Drive the PR to a merged state. This is the path to merge, not a fire-and-forget
push.

1. Watch CI. Subscribe to PR activity so CI results and review comments wake the
   session, rather than polling. On a CI failure, re-diagnose, push a fix, and
   re-check. Treat "make CI green" as a loop with a terminal state.
2. Resolve review threads. Address each actionable comment with a commit or a
   reply explaining why not. Do not contradict an accepted decision to satisfy a
   comment; write a superseding decision instead.
3. Merge once CI is green and review is satisfied. Re-run `cairn scan` after any
   rebase so the merged commit is still clean.

Exit criterion: the PR is merged, CI is green on the target branch, and
`cairn scan` is clean against the merged state.

## Phase 10: Continue

The loop keeps working. Pick the next unit and return to phase 1. Choose the next
task in this priority order:

1. Any finding `cairn lint` reports against the merged state (clear drift first).
2. The next open item in the backlog (`bd ready`, or the repo's tracker).
3. A `CAIRN_PROVENANCE_NO_DECISION` or other advisory the loop has been deferring.
4. A small, self-contained improvement surfaced while doing the last iteration.

When the next candidate is blocked or deferred on a decision rather than on code,
do not treat that as a stop. If the block is a *knowable gap* (the only obstacle
is missing information or an unmade judgment that research can inform), resolving
the gap is itself the next unit. See "When the next unit is a knowable gap" below.

Keep each iteration small and shippable. Stop the loop only when one of these
holds, and record where and why so the next session resumes cleanly:

- the backlog is empty, `cairn lint` is clean, and no knowable gap remains to
  investigate;
- in an attended run, the loop has done the gap homework below and now needs a
  maintainer ratification it cannot make for them (the stop is a question with
  options, not a dead end); unattended, the loop never stops here, it persists the
  recommendation and continues (see "Running the loop autonomously");
- the only remaining work is a true external blocker the loop has already surfaced
  with what it needs and why; or
- you are told to stop.

"Blocked on a maintainer decision" is never a valid *first* response. It is valid
only after the loop has investigated, framed the options, and reduced the decision
to a ratification.

Exit criterion: the next iteration's success criterion is written, or a
decision-ready recommendation has been surfaced. Go to phase 1.

## When the next unit is a knowable gap

Some units are blocked not by code but by an open decision: a deferred or
maintainer-gated backlog item, an unresolved spec question, a design fork, or an
advisory the loop keeps skipping because the right direction is unclear. The
reflex to answer "it is blocked, you decide" wastes the loop. Classify the block
first:

- A **knowable gap** has, as its only obstacle, missing information or an unmade
  judgment that investigation and reasoning can inform. The answer is
  discoverable, or can at least be reduced to a small set of justified options.
- A **true external blocker** cannot be researched away: a missing credential or
  paid API, an upstream fix, an access or hardware dependency, a sanction only a
  human can grant, or a pure priority or taste call with no fact that would move
  it.

For a true external blocker, surface it immediately and plainly: state exactly
what is needed and why, then move to the next unit or stop. Do not manufacture
busywork around it.

For a knowable gap, resolving the gap is the unit. Run this sub-loop instead of
stopping:

1. **Investigate.** Pull the relevant code and artefacts (`cairn rationale`,
   `cairn research`, `cairn sources`), the prior decisions, and any external
   material the question cites. State the gap precisely: what is unknown, and why
   it blocks the unit.
2. **Debate.** Where the choice is genuinely contested, stress-test it rather than
   shipping a first draft. Run an adversarial pass (reformer versus conservative
   subagents; the `adversarial-decision-debate` and `decision-convergence-minutes`
   skills) so the options survive a critic.
3. **Recommend.** Produce a decision-ready package: the gap, two to four concrete
   options, the trade-offs of each, and a recommended option with its
   justification. Persist it the cairn-native way as a research or draft-decision
   artefact under `meta/` so the reasoning is not lost.
4. **Escalate with options.** Put the question to the maintainer with the
   recommendation attached, via `AskUserQuestion`. The maintainer ratifies a
   well-framed choice; they are not handed a blank blocker.
5. **Resume.** On the answer, ratify the decision (promote the draft, close or
   re-scope the item) and continue the loop with the now-unblocked unit or its
   follow-ups.

This keeps the loop honest: it never invents make-work, and it never dead-ends a
decision it could have framed. The maintainer's input is reserved for the actual
judgment, with the homework already done.

## Running the loop autonomously

Phases 8 and 9 above describe the human-reviewed path: one PR per unit, CI and a
reviewer gate the merge. When the loop runs unattended (for example, a long
`/cairn-loop` session grinding through the backlog), it commits verified
iterations straight to the working branch instead, and substitutes an internal
review gate for the human one. The bar does not drop; it moves in-process.

The internal gate is a small review DAG run with subagents before each commit:

1. **Strict build and lint.** Warnings are failures. Run `cargo build` and
   `cargo clippy --all-targets --all-features -- -D warnings` (zero output),
   `cargo test`, then `cairn scan --strict` (non-zero on any Error or Warning)
   and `cairn hook all`. Any non-green result blocks the commit.
2. **Code review (subagent).** Dispatch a review agent over the diff for
   correctness, convention violations, and missed edge cases. Treat its findings
   as blocking until addressed or explicitly judged out of scope.
3. **Simplify (subagent).** Dispatch a simplification pass over the diff for
   reuse, dead code, and naming. Apply what survives review.
4. **Re-verify.** Re-run step 1 after applying review and simplify changes. Loop
   2 through 4 until a pass produces no further blocking findings, then commit.

Only after the gate is clean does the iteration commit to the working branch and
move to phase 10. Each merge is still a real, durable action: keep commits small
and one-unit, and escalate to the maintainer (via `AskUserQuestion`) when a
choice is theirs to make rather than guessing.

A knowable gap behaves the same way unattended, with one difference at the
escalate step: there is no maintainer online to ratify. So the unattended loop
does the investigate, debate, and recommend work, persists the decision-ready
recommendation as a `meta/` research or draft-decision artefact and files a
deferred bead that links to it, then continues to the next available unit. It
never self-answers a choice that is the maintainer's, and it never halts the
whole run waiting for an answer it cannot get. The framed recommendation is
waiting, ratification-ready, when the maintainer next returns.

## When the loop reveals a finding it cannot clear

Sometimes phase 6 surfaces drift that is correct intent the blueprint has not
caught up to (a deliberately added module, a renamed path). That is the loop
working: the map disagreed with the code. Reconcile by updating the blueprint to
match the new reality, write the decision in phase 7 explaining the structural
move, and re-scan. Never silence a finding by ignoring the file; either the code
or the declaration is wrong, and the loop's job is to force that choice into the
open.
