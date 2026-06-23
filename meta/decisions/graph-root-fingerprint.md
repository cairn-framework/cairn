---
id: dec.graph-root-fingerprint
nodes:
  - cairn.kernel.scanner
  - cairn.reconcile
status: accepted
date: 2026-06-23
revisit_triggers:
  - "A real consumer needs O(1) commit-to-commit architecture comparison and the per-node/per-target recompute-and-compare gate proves too slow at repo scale"
  - "cairn adopts a Dolt remote (refs/dolt/*) or any cross-clone reconciliation that needs a single committed graph summary"
  - "InterfaceFingerprint moves off DefaultHasher to a pinned, cross-version-stable hash, removing the toolchain-dependence that disqualifies a committed root today"
---

# Graph-root fingerprint: reject the store, close edge drift, defer the aggregate root

## Context

A design question was raised: "Dolt is git for SQL; should cairn be git for a
knowledge graph?" That frames two very different things, and conflating them
would push cairn toward the exact failure mode it exists to prevent.

The first reading is a **versioned graph STORE**: a Dolt-analogue that keeps the
reconciled graph as the canonical, mutable-with-history artefact. The second is a
**content-addressed fingerprint OF the reconciled graph**, an aggregate hash
bound to a git commit. The first is a new source of truth; the second is a
derived summary of the existing one.

Cairn already fingerprints reality, and already gates drift, at a fine
granularity:

- **Per-target interface hashes.** `InterfaceFingerprint`
  (`src/reconcile/fingerprint.rs`) is a deterministic hash of a node's sorted
  public symbols, persisted as `TargetHashes` (`BTreeMap<String, String>`) in
  `.cairn/state/interface-hashes.json`. The interface gate names the drifted
  target (`CAIRN_INTERFACE_HASH_CHANGED`).
- **Per-node structural fingerprints.** `NodeFingerprint` (kind, parent, sorted
  paths) is collected into a versioned `BlueprintSnapshot`
  (`BTreeMap<String, NodeFingerprint>`, schema version 1;
  `src/scanner/state.rs:65-70`). The blueprint-change gate
  (`check_blueprint_change_decisions`, `src/scanner/checks.rs:48-67`) names the
  node whose shape changed and requires a covering decision
  (`CAIRN_BLUEPRINT_CHANGE_NO_DECISION`).

Both maps use `BTreeMap`, so their iteration order is already deterministic.
`.cairn/` is gitignored (`.gitignore:51`): the persisted state above is
local-derived cache, never committed.

Two facts shape the decision below:

- **There is a real gap: dependency-edge drift is not gated.** Declared
  dependency edges are validated for endpoint existence only (`validate_edges`,
  `src/map/build.rs:96-128`, `CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT`) and built
  in-memory into `Graph.outbound`/`inbound` every scan for cycle, ordering, and
  neighbourhood queries. They are never recorded in `BlueprintSnapshot` (which
  carries `nodes` only) and never drift-compared. So adding, removing, or
  retargeting a declared cross-module dependency is a structural change with no
  covering-decision gate, unlike a node `kind`/`parent` change.
- **An aggregate graph-root hash has no consumer.** Searching `src/` for
  `GraphRoot`/`graph_root`/`Cairn-Graph-Root` returns nothing. Its one unique
  capability, O(1) "did the architecture change between commits A and B?", is
  hypothetical: nothing in cairn calls for it today.

## Decision

1. **Reject a versioned graph store** (the Dolt-analogue). The reconciled graph
   is derived, not authored. Authored inputs (the blueprint and artefacts as
   markdown, source files as code) already live in git's content-addressed object
   store, which gives history, branch, merge, and diff for free. You version the
   inputs and recompute the projection; you do not version the projection. A
   second canonical store is the two-source-of-truth drift trap that
   `dec.no-orchestrator` and `dec.bd-upgrade-plan` already reject in their
   domains.

2. **Close the real gap with the existing pattern: gate dependency-edge drift.**
   Record each node's outbound edge set in `BlueprintSnapshot` (or a sibling
   snapshot keyed by node id, deterministically ordered like the existing maps),
   and extend `check_blueprint_change_decisions` to emit
   `CAIRN_BLUEPRINT_CHANGE_NO_DECISION` for a node whose declared edge set
   changed without a covering decision. This keeps the per-node, actionable
   granularity cairn already commits to (`dec.code-reconciliation`: per-node
   hashes "let the gate identify which module drifted") and reuses the snapshot
   and finding machinery wholesale. No aggregate, no opaque "something changed".

