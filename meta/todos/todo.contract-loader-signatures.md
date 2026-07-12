---
node: cairn.kernel.artefacts
status: open
created: 2026-07-12
---

# Contract loader interface signatures

gh:#63

Parse declared interface signatures from contract bodies so static
contracts can drive interface contradictions. Contract surfacing is
partially covered by dec.contract-leaf-coverage and the wire-leaf-contracts
work, but signature parsing is not done: a contract that declares a
function signature cannot yet contradict the reconciled interface.

Re-minted from GitHub issue #63 by todo.github-issues-cleanup
(2026-07-12); the issue is closed pointing at this artefact.
