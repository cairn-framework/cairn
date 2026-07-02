# Cairn Spec-Rule Registry

This file tracks every Designed integrity, freshness, and rationale-tension rule
from `docs/spec.md` against the code that enforces it. It is the machine-readable
half of the ghost-rule mechanism: `cairn scan` reads this table and emits
`CAIRN_SPEC_RULE_UNIMPLEMENTED` (registry code CK004) when a rule's enforcer is
missing. This turns a Designed-but-unimplemented rule from prose that silently
passes scan into tracked cairn state, the way the spec mandates (spec.md:24). See
`meta/decisions/dec.ghost-rule-tracking.md` for the rationale.

## Format

| Column | Meaning |
|--------|---------|
| Rule | One-line description of the spec rule. |
| Spec | `spec:<line>` anchor into `docs/spec.md`. |
| Code | The `CAIRN_*` finding the rule's enforcer emits, in backticks. Empty (`-`) means no enforcer is named yet. |
| Status | `enforced`, `pending`, or `declared` (see below). |

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
  `spec:634` is deferred by `dec.revisit-trigger-correlator-deferred` pending a
  relevance-judging capability, yet remains `pending` because the rule is Designed.
- `declared`: named in the spec at Declared maturity (see
  `docs/registries/declared-items.md`), not yet designed enough to enforce.
  Exempt: listed for completeness, never flagged. This is the principled line
  between `spec:634` (`pending`: the tension is Designed) and `spec:635`/`spec:636`
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
| Duplicate node IDs | spec:620 | `CAIRN_INTEGRITY_DUPLICATE_ID` | enforced |
| Path ties between leaf nodes | spec:621 | `CAIRN_INTEGRITY_PATH_TIE` | enforced |
| Broken artefact pointer | spec:622 | `CAIRN_ARTEFACT_POINTER_MISSING` | enforced |
| Artefact references non-existent node | spec:623 | `CAIRN_ARTEFACT_UNKNOWN_NODE` | enforced |
| Orphan file under claimed container | spec:625 | `CAIRN_RECONCILE_ORPHANED_FILE` | enforced |
| Verified source missing checksum | spec:476 | `CAIRN_SOURCE_SHA256_MISSING` | enforced |
| Verified source checksum mismatch | spec:624 | `CAIRN_SOURCE_SHA256_MISMATCH` | enforced |
| Module interface hash drift | spec:628 | `CAIRN_INTERFACE_HASH_CHANGED` | enforced |
| Research must cite at least one source unless primary | spec:61 | `CAIRN_RESEARCH_MISSING_SOURCES` | enforced |
| Decision must cite at least one research or source | spec:61 | `CAIRN_DECISION_UNKNOWN_PROVENANCE` | enforced |
| Leaf node should declare a contract | spec:318 | `CAIRN_CONTRACT_LEAF_UNCOVERED` | enforced |
| Contract interface entry should match an extracted symbol | spec:327 | `CAIRN_CONTRACT_INTERFACE_DRIFT` | enforced |
| Synced module should carry test coverage | spec:318 | `CAIRN_TEST_COVERAGE_MISSING` | enforced |
| Todo references exactly one valid node | spec:341 | `CAIRN_TODO_ORPHAN_NODE` | enforced |
| Source referenced by at least one research or decision | spec:474 | `CAIRN_SOURCE_ORPHAN` | enforced |
| External source file must be a URL | spec:474 | `CAIRN_SOURCE_EXTERNAL_URL` | enforced |
| Unverified source persists as a tension | spec:474 | `CAIRN_SOURCE_UNVERIFIED` | enforced |
| Decision supersedes target must be superseded | spec:867 | `CAIRN_DECISION_SUPERSEDES_STATUS` | enforced |
| Decision cites deleted research or source | spec:631 | `CAIRN_DECISION_REFERENCE_UNKNOWN` | enforced |
| Research not linked from any decision | spec:632 | `CAIRN_RESEARCH_ORPHAN` | enforced |
| Decision claims to close a still-open spec question | spec:633 | `CAIRN_DECISION_CLAIM_UNRESOLVED` | enforced |
| Blueprint shape change lacks a covering decision | spec:633 | `CAIRN_BLUEPRINT_CHANGE_NO_DECISION` | enforced |
| Spec rule has no emitting enforcer | spec:24 | `CAIRN_SPEC_RULE_UNIMPLEMENTED` | enforced |

## Pending rules

Designed in the spec but not yet enforced. Each surfaces an Info-level finding
until built.

| Rule | Spec | Code | Status |
|------|------|------|--------|
| ADR revisit_triggers appear relevant to recent changes | spec:634 | - | pending |

## Declared rules

Named in the spec at Declared maturity; not yet designed enough to enforce.
Listed for completeness; exempt.

| Rule | Spec | Code | Status |
|------|------|------|--------|
| Edge divergence (declared edge vs observed import) | spec:635 | - | declared |
| Docstring drift (authored docstring vs map) | spec:636 | - | declared |
