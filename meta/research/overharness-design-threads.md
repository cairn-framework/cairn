---
id: res.overharness-design-threads
nodes: [cairn.root]
sources: [src.maintainer-design-threads-2026-07-30, src.huntley-software-factory]
date: 2026-07-30
---

# Over-harness design threads, 2026-07-30

Synthesises `src.maintainer-design-threads-2026-07-30` (the raw account) and
`src.huntley-software-factory` (the outside influence) into four findings.
It changes no rule: any normative move it suggests goes through a decision
or a todo, and the one gap it names is filed as
`todo.review-gate-machine-check`.

## Thread a: in-harness means declarative injection

The in-harness facet is declarative injection: skills, the agent pack,
guidance files, and CLI calls carry the way of working into whatever harness
appears. Pairing injected guidance with CLI re-query keeps context fresh
instead of stale-copied. MCP is a thin optional adapter for harnesses that
cannot shell out or read the repository; it is de-emphasised as the
headline, not the mechanism. This matches the shipped three-tier agent-pack
architecture (editable source, rendered assets, embedded binary) and the
mechanism evidence in `res.harness-engineering`.

## Thread b: over-harness workflow definitions, and the review gap

The over-harness layer needs first-class declarative workflow definitions
per project type, including a review workflow. The two-lens pre-submit
review is declaratively mandated (AGENTS.md, `cairn-loop-landing`) but only
trust-verified. The external driver checks the terminal token, todo status,
and park state; it never checks that the review ran. CodeRabbit is advisory
in CI. `todo.local-gate-attestation` is the nearest neighbour but owns a
different evidence class, hermetic gate receipts, so the gap is filed as
`todo.review-gate-machine-check`, cross-referencing it.

## Thread c: one shared multi-ref derived index for swarm-scale reads

Swarm-scale reads need one shared multi-ref derived index: claims and
status plus a graph index across origin/main and in-flight branches. The
index is derived and disposable, always rebuildable from repo truth. Sync
direction stays repo to DB; canonicity never moves. DoltLite remains the
parked fallback. The B-queue md5 ledger (the maintainer's supervised queue
infrastructure, outside this repository) is the single-writer prototype of
that coordination plane. This extends the accepted content/coordination
split: content stays git-canonical, and the derived store widens from one
working tree to many refs without gaining authority.

## Thread d: facets are a control plane, not a hierarchy

In-harness and over-harness are control-plane facets over existing
component nodes: a tag or derived view, not a hierarchy. Backend and
frontend are product components on a different axis. Facets are promoted to
containers only on lifecycle evidence. The driver-in-monorepo question
stays open for the driver-v2 proposal; until a superseding decision exists,
`dec.no-orchestrator` binds and the driver stays external.