3. **Defer the aggregate graph-root fingerprint.** It is the correct shape *if* a
   consumer ever appears (a derived Merkle-style fold over the deterministically
   ordered node fingerprints, edges, and interface hashes, reusing
   `InterfaceFingerprint` as the single hash primitive, always recomputed and
   never read back as authority). But it has no consumer today and its value is
   gated on the revisit triggers above. Adopting it now would be the same
   premature abstraction this decision rejects the store for, and an opaque
   aggregate would regress the per-node/per-target finding granularity rather than
   add to enforcement. Recompute-and-compare at the existing granularity stays the
   gate.

4. **Bind nothing to the commit now.** The drift value is always recomputed from
   the tree and is always available, so a committed copy is never load-bearing:
   present-and-matching is redundant, present-and-stale is a worse signal than the
   per-node gate, and absent is the routine state after any rebase, squash, or
   amend. A committed value is also actively unsafe here: `InterfaceFingerprint`
   uses `DefaultHasher` (`src/reconcile/fingerprint.rs:23`), a SipHash whose
   output is not guaranteed stable across Rust versions, while CI runs a floating
   stable toolchain. A cross-machine committed hash could therefore differ from a
   local recompute with zero real drift, producing a false-positive finding. If a
   binding is ever justified (a real cross-clone consumer plus a pinned,
   cross-version-stable hash), a `Cairn-Graph-Root:` commit trailer is the
   least-bad mechanism: it travels with git natively, adds no tree-diff noise, and
   stays non-authoritative. A tracked `.cairn/state/root` would require un-ignoring
   derived state and version a projection (contradicting point 1); a git note is
   invisible to contributors and needs separate refspec plumbing.

## Rationale

**Why not a versioned graph DB.** Dolt exists because SQL had no native version
control: tables are mutable in place. Cairn has the opposite problem already
solved, because its authored inputs are git blobs. A versioned graph store would
duplicate git's job for a projection that should be recomputed, and would
reintroduce divergence between "the graph the store remembers" and "the graph the
code reconciles to" (the same lesson as jsonl-vs-Dolt in `dec.bd-upgrade-plan`).

**Why the edge-drift gate is the real win.** The edge gap is genuine and
non-coverable by anything today, and it sits squarely in cairn's lane: gating
structural drift against covering decisions. Closing it via the snapshot pattern
delivers an actionable finding ("node X's dependency edges changed without a
decision") with no new abstraction, no new source of truth, and no new scan pass
(edges are already recomputed in memory every scan).

**Why defer the aggregate root.** An aggregate hash earns its keep only when a
consumer needs a single comparable value (cross-commit or cross-clone). Cairn has
no such consumer, and the existing recompute-and-compare gate already answers
"did anything drift, and where?" at finer granularity than a root ever could.
Building the root now is cost (a fold, a CLI surface, a hash-stability obligation)
for a capability nothing uses.

**Why no binding now.** Decision point 4 above: the recompute path always wins, so
the bound value is never load-bearing; and a committed value built on
`DefaultHasher` is toolchain-dependent, so it is specifically wrong as a
cross-machine artifact. The gitignore fact rules out the tracked-file option but
does not by itself argue for any binding.

## Consequences

- **Adopt now (this spike's ruling):** reject the versioned store, and adopt the
  *design* of the edge-drift gate (extend `BlueprintSnapshot` with edges; emit a
  per-node change finding). **Defer** the aggregate graph-root fingerprint and any
  commit-binding until a real consumer and a stable hash exist.
- **Implementation is maintainer-gated and not started by this spike.** Per the
  maintainer-directed posture in `meta/session-handoff.md`, scheduling the build
  is George's call. One implementation bead (`cairn-9v1`) carries the actionable
  unit: the dependency-edge-drift gate. The aggregate root and trailer remain a
  recorded, deferred option, not filed for build.
- **Non-goal, recorded explicitly:** cairn does not gain a versioned graph DB, a
  Dolt-style mutable graph store, or any separate canonical graph store; and it
  does not gain a committed graph-root artifact while the hash is toolchain-
  dependent and no consumer exists. The graph stays derived and recomputed from
  git-tracked inputs.
