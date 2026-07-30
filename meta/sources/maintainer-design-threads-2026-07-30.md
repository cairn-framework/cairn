---
id: src.maintainer-design-threads-2026-07-30
file: null
verification: unverified
type: conversation
date: 2026-07-30
---

# Maintainer design threads, 2026-07-30

The maintainer worked through four architecture threads in chat on 2026-07-30,
the same day as the mission ratification recorded in
`src.mission-ratification-2026-07-30`. This source records what was said;
analysis and implications live in `res.overharness-design-threads`.

1. In-harness means declarative injection. Skills, the agent pack, guidance
   files, and CLI calls carry the way of working into whatever harness
   appears. MCP remains a thin optional adapter for harnesses that cannot
   shell out or read the repository, and is de-emphasised as the headline.

2. The over-harness layer needs first-class declarative workflow definitions
   per project type, including a review workflow. Today's two-lens review is
   declaratively mandated but only trust-verified: the external driver checks
   the terminal token, todo status, and park state, never that the review
   ran, and CodeRabbit is advisory in CI.

3. Swarm-scale reads need one shared multi-ref derived index: claims and
   status plus a graph index across origin/main and in-flight branches,
   derived and disposable, always rebuildable from repo truth. Sync direction
   stays repo to DB; canonicity never moves. DoltLite remains the parked
   fallback. The B-queue md5 ledger is the single-writer prototype of that
   coordination plane.

4. In-harness and over-harness are control-plane facets over existing
   component nodes: a tag or derived view, not a hierarchy. Backend and
   frontend are product components on a different axis. Facets are promoted
   to containers only on lifecycle evidence. The driver-in-monorepo question
   stays open for the driver-v2 proposal.

The conversation cited Geoffrey Huntley's software-factory post as an outside
influence; that citation is recorded separately in
`src.huntley-software-factory`.

A chat session is not addressable from the repository, so this record carries
`file: null` and `verification: unverified`. The standing
`CAIRN_SOURCE_UNVERIFIED` Info finding is the designed marker for that state,
not a defect.
