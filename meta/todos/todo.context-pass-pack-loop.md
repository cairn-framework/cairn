---
node: cairn.kernel.cli
status: open
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
   `dec.unified-cairn-dev-entry`: it resolves to loop mode and adds nothing.
4. Record this surface's byte count before and after in this todo on
   completion, feeding the parent's combined metric.

## Acceptance

- Mirror, manifest, determinism, and budget tests green:
  `cargo test -p cairn-agent-pack`.
- No procedure lost: each deleted line is restated once at its owning layer or
  demonstrably answerable from a routed reference; the fail-closed rows,
  terminal tokens, and branch-deletion guardrails survive verbatim or
  strengthened.
- `scripts/check-voice-markers.sh` clean (zero FAIL) over every touched file.
