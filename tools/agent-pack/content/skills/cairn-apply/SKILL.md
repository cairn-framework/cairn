---
name: cairn-apply
description: Apply a change to the codebase. Use when the user wants to implement the tasks in a change, run verification gates, and mark the change as complete.
license: MIT
compatibility: Requires Cairn CLI.
metadata:
  author: cairn
  version: "1.0"
  generatedBy: "1.0"
---

Apply a change: implement its tasks, prove the outcome, and accept it.

**Prerequisites**

- A change directory exists at `meta/changes/<change-id>/`
- proposal.md, design.md, and tasks.md are present

**Steps**

1. **Identify the change to apply**

   If the user doesn't specify a change:
   ```bash
   cairn change list
   ```

2. **Read the change artifacts**

   Read proposal.md, design.md, and tasks.md. The proposal names the outcome and
   the acceptance boundary you have to reach; treat that as the definition of
   done, not the task checkboxes.

3. **Implement the tasks in order**

   For each unchecked task, implement it and mark it complete in tasks.md. Run
   whatever narrow test or check covers just that task as you go. Do not commit
   after every task: the repository decides commit granularity, and most expect
   one reviewable commit per change.

4. **Run the host repository's own gates**

   This is the target repository's call, not Cairn's. In order of authority:

   1. Instructions the repository gives you (`AGENTS.md`, `CLAUDE.md`,
      `CONTRIBUTING.md`, a documented script such as `scripts/*gates*.sh`).
   2. The `gates:` list in `cairn.config.yaml`, if the project configures one.
   3. The project's ordinary build and test commands for its language.

   Never run a battery from another language. If you cannot tell what the gates
   are, ask rather than guessing.

5. **Prove the outcome at its own claim boundary**

   A green internal check is evidence about the code, not about the claim. Prove
   the thing the proposal actually promised, at the nearest place it becomes
   observable:

   | Claim | Insufficient | Proof at the boundary |
   |---|---|---|
   | Behavioural change | unit test passes | run the command or call the API and show the new observable result |
   | UI change | component test passes | load the surface and confirm the rendered state |
   | Generated output | generator has no errors | diff the regenerated artefact and show the delta is the intended one |
   | Operational change | config parses | exercise the path in a running instance, or state plainly that you could not |
   | Bug fix | new test passes | reproduce the original failure first, then show that reproduction now passes |

   If a boundary is genuinely out of reach in this environment, say so and name
   what would close it. Do not upgrade a partial check into a completeness claim.

6. **Run Cairn's acceptance boundary**

   ```bash
   cairn change accept <change-id>
   ```

   This resolves and runs the project's language battery, then checks Cairn's own
   invariants: `cairn lint`, and the suggested-edges queue.

   `accept` picks the battery in this order: an explicit `gates:` list in
   `cairn.config.yaml` wins (an explicit empty list deliberately runs nothing);
   otherwise a Rust project falls back to a built-in cargo battery; every other
   language reports an informational note that no default battery exists and
   continues to the language-agnostic checks. That note means "configure `gates:`
   if you want Cairn to run them", not "this project has no gates". You are still
   responsible for step 4.

   **CC002** means the change's `meta/changes/<change-id>/suggested-edges.json`
   still holds entries whose `triage_state` is `pending`. Those are machine
   suggested blueprint edges, and Cairn will not let them enter the graph without
   a human decision. Resolve it by setting each entry's `triage_state` to
   `accepted`, `rejected`, or `deferred` (with a `triage_note` when the reason is
   not obvious), then re-run `accept`. Do not delete the queue to clear the gate.

7. **Report**

   Summarise what was implemented, the evidence for the outcome, and anything you
   could not prove.

**Mutation authority**

You may, without asking: edit source, tests, and docs inside the change's scope;
create and edit files the design calls for; run builds, tests, linters, and
read-only commands; run `cairn` commands that write inside `meta/` (`todo set`,
`change accept`).

Ask first, every time: `git push`, force-push, or anything that rewrites shared
history; merging, tagging, or releasing; publishing a package; deleting a branch
or a file you did not create in this change; editing CI, secrets, or credentials;
any network call with a side effect; any command touching infrastructure or
production data; `git reset --hard`, `git clean`, or `git stash drop`.

Never: bypass a hook (`--no-verify`, `SKIP=`), weaken a gate to make it pass,
delete or skip a failing test to go green, or commit a secret.

When an operation is denied or the situation is ambiguous, preserve state and
stop cleanly. Leave the working tree as you found it or better, never partially
mutated. Report the blocked outcome plainly: what you were about to do, why you
stopped, what you need in order to continue. A blocked outcome that preserved
state is a success; a guessed mutation is not. Cairn does not adjudicate any of
this: these are instructions to you, not an authorization engine in the binary.

**Working in parallel (optional)**

If your harness runs several workers, Cairn can help you carve out disjoint
scope: `cairn frontier` shows what is buildable now, node `path` claims in the
blueprint show which files a node owns, and `cairn deps <node>` shows what a
change would ripple into. Two units whose owning nodes share no path and no edge
are usually safe to work in parallel.

That is a scope hint, nothing more. Cairn does not schedule, dispatch, or
supervise agents, and it owns no worker pool, queue, or concurrency limit. Your
harness decides whether to parallelise, how many workers exist, and what each one
runs. Whoever invoked the work stays responsible for integrating the results,
proving each claim at its own boundary, and returning control.

**Guardrails**

- Do NOT skip tests. They are the contract with future maintainers.
- Do NOT hardcode values that belong in the project's design tokens or config.
- Do NOT use em-dashes in user-facing copy.
- Prefer updating existing files over creating new ones.
- Fix problems at their source, not the symptom.
