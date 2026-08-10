---
node: cairn.ui
status: open
created: 2026-08-10
blocked_by:
  - todo.console-contrast-honesty
parent: todo.console-state-legibility
---

# Render the evidence the browser already downloaded

Implementation unit split out of `todo.console-state-legibility` under the
sizing rule. This unit owns clause 1 of that todo and nothing else: four
readouts where the wire already carries useful data that the console omits or
mislabels. No new endpoint, no new query, no orchestration surface.

Blocked on `todo.console-contrast-honesty` alone, so this unit's added text is
graded by the composited contrast measurement rather than by a checker that
overstates it. It has no dependency on `todo.console-state-grammar`: that unit
works on the node module, the state tokens, and the legend, while this one
works on the bootstrap data, the channel bar, and the status bezel.

## Task

1. **Surface the recommended unit.** `status.next_recommended`
   (`{command, node, rank, source, title}`) arrives through `fetchStatus()` in
   `src/ui_assets/app-data.js:11` and is read by nothing: no occurrence of
   `next_recommended` exists anywhere under `src/ui_assets/` or `src/ui/`.
   Render the recommended unit together with the rule that selected it, so the
   selection is explicable and not an oracle.

2. **Render backlog titles.** `/api/roadmap` items carry `title`, and
   `itemLabel` in `src/ui_assets/channel-bar.js:55-61` renders `item.stem` as
   the title with `item.path` as the body. That is why the backlog lane reads as
   a column of `meta/todos/todo.*.md` filenames. Render the title, keeping the
   stem or path available as the secondary identifier.

3. **Render the pending ruling in the lane.** `/api/pending` items carry
   `ruling_summary`, `rubric.tier`, and `rubric.unblocks`. `PendingDetail`
   (`src/ui_assets/channel-bar.js:116-161`) renders all three, but only once a
   row is expanded; the collapsed row from `itemLabel`
   (`src/ui_assets/channel-bar.js:63-81`) shows `item.id` with an age and
   ratification meta string. Put the summary and the tier on the collapsed row,
   so the lane is readable without opening every item.

4. **Stop the two readouts contradicting.** `DriftIndicator`
   (`src/ui_assets/status-bezel.js:32-46`) computes `drift` as errors plus
   warnings only, so whenever there are zero errors and warnings but a non-zero
   Info count, the bezel prints the clean copy beside a findings count that is
   not zero. That is this repository's normal state. Qualify the copy so the
   two readouts state the same thing: the clean claim is about the severities
   that block, not about the total.

User-facing strings go in `docs/design-system/copy.toml`; nothing is hardcoded
in the JS or in Rust.

## Acceptance

- The recommended unit and the rule that selected it, the backlog titles, and
  both the pending ruling summary and its `rubric.tier` are visible without
  truncation while the pending row is COLLAPSED, asserted by a harness scenario
  that fails against the current renderers. The tier is named explicitly
  because leaving it in the expanded `PendingDetail` alone would satisfy a
  criterion that said only "summary".
- The bezel copy and the findings count cannot state contradictory things for
  any severity mix, covered by a case with Info findings and no error or
  warning.
- No new hardcoded user-facing string: every added string resolves through
  `copy()`.
- `scripts/check-design-tokens.sh` and `scripts/check-a11y.sh` exit 0.
