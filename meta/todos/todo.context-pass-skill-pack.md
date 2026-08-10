---
node: cairn.kernel.cli
status: blocked
created: 2026-08-10
parent: todo.context-engineering-pass
related: [res.skill-absorption, res.reference-budget-headroom]
blocked_by:
  - todo.context-pass-pack-dev
  - todo.context-pass-pack-loop
  - todo.context-pass-pack-workflows
---

# Context engineering pass over the shipped agent pack

blocked on sub-todos: todo.context-pass-pack-dev (cairn-dev router plus its
nine non-loop references), todo.context-pass-pack-loop (loop-mode plus the
five loop skills and commands/cairn-loop.md), todo.context-pass-pack-workflows
(explore, propose, apply, archive). Decomposed 2026-08-10 under the sizing
rule: a single pass over all 22 canonical files plus mirrors is not one small
reviewable PR. Each child carries the framework, the mirror discipline, the
budget rule where it applies, and its share of the measurement; the iteration
completing the last child flips this todo to done and records the combined
pack first-turn measurement here.

Child of todo.context-engineering-pass (items 2 and 3, one surface: every
canonical file under `tools/agent-pack/content/` has a byte-identical
generated counterpart under `.claude/` (`skills/` and `commands/`; `.claude`
also carries local non-pack files), enforced by
`checked_in_claude_outputs_match_the_real_manifest` in
`tools/agent-pack/tests/determinism_drift_tests.rs`, so canonical content and
mirrors move in the same commit). Plus this surface's share of item 4.
Binding surface: the change rides an ordinary PR with review
(dec.agent-pack-packaging).

## Task

1. Apply progressive disclosure and the FULL writing-for-agents framework
   recorded in `res.skill-absorption` (and restated in the parent's Method
   section: context-pointer audit, step-versus-reference classification with
   reference pushed behind pointers, checkable completion criteria on steps,
   positive-form rules with hard guardrails paired, whole-sentence no-op
   deletion, restatement collapse) to EVERY canonical file under
   `tools/agent-pack/content/`: all ten skill SKILL.md files (cairn-dev,
   cairn-explore, cairn-propose, cairn-apply, cairn-archive, and the five
   loop skills), the cairn-dev references, and `commands/cairn-loop.md`,
   plus their generated adapter mirrors.
2. The cairn-dev router loads one reference; references stop duplicating the
   router and each other.
3. Respect the reference byte budget (`JIT_REFERENCE_BUDGET_BYTES` = 6,000,
   `tools/agent-pack/tests/first_turn_budget_tests.rs`).
   `task-brownfield-decision-extraction.md` (5,983 current size, after the
   compression pass recorded in `res.reference-budget-headroom`),
   `artefact-schemas.md` (5,947), and `finding-codes.md` (5,860) are pressed
   against the ceiling: any addition there is a compression pass, not an
   append.
4. Measure: the pack's first-turn surface before and after, as computed by
   `tools/agent-pack/tests/first_turn_budget_tests.rs` (advertised skill
   metadata plus the router plus `src/cli/agent_guide.md`), recorded in this
   todo on completion. This is the pack half of the parent's combined metric.

If this proves larger than one small reviewable PR, re-split under the
sizing rule rather than growing the PR.

## Acceptance

- Mirror, manifest, determinism, and budget tests green:
  `cargo test -p cairn-agent-pack`.
- First-turn pack surface (per the budget test's definition) reduced and
  recorded here on completion.
- No procedure lost: each deleted line is either restated once at its owning
  layer or demonstrably answerable from a routed reference; retained hard
  guardrails stay paired with their positive target, and steps keep checkable
  completion criteria.
- `scripts/check-voice-markers.sh` runs clean (zero FAIL) over every file
  the pass touches.
