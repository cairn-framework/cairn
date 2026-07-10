---
node: cairn.brownfield
status: open
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
