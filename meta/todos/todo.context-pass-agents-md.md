---
node: cairn.root
status: open
created: 2026-08-10
parent: todo.context-engineering-pass
related: [res.skill-absorption]
---

# Context engineering pass over AGENTS.md

Child of todo.context-engineering-pass (item 1 plus its share of item 4).
Source: `src.context-engineering-claude5`. AGENTS.md front-loads conventions
the tree or `cairn context` already answers; prune it to gotchas and pointers.

## Task

1. Prune AGENTS.md: keep the test-locked strings
   (`tests/command_reference_consistency.rs` asserts them), keep guardrails
   that encode real repo-specific traps, drop restatements of things the tree
   or `cairn context` answers.
2. Apply the writing-for-agents framework in `res.skill-absorption`: audit
   every always-loaded line as a context pointer; push reference content down
   the hierarchy behind pointers; end steps on checkable completion criteria;
   prefer positive-form rules, pairing any line that earns hard-guardrail
   status; delete no-op sentences whole; collapse restatements into leading
   words. The Terminology section stays (ubiquitous-language cache).
3. Measure: token count of AGENTS.md before and after, recorded in this todo
   on completion.

## Acceptance

- `cargo test --test command_reference_consistency` (the AGENTS.md
  consistency assertions) green.
- AGENTS.md token count reduced and recorded here on completion.
- No convention lost: each deleted rule is either enforced by a gate already
  or demonstrably answerable from the tree.
- `scripts/check-voice-markers.sh` runs clean (zero FAIL) over AGENTS.md.
