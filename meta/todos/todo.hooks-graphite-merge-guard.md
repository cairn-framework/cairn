---
node: cairn.kernel.hooks
status: open
created: 2026-07-12
---

# Guard against stack-merge code drops

gh:#86

Dogfood finding: a Graphite-style stacked merge can silently drop code
from intermediate PRs. Investigate a hook or scan check that detects
post-merge loss (e.g. reconciling merged blueprint claims against the
actual tree after a stack lands). Related: dec.loop-resolves-knowable-gaps.

Re-minted from GitHub issue #86 by todo.github-issues-cleanup
(2026-07-12); the issue is closed pointing at this artefact.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It protects the merge path that keeps maintenance work safe.

2026-08-07 audit (todo.roadmap-assumption-audit): re-scope per res.chatgpt-issue-audit: rewrite as exact-head verification of the merge candidate rather than a Graphite-specific guard.
