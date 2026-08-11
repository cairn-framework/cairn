# Cairn Spec-Rule Registry

This file tracks every Designed integrity, freshness, and rationale-tension rule
against the code that enforces it. It is the home for new rules as well as for
the ones the spec already states (`dec.spec-authority-retirement`): the Rule
cell is authoritative, and the Spec cell carries an anchor only where the rule
originated in `docs/spec.md`. It is the machine-readable
half of the ghost-rule mechanism: `cairn scan` reads this table and emits
`CAIRN_SPEC_RULE_UNIMPLEMENTED` (registry code CK004) when a rule's enforcer is
missing. This turns a Designed-but-unimplemented rule from prose that silently
passes scan into tracked cairn state, the way the spec mandates (spec.md:24). See
`meta/decisions/ghost-rule-tracking.md` for the rationale.

## Format

| Column | Meaning |
|--------|---------|
| Rule | One-line description of the spec rule. |
| Spec | `spec:<line>` anchor into `docs/spec.md` where the rule originated there; `-` for a rule this registry owns outright. |
| Code | The `CAIRN_*` finding the rule's enforcer emits, in backticks. Empty (`-`) means no enforcer is named yet. |
| Status | `enforced`, `pending`, or `declared` (see below). |
| Deferred-by | Optional fifth cell on `pending` rows: the decision artefact deferring the rule's build (e.g. `dec.<slug>`). Empty (`-`) or absent means no deferral is recorded. The finding message names it inline and `lint --json` publishes it as `deferred_by`, but only while the named decision is **accepted**: any other target (missing, proposed, deprecated, or superseded) raises `CAIRN_SPEC_RULE_DEFERRED_DECISION_INVALID` and the finding stays live with no published deferral (`dec.loop-selection-deferred-findings`). |

## Status semantics

- `enforced`: the rule is built; its `Code` must be emitted in non-test `src/`.
  A missing emitter is a **regression** and surfaces `CAIRN_SPEC_RULE_UNIMPLEMENTED`
  at **Warning** severity (fails `cairn scan --strict`).
- `pending`: the rule is Designed but not yet built. While its enforcer is absent
  it surfaces `CAIRN_SPEC_RULE_UNIMPLEMENTED` at **Info** severity: a visible,
  tracked advisory that does not block `--strict`. When implemented, add the
  `Code` and promote the row to `enforced`; the finding then clears. `pending`
  does not promise an imminent build: a Designed rule may be **deliberately
  deferred** (its build parked behind a prerequisite capability) with the
  rationale recorded in a decision artefact. The Info finding stays the honest
  tracker either way, so deferral needs no separate status. Example:
  `spec:675` is deferred by `dec.revisit-trigger-correlator-deferred` pending a
  relevance-judging capability, yet remains `pending` because the rule is Designed.
- `declared`: named in the spec at Declared maturity (see
  `docs/registries/declared-items.md`), not yet designed enough to enforce.
  Exempt: listed for completeness, never flagged. This is the principled line
  between `spec:675` (`pending`: the tension is Designed) and `spec:676`/`spec:677`
  (`declared`: their edge-divergence / docstring-drift checks depend on the
  semantic-analysis strategy that spec section 17 deliberately leaves uncommitted,
  so they are not yet designed enough to enforce).

Detection is emission-anchored: a code counts as emitted only where the `"CODE"`
literal is immediately preceded by `error(`, `warning(`, `info(`, or `code:` in
non-test source. A bare reference (match arm, remediation handler, doc comment)
does not count.

## Enforced rules

