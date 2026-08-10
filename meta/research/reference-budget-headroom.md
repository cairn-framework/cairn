---
id: res.reference-budget-headroom
nodes:
  - cairn.kernel.cli
method: primary
date: 2026-08-10
---

# Three routed cairn-dev references have almost no room left

Measured while closing `todo.brownfield-extraction-reference-gaps`, whose two
prose fixes did not fit. Recorded because the constraint is invisible from the
prose: an author reads a reference, sees a gap, and appends, and only the test
run says the addition is impossible.

## What the gate is

`tools/agent-pack/tests/first_turn_budget_tests.rs` pins
`JIT_REFERENCE_BUDGET_BYTES = 6_000` and asserts it over every
`content/skills/cairn-dev/references/*.md` except `loop-mode.md`, which is
excluded because it loads only on explicit invocation. The intent recorded in
that file is that no routed reference becomes a manual.

## What the corpus measured

Byte sizes at `origin/main` `0933887e`, before this unit:

| Reference | Bytes | Headroom |
|---|---|---|
| `task-refactoring.md` | 2,258 | 3,742 |
| `task-bug-investigation.md` | 2,268 | 3,732 |
| `task-architecture-discovery.md` | 2,493 | 3,507 |
| `task-feature-implementation.md` | 2,633 | 3,367 |
| `graph-navigation.md` | 2,836 | 3,164 |
| `blueprint-syntax.md` | 4,587 | 1,413 |
| `command-reference.md` | 5,230 | 770 |
| `task-brownfield-decision-extraction.md` | 5,701 | 299 |
| `finding-codes.md` | 5,860 | 140 |
| `artefact-schemas.md` | 5,947 | 53 |

## What it cost this unit

The two required passages measured about 700 bytes against 299 of headroom, so
the unit became a compression pass over the whole file, final size 5,983 bytes.
Three constraints made that pass harder than word-count arithmetic suggests.

First, wording carries obligations. The first attempt shaved four qualifiers
that read as verbiage and were not: `eligible` before "declared path" (the
owner resolver admits only leaf or `owns-files` nodes,
`src/brownfield/decisions/mod.rs`), `source` before "comments" (the collector
scans surveyed source files, `src/brownfield/decisions/collect.rs`), "drafts no
narrative" (a distinct no-side-effect promise in
`dec.brownfield-extraction-mechanism`), and "bounded, not selective" (the index
emits the closed set unfiltered). Two independent review lenses caught all four
and they were restored. What the landed pass actually spent: wording, one
emphasis clause that restated the sentence above it (an explicit `local`
ratification claim is "not a shortcut around them"), and one framing sentence
that section 3 already carries ("You select which evidence expresses a real
decision"). No obligation went.

Second, the clause pins bite. `router_route_tests.rs` already pinned eleven
clauses of this reference verbatim (`dec.brownfield-extraction-mechanism`
clauses 2 and 3), so three compressions that read as harmless rewordings failed
the suite and were reverted. This unit added ten more in a second test, so
twenty-one clauses of this one reference are now pinned verbatim across two
tests: read both before rewording anything in it.

Third, the acceptance criterion forbade re-pinning the budget, which is the
correct bar: the gate is doing its job, and the alternative on the table was
deleting an obligation to satisfy a constant.

## What it means for the plan

Adding anything to `task-brownfield-decision-extraction.md`, `finding-codes.md`,
or `artefact-schemas.md` is a compression pass, not an append. Budget the
compression before promising the content, and read the clause pins in
`tools/agent-pack/tests/` before rewording. This is evidence for
`todo.context-engineering-pass` item 2 (progressive disclosure across the
references) rather than a defect: the three references at the ceiling are the
ones that pass would split.
