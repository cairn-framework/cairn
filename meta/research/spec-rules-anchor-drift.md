---
id: res.spec-rules-anchor-drift
nodes:
  - cairn.kernel.map
date: 2026-07-30
method: primary
---

# Spec anchors in the spec-rule registry are systematically stale

Measured on 2026-07-30 at `022faef` on `main`, while
`todo.source-tracked-verification-mode` re-checked the
`docs/registries/spec-rules.md` rows its one-line `docs/spec.md` insertion
would move (the todo predicted "several source rows" were mis-anchored; the
audit found the drift is systematic).

## What was measured

Every `Spec` anchor in the registry was compared against the `docs/spec.md`
text at that line before the edit:

- `spec:620` to `spec:628` (duplicate IDs, path ties, artefact pointers,
  unknown nodes, sha256 mismatch, orphan files, interface hash) point into
  section 9.6 "Rename": prose about `cairn rename`, blank lines, and a code
  fence. None states its row's rule.
- `spec:631` to `spec:636` (decision references, research orphans, decision
  claims, blueprint-change decisions, revisit triggers, edge divergence,
  docstring drift) point at rename-propagation prose and the section 10
  scanner heading.
- `spec:865` to `spec:867` (gap decisions, workspace members, supersedes
  status) point at the "Name: Cairn is a working placeholder" open question
  and v0.5.1 resolution notes.
- The source rows at `spec:474`/`spec:476` point into the section 8.5
  research template, not the section 8.6 verification rules (the todo already
  knew these).
- The remaining anchors mostly fail the same "line states the rule" bar,
  verified individually: `spec:61` is a blank line (its two provenance rows'
  rules live elsewhere), `spec:318` states path-uniqueness rather than its
  rows' leaf-contract and test-coverage rules, `spec:327` is the section 8
  heading, and `spec:341` is todo-template frontmatter. Only `spec:24` (the
  Designed-maturity mandate the registry header itself cites) and `spec:515`
  (the tracked-source integrity rule, written 2026-07-30) anchor text that
  states their rules.

The finding-emission half of every row is correct (the coverage check gates
on Code emission, and `cargo test --test finding_code_coverage` passes); only
the `Spec` anchors have drifted, most plausibly from one or more spec
insertions that predate the audit trail.

## What it constrains

The registry header says the Spec cell "carries an anchor only where the rule
originated" in the spec; a reader following those anchors today lands on
unrelated prose for all but two rows. Any unit editing `docs/spec.md` cannot
honour "re-check the rows the edit moved" meaningfully while the baseline is
already wrong. Remediation is tracked by `todo.spec-rules-reanchor`
(re-derive each anchor from the rule text, or set `-` where the rule no
longer originates in the spec, per `dec.spec-authority-retirement`).
