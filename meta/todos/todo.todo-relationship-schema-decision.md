---
node: cairn.root
status: open
created: 2026-07-16
---

# Draft the todo-relationship schema decision

todo.todo-relationship-model-and-issue-links is blocked on a prerequisite that has no owner: a ratified decision (dec.todo-relationship-model) defining a typed todo-relationship schema. Context: todo frontmatter today parses only node/status/created/satisfies; `related:` is parsed for decisions but silently ignored for todos, yet todo files in the wild already carry it (e.g. todo.land-loop-command-rewrite has `related: [dec.loop-command-harness-model]`).

## Task
Draft dec.todo-relationship-model for owner ratification: relationship vocabulary (e.g. blocks/relates/parent), which frontmatter field carries it, whether `related:` on todos becomes parsed or is rejected, and migration for existing in-the-wild `related:` entries. Scope is the decision artefact only; implementation stays in the blocked todo.
