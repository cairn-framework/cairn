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

Archive a completed change: move it to the dated archive.

**Prerequisites**

- All tasks in tasks.md are marked complete
- `cairn change accept <change-id>` has passed
- The project's own quality gates pass (the repository's instructions, its
  `gates:` config, or its ordinary build and test commands for its language)

**Steps**

1. **Identify the change to archive**

   If not specified, list active changes:
   ```bash
   ls meta/changes/
   ```

2. **Verify completion**: every task in `meta/changes/<change-id>/tasks.md` is
   checked. When one is not, stop and report: a change with pending tasks stays
   active.

3. **Run final verification**

   Re-run the project's own gates so the archive lands on a clean tree. Use the
   same commands the repository documents for itself, in its own language.

4. **Archive the change**

   Run the archive command. It validates the change against the current graph,
   applies the blueprint delta, refreshes generated output, logs the archival,
   and moves the change into the dated archive directory:
   ```bash
   cairn change archive <change-id>
   ```

5. **Update any references**: check `cairn.blueprint` (changes blocks),
   README.md, and other documentation; update or remove stale references.

6. **Commit the archive**

   Read the touched paths from `git status --short` first, then stage exactly
   those (never `git add -A` or `git add .`, which would sweep in unrelated
   untracked files):
   ```bash
   git add <each path the archive touched>
   git commit -m "archive(change): move <change-id> to archive"
   ```

**Guardrails**

- Preserve the change directory structure in the archive (proposal.md,
  design.md, tasks.md, specs/).
- The archive is permanent history: never edit archived changes.
