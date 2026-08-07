---
node: cairn.root
status: open
created: 2026-08-03
---

# Roadmap Assumption Audit

Maintainer request 2026-08-03: the next working session should clean up
hidden assumptions in the outstanding items and roadmap, and run an
audit/codebase health pass, rather than building features.

## Task

1. Walk every open todo and active change: surface assumptions that are
   stale after the 2026-08-02 campaign (relationship schema, roadmap
   view, console, briefings, reverse provenance all shipped; three
   decisions signed 2026-08-03) and amend or close on the evidence.
2. Codebase health run: module sizes against the seam rule, dead code,
   duplicate copy keys, fixture freshness, dependency direction spot
   checks, and the standing Info findings (decision accumulation
   thresholds are already firing on four nodes).
3. File one unit per real defect found; fix only trivial ones in-audit.

## Acceptance

- Every open todo carries either a current-state note or a status
  change; the health findings are enumerated with owners.

cairn.root anchor justified (2026-08-07): amends meta/todos across the portfolio.
