---
node: cairn.brownfield
status: done
created: 2026-07-10
---

# Init Wire Agents Md Flag
Setup currently ends with a manual step: "paste `.cairn/AGENTS.md` into your agent's
instructions." That is done-with-you, not done-for-you, and it is the exact mechanism
(a pasted convention file) that already fails users whose agents ignore CLAUDE.md. Add a
wire step to init (e.g. `cairn init --wire CLAUDE.md` or an interactive prompt) that
appends or references the `.cairn/AGENTS.md` section in the project's agent instructions
file automatically and idempotently. Surfaced by an adversarial offer review as the
weakest link in the effort variable of the value equation.

## Resolution (2026-07-12)

Implemented `cairn init --wire [<file>]` in `src/cli/commands/project.rs`. Auto-detects
CLAUDE.md then AGENTS.md (creating the latter if neither exists), or targets an explicit
relative path. Appends a short reference block (not the full guide content) bracketed by
sentinel comments for idempotency. The guide stays the single source of truth in
`.cairn/AGENTS.md`; the wired block just points the agent at it. `--wire` is rejected
with `--from-code` since the brownfield path returns before wiring would run.
