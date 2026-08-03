---
node: cairn.root
status: open
created: 2026-08-03
related: [dec.orchestration-placement]
---

# Orca Plugin Triage

An unstructured experiment built an Orca plugin for orchestrating with
cairn in a separate worktree, without structured guidance from this
repository's decisions. Being cairn-aware it may still have done a fair
job; evaluate it on the merits, do not adopt it blind.

Worktree: `~/orca/workspaces/cairn/Orca-plugin` (branch
`George-RD/Orca-plugin`, based at 531ffa5, 20 uncommitted files, about
257 insertions touching `src/cli/commands/{gap,hook,workspace}.rs`,
`src/query_api/handlers/work_item.rs`, copy, contracts, and docs). Do
not delete the worktree before this triage lands its disposition.

## Task

1. Read the experiment's diff and reconstruct what it was trying to do
   and how.
2. Judge each piece against the mission and the accepted decisions
   (`dec.orchestration-placement` once signed governs where any
   orchestration logic may sit).
3. Record exactly one disposition per piece:
   - cherry-pick: it solves a real problem in our mission as-is;
   - re-plan: the concept has merit but needs a sustainable long-term
     design rather than borrowing a half concept;
   - close: an experiment without merit, recorded and removed.

## Acceptance

- Every changed file in the experiment carries a disposition with
  grounds; anything cherry-picked lands through the normal PR gates.
