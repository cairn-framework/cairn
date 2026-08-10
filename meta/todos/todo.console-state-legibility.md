---
node: cairn.ui
status: done
created: 2026-08-03
blocked_by:
  - todo.console-contrast-honesty
  - todo.console-state-grammar
  - todo.console-wire-legibility
related: [dec.webui-write-authority, dec.webui-design-token-gate, dec.webui-a11y-static-audit-gate, todo.console-orchestration-ux-design]
---

# Console state legibility: render what the wire says, fix what the gate cannot see

Two rounds of defect fixes on the shipped read-only surfaces, measured in
this session against the live webui. Both sit inside the read-only grant
`dec.webui-write-authority` clause 1 already gives, so neither needs a
signature and neither is orchestration-facing. Evidence and measurements
live in `todo.console-orchestration-ux-design`; this unit is the
implementation half.

This unit was decomposed on 2026-08-10 under the sizing rule: it carries nine
distinct changes across the webui, the shared design system in
`docs/design-system/` (which the marketing lane also consumes, and where two
affected rules also live, not only in the webui overrides), and the
visual harness. That is more than one small reviewable PR.

blocked on sub-todos: todo.console-contrast-honesty, todo.console-state-grammar, todo.console-wire-legibility

The three partition the task list below without remainder: clause 3 plus the
`opacity` item of clause 2 go to `todo.console-contrast-honesty`, the rest of
clause 2 goes to `todo.console-state-grammar`, and clause 1 goes to
`todo.console-wire-legibility`. The other two both depend on
`todo.console-contrast-honesty` and on nothing else, so that the contrast
measurement is honest before anything is graded against it; they do not depend
on each other and are ordered only by slug. The iteration completing the last
child flips this parent to done.

## Task

1. **Render the evidence the browser already downloaded.**
   - `status.next_recommended` (`{command, node, rank, source, title}`)
     is fetched by `src/ui_assets/app-data.js` and read by nothing.
     Surface the recommended unit with the rule that selected it.
   - `/api/roadmap` items carry `title`; `channel-bar.js` renders
     `item.stem` and `item.path`, which is why the backlog lane is a list
     of `meta/todos/todo.*.md` filenames. Render the title.
   - `/api/pending` items carry `ruling_summary`, `rubric.tier`, and
     `rubric.unblocks`. `PendingDetail` already renders all three. The
     collapsed row built by `itemLabel` carries the id, node ids, age,
     ratification, and subject hash, but not the summary or the tier.
     Put those two on that row.
   - The bezel reads "No issues" above a findings count, because `drift`
     in `app.js` means error-or-warning only and severity is never
     stated. Qualify the copy so the two readouts cannot contradict.
2. **Fix the state grammar in place.**
   - Replace the `opacity: 0.55` dimming with ink token steps, so dimmed
     text stays above 4.5:1. It is declared both canonically in
     `components.css` and again in `style.css`, and the served sheet
     concatenates the canonical layer first, so both must change.
   - Split `--orphaned` from `--drift`: they are the same amber today,
     and one selector in `components.css` paints both legend keys from
     `--orphaned`, so the token split alone would not separate them.
   - Replace the drift animation (`driftPulse`, `driftBlink`, both in the
     shared `components.css`) with a channel that survives greyscale and
     `prefers-reduced-motion`. The current selectors do not stop both
     animations in both consumers, and the harness hides that by forcing
     animation off before it measures. Motion becomes decoration, not
     state.
   - Put node state into the accessible name. The `.state-dot` is
     `aria-hidden` and the option's accessible name is the node id, so no
     state reaches a screen reader at all.
3. **Make the audit able to see the defect.** `harness/lib/audit.mjs`
   reads uncomposited `getComputedStyle` colour and skips only
   `opacity === 0`, so it computed 10.86:1 where the composited value is
   4.42:1. Composite ancestor opacity before computing contrast, then fix
   the cause so the check returns to zero honestly.

## Acceptance

- The recommended unit, backlog titles, and pending ruling summaries are
  visible without truncation, asserted by a harness scenario.
- Every state pair on the canvas is distinguishable in greyscale and
  under `prefers-reduced-motion`, verified by a capture of each.
