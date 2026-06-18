# Session Handoff — 2026-06-17

Branch: `main`, synced with origin, working tree clean. `dev` retired entirely
(merged, protection removed, deleted local + `origin/dev`, `origin/HEAD → main`).

## What Was Done

- **CI retargeted to `main`** (`38826eb`): `ci.yml` + `dogfood.yml` had triggered
  only on the dead `dev` branch, so `main` got no Rust CI. Both now gate `main`.
- **Graphite (`gt`) removed** (`1f0fd06`): deleted `docs/agent/graphite.md`, the
  CLAUDE.md Graphite section, gt commands in `cairn-dev-workflow.md` +
  `cairn-loop.md` (now plain git + `gh`), and the two graphite tests in
  `command_reference_consistency.rs`. Removed local `.git/.graphite_repo_config`,
  archived `~/.claude/skills/graphite-pr`. (`/forge-pr` is a separate gh-based
  plugin command, retained.)
- **`dev` retired**: fully merged into `main` (0 unique commits). Stripped its
  classic branch protection, deleted local + remote. `main` stays protected via
  the "Copilot review for default branch" ruleset.
- **Beads**: filed `cairn-xiw` (P2), `cairn-1w3` (P3); closed `cairn-ddn` +
  `cairn-sly` (dev-branch tickets, moot post-retirement). `cairn-v1t` is **done**
  (epic closed, PR #135: decisions wired into the provenance graph).

## Current State

- `main` synced + clean. Gates green: `scripts/pre-archive-rust-gates.sh` +
  `scripts/dogfood.sh`. 10 open beads, **no P0/P1**.

## Next: pick a high-ticket item (priority is the user's call)

`cairn-v1t` (prior high-ticket) is done. Open P2 backlog:

| Bead | Lift | Summary |
| --- | --- | --- |
| `cairn-d7s` | large | OMP integration: cairn diagnostics server (LSP/watch-server) |
| `cairn-87n` | medium | Add test-coverage gate to cairn hooks |
| `cairn-xiw` | small | Strict lint+format gate for webui assets (real app.js violations) |
| `cairn-dyc` | small | Plan bd upgrade 1.0.4 to 1.0.5+ (mostly analyzed) |
| `cairn-kb0` | infra | GitHub Pages deploy blocked by env protection (repo-admin) |

P3 spikes: `cairn-t59` (graph-root fingerprint), `cairn-2z9` (beads as task layer),
`cairn-a8z` (state-dependent coverage gate), `cairn-1w3` (lint-strictness warning),
`cairn-y1m` (bead to GitHub-issue sync).

Strongest "high-ticket" candidates: **`cairn-d7s`** or **`cairn-87n`**.

## Agent Entry Points

- `cairn context`; `cairn get <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
