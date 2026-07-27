---
id: res.design-studio-greenfield
nodes:
  - cairn.ui
date: 2026-07-10
method: primary
---

# Design Studio Greenfield: session plan

## Question
What is the exact, repeatable recipe for Track A of `todo.design-studio-exploration`
(the greenfield simulated design studio run), and how do its results feed a
`dec.webui-design-direction` decision?

## Method
Primary planning artefact. This document is read only planning; the run itself
executes in a forked cairn with the `design-studio` skill (OMP plus a browser tool).
Citations to the skill are inline because the project does not hold its bytes.

## Stripped worktree recipe
1. From a clean `main` checkout, add a worktree:
   `git worktree add -b design-studio-greenfield ../cairn-ds-greenfield main`.
   A fork and clone works too; a worktree keeps the review local.
2. Enter it and remove the two directories the design agents must NOT see:
   `rm -rf src/ui_assets docs/design-system`.
3. Add a brief file at the repo root (for example `BRIEF.md`) covering: what cairn
   is (a tool that maps a codebase's architecture as a navigable graph), the graph
   explorer's purpose (read only inspection of nodes, their relationships,
   findings, and linked decisions), and target users (the maintainer and
   contributors navigating a growing Rust monorepo). Keep it plain prose; no code.
4. Reuse frozen fixtures so mocks render real data: copy `harness/fixtures/`
   (the `map.json` and `/api/*` captures) into the fork as the design studio's
   data source. Confirm `map.json` is the real frozen graph and the `/api`
   captures are the same ones the eval harness replays.

## Context given to the design agents versus denied
- Given: the `BRIEF.md` (purpose and users), the frozen `map.json` and `/api`
  captures (real data shapes), and the `design-studio` `workflow.yaml` plus the
  helper prompts under `skills/design-studio/agents/`. The skill lives at
  https://github.com/george-rd/design-studio (portable path
  `skills/design-studio/workflow.yaml`); the run uses OMP with a browser tool.
- Denied: the existing `src/ui_assets` and `docs/design-system` (removed in step 2).
  Per the design studio methodology this defeats code anchoring bias: agents must
  propose structure, tokens, and hierarchy from the brief and data, not by copying
  the current implementation. Also deny any live network and the current
  `docs/design-system/tokens.css` so palette choices are independent.

## Fixture strategy
- Freeze `map.json` and the `/api` responses once (they already are, in
  `harness/fixtures`). The design studio mocks render from these so the greenfield
  output reflects cairn's actual node graph, not a toy sample.
- Keep the fixtures read only in the fork; do not let the studio mutate them.

## How the evaluator scores
- The design studio evaluator is browser based. It loads each produced mock in a
  headless browser and scores by zone: it divides the viewport into zones (for
  example header, graph canvas, inspector, command surface) and scores each for
  hierarchy, density, colour discipline, and state clarity, then aggregates. This
  is the studio's own quality proxy and is distinct from cairn's `ux_defect_score`
  harness. Record the per zone scores so Track A and Track B can be compared on the
  same rubric.

## Expected outputs
- Fast static HTML mock examples the maintainer can open directly (no build step).
- Codified harness output and design system: `design-dna.md` (the visual DNA and
  principles), `tokens.css` (the token set the mocks used), and a skill template
  capturing the winning approach. These land under the fork's harness output
  directory.

## Feeding the decision
- Compare Track A (greenfield) against Track B (iterate current, the `/review`
  audit of the live webui) using the shared zone rubric.
- Write `dec.webui-design-direction` resolving bet A's aesthetic sub decision
  (refined current versus full geological metaphor) on the evidence: cite the zone
  scores, the mock examples, and the `/review` findings. Link the decision from
  `todo.design-studio-exploration` and mark `todo.webui-design-quality.md`'s bet A
  line superseded.
- This research is the session plan behind `todo.design-studio-exploration`; the
  todo body links here via an informed by note.