- Node state reaches a screen reader through the accessible name.
- The contrast audit composites ancestor opacity, and `ux_defect_score`
  is zero with the composited measurement in force.
- Token and accessibility gates stay green (`scripts/check-design-tokens.sh`,
  `scripts/check-a11y.sh`); no hardcoded hex or rem is introduced.

## Why this is separate from the orchestration UX unit

`dec.orchestration-placement` clause 4 gates orchestration-facing console
implementation on the journeys and mockups. These two rounds are neither
orchestration-facing nor speculative: they are defects on surfaces that
already shipped, and fixing them first removes "the console showed
nothing useful" as a confound when the mockups are compared against the
live surface.

2026-08-07 audit (todo.roadmap-assumption-audit): keep; blocker of todo.ui-asset-refresh.

2026-08-10 decomposition: the Task bullets above were corrected in place where
the 2026-08-03 wording proved wrong against the source. The child todos carry
the full detail, and `res.console-state-legibility-decomposition` records the
item-by-item read, what it confirmed, and what it corrected.

2026-08-10, first child delivered: `todo.console-contrast-honesty` landed
clause 3 and the `opacity` item of clause 2. The audit now composites the
ancestor opacity chain in both passes, and `.node-shell.dimmed` recesses by ink
step instead of fading. The honest measurement found three defects where clause
3 predicted one (`.node-name` at 4.42:1, `.node-description` at 2.61:1,
`.node-id` at 2.57:1) and showed the ink ramp has only two levels above the AA
floor on this surface: `res.console-contrast-composited-measurement`. Remaining
here: clause 1 in `todo.console-wire-legibility`, the rest of clause 2 in
`todo.console-state-grammar`, both now unblocked.

2026-08-11, second child delivered: `todo.console-state-grammar` landed the
rest of clause 2. `--orphaned` now resolves to the ember family in both
themes (a darkened ember in light, matching how the light block darkens
amber) instead of aliasing `--drift`, and the legend key and tilt selectors
were split so the two keys render differently, not only the tokens. Drift
gained a static channel: a diamond (border-radius 0 plus a 45deg rotation)
on `.node-module.drift .state-dot` and on
`.state-legend .legend-item.drift .legend-key`; the orphan key tilt moved
from the webui override sheet into `components.css` so both consumers carry
both static channels. `.node-module.drift` and `.node-module.drift
.state-dot` were added to the reduced-motion block so `driftPulse` and
`driftBlink` stop in both consumers, wrapped or not; the harness now asserts
this before injecting its animation kill-switch, in the webui and in the
unwrapped live reference (`reduced_motion_violations`). Node state reaches
the accessible name (`<id>, <state label>` in `node-module.js`), asserted by
a new `nodeStateNamed` harness landmark on every map scenario. As the plan
predicted from `res.console-contrast-composited-measurement`, greyscale
separation comes from the shape channels, not hue: ember and amber sit at
nearly identical luma. Greyscale captures of the four states were taken
in-session against both consumers and not retained; the committed checks are
the harness landmarks plus the reduced-motion assertion. Remaining here:
clause 1 in `todo.console-wire-legibility`.

2026-08-11, third child delivered: `todo.console-wire-legibility` landed
clause 1, closing this parent. The bezel gained a full-width `.status-next`
row naming `status.next_recommended` with the rule that selected it (source
mapped to copy: finding, todo, bead). Backlog rows render `item.title` as the
title with the stem as secondary identifier; a new
`.channel-code.channel-title-prose` component class lifts the fixed-width mono
clamp so prose titles flex and wrap, in the channel bar and inside console
lanes. The collapsed pending row renders `ruling_summary` and `rubric.tier`
in a `.pending-collapsed` block. The bezel clean copy is scoped to blocking
severities ("No blocking issues" plus the severity summary whenever any
finding stands), so the chip and the findings count cannot contradict. All
strings resolve through `copy()`; six new harness landmarks assert the four
readouts, measuring visual truncation (scroll vs client box), and fail
against the pre-change renderers (red run: 6 missing landmarks across
overview-desktop, backlog-tiers, the three console scenarios, and
pending-inbox).
