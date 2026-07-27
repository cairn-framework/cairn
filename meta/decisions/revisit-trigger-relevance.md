---
id: dec.revisit-trigger-relevance
nodes:
  - cairn.root
status: superseded
date: 2026-06-29
informed_by: [res.spec-designed-audit]
superseded_by: [dec.revisit-trigger-correlator-deferred]
---
# spec:634 revisit-trigger relevance: build the correlator, sequenced behind cairn-1me (not deferred)

## Problem

`spec.md:634` lists `revisit_triggers appear relevant based on recent changes` as a
rationale tension. The field is parsed, stored, and rendered, but nothing evaluates
relevance. Bead cairn-9w9 was left in a bare `Deferred: <date>` state. A calendar
deferral encodes neither a dependency nor a decision: it just re-asks the same question
on a date. This decision records the real resolution.

## What the debate settled (do not re-litigate)

A reformer/conservative debate (research `res.spec-designed-audit`, gap 2) established,
against live data:

1. **Node-id / graph-delta matching has zero recall.** 0 of 7 `revisit_triggers` name a
   node id. Killed.
2. **Git-log correlation is rejected.** It breaks scan determinism and couples scan to
   git history, violating `dec.no-orchestrator` scan purity. Killed.
3. **The changes-corpus correlator is the only spec-faithful, deterministic option, and
   it fires a real true positive today.** Correlating committed `meta/changes/` markdown
   against each decision's `revisit_triggers` matches
   `dec.webui-design-quality-direction`'s trigger in three committed files. It reads
   `meta/changes/` the way scan already reads `meta/decisions/`.

## Why build, not defer

The earlier defer leaned on "value below the bar at current scale." That is a
**priority** argument, and priority only gates work under **capacity contention**: you
order by value when you cannot do everything. The backlog is a handful of ready beads and
the dev loop runs continuously, so there is no contention. Value can decide *which task
runs first*, never *whether cairn-9w9 runs at all*. With a thin backlog you do all of
them.

The only thing that justifies not building with idle capacity is genuine **net-negative
value** (building it makes things worse). The correlator is not net-negative:

- it fires a *verified* true positive;
- it is bounded by the active-change count (~2), so it cannot noise-spam;
- it surfaces at **Info** (advisory, trivial cost-of-undo);
- the change-loading infrastructure mostly exists already (`src/changes/`).

Weak-but-real positive, cheap, reversible, idle capacity: **build it.**

## The real dependency (replaces the calendar date)

The correlator needs committed `meta/changes/` available in the scan substrate. Today
`src/artefacts/registry/mod.rs` has `load_decisions` (reads `meta/decisions/` into the
`ArtefactSet`) but **no `load_changes`**: `meta/changes/` is first-class for the
`cairn changes` CLI yet is never loaded into the scan `ArtefactSet`/reconcile graph. That
gap is bead **cairn-1me**.

So the honest order is a dependency edge, not a date:

1. **cairn-1me** load `meta/changes/` into the scan `ArtefactSet` (makes changes queryable
   for every check, not just the correlator).
2. **cairn-9w9** add the changes-corpus correlator that emits the spec:634 relevance
   advisory, consuming changes from the substrate like every other check consumes the set.

## Decision

- Build the changes-corpus correlator (alternatives 1 and 2 are foreclosed).
- Sequence it behind cairn-1me; wire cairn-1me as a blocker of cairn-9w9.
- spec:634 stays `pending` (CK004 Info as the living tracker) until the correlator ships,
  then promote the `docs/registries/spec-rules.md` row to `enforced` and the finding
  clears.
- No calendar deferral; the dependency on cairn-1me is the only gate.
