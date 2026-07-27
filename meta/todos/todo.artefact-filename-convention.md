---
node: cairn.kernel.scanner
status: done
created: 2026-07-27
---

# Artefact filenames carry a typed prefix nothing checks

## Problem

`docs/conventions.md` section 10 and `AGENTS.md` both state the rule:
filenames are slug-only, and the typed prefix lives only in the `id:`
frontmatter (id `res.gas-city.analysis`, filename `gas-city.analysis.md`).

Measured on 2026-07-27, the repository does not follow its own rule:

| Kind | Total | Carrying a typed filename prefix |
|---|---|---|
| decisions | 80 | 26 |
| research | 24 | 9 |
| sources | 14 | 4 |
| todos | 131 | 131 |

Todos are the interesting case: every one is `todo.<slug>.md`, and that is
what `cairn todo new` scaffolds and what `cairn todo set <slug>` resolves, so
for todos the prefix is the de facto convention, not drift. The rule as
written contradicts the shipped tooling.

Nothing detects any of this. `src/scanner/checks.rs` has no artefact-filename
check, so the split has widened silently since the convention was written.

## Task

1. Settle which rule is actually intended. The evidence points at: todos keep
   `todo.` because the CLI resolves slugs through it; decisions, research, and
   sources are slug-only. That is a decision, not a unilateral edit, because
   it either amends `docs/conventions.md` section 10 or renames 39 files.
2. Add one scan check that enforces whatever is settled, so the answer stops
   depending on whoever last read the convention.
3. Rename or amend, whichever the decision chose, in one pass.

## Acceptance

- A decision records the rule, including why todos differ if they do.
- A finding fires on a non-conforming artefact filename, with a test.
- `cairn scan` reports zero artefact-filename findings on this repository and
  no new findings of any kind. It does not report zero findings overall: the
  one deferred CK004 Info is retained on purpose by
  `dec.revisit-trigger-correlator-deferred`.

## Non-goals

- Do not rename files before the decision lands. Renaming a decision file
  changes nothing about its `id:`, but it does churn every prose reference,
  so it is worth doing once against a settled rule.
