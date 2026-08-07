# Skills under .claude/skills/

This directory holds agent skills used by the Cairn workflow.
All skills are **tracked operational config**, not user-local preferences.

## Manifest-generated Cairn pack

| Asset | Purpose |
|---|---|
| `cairn-propose/` | Interactive proposal authoring for new changes. |
| `cairn-apply/` | Apply a change to the codebase. |
| `cairn-archive/` | Archive a completed change. |
| `cairn-explore/` | Explore the Cairn graph and query project state. |
| `cairn-dev/` | Development workflow for iterating on Cairn itself. |
| `cairn-loop-recovery/` | State-recovery procedure for the `/cairn-loop` command. |
| `cairn-loop-landing/` | Land and fail-closed merge procedure for `/cairn-loop`. |
| `.claude/commands/cairn-loop.md` | Native `/cairn-loop` command transport. |

These paths are rendered outputs owned by `tools/agent-pack/manifest.toml`.
Edit their canonical bytes under `tools/agent-pack/content/`, then run
`cargo run -p cairn-agent-pack -- --write`. Other skills in this directory
remain hand-authored development content.

## Boundary policy

`.claude/` in this repo is a tracked operational surface:

- `.claude/settings.json` and `.claude/settings.local.json` are tracked (project-level Claude Code config).
- `.claude/skills/` and `.claude/commands/` are tracked (operational workflow assets).
- Exception: `.claude/skills/impeccable/` is a checkout-local
  third-party install (`npx impeccable install`), gitignored by a
  tracked entry, never tracked or vendored; its generated context
  (`PRODUCT.md`, `DESIGN.md`, `.impeccable/`) is likewise gitignored.
  See the Tooling section of `todo.console-orchestration-ux-design`.
- `.claude/references/` and `.claude/stronghold/` are gitignored (session-local scratch).

If you find yourself editing skills under `~/.claude/` (your user-global directory) for
work that should apply to all Cairn contributors, move it into this repo's `.claude/skills/`
instead. Personal preferences belong in `~/.claude/`; project workflow belongs here.
