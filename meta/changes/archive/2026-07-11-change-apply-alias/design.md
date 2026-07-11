# Design: change-apply-alias

## Approach

Add `"apply"` as a match arm aliasing `"archive"` in `run_change_command`.
Both verbs dispatch to the same `run_archive_command` path. Usage messages use
the invoked verb (`format!("usage: cairn change {cmd} <change-id>")`) so each
shows its own name.

## Changes

ADDED:
- `Some("apply")` match arm in `run_change_command` (`src/cli/mod.rs`).
- `test_cli_apply_aliases_archive` test verifying `apply` works and shows the
  correct usage error.
- `change-apply-alias` change directory with proposal/design/tasks.

MODIFIED:
- `change` command description: `"Manage changes: new, list, show, accept, apply, archive"`.
- Catch-all usage message: includes `apply`.
- `test_every_registered_command_has_description`: updated expected description
  string.
- 8 doc/guide files: `cairn change archive` switched to `cairn change apply` as
  primary verb.

REMOVED:
- (none)

RENAMED:
- (none)
