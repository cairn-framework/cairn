---
node: cairn.kernel.changes
status: done
created: 2026-07-11
---

# Archive Change After Merge

Merged changes stay listed as active: `fix-webui-harness-defects` and
`change-apply-alias` merged (PRs #256, #257) but their change directories
were never applied, so `cairn status` still reports them as active changes.
In iteration 2 (2026-07-11) this caused `cairn change accept` to be run
against the wrong (already-merged) change id.

Two halves, both in scope:

1. Workflow (semi-deterministic): after the PR merges, run
   `cairn change apply <id>` so the change directory moves to
   `meta/changes/archive/`. The harness-mode procedure lives in the
   `cairn-loop-landing` skill (archive-before-commit on the unit branch so
   completion and archival land in the same squash commit).
2. Detector (deterministic): a new info-level scan finding (e.g.
   `CAIRN_CHANGE_TASKS_COMPLETE`) emitted when an active change directory
   has every checkbox in `tasks.md` checked. Message should suggest
   `cairn change apply <id>`. Register the code in
   `docs/registries/error-codes.md` and add copy under `[findings.codes]`
   in `docs/design-system/copy.toml`.

Acceptance:
- Backfill: apply the two stale merged changes so status shows only
  genuinely active ones.
- Test: a fixture change with all tasks checked yields the finding; an
  unchecked task yields none.
- `cairn scan` clean on this repo after backfill.
