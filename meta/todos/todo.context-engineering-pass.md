---
node: cairn.kernel.cli
status: done
created: 2026-08-07
related: [res.skill-absorption]
blocked_by:
  - todo.context-pass-agents-md
  - todo.context-pass-skill-pack
---

# Context engineering pass over AGENTS.md and the skill pack

Done 2026-08-11 under the sizing rule's last-child clause; see the Combined
always-loaded measurement section below. Original decomposition note kept
below for the record.

blocked on sub-todos: todo.context-pass-agents-md (cairn.root, owns item 1),
todo.context-pass-skill-pack (cairn.kernel.cli, owns items 2 and 3).
Decomposed 2026-08-10 under the sizing rule: items 2 and 3 are one surface
(every canonical file under `tools/agent-pack/content/` has a byte-identical
generated counterpart under `.claude/` `skills/` and `commands/`, enforced by
the determinism drift tests), item 4
is folded into each child's acceptance per surface. The Task and Acceptance
below are discharged per surface by the children, except the combined
metric: the iteration completing the last child flips this todo to done AND
records here the combined before/after token count of the always-loaded
surface (AGENTS.md plus the pack first-turn surface as computed by
`tools/agent-pack/tests/first_turn_budget_tests.rs`), summed from the
per-child measurements, so item 4's original combined metric survives the
split.

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

## Combined always-loaded measurement (recorded 2026-08-11, last child done)

Both children are done (todo.context-pass-agents-md, and
todo.context-pass-skill-pack via its three sub-todos, PRs #702, #703, and the
pack-workflows iteration of 2026-08-11), so this todo is done with them under
the sizing rule's last-child clause. AGENTS.md figures are its child's
recorded measurement; pack figures are the budget test's byte terms plus a
direct `o200k_base` re-measurement of the same three terms (advertised skill
metadata, cairn-dev router body, `src/cli/agent_guide.md`) at the pre-pass
commit 2536d8ea and after the last child:

| Surface | Before (tokens; bytes) | After (tokens; bytes) |
|---|---|---|
| AGENTS.md | 3,687; 15,263 | 2,427; 10,047 |
| Pack first-turn surface | 2,160; 9,364 | 1,887; 8,214 |
| Combined always-loaded | 5,847; 24,627 | 4,314; 18,261 |

A 26 percent token reduction (1,533 tokens) over the surface every session
pays before routing anywhere. Per-surface evidence: the two child todos and
res.context-pass-pack-{dev,loop,workflows}.measurement.
