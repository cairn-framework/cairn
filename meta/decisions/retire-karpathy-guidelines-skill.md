---
id: dec.retire-karpathy-guidelines-skill
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-07-25
informed_by:
  - res.harness-engineering
  - res.agent-experiment-linklint
related:
  - dec.init-emits-agent-skills
  - dec.unified-cairn-dev-entry
---

# Retire the `karpathy-guidelines` skill and absorb its coding discipline

## Context

`.claude/skills/karpathy-guidelines/SKILL.md` was a generic coding-discipline
skill carried in this repository's harness assets and pointed at from `AGENTS.md`
as one of two files "worth loading for any coding work". It was never shipped by
`cairn init`: `dec.init-emits-agent-skills` deliberately kept it out of the pack
because it is not about cairn.

`dec.unified-cairn-dev-entry` establishes one logical public entry, `cairn-dev`,
whose default mode is a compact router. A second broadly-scoped, always-load
coding skill sitting beside that router is the exact shape that decision exists to
remove: two public surfaces competing to be loaded first, with no routing between
them.

`res.agent-experiment-linklint` measured the guidance pack costing roughly 9,800
extra input tokens per run for no quality gain that reached the preregistered
threshold. An always-load generic skill is a permanent charge against that budget,
and its content is generic advice a capable model already applies.

## Decision

Retire the `karpathy-guidelines` skill. Delete
`.claude/skills/karpathy-guidelines/` and the `AGENTS.md` pointer to it, and
replace that pointer with one that names `cairn-dev` as the entry.

Absorb only the small additive part of its discipline into the `cairn-dev`
router's "Working discipline" section:

1. State assumptions, and present materially different interpretations rather
   than silently choosing one.
2. Turn the task into a success criterion checkable before starting; for a bug,
   that criterion is a failing test written first.
3. Apply the "would a senior engineer call this overcomplicated" simplicity
   check.

The surgical-change discipline moves to `cairn-loop-implement`, where it governs
an actual step rather than sitting in an always-loaded preamble.

Do NOT import its ask-when-uncertain default. That rule biases toward stopping and
asking, which conflicts with default-to-action; the absorbed form asks only for
materially different interpretations.

This decision does not touch `src.karpathy-llm-wiki`, which is unrelated prior art
cited elsewhere in the graph.

## Relationship to `dec.init-emits-agent-skills`

`related`, not `supersedes`. That decision ruled on which skills `cairn init`
emits, and its statement that `karpathy-guidelines` was deliberately unbundled
remains historically true and is unchanged by this one: the skill was never in the
pack, and it still is not. What changes is that it is no longer carried as a
repository harness asset either. No clause of that decision is contradicted, so no
supersession is required and its status stays `accepted`.

## Consequences

- One public entry for coding sessions in this repository: `cairn-dev`.
- The additive discipline survives at the point of use rather than as a preamble.
- `AGENTS.md` names one skill file instead of two.
- Any future generic coding skill faces the same test: if it is not about cairn
  and it is not routed to, it does not belong beside the router.
