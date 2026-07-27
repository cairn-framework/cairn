---
id: dec.loop-selection-deferred-findings
nodes:
  - cairn.kernel.cli
  - cairn.kernel.map
  - cairn.kernel.query
  - cairn.kernel.scanner
  - cairn.root
status: proposed
date: 2026-07-27
informed_by: [res.loop-selection-deferred-findings]
revisit_triggers:
  - "a deferring decision is superseded and its finding becomes fixable again"
  - "a second finding class acquires a deferral, making the exception load-bearing"
---
# A decision-deferred finding is standing evidence, not a selectable unit

**Proposed, awaiting a maintainer ruling.** It changes the sole normative loop
procedure and its obligation 2 sits against a line in an accepted decision (see
below), so it is not self-ratified. Nothing in the loop changes until it is
accepted.

## Context

Loop mode selects "the first `$CAIRN lint --json` finding; else the top open
todo", stated without exception. One finding can never be fixed:
`CAIRN_SPEC_RULE_UNIMPLEMENTED` for `spec:634`, which
`dec.revisit-trigger-correlator-deferred` keeps standing on purpose as "the
honest living tracker of a Designed-but-unbuilt rule", having foreclosed every
deterministic matcher it investigated as net-negative.

So the procedure points every iteration at a unit the graph's own authority
forbids completing. A session that reads selection literally stalls in Scope or
builds the foreclosed correlator; a session that reasons its way out spends an
orientation pass re-deriving an exception that is not written down, differently
each time. `res.loop-selection-deferred-findings` establishes both the failure
and the machinery that nearly knows better: the scanner sets `deferred_by` at
the emission site, `cairn remediate` answers "no remediation actions are
required", and `cairn scan` already collapses deferred findings to a footnote.

## Decision

A finding that an **accepted** decision defers is not a selectable loop unit.
Default selection skips it and continues to the next finding, then to todos. A
MISSION naming it reports why and ends the iteration, the same as any other
already-settled unit. Both paths are covered, because MISSION precedence runs
before default selection and would otherwise route around the rule.

Two obligations follow, so the rule does not become a silent excuse:

1. The deferral must be machine-visible and accepted, not inferred. Today's
   `deferred_by` is neither published on the wire nor checked for accepted
   status: `validate_deferred_decision_targets` rejects only a missing or
   `Superseded` target, so a `Proposed` decision would qualify. Both gaps close
   before selection may rely on the field. A finding with no such deferral
   remains selectable regardless of severity.
2. The deferred work must be represented by a todo. A finding is evidence that a
   rule is unbuilt; it is not a backlog item, and dropping it from selection
   must not drop the work from the plan. `todo.revisit-trigger-correlator`
   discharges this for `spec:634`, filed `blocked`.

   This is the disputed point. It sits against the closing line of
   `dec.revisit-trigger-correlator-deferred`, "re-file an implementation bead
   when such a capability lands". A blocked record is not an implementation
   item, so nothing becomes buildable or selectable and that foreclosure is
   untouched; a maintainer reading the two as incompatible should strike this
   obligation rather than the foreclosure.

## Rationale

Suppressing the finding instead was rejected: it contradicts
`dec.revisit-trigger-correlator-deferred`, which wants the Info standing.
Deleting a signal to satisfy a selector inverts the dependency between the graph
and the procedure that reads it. Leaving the rule unwritten was rejected because
the cost is paid every iteration and the outcome varies by session, which is the
property loop mode exists to remove.

The rule is deliberately narrow. It keys on a structured field naming an
accepted decision, not on severity and not on a reader's judgement that a
finding "looks deferred". Its remaining weakness is recorded rather than
hidden: the validator compares the registry cell against decision ids and
statuses only, so it cannot yet prove that the named decision is about that
rule, nor that a todo covers the deferred work. A row could therefore name an
unrelated accepted decision. Editing `docs/registries/spec-rules.md` is a
reviewed act, which is the control today; a machine-checkable rule-to-decision
relation is the hardening if that control proves insufficient.

## Consequences

- Implementation is tracked by `todo.loop-selection-deferred-findings`.
- Until that lands, `loop-mode.md` still reads "the first `$CAIRN lint --json`
  finding" with no exception. This decision grants no standing exception in the
  meantime: it places an obligation on the asset, whose canonical copy lives
  under `./tools/agent-pack` and is therefore owned by `cairn.kernel.cli`, a
  node this decision names. A session that meets the disagreement before the
  edit lands should recognise it as the known gap that todo closes and report
  it, not re-derive a private exception. Building the correlator remains
  foreclosed by `dec.revisit-trigger-correlator-deferred` either way.
- `todo.loop-selection-deferred-findings` is `blocked` while this decision is
  `proposed`, so nothing selects it. Once accepted and reopened, the standing
  finding still sorts ahead of every todo, so the follow-up iteration must be
  invoked with a MISSION naming that todo. This decision creates no mechanism
  that routes there by itself.
- `dec.revisit-trigger-correlator-deferred` is not superseded, and superseding
  it would invalidate the `Deferred-by` cell in
  `docs/registries/spec-rules.md` that names it. `todo.revisit-trigger-correlator`
  is filed `blocked` under obligation 2 and reopens when a capability that
  judges relevance rather than proximity lands.
