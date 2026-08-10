---
node: cairn.brownfield
status: blocked
created: 2026-08-07
blocked_by:
  - todo.brownfield-onboard-decisions-index
  - todo.brownfield-extraction-authoring-reference
  - todo.brownfield-extraction-maintainer-ruling
  - todo.brownfield-extraction-external-validation
parent: todo.brownfield-decision-extraction
---

# Implement the brownfield decision-extraction flow

Implementation unit split out of `todo.brownfield-decision-extraction` under
the sizing rule. `todo.brownfield-extraction-mechanism` is done and
`dec.brownfield-extraction-mechanism` (accepted 2026-08-08, binding) rules the
mechanism: a deterministic `cairn onboard decisions` evidence index plus a
shipped `cairn-dev` authoring reference.

That ruling names 26 affected paths across the CLI, brownfield modules, copy
table, docs, agent-pack content, manifest, adapter mirrors, and three test
suites, and it also requires validation against an external repository. That is
more than one small reviewable PR, so this unit was decomposed on 2026-08-10
under the sizing rule and is blocked on sub-todos:
todo.brownfield-onboard-decisions-index (clause 1, the deterministic command),
todo.brownfield-extraction-authoring-reference (clauses 2 and 3, the shipped
reference and its pack wiring), and
todo.brownfield-extraction-external-validation (the end-to-end drafted-artefact
assertion and the external-repository run). The iteration completing the last
child flips this parent to done.

That three-child list is the original decomposition and is kept as the record.
todo.brownfield-extraction-external-validation was itself decomposed on
2026-08-10 under the same rule, into todo.brownfield-extraction-drafting-test
and todo.brownfield-extraction-external-run. The maintainer ruling its criteria
terminate in could not sit under it: no iteration can produce a maintainer's
signature, and a child parked `blocked` under that unit would let a sibling
landing close it with the ruling unmet, because landing closes a parent whose
last OPEN child lands. So todo.brownfield-extraction-maintainer-ruling hangs
here instead and is the blocker that keeps this todo open, after validation
completes, until that ruling lands.

## Task

Build the extraction path for an existing codebase: walk the repository and its
ADR-like material (`docs/adr`, `docs/decisions`, README sections, long-lived
invariant comments), and produce decision drafts carrying `nodes:` bindings
that resolve against the blueprint the user already has.

Drafts stay proposals per `dec.decision-ratification-tiers`: the flow never
writes `status: accepted`. Build exactly the surfaces the mechanism ruling
names; do not re-derive them here.

## Acceptance

- On a brownfield fixture or a real external repository, the flow produces at
  least one decision artefact bound to real nodes, starting from code the user
  never annotated for cairn, and that artefact is accepted by the maintainer
  path rather than by the tool.
- Validation runs against at least one external repository, with the run
  recorded as a source or research artefact. No fixture the change adds is the
  dogfood repo, evidenced by the change's fixture list.
- Behaviour is covered by a test: the drafting entry point is exercised against
  a fixture repository that contains ADR-like material, and the assertion is on
  the drafted artefact's `nodes:` binding and non-accepted status.
- `cairn scan --strict` exits 0.
