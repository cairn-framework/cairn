---
node: cairn.root
status: done
created: 2026-07-22
---

# Spec Authority Retirement

## Priority

P3 cleanup outside the critical path. It may start after
`todo.agent-guidance-router-playbooks` while treatment and OMP work continue.

## Depends on

`todo.agent-guidance-router-playbooks`.

## Invariants

- Routine agent work never bulk-loads `docs/spec.md`. If a routine task needs
  it, the graph, a contract, registry, or JIT reference is incomplete.
- Agent reads are graph-first. The spec remains fallback narrative for humans
  and for questions the graph cannot yet answer.
- Content moves to the layer that consumes and enforces it.

## Scope

- Correct actuator wording from human-only language to operator, human or
  agent, where the operation is agent-capable or deterministic.
- Add the write-side convention with exact homes:
  - rules to spec-rules and error-code registries;
  - planned structure to blueprint ghost nodes;
  - plans to todos and changes;
  - open questions to gap decisions;
  - subsystem design to contracts;
  - rationale to decisions;
  - procedural guidance to skills and JIT references.
- Migrate sections one at a time to pointers after their graph primitive is
  authoritative. Do not rewrite the whole document.
- Require a canonical-home decision for blueprint grammar and artefact-schema
  sections before collapsing either section.

## Acceptance

- The old read-first wording is absent from routine agent surfaces.
- The spec gains no new workflow, plan, rule, or normative subsystem design.
- Each collapsed section points at a real, queryable owner.
- Narrative history and the two-chain explanation remain readable.
