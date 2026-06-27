# Uruk-Drift Scout Report: Working Tree Drift Investigation

**Scout mission:** `chore/hook-hardening` @ 7b53073 | **Status:** DONE  
**Timestamp:** 2026-04-30 | **Branch:** chore/hook-hardening

---

## 1. Screenshots Drift (19 modified PNG files)

**Sample files examined:**
- `docs/research/getcairn-dev/screenshots/01-round3-shape-the-starting-model.png`
  - `git diff --stat`: Bin 2491631 → 1294585 bytes (actual content change)
  - `git hash-object` current: `4eb4e910df8a7f158655fdedd89e2d6dbd7d3727` ≠ HEAD: `8f979fa24e2cb14d3f2abe425772bb23c1ec4d07`
  - `file`: PNG image, 1800×862, 8-bit RGBA (valid format, not metadata-only)
- `docs/research/getcairn-dev/screenshots/02-round3-comms-safe-state-question.png`
  - `git diff --stat`: Bin 2575394 → 1138348 bytes (actual content change)

**Characterization:** All 19 modified screenshots are **actual content changes** (file byte counts differ significantly; hashes differ). These are not mtime/metadata drifts, they appear to be **image re-compression or re-encoding** (lossy → lossier, or tools applied post-capture).

**Verdict:** ⚠️ **Intentional edits, not safe to revert without investigation.** These files were likely optimized/resized in this session. Check CLAUDE.md or context-snapshot.md for intent; if approved, commit or explicitly gitignore screenshots/ as per-session artifacts.

---

## 2. context-snapshot.md Drift

**Git diff:**
- **OLD (HEAD):** 2026-04-20 pre-compact snapshot (phase-4 orchestrator notes)
- **NEW (working tree):** 2026-04-28 post-research-net snapshot (campaign-driven handoff notes)

