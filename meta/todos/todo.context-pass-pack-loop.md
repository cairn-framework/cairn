---
node: cairn.kernel.cli
status: done
created: 2026-08-10
parent: todo.context-pass-skill-pack
related: [res.skill-absorption]
---

# Context pass: loop-mode closure and the five loop skills

Child of todo.context-pass-skill-pack, split 2026-08-10 under the sizing rule.
Owns the loop procedure closure, which cross-references itself and must move
together: `tools/agent-pack/content/skills/cairn-dev/references/loop-mode.md`,
the five loop skill files (`cairn-loop-scope`, `cairn-loop-implement`,
`cairn-loop-recovery`, `cairn-loop-reconcile`, `cairn-loop-landing`
SKILL.md), `commands/cairn-loop.md`, plus their generated `.claude/` mirrors.

## Task

1. Apply the full writing-for-agents framework from `res.skill-absorption`
   (context-pointer audit, step-versus-reference classification with reference
   pushed behind pointers, checkable completion criteria on steps, positive-form
   rules with hard guardrails paired, whole-sentence no-op deletion, restatement
   collapse) to each file above.
2. loop-mode and the five loop skills stop restating each other: each rule
   lives at its owning layer once, the others point. The declared exit-token
   table and the required asset closure list are contracts and stay verbatim.
3. `commands/cairn-loop.md` stays pure transport per
   `dec.unified-cairn-dev-entry`; `router_route_tests.rs` gates it.
4. Record this surface's first-turn terms before and after in this todo on
   completion: the five loop skills' advertised name-plus-description bytes,
   as computed by `tools/agent-pack/tests/first_turn_budget_tests.rs`.
   loop-mode.md is a routed reference outside that metric; its raw byte
   count is local evidence only.

## Acceptance

- Mirror, manifest, determinism, and budget tests green:
  `cargo test -p cairn-agent-pack`.
- No procedure lost: each deleted line is restated once at its owning layer or
  demonstrably answerable from a routed reference; the fail-closed rows,
  terminal tokens, and branch-deletion guardrails survive verbatim or
  strengthened.
- Cross-seam pointers between `loop-mode.md` and the cairn-dev references
  owned by todo.context-pass-pack-dev still resolve after the pass.
- `scripts/check-voice-markers.sh` clean (zero FAIL) over every touched file.

## Measurement (recorded on completion, task item 4)

Advertised name plus description bytes for the five loop skills, per the
terms of `tools/agent-pack/tests/first_turn_budget_tests.rs`:

| Skill | Before | After |
|---|---|---|
| cairn-loop-scope | 362 | 298 |
| cairn-loop-implement | 386 | 334 |
| cairn-loop-recovery | 398 | 345 |
| cairn-loop-reconcile | 384 | 331 |
| cairn-loop-landing | 340 | 321 |
| Total | 1,870 | 1,629 |

loop-mode.md raw bytes (local evidence only, outside the metric): 17,661
before, 16,870 after. Per-file body sizes and the compression rationale:
`res.context-pass-pack-loop.measurement`.
