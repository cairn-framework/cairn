---
node: cairn.kernel.cli
status: open
created: 2026-08-10
parent: todo.context-pass-skill-pack
related: [res.skill-absorption, res.reference-budget-headroom]
---

# Context pass: cairn-dev router and its non-loop references

Child of todo.context-pass-skill-pack, split 2026-08-10 under the sizing rule.
Owns the cairn-dev router surface: `tools/agent-pack/content/skills/cairn-dev/SKILL.md`
and its nine non-loop references (`task-bug-investigation.md`,
`task-refactoring.md`, `task-architecture-discovery.md`,
`task-feature-implementation.md`, `task-brownfield-decision-extraction.md`,
`graph-navigation.md`, `blueprint-syntax.md`, `command-reference.md`,
`finding-codes.md`, `artefact-schemas.md`), plus their generated `.claude/`
mirrors (`loop-mode.md` belongs to todo.context-pass-pack-loop).

## Task

1. Apply the full writing-for-agents framework from `res.skill-absorption`
   (context-pointer audit, step-versus-reference classification with reference
   pushed behind pointers, checkable completion criteria on steps, positive-form
   rules with hard guardrails paired, whole-sentence no-op deletion, restatement
   collapse) to the router and each reference above.
2. The router loads one reference; references stop duplicating the router and
   each other.
3. Respect `JIT_REFERENCE_BUDGET_BYTES` = 6,000
   (`tools/agent-pack/tests/first_turn_budget_tests.rs`).
   `task-brownfield-decision-extraction.md` (5,983), `artefact-schemas.md`
   (5,947), and `finding-codes.md` (5,860) are pressed against the ceiling:
   any addition there is a compression pass, not an append
   (`res.reference-budget-headroom`).
4. Record this surface's first-turn measurement (the router feeds the pack's
   advertised first-turn surface) in this todo on completion, feeding the
   parent's combined metric.

## Acceptance

- Mirror, manifest, determinism, and budget tests green:
  `cargo test -p cairn-agent-pack`.
- No procedure lost: each deleted line is restated once at its owning layer or
  demonstrably answerable from a routed reference; hard guardrails stay paired
  with their positive target; steps keep checkable completion criteria.
- `scripts/check-voice-markers.sh` clean (zero FAIL) over every touched file.
