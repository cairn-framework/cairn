---
id: res.loop-efficiency-observations
nodes:
  - cairn.kernel.cli
date: 2026-07-12
method: primary
---

# Loop efficiency observations (living log)

## Question

Where does cairn's output spend tokens without informing the loop, and what
context is an agent missing at the moment it starts a unit? This is a living
observation log: append a dated entry when a session produces new evidence,
rather than minting a fresh research artefact per anecdote. Hypotheses
graduate to implementation todos (or decisions) once entries corroborate
them; a discovery todo that merely tests a hypothesis may be filed earlier
(as todo.agent-context-bundle does for the bundle hypothesis below).

## Method

Session-level observation of the cairn dev loop running on this repo. Each
entry records what was observed in one session, labelled observed (seen in
transcript output) or hypothesis (single occurrence, plausible mechanism,
unconfirmed).

## Entries

### 2026-07-12 (loop session, PRs #274-#278)

Observed:
- The decision-deferred `CAIRN_SPEC_RULE_UNIMPLEMENTED` info finding printed
  in full, identically, on every `cairn scan`, `cairn lint`, `cairn hook all`,
  and pre-push gate invocation across the session. Unchanged and not
  actionable during this session (its deferral decision exists); the
  observation supports presentation deduplication, not suppression - a
  deferred finding becomes relevant again when its decision or the
  implementation state changes.
- `cairn status` output is O(backlog): it lists every open todo plus several
  identical trailing log entries. The loop consumed only the "Next
  recommended" line, the finding count, and active changes.
- "Next recommended" matched the unit actually executed at all four
  iteration boundaries; bd was never invoked. Native-todo adoption was
  strong this session (first clean single-tracker session; one data point,
  not proof the fallback pattern is gone).
- Closing one unit required coordinated manual edits across three todo
  files (status flip, run record, supersession pointer), and a reviewer
  caught a genuine lifecycle contradiction introduced during those edits.
- A post-merge `cairn scan` dirtied `map.json`, requiring a separate chore
  PR (#277) to land the snapshot refresh.

Hypotheses (single occurrence each):
- When the recommended unit is blocked on an unmet dependency, `cairn
  status` offers no fallback candidate; a top-3 with one-word blocked
  reasons would remove a manual triage step.
- Snapshot refresh could ride the triggering PR instead of a chore PR, but
  a pre-push hook that mutates the feature branch produces diffs the author
  did not stage; a scripted one-command follow-up may capture most of the
  value without hook-mutation semantics. Needs a decision if pursued.
- Temporal context: an agent implementing a todo cannot cheaply see how
  often the target node has changed before (a per-node revision count or
  version marker derived from git history plus the archive trail). Value
  unproven; parked here until the context-bundle work surfaces a concrete
  need.
