---
node: cairn.kernel.cli
status: done
created: 2026-07-16
---

# Make /cairn-vibe worker assignments carry the Loop discipline explicitly

`/cairn-vibe` (`.claude/commands/cairn-vibe.md`) tells the director that each
unit "follows the /cairn-loop discipline", but workers are separate sessions
that never see `.claude/commands/cairn-loop.md`. Nothing guarantees the
worker learns the Propose rule (substantial work invokes `cairn-propose`
then `cairn-apply`; small surgical work states the success criterion
inline). The later "if the unit had a `meta/changes/<id>/` change directory"
wording makes proposals look optional even for substantial units.

## Task

Amend the Vibe command's Execute phase (phase 3) with an explicit
worker-assignment contract the director must include verbatim in every
worker prompt:

1. Exact todo slug and node, and the one-unit-only boundary (stop after the
   unit's PR is open; never select further work).
2. The Propose rule stated directly: substantial work scaffolds a change via
   the `cairn-propose` skill and implements via `cairn-apply`; surgical work
   states a written success criterion inline.
3. The existing per-unit requirements (feature branch from `origin/main`,
   test for changed behaviour, gates, `cairn todo set` for status, one PR
   per unit), so the assignment is self-contained without reading the Loop
   command.

Do not have workers run `/cairn-loop` itself: its autonomous
select-next-unit-and-continue behaviour conflicts with Vibe's bounded
one-unit assignment.

## Acceptance

- `.claude/commands/cairn-vibe.md` phase 3 contains the explicit assignment
  contract, including the substantial-vs-surgical Propose rule, and no
  longer relies solely on "follows the /cairn-loop discipline" by reference.
- The conditional change-directory wording in phase 4 is reconciled with the
  rule: substantial units are expected to create a change directory, and
  phase 4 archives any that exists regardless of how the unit was
  classified.
- Prose follows house style: no em-dashes, plain English.
