---
id: res.chatgpt-issue-audit
nodes:
  - cairn.root
sources: [src.chatgpt-issue-audit]
date: 2026-08-07
---

# ChatGPT open-issue audit (external, unverified)

An external model audit of the 42 open mirrored GitHub issues, commissioned
by the maintainer and handed over in-session on 2026-08-07. This artefact is
a condensed capture of its dispositions plus a staleness record from the
receiving session; the audit itself modified nothing. It is evidence for
`todo.roadmap-assumption-audit`, never authority: every disposition must be
re-verified against live todo frontmatter, decision status, and the archive
trail before acting, and any disposition that contradicts an accepted
decision requires a refining decision, not an issue closure.
The `cairn.root` anchor is justified because the audit spans the whole
todo portfolio, matching its consuming todo's anchor.

## Staleness at capture (verified 2026-08-07, post rung-3 merge)

The audit predates the rung-3 substrate merge (PR #589, merge commit
9edfdac) and is wrong or stale on these points:

- "The driver may move into the repository only after
  `dec.orchestration-placement` is signed": that decision is `accepted`,
  refined by the accepted `dec.rung-three-coordination-substrate`, and the
  driver's substrate (coord fact store, write-sets, wave composer,
  `cairn ruling run`) is on main. Its step 3 fork is resolved.
- "#562 parallel dispatch granularity: keep as research until a driver
  exists": `todo.parallel-dispatch-granularity` is `done`; rungs 1 to 3
  shipped.
- "#425 UI asset refresh: close as obsolete": its blockers were updated
  2026-08-03 to `todo.console-state-legibility` and
  `todo.console-signed-widening`; it is deliberately blocked, not
  obsolete.
- "#575 console signed widening: only live gate is orchestration
  placement": now blocked by `todo.console-orchestration-ux-design`.
- "There are currently no open PRs": PR #589 was open at receipt.
- "#296 node-overlap conflicts query: useful regardless of eventual lease
  design": needs re-scoping against the shipped wave composer and lease
  read surface rather than kept as written.

## Dispositions as received

Close or replace (10): #582 roadmap assumption audit (close after
encoding), #425 UI asset refresh (obsolete), #298 outcome-first copy
(delivered), #338 repo organisation cleanup (unbounded umbrella), #482
parent/child package cycle (exhausted umbrella, #504 owns the work), #560
ghost-anchored todos guidance (rule conflates work graph with blueprint),
#498 revisit-trigger correlator (dormant non-work tracker), #380 update
awareness (no recorded user failure), #526 lint selection folding (landed
except unratified `defers:` field), #527 local gate attestation (its own
measurements refute the premise: a perfect receipt saves ~45s against
~159s CI).

Replacement units it justifies (2): decision-accumulation signal
correction; hermetic gate parity (pin toolchains, reconcile local and CI
commands, remeasure; no attestation until a recorded threshold fires).

Keep but repair (10): #584 todo taxonomy (after cleanup, derive kinds from
the cleaned corpus), #579 driver in repo (programme epic; split after
signature), #575 console widening (child of #579), #562 dispatch
granularity (re-parent under #579), #559 build/CI observation overlay
(drop stale schema blocker), #543 review gate machine check (subordinate
to declarative workflows), #563 receipts provenance interop (unblock; no
real blocker), #335 typed relationships (unblock; PR #570 shipped the
schema, only GitHub projection remains), #305 webui design quality
(unblock and narrow to Bet D proxies), #291 stack-merge code drop guard
(rewrite as exact-head verification of the merge candidate).

Keep as written (22), grouped: adoption defects #578 (brownfield init
review handoff, called the highest-priority product defect), #580
(module-size adopter scope, 30 of 32 MAG findings were the house rule),
#367 (init-time ignore scaffolding), #558 (brownfield decision
extraction), #504 (nested-package scan), #280 (AutoDocs Arm A, deferred);
integrity defects #546 (ratification candidate pointer bypass), #583
(status/todos path double prefix), #478 (map snapshot freshness gate),
#538 (spec-rules reanchor), #375 (hook remediation pointer), #378 (tag
registry); investigation and contracts #458 (node symbol coverage), #379
(wire-format schemas in batches), #296 (node-overlap query), #573
(decision-to-todo unblock edge), #334 (full issue-body projection,
required by accepted task-tracking authority); evaluation and release
#369 (blueprint authorability eval, now executable), #525 (greenfield
completion eval, release cadence), #301 (plain-language terminology,
ruling first), #552 (next milestone release, refresh inventory at
release), #581 (Orca plugin triage, preserve the diff first).

Proposed execution order as received: make the backlog truthful and close
#582; preserve and triage the Orca experiment; resolve the placement
decision (already resolved, see staleness); fix adopter-facing failures
in order #578, #546, #580, #583, #504, #367, #558; finish the
task/control-plane spine (#334, #335, #573, #579, #575, #296, #562,
#543); keep experiments behind defects.

Bottom line as received: retain 32 of 42, close or replace 10; the
backlog needs a truthfulness pass, not a reset.
