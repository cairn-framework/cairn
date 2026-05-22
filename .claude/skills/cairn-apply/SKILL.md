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

Apply a change - implement tasks, run verification, and mark complete.

I'll guide you through implementing the tasks in a change directory.

**Prerequisites**

- A change directory exists at `meta/changes/<name>/`
- proposal.md, design.md, and tasks.md are present

**Steps**

1. **Identify the change to apply**

   If the user doesn't specify a change, list available changes:
   ```bash
   ls meta/changes/
   ```

2. **Read the change artifacts**

   Read proposal.md, design.md, and tasks.md to understand the scope.

3. **Implement tasks in order**

   For each unchecked task in tasks.md:
   a. Implement the task
   b. Run relevant tests: `cargo test` or language-specific tests
   c. Mark the task complete in tasks.md
   d. Commit: `gt modify -m "feat(...): ..."` or `git commit`

4. **Run verification gates**

   Before marking complete, run:
   ```bash
   cargo build          # zero warnings
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --check
   cargo test
   ```

5. **Run acceptance**

   ```bash
   cairn accept <change-id>
   ```

   This runs the accept-time gates (e.g., CC002 for suggested edges).

6. **Mark complete**

   Update tasks.md to mark all tasks complete.
   Summarize what was implemented.

**Guardrails**

- Do NOT skip tests - they are the contract with future maintainers
- Do NOT introduce hardcoded values - use the design system tokens
- Do NOT use em-dashes in user-facing copy
- Prefer updating existing files over creating new ones
- Fix problems at their source, not the symptom
