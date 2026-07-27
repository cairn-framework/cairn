---
id: res.artefact-filename-drift-audit
nodes:
  - cairn.kernel.artefacts
  - cairn.root
date: 2026-07-27
method: primary
---

# Auditing artefact filename drift: prefix counting undercounts the real set

Measured on this repository on 2026-07-27, while settling
`todo.artefact-filename-convention`.

## What the plan assumed

The todo framed the unit as an open question: "Settle which rule is actually
intended", implying a genuine choice between slug-only filenames and typed
prefixes everywhere, and treating it as a decision the maintainer had not made.

## What the evidence showed

The rule was already settled by the write surface, in three places the todo did
not check:

- `cairn decision new <slug>` writes `meta/decisions/<slug>.md` and carries an
  explicit guard against a pre-existing `meta/decisions/dec.<slug>.md`, which it
  calls a legacy path (`src/cli/commands/decision.rs`).
- The shipped copy for that command already uses the word "legacy" for the
  prefixed form (`docs/design-system/copy.toml`).
- The shipped loop guidance hand-authors research at `meta/research/<slug>.md`
  (`.claude/skills/cairn-loop-reconcile/SKILL.md`).

Only the corpus and the enforcement were missing. That turned the unit from a
judgement call into a ratification, and it is why `dec.artefact-filename-rule`
could be authored `accepted` rather than `proposed`.

## The measurement result worth keeping

The todo counted drift by looking for a typed filename prefix, and found 39
files: 26 of 80 decisions, 9 of 24 research, 4 of 14 sources, with all 134 todos
prefixed by design.

Enforcing the stronger rule (the filename stem must equal the `id` with its
typed prefix stripped) found a fortieth file the prefix count could never see:
`meta/research/gas-city-cairn-integration/analysis.md`, declaring
`id: res.gas-city-cairn-integration`. It carried no typed prefix at all, so
prefix counting scored it as conforming. It was named after its folder rather
than its id, and it was reachable only through an explicit per-file blueprint
pointer, which is the escape hatch `docs/conventions.md` section 10 sanctions
for a layout the same section says MUST NOT exist.

The generalisation: for any convention expressed as "the name must be derived
from the record", auditing by pattern-matching the wrong shape measures only the
violations you already imagined. Comparing against the derivation is what finds
the rest. Here it was a 2.5 percent undercount on a corpus of 40, found only
because the check was written before the migration rather than after it.

## Limits of what this proves

One repository, one convention, one audit. The undercount ratio is not a
general figure. The transferable part is the ordering: write the enforcer first
and let it enumerate the corpus, rather than migrating against a hand count and
adding the check afterwards.
