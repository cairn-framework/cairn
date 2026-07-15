---
id: dec.loop-command-harness-model
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-07-13
informed_by: []
refines:
  - dec.adopt-cairn-dev-loop
related:
  - dec.loop-resolves-knowable-gaps
---

# /cairn-loop as a harness-loop router: one unit per session, fail-closed recovery

## Status and ratification

The owner ratified this decision on 2026-07-13, confirming all eight points:
the node resolution rule (point 6) and single-commit landing convention were
explicit calls; the router-skeleton shape (point 2) and the authority rule
(point 8) were formalised here and are now ratified as written.

## Context

OMP's `/loop N` harness re-injects ONE fixed user message into a fresh session
per iteration. The current `/cairn-loop` command predates this: it self-loops
(phase 10 "Continue" selects the next unit), assumes attended Claude Code
slash-commands (`/reforge`, `/debate`), and has no recovery path for a session
killed mid-flight, so a fresh injected session would stack new work onto a
dirty tree. Under a harness loop the command IS the injected message, so the
command must own exactly one iteration and the harness owns repetition.

## Decision

1. **Harness owns iteration.** The command does exactly one unit of work per
   session, lands it, and ends. It never selects a second unit; the outer
   `/loop` provides continuation. The self-looping phase 10 is deleted.
2. **Router-skeleton (command shape).** The command file keeps observation,
   state classification, the preflight verdict table, typed exit tokens, and
   fail-closed backstops inline; procedures (recovery, landing, scope and
   implement recipes) move to skills, each declaring typed exit tokens the
   router's edges key on. A required skill that fails to load halts the
   iteration (halt, never blind improvisation).
3. **Fail-closed preflight.** Before any selection: read-only observation,
   then a verdict table classifying tree and branch state (surviving loop
   branches classified by PR state, never ancestry). Interrupted work is
   finished or trimmed as THIS iteration's unit; unlandable state is
   quarantined (preserved by commit and push, a blocked recover-todo filed,
   worktree parked clean); anything unclassifiable is LOOP HALTED with a
   report and no writes. Never stash, clean, reset, or delete unmerged work.
4. **Isolation.** All loop work happens in a persistent dedicated worktree
   with a dedicated branch namespace (`loop/*`); ownership is structural
   (anything outside the namespace belongs to other sessions and is never
   touched), so concurrent human sessions cannot collide with the loop.
5. **Single-commit landing.** One unit, one branch, one PR, squash-merged so
   main receives exactly one reviewable commit per iteration. Blanket staging
   is banned; paths are staged explicitly.
6. **Node resolution is fail-closed** (ratified). A mission naming new work
   must name or unambiguously imply exactly one blueprint node: an exact node
   id, or a file path falling under exactly one node's `path`. Anything less
   reports the ambiguity with candidate suggestions and exits, rather than
   guessing an anchor that would poison Scope, deps, and provenance for every
   later iteration.
7. **Terminal tokens.** Every session ends with exactly one of
   ITERATION COMPLETE, LOOP EXHAUSTED, or LOOP HALTED as the final line, so
   the harness and the maintainer can read outcomes mechanically.
8. **Authority rule.** The command (plus the skills it loads) is the sole
   normative orchestrator. Any document describing the loop is descriptive,
   never normative. Consequence when this lands: `docs/agent/cairn-dev-workflow.md`
   is retired into the skills plus a short descriptive overview, with its live
   consumers migrated in the same scoped change. Precedence, stated exactly:
   dec.adopt-cairn-dev-loop stays accepted and is refined, not superseded; the
   loop and its gates stand unchanged. Only its consequence naming that doc
   the canonical loop documentation becomes inoperative once this decision is
   accepted and the retirement lands; authority then lives in the command and
   the skills it loads.

## Relationship to accepted decisions

dec.loop-resolves-knowable-gaps is preserved: the investigate, frame, and
recommend obligation stands. In harness-loop mode the attended "stop and ask"
becomes: persist the recommendation as a `meta/` artefact plus a blocked todo,
report, and end the iteration with ITERATION COMPLETE. The loop never waits on
an answer mid-run.

## Consequences

- The rewritten command (~246 lines) sits uncommitted in worktree
  `../cairn-loop-rewrite` (branch `omp-loop-cairn-loop`), reviewed by
  adversarial, contradiction, and ablation audits this session.
  todo.land-loop-command-rewrite tracks landing; landing is blocked on this
  decision's ratification.
- Skill extraction (recovery and landing procedures) follows the repo's
  pack-promotion precedent: evidence-gated, baselined against the shipped
  pack, not shipped on vibes.
