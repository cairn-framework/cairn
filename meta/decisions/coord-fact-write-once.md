---
id: dec.coord-fact-write-once
nodes:
  - cairn.coord
status: proposed
date: 2026-08-09
informed_by:
  - res.chatgpt-architecture-review
refines:
  - dec.rung-three-coordination-substrate
related:
  - dec.rung-three-coordination-substrate
affects:
  - src/coord/append.rs
  - src/persist.rs
---

# Coordination fact write-once semantics

## Context

`dec.rung-three-coordination-substrate` clause 2 requires one atomically written
file per fact and describes the coordination store as append-only. The initial
implementation selected `persist::atomic_write`, whose rename step is
replace-capable. A repeated fact path could therefore replace bytes already
observed by a reader. The architecture review recorded this as an unverified
write-once gap, and the S8 implementation verified it against the current tree.

## Proposed ruling

Coordination fact paths are write-once. The append path MUST create the target
without replacing it. If the target already exists, append MUST fail and leave
the existing bytes unchanged. The implementation uses the write-once
`persist::atomic_write_once` helper rather than the replace-capable
`persist::atomic_write` helper. Temporary bytes MAY be created while preparing
the operation, but only an exclusive target creation makes the fact visible.

This rule applies to every fact file under the family coordination store,
including files whose content is identical to a new append attempt. A caller
that needs a new observation records a new fact with a new identity; it does
not rewrite an old identity.

## Rationale and consequences

The fact identity and filename are part of the audit surface. Replacing a
published path would let a reader observe one envelope and a later reader
observe another under the same name, defeating append-only verification and
making the derived parse cache unsafe to trust. Failing closed on an existing
path preserves the first published bytes and exposes duplicate or colliding
writes to the caller.

This is a refining rule for the accepted coordination substrate, not an
acceptance of this draft. The implementation and regression test provide the
proposed behavior; the driver's ratification panel decides whether this rule
is accepted. Reversing it requires a further refining decision and an explicit
replacement for the append-only guarantee.

## Ratification state

Proposed on 2026-08-09. No acceptance receipts are claimed here.
