---
node: cairn.root
status: open
created: 2026-07-12
---

# Feedback Issue Closures

gh:#247

Execute the GitHub closures deferred by the todo.capture-feedback-issues
verdict pass. todo.github-issues-cleanup explicitly excludes #232 to #247
from its sweep, so this todo owns them.

## Task
Using the verdict table in todo.capture-feedback-issues:

- Close the fixed-on-main issues (#232, #233, #235) with their recorded close
  rationales.
- Close the wont-fix issue (#240) citing dec.query-json-schema-version, noting
  the documentation residue is tracked in todo.cairn-dev-docs-sync (gh:#243).
- Close each still-valid issue (#234, #236, #237, #238, #239, #241, #242,
  #243, #244, #245, #246) with a comment pointing at its minted
  `meta/todos/todo.*.md` work item (each carries the matching `gh:#NNN` line).
- Close the umbrella #247 last, once every child issue above is closed.

## Acceptance
Issues #232 to #247 are all closed on GitHub, each with a rationale or a
pointer to its owning native todo.
