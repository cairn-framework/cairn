---
node: cairn.ui
status: done
created: 2026-07-10
---

# Design Studio Exploration


## Problem
`dec.webui-design-quality-direction` (accepted 2026-06-25) ratifies four bets in
order D, B, C, A. Bet A ("make the map a real map") is sequenced last and gated on
(1) the bet D design quality scorer and (2) an explicit maintainer aesthetic call
between refined current and a full geological "cairn" metaphor. `todo.webui-design-quality.md`
(status blocked) records bet A as maintainer gated and deferred. This todo runs an
evidence based design exploration with the `design-studio` skill to make that
aesthetic call on evidence rather than taste, then ratifies it as a decision
artefact. It supersedes `todo.webui-design-quality.md` for the bet A direction
question.

## Evidence and context
- `dec.webui-design-quality-direction` sequences bet A last with an unresolved
  aesthetic sub decision that is the maintainer's to make, so the D scorer can
  judge a new layout.
- `todo.webui-design-quality.md` (created 2026-07-03, status blocked): bet A
  blocked on the bet D scorer and the maintainer aesthetic call.
- `design-studio` skill: https://github.com/george-rd/design-studio, portable path
  `skills/design-studio/workflow.yaml` plus helper prompts under
  `skills/design-studio/agents/`, runs in OMP with a browser tool. Its methodology
  is two track (greenfield simulated versus iterate current) and explicitly denies
  design agents the existing CSS and tokens to defeat code anchoring bias.
- Backing session plan: `meta/research/design-studio-greenfield.md`
  (informed by this todo).

## Approach: two tracks
- Track A (greenfield simulated): prepare a stripped worktree or fork of cairn
  WITHOUT `src/ui_assets` and `docs/design-system`, containing a project brief
  (what cairn is, the graph explorer purpose, target users) plus a frozen real
  `map.json` and captured `/api` fixtures (reuse `harness/fixtures`) so mocks
  render real data. Run the full create loop. Deliverable: fast static HTML mock
  examples the maintainer can open immediately, plus the codified harness output
  and design system (`design-dna.md`, `tokens.css`, skill template). Exact recipe
  in `design-studio-greenfield.md`.
- Track B (iterate current): run the `/review` lane against the current live webui
  for an audit and polish direction.
- Compare the two tracks and ratify the direction as a decision artefact
  (`dec.webui-design-direction`).

## Ratification and supersession
- Produce `dec.webui-design-direction` (new decision) that resolves bet A's
  aesthetic sub decision with evidence and references both tracks.
- This todo supersedes `todo.webui-design-quality.md` for the bet A direction call.
  Note: `todo.webui-design-quality.md` stays the home for bets B, C, D tracking;
  only its blocked bet A line is superseded. The executor should update
  `todo.webui-design-quality.md`'s status note to point at this todo and the new
  decision, not delete it.

## Acceptance criteria
- A stripped cairn fork or worktree exists with no `src/ui_assets` or
  `docs/design-system`, a brief file, frozen `map.json`, and captured `/api`
  fixtures rendering real data.
- Track A produces openable static HTML mock examples and `design-dna.md`,
  `tokens.css`, and a skill template under the fork's harness output.
- Track B produces a written `/review` audit of the current webui.
- `dec.webui-design-direction` exists, ratifies the bet A aesthetic direction on
  evidence, and is linked from this todo.
- `todo.webui-design-quality.md` notes the supersession.

## Dependencies and ordering
- Track A can run at any time on a fork and does not touch main.
- Wiring the winning direction into `src/ui_assets` WAITS until
  `todo.simplify-ui-query-api` remaining endpoint flips land, to avoid rebasing
  design over wire churn. This todo produces direction only; implementation is a
  separate follow up.
- Depends on `design-studio-greenfield.md` being available before Track A runs.
- Depends on the `design-studio` skill being installed and available in OMP with a
  browser tool.

Informed by: res.design-studio-greenfield

## Run record (2026-07-12)

Both tracks executed. Track A on branch `design-studio-greenfield`
(worktree `../cairn-ds-greenfield`, commit 30fcd7f): two poles built and
browser evaluated over three iterations; Calibrated Instrument won 8.0 vs
7.8 (zone total 126 vs 124); codified `design-dna.md`, `tokens.css`, and a
skill template under `harness-output/`. Track B audit:
`res.webui-review-audit` (verdict ready_with_nits, direction "refine, do
not redesign"). Direction ratified in `dec.webui-design-direction`
(accepted 2026-07-12); this todo is done.
