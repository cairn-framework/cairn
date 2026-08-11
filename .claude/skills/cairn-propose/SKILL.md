---
name: cairn-propose
description: Propose a new change: scaffold it and write proposal, design, and tasks in one step. Use when the user describes what to build or fix and wants a complete change proposal ready for implementation.
license: MIT
compatibility: Requires Cairn CLI.
metadata:
  author: cairn
  version: "1.0"
  generatedBy: "1.0"
---

Propose a new change: create it and write its artifacts in one step. The result
is `meta/changes/<name>/` holding proposal.md (the outcome, how it will be
proven, and what is excluded), design.md (how), and tasks.md (implementation
steps). Implementation is the `cairn-apply` skill.

**Input**: a change name (kebab-case) or a description of what to build.

**Steps**

1. **Settle what they want to build**: ask when the request is unclear:

   > "What change do you want to work on? Describe what you want to build or fix."

   Derive a kebab-case name from the description ("add user authentication" ->
   `add-user-auth`). Proceed only once the intended outcome is understood. If a
   change with that name exists, ask whether to continue it or start a new one.

2. **Create the change directory**
   ```bash
   cairn change new "<name>"
   ```
   This scaffolds `meta/changes/<name>/` with proposal.md, design.md, tasks.md,
   and an empty specs/ directory.

3. **Read the scaffolded files** to pick up the structure.

4. **Name the outcome and how it will be proven**

   Before writing the design, settle four things and put them in proposal.md.
   Everything downstream depends on them:

   - **Outcome.** What is observably true after this change that is not true now?
     Write it as a state of the world, not a list of edits. "Operators can retry a
     failed import from the run detail page", not "add a retry button".
   - **Acceptance boundary.** The nearest place that outcome becomes observable.
     For a library it may be a public function's return value; for a CLI, the
     command's output and exit code; for a UI, the rendered surface; for an
     operational change, the running instance. Name it concretely.
   - **Evidence.** What will be run or shown at that boundary to prove the
     outcome, and what result counts as proof. "`cairn scan` exits 0 with no
     ORPHANED findings on the fixture", not "tests pass". If the outcome is a bug
     fix, the evidence starts with a reproduction that currently fails.
   - **Exclusions.** What this change deliberately does not do, especially the
     adjacent work a reader would otherwise assume is included. Exclusions are
     what stop the change from growing during implementation.

   Prefer the smallest boundary that actually settles the claim. A boundary you
   cannot reach in this environment is worse than a narrower one you can: if the
   real boundary is out of reach, say so in the proposal and name what would be
   needed, rather than silently substituting a unit test for it.

5. **Write the remaining artifacts**: design.md (the approach and the delta
   operations: ADDED, MODIFIED, REMOVED, RENAMED), then tasks.md (the design
   turned into actionable checkboxes, ending with the task that produces the
   evidence named in step 4).

6. **Report**: the change name and location, the artifacts created, the outcome
   and its acceptance boundary, then prompt: "Ask me to apply the change to
   start working on the tasks."

**Artifact guidelines**

- Follow the scaffolded structure from `cairn change new`.
- Plain English, no em-dashes (use periods, colons, commas, or parentheses).
- Write for a staff engineer reading the change in six months: concise and
  focused.
- Cite accepted decisions (`cairn rationale <node>` lists what already binds the
  node) rather than restating them as if this proposal were deciding them.
- If context is critically unclear, ask. Otherwise make a reasonable decision and
  record it, to keep momentum.
