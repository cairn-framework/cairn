---
name: "Cairn Dev Loop"
description: Run one iteration of cairn development, driven by cairn's own graph queries and gates
category: Workflow
tags: [workflow, cairn, dogfood]
---

Run the Cairn Dev Loop: a coding workflow that develops cairn, using cairn. The
full loop is documented in `docs/agent/cairn-dev-workflow.md`; load it for phase
detail. This command is the orchestrator. It does not stop at one change: each
iteration ends by selecting the next unit of work and looping.

**Input**: the argument after `/cairn-loop` is a description of the change to
make, or a count of iterations to run (e.g. `/cairn-loop 5`). With no argument,
draw the next unit from the backlog (`bd ready`) or, if empty, from the first
finding `cairn lint` reports. Ask with the **AskUserQuestion tool** only when the
next unit is genuinely ambiguous or the work would alter an accepted decision.

**Setup**

Cairn must reflect current source. Build and put it on PATH first:

```bash
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

**Steps**

Work the ten phases in order. Do not advance until a phase's exit criterion is
met. Track progress with `bd` (this repo's tracker), not TodoWrite.

1. **Orient** — `cairn context` and `cairn lint --json`. Record the baseline
   finding count (target zero) and name the node(s) you will touch. Triage any
   pre-existing findings before adding work.

2. **Scope** — for each target node: `cairn neighbourhood <node>
   --include-todos --include-changes`, `cairn rationale <node>`, and
   `cairn dependents <node> --transitive`. Respect accepted decisions. Write a
   verifiable success criterion.

3. **Propose** — for substantial work, invoke the `cairn-propose` skill to
   scaffold a change, then `cairn-apply`. For small surgical work, state the
   success criterion inline and plan any decision artefact.

4. **Implement** — make the smallest change that satisfies the criterion
   (`karpathy-guidelines` skill). New files must fall under a node `path` or get
   a new Module. New cross-module calls get a blueprint edge. CLI strings go in
   `docs/design-system/copy.toml`. Run `cairn scan` whenever you add or move
   files.

5. **Test** — add or extend a test for changed behaviour; for a bugfix, write the
   test first so it fails before the fix and passes after. Unit tests live with
   the module under `#[cfg(test)]`, cross-module tests under `tests/`. Run
   `cargo test` to green.

6. **Verify** — the gate. Run `cargo build`, `cargo clippy --all-targets
   --all-features -- -D warnings`, and `cargo test` when Rust changed; always run
   `cairn scan` (zero findings) and `cairn hook all` (exit 0). Fix the cause of
   any error finding. Never bypass hooks.

7. **Record** — if structure changed or a non-obvious tradeoff was made, write a
   decision artefact (`meta/decisions/`). Confirm intent with `cairn rationale`.

8. **PR** — commit with plain git, push, and open one PR per logical unit with
   `gh pr create` or the GitHub MCP tools. Before submitting, run `/reforge` then
   `/debate` on the diff per `CLAUDE.md`.

9. **Merge** — drive the PR to merged. Subscribe to PR activity so CI and review
   events wake the session; on a CI failure, fix and re-push until green. Resolve
   review threads. Merge once CI is green and review is satisfied, then re-run
   `cairn scan` against the merged state.

10. **Continue** — pick the next unit (a `cairn lint` finding first, else the
    backlog, else a deferred advisory or an improvement spotted this iteration)
    and go to phase 1. Stop only when the backlog is empty and `cairn lint` is
    clean, when a gate is blocked on a maintainer decision, or when told to stop.

**Output**

Per iteration, summarize: the success criterion, the nodes touched, the test
added, the final `cairn scan` finding count, CI and merge status, and the next
unit selected. If a gate or CI blocked, report the finding and where it stuck
rather than waving it through.

**Guardrails**

- A clean `cairn scan` (zero findings) is the target state. A finding is a
  blocked iteration, not a formality.
- Behaviour without a test is not done. Reconcile drift by fixing the code or the
  blueprint, never by ignoring a file.
- Do not contradict an accepted decision without writing a superseding one.
- The loop keeps working across iterations, but each merge is a real outward
  action: keep PRs small, and escalate to the maintainer when a choice is theirs
  to make.
