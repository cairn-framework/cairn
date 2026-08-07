---
node: cairn.kernel.artefacts
status: open
created: 2026-08-03
---

# Todo Taxonomy

Maintainer request 2026-08-03: cairn-based tasks are all labelled
plainly "todo", which hides what kind of work each one is. Decide how to
make the taxonomy richer and more self-describing: a naming convention
(typed stem prefixes), a tagging convention (a frontmatter field), or
both.

## Task

1. Survey the live todo set and cluster the kinds that actually occur
   (bug, feature unit, decision-tracking, triage, audit, recurring
   hygiene, next-session pointer).
2. Decide the mechanism: stem naming convention, a typed frontmatter
   field validated by the scanner, or both; weigh against
   `dec.todo-relationship-model` (schema amendments are decisions, not
   drive-bys).
3. Record the ruling as a decision if the schema moves; implement the
   chosen convention and update authoring guidance.

## Acceptance

- A reader can tell a todo's kind from `cairn todos` output without
  opening the file; existing todos migrate or are exempted explicitly.

2026-08-07 audit (todo.roadmap-assumption-audit): keep; derive kinds from the cleaned corpus now that this audit has amended it.
