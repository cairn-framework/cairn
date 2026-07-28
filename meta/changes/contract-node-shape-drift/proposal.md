# Proposal: Contract node-shape drift

## Motivation

Three staleness directions exist between a node, its code, and its contract
prose. Two have coverage today:

- code-vs-contract, partially, by the opt-in `interface:` block check
  (`CAIRN_CONTRACT_INTERFACE_DRIFT`, `src/scanner/checks.rs`). A contract with no
  `interface:` block is never compared. The adjacent
  `CAIRN_INTERFACE_HASH_CHANGED` (`src/hooks/mod.rs`) is not this direction: it
  compares current target hashes against `.cairn/state/interface-hashes.json`
  and never reads a contract;
- blueprint-vs-decision, by `CAIRN_BLUEPRINT_CHANGE_NO_DECISION`, backed by
  `BlueprintSnapshot` and `NodeFingerprint` (`src/scanner/state.rs`,
  `src/scanner/checks.rs`).

The third is uncovered: a contract authored against one node shape stays
silently marked current after the node's declaration changes, with nothing
recording that it was never re-read. A contract that says "this module depends
only on the parser" keeps saying it after the blueprint adds three outbound
edges, and nothing in the graph notices.

`meta/todos/todo.contract-blueprint-staleness.md` (node `cairn.kernel.scanner`)
proposes closing that gap with the existing fingerprint machinery: record the
node's shape as a baseline when a contract is accepted, then compare against it
on scan. The baseline is a reduced record derived from `NodeFingerprint`, not
that struct itself; `design.md` question 3 says why. That todo requires this
proposal before any scanner code is written, because the tier interacts with
`cairn scan --strict`, which exits non-zero on any Warning.

This change settles the four open questions and writes the acceptance criteria.
It writes no scanner code.

## Scope

Settle and record, as testable acceptance criteria under `specs/`:

1. The finding tier, and why it is honest given the single-advisory-channel
   argument in `dec.revisit-trigger-correlator-deferred`.
2. The finding code name, where its registry number is allocated, and its
   user-facing wording.
3. The `.cairn/state/contract-baselines.json` schema, which node-shape fields it
   compares, and what a repository with contracts but no baseline file does on
   its first scan after upgrade.
4. The recording point, confirmed against the code.

Land the `pending` rule row so the rule is visible in the registry rather than
held in this document alone. The code's registry number is not allocated here:
`docs/conventions.md` rule 2 binds allocation to the commit that introduces the
code in Rust, and that is the enforcer's.

## Out of scope

- Any scanner, state, or CLI code. The enforcer is a separate unit, and this
  change plus its prerequisite are what unblock it.
- Automatically backfilling baselines the scanner was never told to record. The
  rejection is argued in `design.md`; a user explicitly recording one is not
  backfill.
- Building the non-generative re-record surface this change's evidence proved
  necessary. It is authored here as
  `meta/todos/todo.contract-baseline-rerecord-surface.md` and built as its own
  unit.
- Any change to `CAIRN_INTERFACE_HASH_CHANGED` or
  `CAIRN_BLUEPRINT_CHANGE_NO_DECISION`. This rule sits beside them, not over
  them.

## Acceptance criteria

- `cairn change list` shows `contract-node-shape-drift` as active.
- `specs/contract-node-shape-drift.md` states the tier and the finding code name
  as criteria an implementer can test.
- `cairn scan` reports no new Error or Warning findings.

### Three new Info findings: an acknowledged conflict in the originating todo

`todo.contract-node-shape-drift-proposal` asks for two things that cannot both
hold in the same commit. Its Scope requires the rule row in
`docs/registries/spec-rules.md`; its Acceptance requires that `cairn scan`
report no new findings; and its Scope also forbids scanner code in this unit. By
the registry's own documented semantics, a `pending` row whose enforcer is not
yet emitted surfaces `CAIRN_SPEC_RULE_UNIMPLEMENTED` (CK004) at Info, whether or
not the row names a code. So adding the mandated row necessarily adds a finding,
and the only way to satisfy the literal Acceptance is to omit a mandated artefact
or to write the enforcer the same Scope forbids.

Resolution taken here: add the row, accept the Info, and record the conflict
rather than reinterpret the criterion. That Info is the mechanism working as
designed. It is the tracked, non-blocking record of a Designed-but-unbuilt rule,
and it clears when the enforcer lands. The alternative, holding the row back
until implementation, would leave the rule invisible to anyone reading the
registry, which is the failure the registry exists to prevent.

Two further Infos come from the provenance chain, not from the rule. The
conventions require the deferral decision to chain to evidence, and the two
source artefacts recording that evidence are registered `unverified` on purpose:
hash-pinning live source files would make an ordinary edit to
`src/summariser/store.rs` a structural error. `CAIRN_SOURCE_UNVERIFIED` at Info
is the accurate tracker for a citation that is not hash-pinned.

Measured against `origin/main` at the time of writing, `cairn scan` goes from one
finding to four: two `CAIRN_SOURCE_UNVERIFIED` Infos, plus two deferred
summaries, one pre-existing and one for this rule, which
`dec.contract-node-shape-drift-deferred` collapses. Errors and warnings are zero
in both states and `cairn scan --strict` exits 0.
