---
node: cairn.kernel.cli
status: open
created: 2026-07-12
---

# Change Read Surface

gh:#241

Change lifecycle progress and gate status remain invisible (active-change
definition conflicts are fixed by todo.status-active-changes-bug).

## Evidence (verified on main, 2026-07-12)
- `cairn change show <id> --json` has no `progress` or `gate` fields.
- No read-only gate preview: `cairn change accept --dry-run <id>` parses
  `--dry-run` as the change id and runs the mutating gate; no dry_run in
  src/cli or src/changes.
- Gate itself is still cargo-hardcoded (see todo.accept-language-aware-gates).

## Task
Add a change progress/gate read surface: task completion counts and a gate
preview (`--dry-run` or equivalent) on `cairn change show`/`accept`.