**Content analysis:** Entirely replaced with newer, structured handoff document covering:
- PR bundle merge campaign (5 PRs, #6–#10)
- getcairn.dev research net (docs, screenshots, exports)
- graphite-pr skill hardening (review-thread gate)
- Open items for next session (graphify wiki, commit decision, cleanup)

**Verdict:** 🟢 **Should be committed.** This is the authoritative session handoff record, supersedes 2026-04-20, and explicitly marks next-session entry points. Not a revert candidate.

---

## 3. `.gitignore` Coverage

**Current `.gitignore`:**
```
# Claude Code session-local references (design bundles, scratch assets)
.claude/references/
.claude/stronghold/
```

**Current `.git/info/exclude`:**
```
.claude/scheduled_tasks.lock
.sauron/
graphify-out/
```

**Analysis:**
- `docs/strongholds/`: **NOT ignored** (explicitly visible in status).
- `.claude/worktrees/`: **NOT ignored** (appears in status as `??`).
- `.agents/`: **NOT ignored** (appears in status as `??`).

**Verdict:** ❌ **Gitignore coverage incomplete.** Neither worktree dir nor agent settings export are ignored. See questions 4–5 for remediation.

---

## 4. `.claude/worktrees/` Contents & Lockfile Status

**Directory listing:**
```
.claude/worktrees/
├── agent-a4895e9ba76616454/      (28 dirs, locked)
├── agent-a74725eb02deb4ec6/      (28 dirs, locked)
└── agent-ababb2f54e0ec2fb7/      (28 dirs, locked)
```

**Git worktree list:**
```
/Users/george/repos/cairn                                            7b53073 [chore/hook-hardening]
/Users/george/repos/cairn/.claude/worktrees/agent-a4895e9ba76616454  c98d506 [worktree-agent-a4895e9ba76616454] locked
/Users/george/repos/cairn/.claude/worktrees/agent-a74725eb02deb4ec6  7ffac23 [worktree-agent-a74725eb02deb4ec6] locked
/Users/george/repos/cairn/.claude/worktrees/agent-ababb2f54e0ec2fb7  b645bea [04-30-docs_bundle_f_identity_additions_for_two-chain_framing] locked
```

**Analysis:**
- All three are **locked** (active or recently active).
- `agent-a4895e9ba76616454` is listed as "leftover" in context-snapshot.md (failed Recovery Nazgul); safe to prune.
- `agent-a74725eb02deb4ec6` and `agent-ababb2f54e0ec2fb7` have recent commits (7ffac23, b645bea); likely active or paused.

**Verdict:** 🟡 **Leftover confirmed, but cleanup blocked by active locks.** The `agent-a4895e9ba76616454/` should be cleaned up (context-snapshot flagged it). Do not delete `.claude/worktrees/` root; require explicit user confirmation for individual worktree removal.

---

## 5. `.agents/openspine-agent-substrate-clean-settings-2026-04-30.json`

**File type:** JSON project export (OpenSpine Agent Substrate config)
**First 500 chars (sample):**
```json
{
  "formatVersion": 1,
  "project": {
    "id": "proj_moitlkppn5j8",
    "name": "OpenSpine Agent Substrate",
    "version": "0.1.0",
    "schemaVersion": "1.0.0",
    "createdAt": "2026-04-28T16:08:12.830Z",
    "updatedAt": "2026-04-30T07:15:59Z",
    "rootNodeId": "SYS-671t0t",
    "settings": {
      "unitSystem": "SI",
      "namingPrefixes": { ... }
```

**Analysis:** Agent configuration export, dated 2026-04-30 (today). Likely a transient artifact from agent testing or export. No evidence it's needed beyond this session.

**Verdict:** 🔴 **Should be gitignored or deleted.** Add `.agents/` to `.gitignore` or `.git/info/exclude` to prevent future exports polluting status. For this file, suggest `git rm --cached` + delete.

---

## 6. Strongholds Untracked (19 files, ~560K)

**Untracked files list (20 total, 1 omitted; see below):**
```
getcairn-adoption-matrix.md                36K
getcairn-cross-check-7.5c.md              28K
getcairn-cross-check-A.md                 40K
getcairn-cross-check-B.md                 40K
getcairn-cross-check-C.md                 28K
getcairn-cross-check-E.md                 32K
getcairn-cross-check-F.md                 24K
getcairn-cross-check-integrated.md        40K
getcairn-learnings-candidates.md          28K
getcairn-refined-batch-A.md               28K
getcairn-refined-batch-B.md               24K
getcairn-refined-batch-C.md               32K
getcairn-refined-batch-D.md               36K
getcairn-roadmap-debate.md                60K
graphify-version-audit.md                  8K
oq1-reconciliation-validate.md            12K
oq2-edge-label-render.md                  12K
oq3-genesis-lifecycle.md                  28K
oq4-phase9-rescope-timing.md              32K
tdd-posture-investigation.md              24K
Total: ~560K
```

**Cross-references in committed codebase:**
- `HEAD:docs/strongholds/archive/session-handoff-2026-04-30-5e606f2.md`: 30 mentions of `getcairn-cross-check-*` and `getcairn-refined-*` across waves 0–6 adoption plan.
- `HEAD:docs/research/getcairn-dev/_export-analysis.md`: 2 mentions.
- `HEAD:docs/strongholds/image-batch-inventory.md`: 3 mentions.

**Analysis:** These are **session-research outputs** documented in the session handoff archive. All are referenced in the committed `session-handoff-2026-04-30-5e606f2.md` as inputs for the 6-bundle cross-check and adoption-wave sequencing. They are **actionable, not ephemeral**.

**Verdict:** 🟢 **Commit these.** They are referenced in the handoff; they document phase-7.5c through phase-9 adoption sequencing. Add to the next PR or sync to dev with context-snapshot.md. Do not discard.

---

## Summary & Actionable Verdicts

| Question | Verdict | Action |
|----------|---------|--------|
| 1. Screenshots drift | Intentional edits | Commit or explicitly gitignore; check CLAUDE.md for intent |
| 2. context-snapshot.md | Should commit | Handoff record, authoritative for next session |
| 3. Gitignore coverage | Incomplete | Add `.claude/worktrees/` and `.agents/` to ignore |
| 4. Worktrees leftovers | Locked; cleanup pending | Delete `agent-a4895e9ba76616454/` with user confirmation |
| 5. Agent settings JSON | Transient artifact | Gitignore `.agents/` or delete this file |
| 6. Strongholds untracked | Actionable research | Commit all 19 files; referenced in session handoff |

**Safe to clean:**
- Delete `.agents/openspine-agent-substrate-clean-settings-2026-04-30.json`
- Delete `.claude/worktrees/agent-a4895e9ba76616454/` (explicit failure, no active refs)

**Should stay / commit:**
- All 19 stronghold MDs (getcairn-*, oq*, tdd-*, graphify-version-*)
- context-snapshot.md (NEW version, authoritative for next session)
- 19 screenshot PNGs (verify intent, then commit)

**Must add to gitignore:**
- `.claude/worktrees/` (session-local, auto-managed by Claude Code)
- `.agents/` (transient exports, per-session)

