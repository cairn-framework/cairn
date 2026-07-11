---
node: cairn.kernel.cli
status: open
created: 2026-07-10
---

# Hook Git Install

Quick look-into, no planning or research done yet. `cairn hook
structural|interface|tension|all` exists as runnable commit gates, but
nothing installs them: users (and this repo) wire them into git hooks by
hand via pre-commit config. Investigate a `cairn hook install` subcommand
(or equivalent) that writes a pre-commit/pre-push git hook running
`cairn hook all`, so findings must be addressed before committing.
Consider whether the cairn plugin (agent-side install) should do this
during `cairn init` or as an explicit opt-in, and how it coexists with an
existing `.pre-commit-config.yaml` or `core.hooksPath`.

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
