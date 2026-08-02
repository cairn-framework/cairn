---
node: cairn.summariser
status: done
created: 2026-07-11
---

# Summariser: generate node summaries in topological order

Borrowed concept from AutoDocs/Sita (2026-07-11): they generate docs dependencies-first so dependents inherit precise context (DB, then auth, then services, then UI). Cairn already has `cairn order` (topological sort). Make the summariser and docstring generation traverse nodes in that order and include the already-generated summaries of a node's dependencies in the prompt context for its dependents.

Scope:

- `cairn summarise` and any batch summarisation path order work by the existing topological sort.
- Dependency summaries are passed as context when summarising a dependent node.
- No change to artefact formats or wire output.

Acceptance: summarising a node whose dependencies have summaries demonstrably includes them in context; behaviour covered by a unit test; existing summariser tests pass.

## Priority (added 2026-07-11 after backlog review)

DEFER (low priority). This is a summary-quality nicety borrowed from AutoDocs with
no user complaint driving it. Worthwhile but not ahead of first-run, terminology,
or positioning work. Keep the design; schedule opportunistically.

## Mission disposition

2026-08-02: close against dec.cairn-mission. Serves none. It is a deferred nicety with no driving user feedback.
