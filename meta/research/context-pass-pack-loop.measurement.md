---
id: res.context-pass-pack-loop.measurement
nodes:
  - cairn.kernel.cli
method: primary
date: 2026-08-11
related: [res.skill-absorption, res.context-pass-pack-dev.measurement]
---

# Context pass measurement: loop-mode closure and the five loop skills

First-hand measurement from `todo.context-pass-pack-loop` (2026-08-11), the
second child of `todo.context-pass-skill-pack`. Terms follow
`tools/agent-pack/tests/first_turn_budget_tests.rs`.

The surface's first-turn terms (the five loop skills' advertised name plus
description bytes, before and after) are recorded once, in
`todo.context-pass-pack-loop` as its task mandates; this artefact carries the
evidence the todo does not: per-file body sizes, what the pass changed, and an
incident note.

## File sizes (local evidence, outside the first-turn metric)

| File | Before | After |
|---|---|---|
| references/loop-mode.md | 17,661 | 16,870 |
| cairn-loop-scope/SKILL.md | 3,873 | 3,782 |
| cairn-loop-implement/SKILL.md | 3,437 | 3,385 |
| cairn-loop-recovery/SKILL.md | 6,190 | 5,902 |
| cairn-loop-reconcile/SKILL.md | 6,260 | 6,207 |
| cairn-loop-landing/SKILL.md | 6,694 | 6,563 |
| commands/cairn-loop.md | 2,336 | 2,336 |

loop-mode.md is a routed reference outside the first-turn metric and the JIT
budget by design; its byte count is local evidence only. commands/cairn-loop.md
was reached and left unchanged: it is already pure transport per
`dec.unified-cairn-dev-entry`, with no restatement found.

## What the pass changed

- The five skill descriptions now carry only their trigger contract (what the
  procedure is, which loop-mode step loads it, the not-for-ordinary-sessions
  anti-trigger). The declared-exits restatement moved out: loop mode's required
  asset closure table is the single owner of the exit-token contract.
- `cairn-loop-recovery` and `cairn-loop-landing` predated
  `dec.unified-cairn-dev-entry` and still addressed "the cairn-loop command"
  and "the router" as their caller. Both now name loop mode, matching the
  authority the decision relocated.
- Restatement collapse: loop-mode's Scope, Implement, Reconcile, and Land
  sections stop summarising the procedures of the skills they load (Land keeps
  only its call convention: parameters and entry point); the receipt-protocol
  statement lives once in `cairn-loop-reconcile` ("Never contradict an
  accepted decision silently") with `cairn-loop-scope` pointing at it; the
  branch-name derivation rule is owned by loop-mode's Isolation rule, with
  landing's Inputs keeping the three tail forms as its parameter contract; the
  staging ban lives once in landing's Guardrails, paired with its positive
  form in the Land step.
- Contracts kept verbatim: the exit-token table, the required asset closure
  list, every preflight fail-closed row, the terminal tokens, and the
  branch-deletion guardrails.
- Content locks held: `reconcile_step_tests.rs` pins the reconcile skill's
  selection and terminal-token disclaimers; the pass initially reworded them,
  review and the test caught it, and the accepted wording from
  `dec.loop-reconcile-step` ("never retries, never interprets a terminal
  token") was restored. The mirror, manifest, determinism, budget, and
  router-route suites all pass.

## Incident: harness edit-path resolution

During this unit, edit calls carrying relative file paths were resolved
against the session's working directory (the main checkout) rather than the
loop worktree the content was read from, writing six files outside
`../cairn-loop` in breach of loop-mode's Isolation rule. The writes were
wholly this session's own authored hunks; they were transplanted to the loop
worktree as a patch (which applied cleanly, proving identical bases) and the
main checkout was restored to its committed state. Standing consequence for
OMP sessions in this repo: address every edit by absolute path when the loop
worktree is not the session's working directory.

## Limits

Byte counts are a proxy for context load. No CLI behaviour changed and no
accepted rule changed; the deleted sentences are collapsed restatements whose
surviving copies the owning files point to. This research informs the parent's
combined measurement and any future budget tightening, nothing else.
