# Hook installation

## Acceptance criteria

- `cairn hook install` writes an executable marked Cairn `pre-commit` hook in the repository Git hooks directory.
- `cairn hook install --pre-push` targets `pre-push` instead.
- Reinstalling an owned hook is idempotent and does not rewrite it.
- An existing unmarked hook is never overwritten and produces a clear refusal.
- `.pre-commit-config.yaml` prevents raw hook installation and explains the conflict.
- `core.hooksPath` determines the hook directory.
- `cairn hook status` reports installed or absent for the selected hook.
- `cairn hook uninstall` removes only an owned hook and refuses unowned files.
- Lifecycle commands work without a Cairn blueprint and support human and JSON output.
