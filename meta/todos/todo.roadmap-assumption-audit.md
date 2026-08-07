---
node: cairn.root
status: done
created: 2026-08-03
related: [res.chatgpt-issue-audit]
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

## Outcome (2026-08-07)

Executed as one loop iteration. Every non-done todo carries a dated
current-state note or a status change; the health run is enumerated with
owners in res.roadmap-audit-health.

- Status flips: todo.build-ci-observation-overlay open (blockers done),
  todo.workflow-serialises-validation blocked (blocker live); both
  CAIRN_TODO_STATUS_CONTRADICTION findings cleared.
- Trivial in-audit fix: two stale dead_code allows removed from
  src/summariser/backend/mod.rs (LocalCommandBackend is constructed at
  src/query_api/mod.rs:588).
- Units filed: todo.decision-accumulation-signal,
  todo.hermetic-gate-parity, todo.coord-fact-store-hardening,
  todo.coord-cursor-semantics, todo.query-contract-volatile-facts.
- Second external audit received mid-session (architecture review at
  head 9edfdac); captured as src.chatgpt-architecture-review plus
  res.chatgpt-architecture-review with in-session verification results.
- Maintainer confirmations queued in notes rather than acted on:
  closures of todo.local-gate-attestation, todo.update-awareness,
  todo.repo-organisation-cleanup, todo.ghost-anchored-todos-guidance,
  todo.positioning-outcome-first-copy; unblock of
  todo.receipts-provenance-interop; driver sequencing after coord
  hardening.
