# Design: hook-git-install

## Approach

Keep lifecycle operations in the existing `hook` CLI command. Resolve the
repository's Git directory with `git rev-parse`, honour `core.hooksPath`, and
write only a marked Cairn script for `pre-commit` or `pre-push`.

The marker is a stable comment in the generated script. Install refuses to
replace any unmarked existing file, including a pre-commit-managed hook when
`.pre-commit-config.yaml` is present. Uninstall deletes only a marked script.
Status reports the selected hook path and whether the marker is present.
Human output is concise and JSON output uses a stable command/status/data
envelope. No operation runs the project scanner, allowing installation in a
clean Git repository.

## Changes

ADDED:
- Git hook lifecycle helpers and focused unit tests.

MODIFIED:
- `cairn hook` dispatch, help, command docs, integration contract, and copy.

REMOVED:
- None.

RENAMED:
- None.
