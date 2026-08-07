---
node: cairn.kernel.scanner
status: open
created: 2026-08-07
related: [todo.roadmap-assumption-audit, res.chatgpt-issue-audit]
---

# Decision accumulation signal correction

`CAIRN_DECISION_ACCUMULATION` fires on five nodes (cairn.root,
cairn.kernel.artefacts, cairn.kernel.cli, cairn.kernel.scanner, cairn.ui)
and has stood for weeks as folded Info noise. The 2026-08-07 issue audit
(res.chatgpt-issue-audit) names the signal itself as the defect: a
threshold that fires permanently on healthy nodes is miscalibrated, and a
standing finding nobody acts on trains readers to ignore the class.

## Task

1. Establish what action the finding is supposed to trigger (consolidate
   decisions? split the node? raise a review?) and whether that action is
   plausible on the five firing nodes.
2. Recalibrate: threshold, per-node override, decay window, or a
   remediation pointer that names the concrete next step. Extend the
   existing generic config patterns (thresholds, gates) rather than
   inventing a parallel mechanism.
3. Cover the new behaviour with a test.

## Acceptance

- Each of the five current instances either carries an actionable
  remediation or no longer fires.
- A test pins the recalibrated threshold behaviour.
