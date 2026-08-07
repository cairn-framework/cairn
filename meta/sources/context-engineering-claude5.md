---
id: src.context-engineering-claude5
file: https://claude.com/blog/the-new-rules-of-context-engineering-for-claude-5-generation-models
verification: external
type: article
date: 2026-07-24
---

# The new rules of context engineering for Claude 5 generation models

Anthropic (Thariq Shihipar), 2026-07-24. Read in full on 2026-08-07 while the
maintainer redirected cairn's ratification regime toward reviewer panels.

Claims relevant to this repository, quoted or tightly paraphrased:

- "We removed over 80% of Claude Code's system prompt" for Claude 5 generation
  models "with no measurable loss on our coding evaluations."
- "We were overconstraining Claude Code, both through our system prompt and in
  our CLAUDE.md files and skills." Conflicting rules force the model to think
  about the rules instead of the work.
- Then: give Claude rules. Now: let Claude use judgement. Guardrails written
  for worst-case older-model behaviour can be deleted; surrounding context and
  judgement handle the cases the rules were guessing at.
- Then: give examples. Now: design interfaces. Examples constrain the
  exploration space; expressive parameters and typed vocabularies hint usage
  better than sample invocations.
- Then: put it all upfront. Now: progressive disclosure. A tree of files loaded
  at the right time beats a central repository of every known practice.
- Then: repeat yourself. Now: state it once, in the tool description.
- Rubrics are references: "Rubrics allow Claude to try and verify your taste in
  a particular field... by spinning up verifier agents with those rubrics."

Consumed by `dec.reviewer-panel-ratification` (rubric-driven verifier panels as
the ratification instrument) and `todo.context-engineering-pass` (the
overconstraint prune over AGENTS.md and the skill pack).
