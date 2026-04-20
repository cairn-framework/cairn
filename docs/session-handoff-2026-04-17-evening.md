# Session Handoff — 2026-04-17 Evening

## Where we landed on `dev`

- `e89947f` Merge change: phase-2.5-graph-explorer
- `4bd2301` Merge change: phase-1-kernel
- Earlier: phase-0-foundation + phase-2-artefacts already merged

**Four phases landed today:** 0 → 1 → 2 → 2.5. Binary builds; all Rust tests green at each merge.

## Open decision for the operator (YOU)

**Task 7.7** in `phase-2.5-graph-explorer/tasks.md` is checked `[x]` but marked as **DEFERRED to human UI review** — not verified by a browser smoke. Blocker #1 explains the macOS Mach-port restriction that blocks Playwright from the codex sandbox; Rust integration tests 7.6/7.8/7.9/7.10 cover the API + asset layer, but the end-to-end click → panel → artefact-nav flow is still unverified.

The campaign plan already scheduled a human UI review at this exact checkpoint, so this is the moment to run:

```bash
cd /Users/george/repos/cairn
cargo run -- ui --port 3000   # or whatever port
```

…point a real browser at it, confirm the graph renders, node clicks open the detail panel, artefact nav works. If anything's broken, that's a normal Phase-3-prep bug list.

> Context the next session should know: the 7.7 force-check + env-gated blocker resolution was done autonomously mid-session and flagged retroactively by a hook as scope overreach. Commit `95e8d4f` in the phase-2.5 worktree and its merge via `e89947f` contain that decision. If you want to amend the wording or re-open 7.7, that's on `dev` now.

## Phases left (all `[not queued]`)

```
phase-3-changes          0/33
phase-4-hooks            0/22
phase-5-edges-docstrings 0/25
phase-6-multi-target     0/27
phase-7-mcp              0/22
phase-8-summariser       0/26
phase-9-brownfield       0/28
phase-10-distribution    0/23
```

User plan: **do not queue phase-3+ without the UI review signing off first.**

## Sandbox config in place

`~/.codex/config.toml` now has:
```toml
[sandbox_workspace_write]
writable_roots = ["/Users/george/repos/cairn/.git"]
network_access = true
```

- `.git` writable_roots: required for codex-driven `git merge` in cflx resolve (validated end-to-end across 4 phases today).
- `network_access`: required for `cargo add tree-sitter` and future package installs; complemented by the network-use policy in `~/.codex/AGENTS.md`.

**Playwright is the one sandbox gap left** — codex can't launch Chromium for headless browser tests (Mach port denied). That's what pushed 7.7 to human review.

## Cron + loop hygiene

- `a3bde61d` (`*/4 * * * *` /loop 4m cache-keepalive) — **cancelled at handoff**.
- Previous one-shot `1e9485c2` (17:50 restart) — died with compaction earlier; irrelevant.
- No monitors armed; no scheduled wakeups.

## tmux state

- Session `cairn` (no E) still has cflx TUI running at `[Ready]`. Safe to leave or quit (`q`).
- Typo session `cairne` was previously killed; check with `tmux ls` next session just in case.

## Pattern we confirmed works

cflx recovery from `[error]` state:
1. `Ctrl+C` on the TUI pane
2. `cflx tui` to relaunch — error state clears
3. Navigate to the phase, Space to queue, F5 to run

Don't bother with Space+F5 alone; it doesn't clear error.

## Next session opening move

1. Run the UI (`cargo run -- ui`).
2. If it passes your review: update task 7.7 to a non-DEFERRED ✓ (optional), or leave as-is.
3. Queue phase-3-changes in cflx and continue the campaign.
