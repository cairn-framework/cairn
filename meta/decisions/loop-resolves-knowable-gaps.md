---
id: dec.loop-resolves-knowable-gaps
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-06-27
informed_by: []
refines:
  - dec.adopt-cairn-dev-loop
---

# The dev loop resolves knowable gaps instead of dead-stopping on them

## Context

The Cairn Dev Loop (dec.adopt-cairn-dev-loop) ended phase 10 by stopping when "a
gate is blocked on a decision that is the maintainer's to make." In practice this
produced a dead stop the moment the only remaining backlog item was deferred or
maintainer-gated: the loop reported "nothing to do, it is blocked, you decide"
and yielded, handing the maintainer an unframed question with no homework done.
Most such blocks are not external walls; they are
missing information or an unmade judgment that investigation and reasoning can
inform. Treating them as terminal wastes the loop and pushes work the loop could
have done back onto the maintainer.

## Decision

Phase 10 no longer treats "blocked on a maintainer decision" as a valid first
response. When the next candidate is blocked on a decision rather than on code,
the loop classifies it:

- A **knowable gap** has, as its only obstacle, missing information or an unmade
  judgment that research can inform. Resolving the gap becomes the work unit. The
  loop runs an investigate, debate, recommend, escalate sub-loop: pull the code,
  artefacts, prior decisions, and cited sources; stress-test contested options
  with an adversarial pass; produce a decision-ready package (the gap, two to
  four options, trade-offs, a recommended option with justification) persisted as
  a `meta/` artefact; then escalate to the maintainer with the recommendation
  attached.
- A **true external blocker** (missing credential or paid API, upstream fix,
  access or hardware dependency, a human-only sanction, or a pure priority or
  taste call with no fact that would move it) cannot be researched away. The loop
  surfaces it immediately and plainly, stating what is needed and why, then moves
  on or stops.

Unattended, the loop does the same investigate, debate, and recommend work, but
since no maintainer is online to ratify it persists the recommendation as a
`meta/` artefact plus a deferred bead and continues to the next unit. It never
self-answers a maintainer's choice and never halts the whole run on an answer it
cannot get.

In an attended run the loop stops after surfacing a decision-ready recommendation
that needs ratification; in an unattended run that condition instead becomes a
persisted recommendation plus a deferred bead and continuation (above). Either
way the loop also stops when the backlog is empty and `cairn lint` is clean with
no knowable gap left, when the only remaining work is a true external blocker it
has already surfaced, or when told to stop.

## Rationale

This refines dec.adopt-cairn-dev-loop; it does not reopen dec.no-orchestrator.
The change lives in the orchestration layer (the `/cairn-loop` command and the
workflow doc), not in cairn the binary, which stays in its semantic lane. The
behaviour mirrors how a good gap-surfacing tool works: it reserves the
maintainer's attention for the actual judgment and arrives with the options
already framed and a recommendation justified. It reuses skills the repo already
has (`adversarial-decision-debate`, `decision-convergence-minutes`) rather than
adding machinery.

## Consequences

- `docs/agent/cairn-dev-workflow.md` gains a "When the next unit is a knowable
  gap" section, an unattended-mode clause, and a redefined phase-10 stop
  condition.
- `.claude/commands/cairn-loop.md` is updated to match: investigate and frame
  before asking, and escalate with recommended options.
- The loop will produce more `meta/` research and draft-decision artefacts as it
  frames gaps, which is the intended provenance trail.
- A maintainer-gated item is now surfaced as a specific question with a
  recommended answer, not a bare blocker.
