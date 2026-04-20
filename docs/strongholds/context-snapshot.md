# Context Snapshot — 2026-04-20 (pre-compact)

**Session role:** orchestrator (user: "manage CFLX dev, don't develop stuff yourself unless really have to get involved").

## Active tasks

| # | Task | Status |
|---|---|---|
| 1 | Land phase-4-hooks merge | DONE — commit `f7f00e9` on `dev` |
| 2 | Debate §3 gap question | DONE — recorded at `.orchestrator/decision-phase4-section3.md` |
| 3 | Ship phase-6-multi-target | IN PROGRESS — parser fixed, 16/27 tasks pending |
| 4 | Ship phase-7-mcp | QUEUED |
| 5 | Ship phase-8-summariser | QUEUED |
| 6 | Ship phase-9-brownfield | QUEUED |
| 7 | Final verification + memory sync | QUEUED |

## Key decisions (this session)

1. **Phase-4 §3 is shipped, not gapped.** `/palantir-debate` round 2 verified all five §3 checkmarks against Apply commit `0a3faf1`. Stash entries on `dev` were superseded drafts, not canonical work. No micro-phase; no rescope.
2. **Ownership.** George handed orchestrator ownership of phases 4-9. Use `/palantir-debate` for judgment calls; record each decision under `.orchestrator/`.
3. **Orchestrator-only role.** User clarified: manage cflx; do not implement directly. Parser-fix handover (below) was the one exception — justified because cflx/opencode was stuck in a local minimum for 50+ minutes on a parser bug I had a diagnosis for.
4. **`.orchestrator/` is gitignored** (commit `d5f482d`). Scratch files live outside the working tree.

## Current git state

| Location | State |
|---|---|
| `dev` (main repo) | clean; phase-4 landed at `f7f00e9` |
| Phase-6 worktree `/Users/george/.local/share/cflx/worktrees/cairn-ba64eedb/phase-6-multi-target` | dirty with parser fix + codex's prior §2 work (8 modified, 4 new) |
| Other worktrees | not actively worked |

### Phase-6 worktree dirty files

```
 M Cargo.lock / Cargo.toml
 M openspec/changes/phase-6-multi-target/tasks.md  ← premature §6 verification marks stripped
 M openspec/registries/error-codes.md
 M src/reconcile/{code,mod}.rs
 M src/scanner/{config,mod,state}.rs               ← my parser rewrite in config.rs
 M tests/kernel.rs
?? src/reconcile/{go,python,target,typescript}.rs  ← codex's reconcilers
```

## Unfinished work (phase-6)

Tasks file: `openspec/changes/phase-6-multi-target/tasks.md`

- [x] 1.x Target model (done)
- [x] 2.x Reconcilers incl. canonical extraction §2.6 (done — codex shipped TS/Python/Go + trait dispatch)
- [x] 3.1 state keyed by node+target / 3.2 migrate single→multi hash (done)
- [ ] **3.3 divergence detection** (parser fix unblocked this; test passes)
- [ ] **3.4 intentional asymmetry markers** (parser fix unblocked this; `matches()` works)
- [ ] **3.5 hash-stability tests** (private symbols, comments, formatting, source order)
- [ ] **3.6 contradiction + tension tests**
- [ ] **4.x CLI + output** (get/files/lint/scan target-level; JSON schemas; snapshots)
- [ ] **5.x docs** (path-list reconciliation; languages; state format/migration)
- [ ] **6.x verification** (build/clippy/fmt/test/test --locked/cflx validate --strict)

## Parser fix (what landed in the worktree, needs codex to build on)

**File:** `src/scanner/config.rs` — YAML state machine for `intentional_asymmetry:` blocks.

**Bug:** the `-` handler pushed a new blank `IntentionalAsymmetry` on every target-path entry, destroying node/contract_role/reason. Diagnosis at `.orchestrator/phase-6-yaml-parser-diagnosis.md`.

**Fix shipped:**
- Added `let mut in_asymmetry_targets = false;` state flag
- Guarded outer `targets:` handler with `!in_asymmetry` so nested `targets:` inside `intentional_asymmetry:` stops exiting asymmetry mode
- New `-` handler branches on `in_asymmetry_targets`: if true, push path into current asym; if false, finalize + start new block
- Removed `current_asymmetry_target: Option<PathBuf>` variable entirely
- Stripped 3 DEBUG eprintlns that codex had left in `matches()`
- Simplified EOF flush to single `if !asym.node.is_empty()` push

**Verification:** `test_divergence_intentional_asymmetry_ct002` passes; full gate battery (build, clippy -D warnings, fmt, test) green at time of fix.

## Outstanding lessons / bugs filed in head

1. **cflx Ctrl+C doesn't propagate to opencode child.** Exiting cflx TUI (Esc or q) leaves opencode running in background under PID it spawned. Workaround: `pgrep -fl opencode` and kill the runaway PID before Edit tool gives "file modified by linter" errors. Worth filing upstream.
2. **cflx TUI display lag.** Merge `f7f00e9` landed on `dev`, but TUI kept showing "[merge wait]". Restarting cflx cleared it.
3. **codex merge sweeps orchestrator scratch.** First phase-6 queue swept `.orchestrator/phases-6-9-scope.md` into `dev`. Fixed by gitignoring `.orchestrator/` (commit `d5f482d`).

## Next action when session resumes

1. Confirm codex API is available again (user said "~10 min, maybe").
2. In cflx TUI: select `phase-6-multi-target` → Space → F5 to resume Apply. Codex will pick up from the parser-fixed worktree and continue §3.5 onward.
3. Keep 15m cron monitor running (`15m monitor the cairn tmux session ...`).
4. Do **not** hand-edit code in worktrees. If codex stalls on another local minimum for 30+ minutes, diagnose via scout, write fix-sketch to `.orchestrator/`, then hand back — don't implement.

## Memory index entries updated

- `project_cairn_phase_ownership_4_to_9.md` (new)
- existing: `project_cflx_workflow.md`, `project_codex_sandbox_preauth.md`, `reference_tmux_cairn_session.md`
