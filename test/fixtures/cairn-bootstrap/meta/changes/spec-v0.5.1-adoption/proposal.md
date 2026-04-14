# Proposal: Adopt v0.5.1 schema changes in the bootstrap

## Why

The Cairn spec moved from v0.5 to v0.5.1 with three schema-level changes that
affect existing bootstrap artefacts:

1. Container is now optional in the DSL grammar.
2. ADR frontmatter gained three optional fields (supersedes, refines, related).
3. Source frontmatter gained a required `verification` field with three states.

The bootstrap was authored against v0.5. To stay internally consistent and to
serve as a demonstration of v0.5.1 in action, the bootstrap needs updating.

## Scope

- Demote `Container Summariser` to a top-level Module (it holds one thing).
- Add `verification` field to all eight source sidecars with the correct state.
- Leave ADRs unchanged; none of them need supersedes/refines/related populated.
- Leave research artefact, directory layout, and other ADRs unchanged.

## Pending decisions (resolved before merge)

This change carries a small queue of design questions that surfaced from the
v0.5.1 edit and from related conversations. Each is captured below as a
candidate decision. They are not yet ADRs because they are still under
discussion — once decided, they become accepted ADRs and merge into the main
tree alongside the schema updates.

### Pending: Cavekit learnings

Status: investigated. The cavekit repo
(https://github.com/JuliusBrussee/cavekit) is a Claude Code plugin with a
five-phase pipeline (research → design → draft → architect → build → inspect),
explicit numbered requirements with acceptance criteria, automatic dependency
tier assignment, parallel work dispatch, and Codex as a cross-model
adversarial reviewer.

**Adopted into v0.5.1 spec:**

- `cairn order` query for dependency-tier ordering. Cavekit demonstrates the
  use case concretely (parallel work scheduling), but the query itself is
  structural graph topology and belongs in Cairn regardless of downstream
  consumer.
- `cairn depends` as the inverse of `dependents`. Obvious gap once both
  directions of the edge graph became relevant.

**Proposed for future spec revision (not yet decided):**

- Agent dissent and cross-model peer review may merit first-class artefact
  types, potentially unified. Captured as open question 12 in v0.5.1. Needs
  design before adoption.
- Explicit coverage block in contracts, mapping requirements to reality-layer
  elements. Cavekit formalises this as a coverage matrix; Cairn currently
  leaves the linkage informal. May or may not need formalising; depends on
  whether the informal linkage proves sufficient in real use.

**Not a spec concern (packaging/distribution decision):**

- Distribute Cairn as a Claude Code plugin in addition to a standalone CLI.
  Lowers the barrier for users who live in Claude Code. Decide during
  packaging, not during spec work.

**Deliberately rejected:**

- Cavekit's orchestration layer (build loops, tier gates, parallel dispatch,
  fix cycles). Same line as OpenSpec: workflow is not Cairn's space. Cairn
  provides the substrate; workflow tools consume it.

### Pending: Agent dissent artefact (open question 12)

Status: parked for v0.6. Conversation surfaced the idea that agents working
within a spec often have suppressed preferences worth capturing. Captured as
open question 12 in v0.5.1; needs proper design before becoming a decision.

Outcome: deferred. Not blocking this change.

### Pending: meta/ directory layout (open question 10)

Status: parked. Current bootstrap uses by-type-then-by-subsystem
(`meta/decisions/kernel/`). Alternative is by-node-then-by-type
(`meta/kernel/parser/decisions/`). Both have appeal. Decision deferred until
more bootstrap experience accumulates.

Outcome: deferred. Bootstrap stays as-is.

### Pending: ID stability across DSL restructuring

Status: surfaced during this change, captured as v0.5.1 open question 13.

When Reconcilers was demoted from a Container to a sibling Module during this
adoption, the CodeReconciler's ID changed from `cairn.reconcilers.code` to
`cairn.code-reconciler` to reflect the flatter structure. The change was
mechanical in this case (grep found no references in other artefacts), but it
surfaces a real question: what happens when a restructure breaks IDs that
were stable by contract?

Two defensible answers documented in open question 13. Decision needed before
v1.0 ships. Not blocking this adoption change.

## Implementation order

1. ~~Investigate cavekit; record findings in this proposal.~~ Done.
2. ~~Update sources with `verification` field.~~ Done (9 sources).
3. ~~Demote Summariser in cairn.dsl.~~ Done. Also demoted Reconcilers container (held one module); renamed CodeReconciler ID to match flatter structure.
4. ~~Verify the bootstrap still parses cleanly against v0.5.1.~~ Done. All edges resolve; no stale references in artefacts.
5. Archive this change.

## Notes

This change is itself a demonstration of the change-directory pattern working
on a real schema migration. The workflow felt light, not heavy — which is the
right signal for a v1 framework. One piece of real learning surfaced (ID
stability, open question 13), which is exactly what bootstrapping on oneself
is meant to do.
