---
id: dec.domain-expandability
nodes:
  - cairn.root
  - cairn.reconcile
status: accepted
date: 2026-06-27
informed_by:
  - res.cairn-domain-expandability
---

# Cairn is domain-agnostic at the kernel; non-code domains arrive via pluggable reconcilers

## Context

Cairn is built to map software architecture, but the underlying primitives
(the System/Container/Module/Actor blueprint grammar and the
Research -> Decision -> Blueprint -> Contract -> Code two-chain) are not
inherently code-specific. The question of whether cairn extends to non-software
domains (hardware/MBSE systems, org structure, product BOMs, research
programmes) was analysed during the getcairn.dev adoption research phase (see
`res.cairn-domain-expandability`, citing `src.cairn-domain-expandability`).

The spec already commits to the resulting direction (`docs/spec.md` §157-158 on
the pluggable reconciler interface, §780/§782 placing additional reconcilers in
the phase-10 distribution/extension band, §852 on multi-target dispatch), and
`meta/contracts/reconcile.md` declares the domain-agnostic `Reconciler` trait.
That direction had no decision artefact carrying its provenance, so the
informing research read as orphan.

## Decision

Record the architectural commitment the spec already implies:

- The blueprint grammar and the two-chain model are **domain-agnostic**. They
  are not specialised for code and require no change to serve other domains.
- The **code reconciler is the only domain-specific component**. The reconciler
  interface (`cairn.reconcile`) is a pluggable, domain-agnostic trait: given a
  node, produce a content-addressable fingerprint of its current state and a
  list of claimed sub-elements.
- **Non-code reconcilers are deferred** to the phase-10 distribution/extension
  band. v1 ships exactly one reference implementation, the code reconciler.

## Rationale

The reality layer must produce a deterministic, content-addressable fingerprint
for drift detection to hold; without it the authority chain collapses to
documentation. Keeping the only domain-specific surface behind one pluggable
trait means new domains are added by writing a deterministic reconciler, not by
mutating the kernel. Deferring non-code reconcilers keeps v1 scope bounded while
preserving the extension path.

## Consequences

- Adding a non-code domain (org structure, BOMs, research programmes) means
  implementing a new `Reconciler` and registering it; the kernel, grammar, and
  artefact model are untouched.
- This decision documents an existing spec-stated commitment; it introduces no
  new scope and does not reopen `dec.no-orchestrator` or `dec.code-reconciliation`.
- Until a phase-10 reconciler lands, non-code domains remain analysis-only;
  this decision is the durable provenance anchor for that future work.
