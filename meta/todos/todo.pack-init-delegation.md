---
node: cairn.kernel.cli
status: blocked
created: 2026-07-25
---

# Pack Init Delegation

Second child of `todo.agent-pack-claude-bootstrap`. Closes the first-run gap so
a fresh repository reaches a wired pack with one command.

## Priority

P1. The adoption slice the parent exists for; useless before the installer
exists, so it follows it.

## Depends on

`todo.pack-install-lifecycle`.

## Scope

- `cairn init --wire` delegates to the pack installer instead of writing skill
  files itself, per `dec.agent-pack-packaging` clause 4. One code path owns
  emission, and the manifest records it.
- `cairn init --wire` is the documented greenfield path.
- Support `cairn init --from-code --apply --wire [path]`.
- After a successful brownfield apply, backfill `.cairn/AGENTS.md` and the
  rendered pack, then wire the selected agent instructions file.
- Never scaffold or wire after a failed brownfield apply.

## Acceptance

- Fresh greenfield and brownfield repositories both reach an installed and
  wired Claude pack with one command.
- Re-running each path is idempotent.
- A failed brownfield apply leaves no scaffold, no pack, and no wire block.
- Modified user files are reported and never overwritten.
- README, quickstart, agent setup, command help, copy, and snapshots describe
  the behaviour that actually shipped.

