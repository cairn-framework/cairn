---
node: cairn.kernel.hooks
status: done
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

## Outcome

Already satisfied; closed 2026-07-27 without new work. The premise above is
obsolete: the gate exists and runs.

- `src/hooks/architecture.rs` implements the architecture-decision gate.
- `src/hooks/mod.rs` includes it in `cairn hook all` and its blocking
  semantics, so `cairn hook all` exercises it.
- `tests/hooks_architecture.rs` proves the add and reassign failures and the
  paired-decision success.
- `scripts/dogfood.sh` invokes `cairn hook all`, and `.github/workflows/
  dogfood.yml` runs that script, so the CI enforcement this todo asked for is
  in place.

Nothing flagged the staleness because nothing re-checks an open todo's premise
against the code. Selection is oldest-created first
(`src/query_api/handlers/next_selection.rs`), so this sat eighth in the queue
while already done.
