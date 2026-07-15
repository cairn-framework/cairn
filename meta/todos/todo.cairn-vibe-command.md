---
node: cairn.kernel.cli
status: done
created: 2026-07-15
related: [dec.loop-command-harness-model]
---

# Cairn Vibe Session command

gh:#330

`/cairn-vibe` is an attended director-style slash command, the cousin of
`/cairn-loop`. It picks a themed release-sized block of todos, asks a single
up-front question for the user to approve the theme, todos, semver, and release
disposition, then executes hands-off: landing each unit as an independently
reviewed PR and releasing per the approved plan.

## Done

2026-07-15: Shipped as `.claude/commands/cairn-vibe.md` via PR #330, reviewed
by independent reviewer subagents.
