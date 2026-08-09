---
node: cairn.kernel.artefacts
status: open
created: 2026-08-09
---

# A proposed supersession has no representable shape

Surfaced by the reconcile step of the Arm A run
(`res.autodocs-arm-a-brownfield-run`), not by that run's target repository.

## Evidence

`cairn-loop-reconcile` section 4 prescribes exactly this shape when a unit's
evidence invalidates an assumption inside an accepted decision whose acceptance
is the maintainer's call: "land the proposed decision plus a blocked todo and
say so plainly. Never self-ratify a binding decision."

That shape cannot be expressed. Writing the proposed successor with
`supersedes: [dec.<target>]` while the target is still `accepted` produces:

```
Warning: CAIRN_DECISION_SUPERSEDES_STATUS decision `dec.<successor>` supersedes
`dec.<target>` but target is not superseded
```

which fails `cairn scan --strict` (exit 1) and so blocks the very iteration
reconcile told the loop to land. The validator
(`src/artefacts/registry/validate/mod.rs:161-168`) keys only on the target's
status; it does not inspect the successor's. The only two ways out both cost
something:

- Demote the target to `superseded` now. That self-ratifies an amendment to a
  decision the loop may not accept, which `cairn-loop-reconcile` section 4
  forbids outright.
- Drop the structured link and carry the intent in prose plus the ratification
  todo. Gate stays green, but the graph cannot answer "what supersedes this, and
  is it pending".

The second was taken for `dec.autodocs-arm-a-item-7-correction`, with `related:`
standing in and `todo.autodocs-arm-a-item-7-ratification` carrying the deferred
frontmatter edits. It works, and it is invisible to every query.

## Scope

Decide how a pending supersession is modelled, then implement it. The obvious
candidate is to make `CAIRN_DECISION_SUPERSEDES_STATUS` conditional on the
successor's own status: a `proposed` successor may name its target without the
target being `superseded` yet, while an `accepted` successor still requires it.
Other shapes (a distinct `supersedes_pending:` field) are worth weighing against
that, since they cost a schema addition.

## Acceptance

- A `proposed` decision naming an `accepted` target through the chosen mechanism
  passes `cairn scan --strict`.
- An `accepted` decision naming a target that is not `superseded` still warns.
- Both are covered by tests.
- `cairn decisions <node>` can distinguish a live supersession from a pending
  one.
