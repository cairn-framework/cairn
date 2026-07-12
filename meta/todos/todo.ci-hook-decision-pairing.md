---
node: cairn.kernel.hooks
status: open
created: 2026-07-12
---

# CI hook pairing blueprint changes with decisions

gh:#68

Enforce that blueprint architecture changes require a paired decision
artefact. CAIRN_BLUEPRINT_CHANGE_NO_DECISION exists at scan level; the
concrete CI/pre-push enforcement rule for edge/node structure edits
without a decision is not yet present as a hook gate.

Re-minted from GitHub issue #68 by todo.github-issues-cleanup
(2026-07-12); the issue is closed pointing at this artefact.
