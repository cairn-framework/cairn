---
id: dec.coord-fact-write-once
nodes:
  - cairn.coord
  - cairn.persist
status: proposed
date: 2026-08-09
informed_by:
  - res.chatgpt-architecture-review
refines:
  - dec.rung-three-coordination-substrate
affects:
  - src/coord/append.rs
  - src/persist.rs
  - src/coord/verify.rs
---

# Coordination fact write-once semantics

## Context

`dec.rung-three-coordination-substrate` clause 2 requires one atomically written
file per fact and describes the coordination store as append-only. The initial
implementation selected `persist::atomic_write`, whose rename step is
replace-capable. A repeated fact path could therefore replace bytes already
observed by a reader. The architecture review's in-session verification
confirmed this write-once gap against the current tree.

Format 1 fact inputs retain canonical whole-second UTC `recorded_at` values,
with fail-closed rejection of fractional or offset spellings. The driver
verified the live store at this ruling as nine facts with zero fractional
timestamps, so no legacy timestamp migration obligation exists.

## Proposed ruling

Coordination fact paths are write-once. The append path MUST create the target
without replacing it. If the target already exists, append MUST fail and leave
the existing bytes unchanged. The implementation uses the write-once
`persist::atomic_write_once` helper rather than the replace-capable
`persist::atomic_write` helper. `atomic_write_once` deliberately retains the
temporary file's non-group-writable mode, unlike `atomic_write_bytes`'s
0o666 default, because immutable fact bytes are never group-writable. Temporary
bytes MAY be created while preparing the operation, but only an exclusive target
creation makes the fact visible.

This rule applies only to fact files under `facts/` and their immutable moved
copies under `archive/`; it excludes lease tokens under `leases/` and
`singleton/`, the derived `cache/observed.json`, and immutable `sidecars/`, each
of which has its own persistence rule. It includes files whose content is
identical to a new append attempt. Compaction now moves a live fact from
`facts/` to `archive/` with an exclusive no-replace operation, and `verify`
refuses a fact identity that appears in both live and archived sets. A later
re-append is a new observation with a new `recorded_at`, filename, and identity,
so it does not clash with the archived path.

## Rationale and consequences
The fact identity and filename are part of the audit surface. Replacing a
published path would let a reader observe one envelope and a later reader
observe another under the same name, defeating append-only verification and
changing the recorded history under a stable path. Full reads now parse the
immutable facts directly; no parse-cache trust or regeneration rule remains.

Oversized decline preimages use immutable `sidecars/` paths keyed by the
digest and compact observation second. A later observation gets a distinct
sidecar; a same-second retry for the same digest collides and fails closed,
which is the sidecar's own write-once rule rather than a fact-file replacement.

This is a refining rule for the accepted coordination substrate, not an
acceptance of this draft. The implementation and regression test provide the
proposed behavior; the driver's ratification panel decides whether this rule
is accepted. Reversing it requires a further refining decision and an explicit
replacement for the append-only guarantee.

## Ratification state

Proposed on 2026-08-09. No acceptance receipts are claimed here.
