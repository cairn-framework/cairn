---
node: cairn.ui
status: blocked
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
