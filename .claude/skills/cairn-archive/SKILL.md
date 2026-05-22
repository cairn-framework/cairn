---
name: cairn-archive
description: Archive a completed change. Use when the user wants to move a finished change to the archive and clean up.
license: MIT
compatibility: Requires Cairn CLI.
metadata:
  author: cairn
  version: "1.0"
  generatedBy: "1.0"
---

Archive a completed change - move it to the archive and finalize.

I'll help you archive a change that has been fully implemented and accepted.

**Prerequisites**

- All tasks in tasks.md are marked complete
- `cairn accept <change-id>` has passed
- Quality gates pass: `cargo build`, `cargo clippy`, `cargo fmt --check`, `cargo test`

**Steps**

1. **Identify the change to archive**

   If not specified, list active changes:
   ```bash
   ls meta/changes/
   ```

2. **Verify completion**

   Check that all tasks are complete:
   ```bash
   cat meta/changes/<change-id>/tasks.md
   ```

3. **Run final verification**

   Ensure the project is in a clean state:
   ```bash
   cargo test
   cargo fmt --check
   ```

4. **Archive the change**

   Move the change directory to the archive:
   ```bash
   mkdir -p meta/changes/archive
   mv meta/changes/<change-id> meta/changes/archive/
   ```

5. **Update any references**

   Check if the change is referenced in:
   - `cairn.blueprint` (changes blocks)
   - README.md
   - Other documentation

   Update or remove stale references.

6. **Commit the archive**

   ```bash
   git add -A
   git commit -m "archive(change): move <change-id> to archive"
   ```

**Guardrails**

- Do NOT archive a change with pending tasks
- Do NOT archive a change that fails acceptance gates
- Preserve the change directory structure in the archive (proposal.md, design.md, tasks.md, specs/)
- The archive is permanent history - do not edit archived changes
