---
node: cairn.kernel.scanner
status: done
created: 2026-07-27
---

# Contract Node Shape Drift Proposal

Author a change proposal that chooses the finding tier and wording for
`todo.contract-blueprint-staleness` before any scanner code is written. That
todo requires it: "wording and tier should get a change proposal before
implementation".

The tier is the load-bearing question. `cairn scan --strict` exits non-zero on
any Warning, so a Warning-tier check turns a green strict run red in every
repository whose node declaration has moved since the baseline was recorded.
Whether that bites on upgrade or only later depends on the migration semantics
this proposal picks, which is why the two questions are settled together.

## Scope

Create the change under `meta/changes/` with `cairn change new`, and settle
exactly these four questions in it:

1. **Tier.** Warning (blocks `--strict`) or Info (advisory only), and what
   makes the chosen tier honest given the single advisory channel argument in
   `dec.revisit-trigger-correlator-deferred`.
2. **Finding code and message wording.** A new code registered in
   `docs/registries/error-codes.md`, with its rule row in
   `docs/registries/spec-rules.md`, and user-facing text in
   `docs/design-system/copy.toml`.
3. **Baseline state schema and migration.** The versioned shape of
   `.cairn/state/contract-baselines.json`, which `NodeFingerprint` fields it
   compares (parent, kind, edges; paths excluded, since path-only edits are
   already ungated in `check_blueprint_change_decisions`), and what a
   repository with contracts but no baseline file does on its first scan after
   upgrade. Two coherent answers exist and they imply different blast radii:
   record-on-accept only (silent until the next `cairn draft accept`, so no
   upgrade-day findings), or backfill current fingerprints at first scan (also
   silent, but then every later blueprint edit flags). Neither can flag on
   upgrade day, so the proposal must not claim it does.
4. **Recording point.** Confirm `src/summariser/accept.rs` is the only place a
   baseline is written or rewritten. Note that its stored
   `accepted_interface_hash` is computed from contract text, not from node
   shape, so the baseline is a second recorded value alongside it, not a
   reinterpretation of it.

Write the acceptance criteria under the change's `specs/`. Do not write scanner
code in this unit.

## Depends on

None.

## Acceptance

`cairn change list` shows the new change as active, its `specs/` state the
tier and the finding code as testable criteria, and `cairn scan` reports no new
findings.

## Outcome

Delivered as `meta/changes/contract-node-shape-drift/`. Its `proposal.md`
records why the mandated `pending` rule row necessarily adds one non-blocking
CK004 Info, which is the one acceptance clause this unit could not satisfy as
literally written. Errors and warnings stay at zero and `cairn scan --strict`
exits 0.

The unit also proved a prerequisite the plan did not have: re-recording a
baseline is unreachable with the summariser disabled, so
`todo.contract-baseline-rerecord-surface` was authored and
`todo.contract-blueprint-staleness` stays `blocked` behind it. That supersedes
this todo's Scope question 4, which asked to confirm `src/summariser/accept.rs`
as the only writer: it is the accept-time writer, and the re-record surface is
the second sanctioned one. The scanner writes the file at no point.

Scope question 2 is likewise narrowed. It asked for the code to be registered in
`docs/registries/error-codes.md` here; `docs/conventions.md` rule 2 binds that
allocation to the commit introducing the code in Rust, so this unit lands the
`pending` rule row with an empty `Code` cell and the enforcer's commit allocates
the number.
