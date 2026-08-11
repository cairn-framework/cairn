---
node: cairn.kernel.cli
status: done
created: 2026-08-10
parent: todo.context-pass-skill-pack
related: [res.skill-absorption]
---

# Context pass: workflow skills (explore, propose, apply, archive)

Child of todo.context-pass-skill-pack, split 2026-08-10 under the sizing rule.
Owns the four standalone workflow skills:
`tools/agent-pack/content/skills/cairn-explore/SKILL.md`,
`cairn-propose/SKILL.md`, `cairn-apply/SKILL.md`, `cairn-archive/SKILL.md`,
plus their generated `.claude/` mirrors.

## Task

1. Apply the full writing-for-agents framework from `res.skill-absorption`
   (context-pointer audit, step-versus-reference classification with reference
   pushed behind pointers, checkable completion criteria on steps, positive-form
   rules with hard guardrails paired, whole-sentence no-op deletion, restatement
   collapse) to each file above, starting with the skill descriptions, which
   decide whether the right skill triggers.
2. Record this surface's first-turn terms before and after in this todo on
   completion: the four skills' advertised name-plus-description bytes, as
   computed by `tools/agent-pack/tests/first_turn_budget_tests.rs`.

## Acceptance

- Mirror, manifest, determinism, and budget tests green:
  `cargo test -p cairn-agent-pack`.
- No procedure lost: each deleted line is restated once at its owning layer or
  demonstrably answerable from a routed reference; hard guardrails stay paired
  with their positive target.
- `scripts/check-voice-markers.sh` clean (zero FAIL) over every touched file.

## Measurement (recorded on completion, 2026-08-11)

Advertised name-plus-description bytes, computed as
`tools/agent-pack/tests/first_turn_budget_tests.rs` measures them:

| Skill | Before | After |
|---|---|---|
| cairn-explore | 316 | 284 |
| cairn-propose | 227 | 209 |
| cairn-apply | 159 | 159 |
| cairn-archive | 119 | 119 |
| Total | 821 | 771 |

File-size evidence and the compression rationale:
res.context-pass-pack-workflows.measurement.
