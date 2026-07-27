---
node: cairn.kernel.cli
status: blocked
created: 2026-07-27
---

# Loop Selection Skips Decision-Deferred Findings

## Problem

`loop-mode.md` selects "the first `$CAIRN lint --json` finding; else the top
open todo", with no exception, and MISSION precedence item 2 selects a named
finding code exactly. One finding can never be fixed:
`CAIRN_SPEC_RULE_UNIMPLEMENTED` for `spec:634`, which
`dec.revisit-trigger-correlator-deferred` (accepted) deliberately keeps standing
as evidence of a Designed-but-unbuilt rule. Selection and the graph's own
authority disagree permanently, so every iteration either re-derives the
exception by hand or follows the letter and stalls.

Recommended by `dec.loop-selection-deferred-findings`, which is `proposed`;
evidence and rejected options in `res.loop-selection-deferred-findings`. This
todo is `blocked` until that decision is accepted. Unblock with
`cairn todo set loop-selection-deferred-findings open` once it is, implement
under it, and do not author a competing decision.

While this todo is `blocked`, selection skips it and a MISSION naming it reports
and ends. Once it is accepted and reopened, reaching it still needs a MISSION
that names it: until the loop-mode edit below lands, default selection takes the
standing finding ahead of every todo.

## Scope

Make the published deferral trustworthy:

- Publish `deferred_by` only for a deferral naming an `Accepted` decision.
  Tightening `validate_deferred_decision_targets` alone is not enough: it only
  adds `CAIRN_SPEC_RULE_DEFERRED_DECISION_INVALID`, while
  `validate_spec_rule_coverage` (`src/map/spec_rule_coverage.rs`) copies
  `rule.deferred_by` for any `Pending` rule without consulting decisions, and
  nothing clears it afterwards. Set the field from an accepted-status lookup, or
  clear it when validation fails.
- That widens the rejected set, so the finding's copy must follow it:
  `CAIRN_SPEC_RULE_DEFERRED_DECISION_INVALID` in `docs/design-system/copy.toml`
  and CK032 in `docs/registries/error-codes.md` both still say "missing or
  superseded" and ask for a "live" decision, which a `Proposed` one satisfies.

Put the field on the surface the selector actually reads. It is not one
projection, and `serde(skip)` is not the only gate:

- `lint --json` and `scan --json` are hand-built by
  `query_api::serialise::findings_json` (`code`, `severity`, `message`, `node`,
  `path`) under the `query_api::SCHEMA_VERSION` envelope. That is the surface
  loop selection reads; the field must be added there explicitly.
- `render_findings` (`src/cli/format/render.rs`) carries a second JSON branch
  emitting `code`, `severity`, `message`. Update it or confirm it is unreachable.
- Dropping the `serde(skip)` in `src/map/graph.rs` also changes every
  serde-derived consumer of `Finding`: `map.json` via `src/scanner/snapshot.rs`,
  and `WatchEvent` via `src/watch.rs` (node `cairn.watch`). Bump each owning
  schema version whose shape changes and rebase the committed snapshots.

Amend the procedure:

- Amend `.claude/skills/cairn-dev/references/loop-mode.md` in both places
  selection can reach a finding: the `## Select ONE unit` paragraph and MISSION
  precedence item 2. A finding carrying a validated `deferred_by` is not
  selectable, because an accepted decision has already ruled it must keep
  standing; skip to the next finding, then to todos.
- Resolve a MISSION per finding instance, not per code. One code can cover a
  deferred instance and a live one at once: `validate_spec_rule_coverage` emits
  one `CAIRN_SPEC_RULE_UNIMPLEMENTED` per registry row, so an `enforced` row
  missing its enforcer yields a selectable Warning under the same code as the
  deferred `spec:634` Info. A MISSION naming the code takes the first sorted
  non-deferred instance, and reports settled only when every instance carrying
  that code has a validated deferral.
- Mirror the identical edit into
  `tools/agent-pack/content/skills/cairn-dev/references/loop-mode.md`. The two
  copies must stay byte-identical; `determinism_drift_tests.rs` pins the pair.
- Pin both obligations with phrase assertions over the loop-mode body, in the
  style of the loop-mode assertions in
  `tools/agent-pack/tests/reconcile_step_tests.rs`.

## Acceptance

- A deferral naming a `Proposed`, `Deprecated`, missing, or `Superseded`
  decision raises `CAIRN_SPEC_RULE_DEFERRED_DECISION_INVALID` and publishes no
  `deferred_by`; only an `Accepted` one publishes. One test per status asserts
  both halves.
- `cairn lint --json` reports `deferred_by` for the `spec:634` finding.
- A mixed case (one deferred and one live finding sharing a code) selects the
  live one rather than reporting the code settled.
- Loop mode's default selection and its MISSION path both name the deferral
  exception, and the new assertions fail if either sentence is removed.
- Both loop-mode copies stay byte-identical and the pack tests pass.

## Depends on

`dec.loop-selection-deferred-findings` being accepted. Nothing else:
`todo.revisit-trigger-correlator` records the deferred work this rule learns to
skip, and does not gate this change.
