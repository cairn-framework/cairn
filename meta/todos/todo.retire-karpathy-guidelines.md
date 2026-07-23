---
node: cairn.kernel.cli
status: open
created: 2026-07-23
---

# Retire Karpathy Guidelines

## Priority

P3. Guidance-surface de-duplication, not urgent. Sequence it inside the
agent-guidance program so it lands measured, not as an ad-hoc edit.

## Depends on

`todo.agent-guidance-baseline` (must price the current guidance surface, with
the karpathy skill in place, before this changes it) and
`todo.agent-guidance-router-playbooks` (establishes the compact router and JIT
structure, so absorbed content is placed correctly rather than dumped into
always-on guidance against the token ceiling).

## Problem

The root `AGENTS.md` points coding work at
`.claude/skills/karpathy-guidelines/SKILL.md`, a generic skill deliberately not
bundled in the pack (`dec.init-emits-agent-skills`). Its four rules (think
before coding, simplicity, surgical changes, goal-driven execution) are largely
redundant with cairn's own `AGENTS.md` Guardrails and the dev loop, so cairn's
product-facing guidance leans on a foreign, unshipped dependency for behaviour
cairn should own. Per `dec.no-orchestrator`, general coding competence is the
harness's job, but the small genuinely-additive framing is worth owning natively
so the guidance surface is self-contained and the foreign pointer drops.

## Scope

- Extract only the behaviours not already covered by cairn's Guardrails: state
  assumptions explicitly and present materially-different interpretations rather
  than picking silently; the concrete "transform the task into a verifiable
  success criterion (for a bug, write the failing test first, then pass it)"
  framing; and the simplicity heuristic ("would a senior engineer call this
  overcomplicated"). Do not import the ask-when-uncertain default wholesale; keep
  it compatible with the default-to-action posture.
- Incorporate that minimal set into cairn's native emitted agent guidance
  (`src/cli/agent_guide.md`), so `cairn init` users get it. Prefer a JIT
  reference over always-on text, coordinated with `router-playbooks`, to respect
  the token ceiling.
- Remove the `.claude/skills/karpathy-guidelines/` skill and the `AGENTS.md`
  pointer, leaving `cairn-dev` as the sole load-bearing skill reference.
- Update the mention in `dec.init-emits-agent-skills` (it names karpathy as "not
  bundled") so no dangling reference remains after removal.

## Non-goals

- Do not touch the unrelated Karpathy "LLM Wiki" prior art
  (`src.karpathy-llm-wiki`, `docs/spec.md`, the bootstrap fixtures). That is a
  separate citation for cairn's provenance-chain framing, not this skill.

## Acceptance

- The additive behaviours appear in cairn's native emitted guidance; no
  reference to `karpathy-guidelines` remains in `AGENTS.md` or the emitted pack.
- `.claude/skills/karpathy-guidelines/` is removed and
  `dec.init-emits-agent-skills` no longer leaves a dangling reference.
- The three `AGENTS.md` tests, `cairn scan`, `cairn hook all`, and the file-size
  gate pass; no unrelated Karpathy LLM-Wiki reference is touched.
- The baseline has already priced the pre-change surface, so the effect of the
  change is attributable.

## Rationale

Recorded from the guidance-surface review this session: karpathy-guidelines
overlaps cairn's Guardrails (verified against `AGENTS.md`), its one
non-duplicative rule (ask when uncertain) is at most a mild lean against
default-to-action, and it is a foreign, unshipped dependency in product-facing
guidance. Owner decision: absorb the small additive value natively, then retire
the skill.
