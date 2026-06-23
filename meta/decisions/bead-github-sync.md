---
id: dec.bead-github-sync
nodes:
  - cairn.root
status: accepted
date: 2026-06-23
informed_by: [res.gas-city-cairn-integration]
revisit_triggers:
  - "A maintainer accepts GitHub issues as a sanctioned projection of bead state and asks for a documented one-way mirror policy"
  - "cairn adopts a Dolt remote (refs/dolt/* on origin), changing the cross-machine sync model this analysis assumes"
  - "bd changes the `bd github` contract (pull/push/sync) or drops native GitHub-issue identity mapping"
---

# Bead-GitHub sync: do not adopt GitHub issues as a second source of truth

## Context

`cairn-y1m` asked whether and how cairn should sync bead issue state with GitHub
issues, originally framed as a ForgeDock-style workflow-state label layer
(`workflow:investigating`, `bead:open`, structured HTML-comment annotations,
`gh` as the query interface). `dec.bd-upgrade-plan` (item 4) re-scoped the spike:
evaluate the *existing* `bd github` surface first, and isolate the one open
question that is genuinely the maintainer's, namely whether to accept GitHub
issues as a second source of truth at all and, if so, one-way versus
bidirectional.

Two facts shrink the question before any design work:

- **`bd github` already ships in the installed bd 1.0.4.** `bd github
  pull/push/sync/status/repos` exists today, configured via
  `github.token`/`github.owner`/`github.repo` (or the `GITHUB_*` env vars). It
  carries its own bead-id to issue-number identity mapping. No custom label
  layer needs to be built to get bead-GitHub interchange.
- **The canonical store is settled.** `dec.bd-upgrade-plan` fixed Dolt as the
  local storage engine and `.beads/issues.jsonl` (jsonl-in-git, upsert-only) as
  the committed cross-machine projection, with no Dolt remote on `origin`.

## Decision

**Recommendation: defer. Do not adopt GitHub issues as a second source of truth,
and do not build any bead-GitHub sync surface inside cairn.** Three rulings
follow.

1. **Do not build a custom label/annotation layer.** A ForgeDock-style
   `workflow:*` / `bead:*` label scheme plus HTML-comment annotations would
   duplicate `bd github`, which already exists. Custom code here is rejected.
2. **Do not put bead-GitHub sync in cairn.** Per `dec.no-orchestrator`, cairn
   owns the semantic layer (blueprint, typed artefacts, drift gate); work-item
   coordination and process state live in the storage/orchestration layer
   (beads, an external orchestrator). A GitHub-issue mirror is coordination, not
   architecture truth, so it belongs to `bd github` or an orchestrator pack, not
   to a cairn command, hook, or reconciler.
3. **Keep a single source of truth: Dolt-local plus jsonl-in-git.** Do not
   promote GitHub issues to canonical. If cross-platform visibility is ever
   wanted, the only sanctioned shape is the one-way mirror in the design sketch
   below, run opt-in, with GitHub treated as a read-only projection.

The maintainer's reserved call (whether to ever accept GitHub as a sanctioned
projection) stays open: adopting it would be a superseding decision, not a
reversal of any guarantee this one makes.

## Answers to the spike questions

1. **Should we?** No, not as a second source of truth. The canonical store
   (Dolt-local, jsonl-in-git) already gives cross-machine sync. A GitHub mirror
   adds a divergence surface without replacing anything. The merge-conflict
   class the team already hit on `issues.jsonl` is the concrete cost; visibility
   is the only benefit, and it does not require canonical status.
2. **Could we?** Yes, mechanically. `bd github sync` maps bead status and
   priority to GitHub issue state and labels today. Bidirectional sync is
   feasible but reintroduces the two-writer merge-conflict class. One-way
   (bead to GitHub) is safe because GitHub never feeds back into the canonical
   store.
3. **Mechanism?** `bd github` (the bd CLI), not a cairn hook, bd plugin, or
   GitHub Action that re-parses `issues.jsonl`. The mapping lives in bd, which
   already owns it.
4. **Identity?** Use `bd github`'s native bead-id to issue-number mapping. Do
   not invent a `bead-id:cairn-xxx` label or store issue numbers in bead
   metadata by hand.
5. **Scope boundary vs OMP/cairn task-sync (cairn-d7s diagnostics server)?**
   Distinct surfaces. cairn-d7s exposes cairn graph state (LSP/watch
   diagnostics); this spike is about bead process state mirrored to GitHub. They
   do not overlap, and neither pulls GitHub-issue sync into cairn.

## Design sketch (only if a maintainer later opts in)

A one-way, opt-in mirror. Never canonical, never bidirectional.

- **Direction:** bead to GitHub only. GitHub issues are a read-only projection;
  edits made on GitHub are not read back. The canonical store stays Dolt-local
  plus jsonl-in-git.
- **Mechanism:** `bd github push` (or `bd github sync` constrained to push),
  invoked manually or from an orchestrator pack, never from a cairn command or
  hook.
- **Identity:** `bd github`'s native bead-id to issue-number mapping.
- **Status and priority mapping:** bead `open`/`in_progress`/`closed` to GitHub
  open/closed plus a small label set for the in-between and priority, owned by
  bd's mapping, not a cairn convention.
- **Cairn's role:** none. Cairn neither writes to GitHub nor reconciles GitHub
  state into the graph.

## Risks

- **Two writers, one truth (rejected path).** Bidirectional sync lets a GitHub
  edit and a `bd` edit diverge, recreating the `issues.jsonl` merge-conflict
  class. The one-way mirror avoids this by construction.
- **Scope creep into cairn (rejected by ruling 2).** A GitHub-sync command in
  cairn would erode `dec.no-orchestrator` and blur the storage/semantic
  boundary.
- **Staleness of a one-way mirror.** A push-only mirror lags between pushes.
  This is acceptable for a visibility projection and is the price of avoiding
  bidirectional conflict.

## Consequences

- `cairn-y1m` is satisfied and can be closed: the spike's deliverable (a
  recommendation with a design sketch and risks) is this document.
- No code, command, hook, or blueprint edge is added for bead-GitHub sync.
- The repo continues to track work in beads with Dolt-local plus jsonl-in-git as
  the single source of truth.
- If a maintainer later wants the mirror, the sanctioned shape is the one-way
  `bd github push` design above, recorded by a superseding decision; cairn
  remains uninvolved.
