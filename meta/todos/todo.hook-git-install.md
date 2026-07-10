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
