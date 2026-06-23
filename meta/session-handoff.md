# Session Handoff: 2026-06-23 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main` (`7391934`).

## What Was Done

- **`cairn-dyc` shipped** (PR #137, `7391934`): deterministic bd upgrade plan.
  - New decision `meta/decisions/bd-upgrade-plan.md` (`dec.bd-upgrade-plan`,
    node `cairn.root`): keep jsonl-in-git, pin `export.auto`/`export.git-add` in
    `.beads/config.yaml` so a future bd 1.0.5+ upgrade (auto-export default flips
    to opt-in) cannot silently break jsonl sync, and **defer** the version bump.
  - Rationale: `bd github` (pull/push/sync) and `--defer` already exist in 1.0.4,
    and this repo has no Dolt remote (`refs/dolt/*` empty), so the cross-clone
    migration hazard is dormant. The doc carries the deterministic upgrade runbook
    for when a revisit trigger fires.
  - `cairn-y1m` re-scoped (note appended): evaluate the existing `bd github`
    rather than a custom label layer; the do/don't on a second source of truth is
    a **maintainer** decision, still OPEN.
  - Bead closed; jsonl regenerated via `bd export` (not hand-edited).
- **Pre-existing CI red cleared** (PR #138, `4cc6839`): `fix(map): use is_ok_and
  in test_coverage`. CI's `dtolnay/rust-toolchain@stable` floated to **rust
  1.96.0**, whose `clippy::pedantic` flagged `map(..).unwrap_or(false)` as
  `clippy::map_unwrap_or` in `src/map/test_coverage.rs` (from `cairn-87n`). This
  was blocking CI on **every** PR. Verified green on local 1.96.0 (`cargo clippy
  --all-targets --all-features -- -D warnings` exit 0; `map::test_coverage` tests
  pass).

## Current State

- `cairn lint` / `cairn scan` clean (0 findings) on merged `main`.
- 6 ready beads: 1 P2 (`cairn-kb0`, admin/infra) + 5 P3 spikes.
- No open PRs from this session (both merged).

## Known standing issue (not code)

- **`cairn-kb0`**: the `Deploy docs to GitHub Pages` workflow (`pages.yml`) fails
  on every push to `main`: *"Branch main is not allowed to deploy to github-pages
  due to environment protection rules."* This is a **repo-settings** decision for a
  maintainer: either set the `github-pages` environment deployment-branch policy to
  allow `main` (Settings -> Environments -> github-pages), set Pages source to
  "GitHub Actions" (Settings -> Pages), or remove/limit the `pages.yml` triggers.
  It does not block code; all code gates are green.

## Next: maintainer-directed

The actionable coding backlog is empty (`cairn lint` clean). What remains needs a
maintainer choice:

| Bead | Priority | Nature |
| --- | --- | --- |
| `cairn-kb0` | P2 | Repo-admin: Pages env protection (security/deploy policy) |
| `cairn-y1m` | P3 | Spike: adopt existing `bd github`? (second source of truth) |
| `cairn-1w3` | P3 | Spike: cairn warns when a tracked toolchain's lint isn't strict |
| `cairn-2z9` | P3 | Spike: beads as cairn's first-class task layer (per-node todos) |
| `cairn-t59` | P3 | Spike: git-native graph-root fingerprint |
| `cairn-y7p` | P3 | Spike: browser UI/UX iterative AI fix loop |

P3 spikes are marked "do not implement yet" and were previously left for George to
direct. Recommend George either greenlights a specific spike for implementation or
resolves `cairn-kb0`'s settings question.

## Agent Entry Points

- `cairn context`; `cairn get <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
