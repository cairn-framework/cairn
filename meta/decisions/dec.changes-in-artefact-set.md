---
id: dec.changes-in-artefact-set
nodes:
  - cairn.kernel.artefacts
  - cairn.kernel.changes
status: accepted
date: 2026-06-29
informed_by: [res.spec-designed-audit]
related: [dec.revisit-trigger-relevance]
---
# Active changes load into the ArtefactSet as a text-only corpus

## Problem

`dec.revisit-trigger-relevance` made loading `meta/changes/` into the scan substrate
the honest next step (bead cairn-1me): the artefact registry had `load_decisions`,
`load_research`, `load_sources` but no `load_changes`, so active changes were
first-class for the `cairn changes` CLI yet invisible to every scan check. The
question was *where* the loader lives and *what shape* it carries.

## Decision

`load_changes` lives in the artefact registry (`src/artefacts/registry/`, node
`cairn.kernel.artefacts`) alongside the other `load_*` loaders, and populates a new
`ArtefactSet.changes: Vec<ChangeRecord>`. `ChangeRecord` is a text-only view
(`id`, `path`, `title`, `proposal`, `design`): exactly what scan checks and queries
need to correlate change prose against decision `revisit_triggers`.

The registry enumerates `meta/changes/` directly (skip `archive/`, require
`proposal.md`) rather than calling `changes::discover`.

## Why not reuse `changes::discover` / load in the scanner

The `changes` module already depends on `artefacts` (frontmatter) and on `scanner`
(it runs a scan to validate deltas before archive). Because `changes -> artefacts`
and `changes -> scanner -> artefacts` already hold, making the typed registry depend
on the `changes` module (`artefacts -> changes`) would close a cycle, and loading
changes *from* the scanner (`scanner -> changes`) would close another, each a
`CAIRN_ORDER_CYCLE` error. The export builder independently arrived at the same
direct-enumeration technique (`src/cli/export/builder.rs`, "Cycle 3 fix"); note its
fix addressed a path-doubling bug rather than a module cycle and it still calls
`changes::load_change`, so only the enumeration approach is shared, not the
cycle-avoidance motive.

Carrying only a text-only `ChangeRecord` keeps the change-application machinery
(blueprint deltas, artefact operations) in the `changes` module where it belongs;
the registry never pulls it in. The trade-off is a small, stable duplication of the
`meta/changes/` enumeration rule (also present in the export builder); unifying the
three enumerators is a separate cleanup, not required here.

## Consequence

The spec:634 revisit-trigger correlator (bead cairn-9w9) can now consume changes
from `&ArtefactSet` like every other check, with no new cross-module edge.
