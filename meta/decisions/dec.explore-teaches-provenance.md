---
id: dec.explore-teaches-provenance
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-06-27
informed_by:
  - res.cairn-oax-skill-promotion
---
# Promote skill value into the pack by merge, not by adding skills

## Context

Bead cairn-oax asked which of seven personal managed cairn-* skills to promote
into the `cairn init` install pack, eval-hardened before publish. An adversarial
debate and a with/without eval (res.cairn-oax-skill-promotion) measured the
candidates against the SHIPPED five-skill pack, not against nothing.

## Decision

The on-ramp pack grows by merging a skill's genuinely non-overlapping value into
the existing pack skill that owns the topic, not by adding more skills. Concretely:
`cairn-first-state-investigation` is not shipped as a separate skill. Its one
durable value over the pack, the provenance and decisions query path
(`cairn rationale`, `decisions`, `research`, `sources`) plus the caveat that the
graph is not a source-symbol index, is folded into `cairn-explore`, whose command
list previously covered structure and health only. The other six managed skills
stay personal.

## Rationale

The eval showed the pack already cues cairn usage; the only measured lift was
provenance, and only because `cairn-explore` never mentioned `cairn rationale`.
Shipping a fourth state skill would duplicate `cairn-explore`, enlarge the trigger
surface, and create two skills with overlapping intent: an easy-now, hard-later
maintenance cost. Merging fixes a real gap in `cairn-explore` at zero new trigger
surface and keeps single-source via `include_str!`. `cairn-provenance-coverage`
is excluded outright: it advises wiring decision pointers, which this repo
deliberately does not do (dec.adopt-cairn-dev-loop).

## Consequences

`cairn-explore` now teaches provenance, so a fresh repo's agent traces decisions
with `cairn rationale` instead of grepping `meta/decisions/`. Future promotion
requests are judged on marginal lift over the current pack, and prefer merging
into the owning skill over adding a new one. The remaining managed skills can be
revisited individually if a later eval shows non-overlapping lift.
