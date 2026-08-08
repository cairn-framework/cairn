---
node: cairn.brownfield
status: open
created: 2026-08-07
blocked_by: [todo.brownfield-extraction-mechanism]
parent: todo.brownfield-decision-extraction
---

# Implement the brownfield decision-extraction flow

Implementation unit split out of `todo.brownfield-decision-extraction` under
the sizing rule. Blocked until `todo.brownfield-extraction-mechanism` rules on
which mechanism does the drafting; this unit builds exactly what that ruling
names and nothing else.

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
