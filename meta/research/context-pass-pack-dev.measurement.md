---
id: res.context-pass-pack-dev.measurement
nodes:
  - cairn.kernel.cli
method: primary
date: 2026-08-11
related: [res.skill-absorption, res.reference-budget-headroom]
---

# Context pass measurement: cairn-dev router and non-loop references

First-hand measurement from `todo.context-pass-pack-dev` (2026-08-11), the
first child of `todo.context-pass-skill-pack`. Terms follow
`tools/agent-pack/tests/first_turn_budget_tests.rs`.

The surface's first-turn terms (router bytes and cairn-dev advertised
metadata, before and after) are recorded once, in
`todo.context-pass-pack-dev` as its task mandates; this artefact carries only
the evidence the todo does not: per-reference sizes and the compression
rationale.

## Reference sizes (local evidence, outside the first-turn metric)

| Reference | Before | After |
|---|---|---|
| task-bug-investigation.md | 2,268 | 1,795 |
| task-refactoring.md | 2,258 | 1,917 |
| task-architecture-discovery.md | 2,493 | 2,279 |
| task-feature-implementation.md | 2,633 | 2,620 |
| graph-navigation.md | 2,836 | 3,331 |
| finding-codes.md | 5,860 | 5,809 |
| task-brownfield-decision-extraction.md | 5,983 | 5,979 |
| blueprint-syntax.md | 4,587 | 4,587 |
| command-reference.md | 5,230 | 5,219 |
| artefact-schemas.md | 5,947 | 5,947 |

Every reference sits within the 6,000-byte `JIT_REFERENCE_BUDGET_BYTES`
ceiling. graph-navigation grew by design: it is now the single owner of the
span and language-server discipline the four task references each restated,
and those references point at it instead. task-feature-implementation held
its size because its while-you-write rules are the owning layer for blueprint
honesty specifics the slimmed router now points to. finding-codes moved
`CAIRN_CLI_MISSING_NODE` out of the hook-blocking Error section into a CLI
invocation-errors section, since it is a usage error the scan never emits.
blueprint-syntax and artefact-schemas were reached and left unchanged: pure
reference material already behind pointers, with no restatement or no-op
sentences found; command-reference only had a sibling pointer corrected.

## Limits

Byte counts are a proxy for context load. No CLI behaviour changed and no
accepted rule changed; the deleted sentences are collapsed restatements whose
surviving copies the owning files point to. This research informs the parent's
combined measurement and any future budget tightening, nothing else.
