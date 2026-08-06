---
node: cairn.ui
status: open
created: 2026-08-03
related: [dec.webui-write-authority, dec.webui-design-token-gate, dec.webui-a11y-static-audit-gate, todo.console-orchestration-ux-design]
---

# Console state legibility: render what the wire says, fix what the gate cannot see

Two rounds of defect fixes on the shipped read-only surfaces, measured in
this session against the live webui. Both sit inside the read-only grant
`dec.webui-write-authority` clause 1 already gives, so neither needs a
signature and neither is orchestration-facing. Evidence and measurements
live in `todo.console-orchestration-ux-design`; this unit is the
implementation half.

## Task

1. **Render the evidence the browser already downloaded.**
   - `status.next_recommended` (`{command, node, rank, source, title}`)
     is fetched by `src/ui_assets/app-data.js` and read by nothing.
     Surface the recommended unit with the rule that selected it.
   - `/api/roadmap` items carry `title`; `channel-bar.js` renders
     `item.stem` and `item.path`, which is why the backlog lane is a list
     of `meta/todos/todo.*.md` filenames. Render the title.
   - `/api/pending` items carry `ruling_summary`, `rubric.tier`, and
     `rubric.unblocks`; the lane renders `item.id` truncated to a few
     characters. Render the summary and tier.
   - The bezel reads "No issues" above a findings count, because `drift`
     in `app.js` means error-or-warning only and severity is never
     stated. Qualify the copy so the two readouts cannot contradict.
2. **Fix the state grammar in place.**
   - Replace the `opacity: 0.55` dimming in `style.css` with ink token
     steps, so dimmed text stays above 4.5:1.
   - Split `--orphaned` from `--drift`: they are the same amber today and
     the legend renders both keys identically.
   - Replace the drift animation (`driftPulse`, `driftBlink`) with a
     channel that survives greyscale and `prefers-reduced-motion`, which
     `components.css` disables and `harness/eval.mjs` emulates. Motion
     becomes decoration, not state.
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
