---
node: cairn.kernel.scanner
date: 2026-05-08
reviewer: bootstrap-fixture
review_type: human
---

# Review: dec.module-path-mapping (human acceptance)

## Subject

`dec.module-path-mapping` (in `meta/decisions/kernel/`) lands the rename of `Reconciliation` to `Scanner`, simplifies `ReconcilerInterface` to `Reconciler`, drops the `path` declaration on `Container Kernel`, and corrects every kernel module path to match the actual `src/` layout. This review captures the human acceptance pass on the rename itself.

## Verdict

Accepted.

## Justification

1. The rename pattern produces three distinct stems: `reconciler` (interface), `scanner` (engine), `code-reconciler` (concrete impl). Reading the blueprint aloud no longer requires disambiguation. The original `reconciler / reconciliation / code-reconciler` triple shared the `reconcil-` stem and conflated the engine with the interface in casual conversation.
2. Path-less containers are a documented spec affordance (`docs/spec.md` §7: a node has "optionally a path"). Dropping the path on the Kernel container is conformant; the container remains a logical grouping label.
3. Every renamed module's `informed_by:` reference points back to `dec.module-path-mapping`, preserving the provenance trail for future readers.

## Reservations

- The path-less Reconciler module elevated `CAIRN_CONTRACT_MISSING` from Warning to Error in the scan output until its contract file existed. Resolved by issue #52, which authored the contract alongside this review.
- The non-bootstrap fixture at `test/fixtures/cairn.blueprint` (a kernel-MVP self-description) still carries the old taxonomy. Tracked separately as a follow-up issue; not a blocker for this rename.

## Sign-off

Reviewer: `bootstrap-fixture` (proxy identifier for the human acceptance pass at the time the decision landed).
