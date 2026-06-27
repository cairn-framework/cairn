# Session Handoff: 2026-06-27 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main`.

## What Was Done (prior iteration)

1. **PR #174 merged** (`cairn-oax`): folded the provenance query path
   (`cairn rationale`/`decisions`/`research`/`sources`) into the shipped
   `cairn-explore` skill instead of shipping a fourth state skill.
   - Eval-driven: a with/without eval against the SHIPPED five-skill pack (not
     against nothing) showed the pack already cues cairn usage; the only marginal
     lift was provenance, because `cairn-explore` named no provenance command.
   - Recorded as a provenance chain: `dec.explore-teaches-provenance` informed_by
     `res.cairn-oax-skill-promotion` (sources `src.cairn-oax-skill-promotion-eval`,
     pointing at `archive/strongholds/cairn-oax-skill-promotion-eval.md`).
   - `cairn-oax` closed. Promotion policy set: judge marginal lift over the
     current pack, prefer merging a skill's unique value into the owning pack
     skill over adding a new skill.

## Terminal state

- `cairn next`: nothing to do. `cairn lint --json`: 0 findings. `cairn scan
  --strict`: clean. `cairn hook all`: pass.
- `bd ready`: empty. `bd blocked`: none. No open PRs.

## Open deferred beads (awaiting maintainer ratification, NOT code-blocked)

Two deferred beads are the only forward candidates. Both gate on ratifying one
proposed decision; the investigate/debate/recommend homework is done and
persisted. Do NOT self-execute: this is the maintainer's structural call.

- **cairn-iy2** (P2, deferred): no primitive for a Designed-but-unimplemented
  *rule/capability* (spec:24). Recommendation drafted at
  `meta/decisions/dec.ghost-rule-tracking.md` (status: **proposed**), recommending
  (a) tracking-bead convention (already in force) AND (b) a machine-readable
  spec-rule registry + scan check. On ratification: flip the decision to accepted,
  open an implementation bead for (b).
- **cairn-9w9** (P3, deferred): revisit-trigger relevance (spec:634). Gated on
  cairn-iy2 (the relevance heuristic is part of the ghost-rule mechanism). Resume
  only after iy2 is ratified.

## Loop status -> STOP (escalation pending)

Re-invoked 2026-06-27 (attended). Escalated `dec.ghost-rule-tracking` for
ratification via AskUserQuestion; the prompt **auto-selected after timeout** (no
live ratification). Per the dev-loop unattended protocol the recommendation stays
persisted and the beads stay deferred rather than self-executed. Sanctioned stop:
backlog has no ready or actionable-without-ratification unit. Next session: if the
maintainer ratifies (a)+(b), flip dec.ghost-rule-tracking to accepted, open the
impl bead, build the registry + scan check, then resume cairn-9w9.

## Agent Entry Points

- `cairn context` / `cairn rationale <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
