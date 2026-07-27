---
node: cairn.kernel.cli
status: done
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

## Review note (2026-07-16)

Severity correction, source-verified: `cairn change accept --dry-run <id>` parses `--dry-run` as the change id and runs the full gate battery (src/cli/accept/mod.rs runs gates before change_id is used; dispatch takes the literal token), but `accept` does not mutate the tree (apply/archive are separate commands). So this is a UX/orientation gap (wasted gate cycle, confusing lint of a bogus id, missing preview), not a data-integrity hazard.

## Resolution (2026-07-28)

- `cairn change show`/`change list` JSON carries `progress: {completed, total,
  remaining}`, parsed from the change's `tasks.md` checkboxes by the shared
  `artefacts::count_tasks` (the same parser behind
  `CAIRN_CHANGE_TASKS_COMPLETE`). The human `change show` prints
  `Tasks: <n>/<m> complete` when the change tracks any task.
- `cairn change accept --dry-run [<id>]` resolves the gate recipe and lists every
  step as `planned` with the command it would run, spawning nothing;
  `gate_outcome` is `preview`. `--dry-run` is a registered flag on that command,
  so it is no longer swallowed as the change id, and a mistyped flag is still
  rejected.
- Query wire payloads bump to `schema_version` 4 per `dec.query-json-schema-version`.
