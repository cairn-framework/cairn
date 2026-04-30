# graphify version audit (2026-04-29)

## Versions

| | Version | Released |
|---|---|---|
| Installed (`pip show graphifyy`) | **0.4.23** | 2026-04-18 |
| Latest on PyPI | **0.5.5** | 2026-04-29 (today) |

Gap: 32 patch releases (0.4.24 → 0.5.5): about 11 days of fixes. Repo: https://github.com/safishamsi/graphify

## Verdict on the 3 observed issues

| Issue | Fixed upstream? | Fix release | Confidence |
|---|---|---|---|
| 1. cost.json shows 0 input/output tokens | **Maybe**: no release note explicitly mentions chunk-aggregator token rollup. v0.5.5 ships an opt-in **Kimi K2.6 backend** for semantic extraction, which would replace the Claude-subagent path entirely (and hence the broken aggregator). Worth filing upstream as a separate bug. | n/a (not addressed by name) | low |
| 2. 73 `file_type: "concept"` warnings | **Yes: directly fixed.** Release notes for **v0.5.5**: *"concept file_type fix (#601), nodes with file_type: concept no longer produce validation warnings."* | 0.5.5 | high |
| 3. Oversized image / sips handling | **No**: no release between 0.4.23 and 0.5.5 mentions vision pre-processing, image resize, or `sips`. The patched per-subagent prompt with `dangerouslyDisableSandbox: true` is still load-bearing. | n/a | high |

Bonus: weak Community 0 cohesion (0.02) is not addressed by a named clustering fix in the changelog. 0.5.1 fixed node-ID collisions (two `utils.py` getting merged), which can affect community quality but is unlikely to be the root cause for a docs-only corpus.

## Other relevant fixes you'd inherit by upgrading

- **0.4.24** Skill cleans up `.graphify_chunk_*.json` temp files at end of run (#464).
- **0.4.27** `graph_report.md` is now byte-identical across runs (eliminates infinite commit churn from post-commit hook).
- **0.4.32** `/graphify <path>` now correctly places `graphify-out/` *inside* the target path; git hooks work from worktree checkouts (#516).
- **0.5.0** `graphify clone` (clones into `~/.graphify/repos/<owner>/<repo>`: explains why the SKILL doc references that path even though it doesn't exist yet on this machine), `merge-graphs`, shrink guard.
- **0.5.1** Portable `source_file` paths in `graph.json`; node-ID collision fix.
- **0.5.2** Hotfix for Claude Code v2.1.117+: the PreToolUse hook from `graphify claude install` was silently broken (Glob/Grep matcher → Bash matcher).
- **0.5.3** Splits AST and semantic caches into `cache/ast/` and `cache/semantic/`, old flat `cache/` is still readable as fallback. **Important migration note for our existing 3.2 MB flat `graphify-out/cache/`.**
- **0.5.4** SSRF DNS-rebinding hardening on `safe_fetch` and `download_audio`.
- **0.5.5** Phantom god-node fix (member-call callees), `graphify update` remembers scan root via `graphify-out/.graphify_root`, optional Kimi K2.6 backend.

## Worktree / scratch state to clean

| Path | Size | Safe to delete? | Notes |
|---|---|---|---|
| `/Users/george/repos/cairn/.claude/worktrees/agent-a4895e9ba76616454/` | n/a (worktree) | **NOT YET**: `git worktree list` reports it as `locked claude agent agent-a4895e9ba76616454 (pid 84577)`. PID 84577 is the lock owner. Verify the process is dead before unlocking. The worktree's only diff vs `dev` is untracked `docs/research/` (228 files / 15k deletions in the working tree, no commits to lose). Once confirmed orphaned: `git worktree unlock <path> && git worktree remove <path>` (or `--force`), then `git branch -D worktree-agent-a4895e9ba76616454`. | leftover from earlier failed Recovery Nazgul, NOT created by graphify |
| `~/.graphify/` | does not exist | n/a | only created when you use `graphify clone` (a 0.5.0+ feature). Nothing to prune. |
| `/Users/george/repos/cairn/graphify-out/cache/` | 3.2 MB (~430 files) | **KEEP**: incremental SHA256 semantic cache. Per-issue instruction. Note: on upgrade to 0.5.3+, graphify reads this flat layout as a one-time migration fallback, then writes new entries into `cache/ast/` and `cache/semantic/`. Safe to leave in place. | |
| `/Users/george/repos/cairn/graphify-out/memory/` | 4 KB (1 file) | **KEEP**: Q&A feedback loop, extracted on `--update`. | |
| `/Users/george/repos/cairn/graphify-out/.graphify_python` | 44 B | **KEEP**: interpreter pinning marker (`/opt/homebrew/opt/python@3.14/bin/python3.14`). | |
| `.graphify_chunk_*.json` (root or graphify-out) | 0 files | n/a | no stragglers, the v0.4.24 skill cleanup is already in our skill copy or the run completed cleanly. |
| `$TMPDIR/cairn-snapshots-*` | many | leave alone | these are **cflx/cairn snapshot artifacts**, not graphify. Out of scope for this audit. |

Net actionable cleanup from graphify alone: zero bytes: graphify is well-behaved on this run. The locked agent worktree is real, but it's a Recovery Nazgul leftover, not a graphify artifact.

## Update command

```bash
pip install --upgrade graphifyy
```

If `pip` errors with the SSL/cache warning seen during this audit (corrupt `~/Library/Caches/pip` ownership), use:

```bash
pip install --no-cache-dir --upgrade graphifyy
```

After upgrading, optionally re-run `graphify claude install` to refresh the PreToolUse hook (v0.5.2 changed the matcher; check whether our `~/.claude/skills/graphify/SKILL.md` already reflects this, if it's a graphify-managed install, run the install; if it's a Mordor-adapted skill, reconcile via `/lineage` first).

## Risks / migration notes

1. **Cache layout migration (0.5.3).** First post-upgrade run will read the existing flat `graphify-out/cache/` as fallback and start writing into `cache/ast/` + `cache/semantic/`. Expect cache hit rate to recover gradually. No data loss.
2. **PreToolUse hook (0.5.2).** If the graphify hook is registered in `settings.json`, it has been silently dead on Claude Code v2.1.117+. Either re-run `graphify claude install` post-upgrade or accept that the graph-context reminder hasn't been firing.
3. **Skill drift.** Our `~/.claude/skills/graphify/SKILL.md` is likely the upstream-shipped one (graphify ships its own skill). The 32 releases include several skill-prompt fixes (path handling, chunk cleanup, Windows UTF-8, OpenCode install). If we've **patched** SKILL.md to do the sips-resize dance for oversized images, **that patch will be overwritten** by `graphify install` post-upgrade. Diff before reinstalling, port the sips patch forward.
4. **Issue 3 not fixed.** Even at 0.5.5, oversized-image handling still requires our manual sips intervention. Worth filing as an upstream issue with a repro from the 29 oversized files we hit.
5. **Issue 1 not fixed.** The cost.json zero-token problem is not addressed by name in the changelog. File upstream separately. The Kimi K2.6 path (0.5.5, opt-in) is an unrelated alternative worth evaluating but does not fix the Claude-subagent aggregator.
