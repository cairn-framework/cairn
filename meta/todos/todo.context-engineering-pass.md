---
node: cairn.kernel.cli
status: open
created: 2026-08-07
related: [res.skill-absorption]
---

# Context engineering pass over AGENTS.md and the skill pack

Source: `src.context-engineering-claude5`. Anthropic removed over 80 percent of
Claude Code's system prompt for Claude 5 generation models with no measured
loss, and names the shifts: rules to judgement, examples to interface design,
upfront loading to progressive disclosure, repetition to single statements.

This repository's agent surfaces predate that: AGENTS.md front-loads
conventions the tree already answers, several cairn-dev references restate each
other, and the shipped skill pack carries worst-case guardrails written for
older models.

## Task

1. Prune AGENTS.md to gotchas and pointers: keep the test-locked strings
   (`tests/command_reference_consistency.rs` asserts them), keep the guardrails
   that encode real repo-specific traps, drop restatements of things the tree
   or `cairn context` answers.
2. Apply progressive disclosure to `.claude/skills/cairn-dev/references/`:
   the router loads one reference, references stop duplicating the router.
3. Same pass over `tools/agent-pack/content/` before the next pack release
   (binding surface: the change rides an ordinary PR with review).
4. Measure: token count of the always-loaded surface before and after.

## Method (added 2026-08-07, from res.skill-absorption)

Apply the writing-for-agents framework recorded in `res.skill-absorption`:
audit every always-loaded line as a context pointer (does its wording trigger
the right reach?); classify content as step or reference and push reference
down the information hierarchy behind pointers; end steps on checkable
completion criteria; replace negations with the positive target unless a line
earns hard-guardrail status (then pair it); hunt no-ops sentence by sentence,
deleting whole sentences; collapse restatements into leading words. The
Terminology section stays, justified as the ubiquitous-language cache.

## Acceptance

- `cargo test` (AGENTS.md consistency assertions) green.
- Always-loaded token count reduced and recorded in the todo on completion.
- No convention lost: each deleted rule is either enforced by a gate already
  or demonstrably answerable from the tree.
- `scripts/check-voice-markers.sh` runs clean (zero FAIL) over every file
  the pass touches, so absorbed guidance lands in the repo voice.

2026-08-07 audit (todo.roadmap-assumption-audit): keep; no stale assumptions found this pass.

2026-08-10 measurement (`res.reference-budget-headroom`, from
todo.brownfield-extraction-reference-gaps): three of the ten routed references
are pressed against their shipped ceiling, so item 2 has less room than it looks
for those three and normal headroom elsewhere. `JIT_REFERENCE_BUDGET_BYTES` is
6,000 (`tools/agent-pack/tests/first_turn_budget_tests.rs`), and
`task-brownfield-decision-extraction.md` sat at 5,701 bytes, so two required
passages only fitted after compressing wording in the same file.
`finding-codes.md` (5,860) and `artefact-schemas.md` (5,947) have less headroom
still. Any addition to those three is a compression pass, not an append, and the
four qualifiers that first pass dropped are the reason to budget it deliberately.
