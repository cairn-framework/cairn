---
node: cairn.kernel.cli
status: open
created: 2026-07-12
---

# Output Token Efficiency

## Problem
Agent loops pay for cairn's output on every invocation, and two surfaces
spend tokens without informing the loop (res.loop-efficiency-observations,
2026-07-12 entry):

- Decision-deferred findings print their full text identically on every
  `scan`, `lint`, and `hook` run. Such a finding is not actionable while
  both its deferral decision and the relevant implementation state are
  unchanged, so it should collapse to
  one summary line (count plus deciding artefact id) with full text behind
  a flag - presentation deduplication, not suppression: it must resurface
  when the decision or implementation state changes.
- `cairn status` is O(backlog): full open-todo list plus repeated trailing
  log entries, while the loop consumes only the recommendation, finding
  count, and active changes. Wants a brief default or `--brief` mode, a
  deduplicated log tail, and a capped todo list with a count.

Secondary (weaker evidence, one occurrence): when the recommended unit is
blocked on an unmet dependency, status offers no fallback; consider top-3
recommendations with a one-word blocked reason each.

## Acceptance
- Deferred findings render as a single summary line in scan/lint/hook
  output while their deferral decision and the relevant implementation
  state are unchanged; full text reachable via a flag. When the deferral
  decision is removed or superseded, or the deferred rule gains an
  enforcer, the finding renders in full again - covered by a behavioural
  test for both directions. Existing finding assertions in tests updated,
  no wire-format change to `--json` output.
- `cairn status` has a token-lean mode (or lean default) whose output does
  not grow with backlog size (open todos appear as a count), asserted by a
  test on a representative backlog-scaled fixture chosen during
  implementation. The full listing stays reachable.
- Behaviour covered by tests; user-facing strings resolved via
  `docs/design-system/copy.toml` per the loop convention.

Informed by: res.loop-efficiency-observations
