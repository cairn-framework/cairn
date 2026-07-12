---
node: cairn.kernel.cli
status: done
created: 2026-07-10
---

# Hook Git Install

Implemented explicit Cairn Git hook lifecycle commands. `cairn hook install`
writes an executable owned hook, `status` reports ownership, and `uninstall`
removes only owned hooks. The commands honour `core.hooksPath`, refuse
pre-commit framework conflicts and unowned hooks, support `--pre-push`, and
work without a blueprint. Installation is never implicit during `cairn init`.

## Scoping (added 2026-07-11 after backlog review flagged this under-specified)

Deliverable: a `cairn hook install` subcommand that writes a git hook (default
pre-commit, `--pre-push` option) invoking `cairn hook all`, plus `cairn hook
uninstall` and `cairn hook status`. Must be idempotent and must not clobber an
existing hook.

Coexistence decisions to make during the research pass:

- If `.pre-commit-config.yaml` exists, do not write a raw git hook; instead print
  the pre-commit stanza to add (pre-commit owns `.git/hooks`). Detect and warn.
- If `core.hooksPath` is set to a non-default dir, write there, not `.git/hooks`.
- Decide init-time behaviour: `cairn init` should NOT silently install a hook;
  offer it as an explicit opt-in prompt or a documented `cairn hook install` step.

Acceptance: `cairn hook install` on a clean repo writes an executable pre-commit
hook running `cairn hook all`; re-running is a no-op; running with an existing
non-cairn hook refuses and explains; `cairn hook status` reports installed/absent;
`cairn hook uninstall` removes only the cairn-authored hook. Added per
docs/skills/cairn-add-cli-command (dispatch, `--json`, command_reference tests).
