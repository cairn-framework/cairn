---
node: cairn.brownfield
status: blocked
created: 2026-08-09
---

# Maintainer ruling: withdraw the item 7 claim from the Arm B drop

Blocked on the maintainer. Not agent-actionable: `cairn-loop-reconcile`
section 4 forbids the loop from self-ratifying a binding decision, and this
target was in addition signed by the maintainer personally.

## The forced choice

`dec.autodocs-head-to-head-arm-b` (accepted, maintainer-ratified, PR #528 sheet
W6) says in its Rationale that dropping Arm B and running Arm A "also unblocks
the large-brownfield measurement `res.codeatlas-analysis` item 7 deferred".

Arm A has now run (`res.autodocs-arm-a-brownfield-run`) and that claim is false.
Item 7 waits on a *large* brownfield repository. AutoDocs holds 284 tracked
files, which is medium, and the run measured no JSON output token counts, which
is item 7's actual quantity.

Accept `dec.autodocs-arm-a-item-7-correction`, or reject it.

- **Accepting** withdraws exactly that one sentence. The Arm B drop, its revisit
  trigger, and its refusal to substitute a local model for the quality axis all
  survive, restated inside the successor. No downstream work moves: item 7 was
  deferred before Arm A and stays deferred after it.
- **Rejecting** asserts that Arm A did satisfy item 7, which requires reading
  the 284-file measurement as large enough and would make item 7 eligible for
  selection now.

Recommendation: accept. It only subtracts a claim the evidence disproves.

## Depends on

The maintainer's ruling. Nothing else.

## Acceptance

On accept, in one landing:

- `dec.autodocs-head-to-head-arm-b` set `status: superseded`, and
  `dec.autodocs-arm-a-item-7-correction` gains
  `supersedes: [dec.autodocs-head-to-head-arm-b]`, replacing the placeholder
  `related:` entry. These two must land together: the
  `CAIRN_DECISION_SUPERSEDES_STATUS` check keys on the target's status, so the
  link without the demotion fails `cairn scan --strict`.
- `dec.autodocs-arm-a-item-7-correction` set `status: accepted`. That check does
  not inspect the successor's status, so this transition belongs in the same
  landing for semantic reasons, not because the validator forces it.
- `todo.autodocs-head-to-head` History: the 2026-08-09 entry ends "pending
  `todo.autodocs-arm-a-item-7-ratification`". Amend it to record the ruling and
  its date, so the parent no longer describes the correction as awaiting a
  decision.
- This todo set `done`.

On reject, in one landing:

- `dec.autodocs-arm-a-item-7-correction` set `status: deprecated` and its
  Decision section amended to record the ruling. `deprecated` is the schema's
  non-accepted terminal (`DecisionStatus` admits only `proposed`, `accepted`,
  `deprecated`, `superseded`; there is no `rejected`), and
  `dec.contract-node-shape-drift-deferred` is the precedent. Do not delete the
  file: the parent `todo.autodocs-head-to-head` cites it by id in its Scope,
  History, and Mission disposition, and this todo cites it throughout, so
  removal would leave dangling references.
- `dec.autodocs-head-to-head-arm-b` left accepted and unchanged.
- Three current-state claims written on the assumption that the correction
  would stand must be reconciled in the same landing, or the graph will assert
  both outcomes at once:
  - `todo.autodocs-head-to-head` Scope: the struck item 7 bullet is reinstated
    as live, not struck.
  - `todo.autodocs-head-to-head` History and Mission disposition: both assert
    the run falsified the item 7 claim and that item 7 stays deferred, and the
    History entry additionally says the correction is pending this todo. Amend
    both to record that the maintainer ruled otherwise, citing this todo.
  - `res.codeatlas-analysis` item 7: the 2026-08-09 note says the deferral
    stands. Replace it with the ruling and lift the deferral. Rejection means
    Arm A satisfied item 7, which is the same thing as item 7 no longer being
    deferred; leaving the deferral in place would contradict the ruling.
- This todo set `done`.
