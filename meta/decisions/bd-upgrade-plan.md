---
id: dec.bd-upgrade-plan
nodes:
  - cairn.root
status: accepted
date: 2026-06-23
revisit_triggers:
  - "A concrete need requires a bd feature only present in 1.0.5+ (e.g. server-validated custom issue types)"
  - "cairn adopts a Dolt remote (refs/dolt/* on origin), making cross-clone migration safety load-bearing"
  - "bd ships a release that drops or changes the jsonl export contract this repo relies on"
---

# bd upgrade plan: keep jsonl-in-git, pin export config, defer the version bump

## Context

This repo tracks work in beads (`bd`). The installed tool is **1.0.4** (2026-05-07);
the latest stable is **1.0.5** (2026-05-28), and an unreleased 1.0.6 line adds
cross-clone Dolt remote merge-safety work. Bead `cairn-dyc` asked for a deliberate
upgrade plan rather than a blind `bd upgrade`, because crossing minor versions here
touches schema migrations and an opt-in default flip that can silently break this
repo's bead sync.

Two facts establish the blast radius:

- **Sync model is jsonl-in-git, not Dolt remote.** `git ls-remote origin 'refs/dolt/*'`
  returns nothing: there is no Dolt remote on `origin`. Cross-machine bead interchange
  is the git-committed `.beads/issues.jsonl`. bd does run a local Dolt server as its
  storage engine (`.beads/dolt` in a normal clone; in a git-worktree layout the store
  lives in the main worktree's `.beads/` and bd resolves to it from any worktree), but
  that store is local-only and is never pushed to `origin`.
- **Auto-export is currently unpinned.** `bd config show` reports `export.auto` and
  `export.git-add` as *defaults* (`true` on 1.0.4), not values set in
  `.beads/config.yaml`. On 1.0.5+ the auto-export default flips to OPT-IN (false). If
  we upgrade without pinning, the jsonl stops auto-refreshing and auto-staging, and
  git-based bead sync breaks silently.

Re-checking the claimed upgrade synergies against the *installed* 1.0.4 surface
shrinks the case for upgrading now:

| Claimed 1.0.5 synergy | Reality on 1.0.4 |
|---|---|
| `bd github` native sync (pull/push/sync) | **Already present in 1.0.4.** No upgrade needed. |
| `bd create --defer <date>` -> deferred status | **Already present in 1.0.4** (`--defer` on create and update). |
| Opt-in jsonl / Dolt-canonical formalization | Cosmetic here: we already treat Dolt as canonical and jsonl as a passive export. |
| `types.custom` server-validated custom issue types | New in 1.0.5. We use a `spike` *label* today; `status.custom` is already available on 1.0.4. |
| Ergonomics (per-id close reasons, `--skip-labels`, count-only JSON) | Minor quality-of-life. |

The two synergies most often cited (GitHub sync, defer) are already in hand, so the
upgrade buys mainly `types.custom` plus ergonomics, against the cost of crossing
migrations 0040-0042 (FK/cascade) and the unreleased dependencies-PK reshape (0050).

## Decision

1. **Keep jsonl-in-git.** Dolt stays the local storage engine; `.beads/issues.jsonl`
   stays the committed, human-diffable, upsert-only projection used for cross-machine
   sync. Do not adopt a Dolt remote at this time.
2. **Pin auto-export now**, independent of any upgrade, by writing to
   `.beads/config.yaml`:

   ```yaml
   export.auto: true
   export.git-add: true
   ```

   On 1.0.4 these match the defaults (a safe no-op functionally), but they pre-harden
   the repo so a future 1.0.5+ upgrade cannot silently disable jsonl sync.
3. **Defer the bd version bump.** Stay on 1.0.4 until a revisit trigger fires. The
   high-value synergies (`bd github`, `--defer`) are already available; the remainder
   does not justify crossing the migration boundary today.
4. **Re-scope `cairn-y1m`** (bead<->GitHub sync spike) to evaluate the *existing*
   `bd github` rather than building a custom ForgeDock-style label layer. The open
   question is the maintainer's: whether to accept GitHub issues as a second source of
   truth at all (divergence risk) and, if so, one-way (bead -> GH) vs bidirectional.
5. **Do not adopt `types.custom` yet.** Continue using the `spike` label. Adopting
   `--defer` to declutter `bd ready` of long-lived P3 spikes is recommended but
   optional and is not mandated by this decision.

## Deterministic upgrade procedure (when a trigger fires)

Run this exact sequence; do not `bd upgrade` blindly.

1. **Pre-flight.** Commit/push all bead work. Confirm `.beads/config.yaml` pins
   `export.auto: true` and `export.git-add: true` (done by this decision).
2. **Single-machine path (this repo today, no Dolt remote).**
   - `bd upgrade` (or reinstall) to the target version.
   - `bd doctor` and resolve any migration-content-skew warning.
   - Verify auto-export: make a trivial bead edit, confirm `.beads/issues.jsonl`
     refreshes and is git-staged.
   - Commit the refreshed jsonl.
3. **Dolt-remote path (NOT applicable here; documented for completeness).** If a
   `refs/dolt/*` remote ever exists, all clones must `bd dolt push` + `pull` to the
   same state BEFORE upgrading. One designated migrator runs the upgrade with
   `BD_ALLOW_REMOTE_MIGRATE=1` and pushes; every other clone upgrades then pulls.
   Independently-migrated clones that cross migrations 0040-0042 / 0050 un-synced
   become permanently un-mergeable (Dolt hard-refuses), and 1.0.5's forward-schema
   guard hard-fails older bd against a newer-migrated DB.
4. **Re-evaluate `types.custom`** for promoting the `spike` label to a real type once
   on 1.0.5+.

## Rationale

- The dominant near-term risk is the opt-in auto-export flip, and it is fully
  mitigated by pinning config now, regardless of when the upgrade happens.
- The cross-clone migration hazard is dormant because this repo has no Dolt remote;
  pinning the procedure in writing keeps it dormant safely if a remote is ever added.
- Deferring the bump avoids change for change's sake: the synergies that would have
  justified it are already on 1.0.4.

## Consequences

- `.beads/config.yaml` now pins `export.auto`/`export.git-add`; the jsonl sync is
  upgrade-safe.
- `cairn-dyc` is satisfied and closed; `cairn-y1m` is re-scoped to "evaluate
  `bd github`" with a maintainer do/don't decision still pending.
- The repo stays on bd 1.0.4 until a revisit trigger fires; this document is the
  runbook for that future upgrade.
