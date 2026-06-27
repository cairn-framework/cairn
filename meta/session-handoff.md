# Session Handoff: 2026-06-27 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main`.

## What Was Done

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
     skill over adding a new skill. The other 6 managed skills stay personal
     (redundant with the pack, niche to this repo, or contradicting an accepted
     decision, e.g. `cairn-provenance-coverage` vs dec.adopt-cairn-dev-loop).

## Terminal state

- `cairn next`: nothing to do. `cairn lint`: 0 findings. `cairn hook all`: pass.
- `bd ready`: empty. `bd list`: no open issues. No open PRs.

## Next unit selected -> STOP

Backlog empty, lint clean, no open PRs. Sanctioned terminal stop.

## Agent Entry Points

- `cairn context` / `cairn rationale <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
