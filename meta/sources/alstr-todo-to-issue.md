---
id: src.alstr-todo-to-issue
file: https://github.com/alstr/todo-to-issue-action
verification: external
type: tool
date: 2026-07-10
---

# alstr/todo-to-issue-action

GitHub Action that opens issues from `- [ ]` checkboxes found in repository
markdown or code. Considered and rejected as the sync mechanism: it is keyed to
the checkbox-todo model, has no notion of cairn's `meta/todos/*.md` frontmatter
(`status: done` closes an issue, exactly one node ID per todo), performs no
inward triage of externally filed issues, and offers no stable per-todo identity
marker for idempotent upsert. A bespoke script maps cleanly to the ratified
design instead.
