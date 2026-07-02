---
id: dec.persistent-map-snapshot
nodes:
  - cairn.kernel.scanner
status: accepted
date: 2026-07-02
informed_by: [res.vision-refactor-audit]
related: [dec.graph-root-fingerprint]
revisit_triggers:
  - "map.json exceeds the pre-commit large-file gate on some future repo (drop findings, then symbol signature strings, before dropping the file)"
  - "A real consumer needs O(1) commit-to-commit architecture comparison beyond a plain git diff of map.json"
---

# Persistent map snapshot: a committed, deterministic `map.json`

## Context

`dec.graph-root-fingerprint` (2026-06-23) rejected a versioned graph *store*
(a Dolt-analogue holding the reconciled graph as canonical, mutable-with-history
state) because it would compete with git as the source of truth. That ruling
stands: this decision does not introduce a store. `res.vision-refactor-audit`
finding 2 is a narrower gap the earlier decision did not resolve: there is no
committed, machine-readable snapshot of a scan's *output* at all. The full
`ScanResult` lives only in the gitignored `.cairn/state/reconciler-cache.json`
optimisation cache, which is silently recomputed on any version mismatch and
was never meant to be read by anything but the scanner itself.

## Decision

`scan()` writes `map.json` at the project root alongside the existing `map.md`
and `.cairn/log.md`: a `MapSnapshot { schema_version, interface_hash, nodes,
edges, findings }` built deterministically from `ScanResult` (`BTreeMap`
ordering, no timestamps). `map.json` is committed, not gitignored. This
**supersedes ruling 1 only** of `dec.graph-root-fingerprint` ("reject a
versioned graph store"): `map.json` is a derived, rebuildable measurement
record regenerated wholesale on every scan, not a mutable store with its own
history or write API. Rulings 2–4 of `dec.graph-root-fingerprint` (close the
real gap via edge-drift gating, defer the aggregate root fingerprint, bind
nothing to the commit) stand unchanged; `map.json` is not a fingerprint and
does not replace `InterfaceFingerprint` or the per-target hash gate.

## Rationale

Because the payload is deterministic and timestamp-free, re-running `scan()`
on an unchanged tree produces a byte-identical file: git history on `map.json`
then records only real architectural drift, giving reviewers a visible
symbol/state/finding diff on every PR without cairn owning a second source of
truth. `load_project()` stays pure and does not write it, preserving the
existing scan-persists/load-is-pure split.

## Consequences

- `map.json` carries `NodeRecord.symbols` from `dec.symbol-reality-layer`,
  making the snapshot the durable form of the symbol-level reality layer.
- If `map.json` ever exceeds the pre-commit large-file gate, `findings` is
  dropped from the snapshot first, then symbol `signature` strings (name/kind/
  location survive), per the revisit trigger above. Not pre-optimised now: at
  24 nodes this repo's own snapshot fits comfortably.