| Rule | Spec | Code | Status |
|------|------|------|--------|
| Duplicate node IDs | spec:661 | `CAIRN_INTEGRITY_DUPLICATE_ID` | enforced |
| Path ties between leaf nodes | spec:662 | `CAIRN_INTEGRITY_PATH_TIE` | enforced |
| Broken artefact pointer | spec:663 | `CAIRN_ARTEFACT_POINTER_MISSING` | enforced |
| Artefact references non-existent node | spec:664 | `CAIRN_ARTEFACT_UNKNOWN_NODE` | enforced |
| Orphan file under claimed container | spec:666 | `CAIRN_RECONCILE_ORPHANED_FILE` | enforced |
| Verified source missing checksum | spec:510 | `CAIRN_SOURCE_SHA256_MISSING` | enforced |
| Verified source checksum mismatch | spec:665 | `CAIRN_SOURCE_SHA256_MISMATCH` | enforced |
| Module interface hash drift | spec:669 | `CAIRN_INTERFACE_HASH_CHANGED` | enforced |
| Research must cite at least one source unless primary | - | `CAIRN_RESEARCH_MISSING_SOURCES` | enforced |
| Decision must cite at least one research or source | spec:70 | `CAIRN_DECISION_UNKNOWN_PROVENANCE` | enforced |
| Leaf node should declare a contract | spec:356 | `CAIRN_CONTRACT_LEAF_UNCOVERED` | enforced |
| Contract interface entry should match an extracted symbol | spec:360 | `CAIRN_CONTRACT_INTERFACE_DRIFT` | enforced |
| Proposed gap decision is an unresolved question | spec:772 | `CAIRN_GAP_UNRESOLVED` | enforced |
| Workspace member root or blueprint fails to load | - | `CAIRN_WORKSPACE_MEMBER_MISSING` | enforced |
| Synced module should carry test coverage | - | `CAIRN_TEST_COVERAGE_MISSING` | enforced |
| Todo references exactly one valid node | spec:379 | `CAIRN_TODO_ORPHAN_NODE` | enforced |
| Source referenced by at least one research or decision | spec:515 | `CAIRN_SOURCE_ORPHAN` | enforced |
| External source file must be a URL | spec:511 | `CAIRN_SOURCE_EXTERNAL_URL` | enforced |
| Unverified source persists as a tension | spec:512 | `CAIRN_SOURCE_UNVERIFIED` | enforced |
| Decision supersedes target must be superseded | spec:418 | `CAIRN_DECISION_SUPERSEDES_STATUS` | enforced |
| Decision cites deleted research or source | spec:672 | `CAIRN_DECISION_REFERENCE_UNKNOWN` | enforced |
| Research not linked from any decision | spec:673 | `CAIRN_RESEARCH_ORPHAN` | enforced |
| Decision claims to close a still-open spec question | - | `CAIRN_DECISION_CLAIM_UNRESOLVED` | enforced |
| Blueprint shape change lacks a covering decision | - | `CAIRN_BLUEPRINT_CHANGE_NO_DECISION` | enforced |
| Spec rule has no emitting enforcer | spec:24 | `CAIRN_SPEC_RULE_UNIMPLEMENTED` | enforced |
| Contract has not been reviewed against its node's current declared shape | - | `CAIRN_CONTRACT_NODE_SHAPE_DRIFT` | enforced |
| Contract asserted `NAME = N` numeral must match the node's source constant | - | `CAIRN_CONTRACT_NUMERAL_DRIFT` | enforced |
| Tracked source path must resolve | spec:515 | `CAIRN_SOURCE_READ_FAILED` | enforced |
| Tracked source canonical path must stay under the repository root | spec:515 | `CAIRN_SOURCE_READ_FAILED` | enforced |
| Tracked source must not declare a sha256 | spec:515 | `CAIRN_SOURCE_SHA256_UNEXPECTED` | enforced |
| Local decision nodes stay within one container | - | `CAIRN_DECISION_TIER_SPAN` | enforced |
| Local decision does not supersede another decision | - | `CAIRN_DECISION_TIER_SUPERSEDES` | enforced |
| Local decision affects no binding-surface path | - | `CAIRN_DECISION_TIER_BINDING_PATH` | enforced |
| Accepted local decision receipts converge | - | `CAIRN_DECISION_CONVERGENCE_UNMET` | enforced |
| Commit accepting a local decision changes only its affected paths | - | `CAIRN_HOOK_AFFECTS_SUBSET` | enforced |
| Receipt subject hash equals the recomputed decision manifest | - | `CAIRN_HOOK_MANIFEST_MISMATCH` | enforced |

## Pending rules

Designed in the spec but not yet enforced. Each surfaces an Info-level finding
until built.

| Rule | Spec | Code | Status | Deferred-by |
|------|------|------|--------|-------------|
| ADR revisit_triggers appear relevant to recent changes | spec:675 | - | pending | dec.revisit-trigger-correlator-deferred |

## Declared rules

Named in the spec at Declared maturity; not yet designed enough to enforce.
Listed for completeness; exempt.

| Rule | Spec | Code | Status |
|------|------|------|--------|
| Edge divergence (declared edge vs observed import) | spec:676 | - | declared |
| Docstring drift (authored docstring vs map) | spec:677 | - | declared |
