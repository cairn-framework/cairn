---
id: dec.adopt-cairn-dev-loop
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-06-05
informed_by: []
---

# Adopt the Cairn Dev Loop as the development workflow

## Context

The repo dogfoods cairn but had no single, written iteration workflow that used
cairn to drive its own development. The cairn-native skills (cairn-explore,
cairn-propose, cairn-apply, cairn-archive) and the CLI gates existed
independently, with no canonical sequence tying orientation, scoping, proposal,
verification, and provenance into one repeatable loop.

## Decision

Adopt a ten-phase coding loop, the Cairn Dev Loop, documented in
`docs/agent/cairn-dev-workflow.md` and runnable as `/cairn-loop`. The phases are
orient, scope, propose, implement, test, verify, record, PR, merge, continue,
each gated by cairn's own queries (`context`, `lint`, `neighbourhood`,
`rationale`, `dependents`) and gates (`scan`, `hook all`), plus the language
gates (`cargo test`, `clippy`) and the path to merge (CI green, review resolved).
The loop is continuous: phase ten selects the next unit and returns to phase one.
A clean iteration is code merged, CI green, `cairn scan` clean, and the
next task identified.

## Rationale

The framework should verify its own development. Using cairn to orient before
coding and to gate the result makes the dogfooding signal load-bearing rather
than aspirational: every iteration must leave `cairn scan` clean. The loop reuses
the existing skills and CLI surface instead of adding new machinery, so it stays
thin and surgical.

One deliberate boundary: the loop does not wire a `decisions` pointer into
`cairn.blueprint`. Provenance coverage in cairn is all-or-nothing (the first
declared decision makes every uncovered leaf node raise
`CAIRN_PROVENANCE_NO_DECISION`), so ingesting decisions is a repo-wide commitment
left for a dedicated iteration. Until then, decision records live in
`meta/decisions/` as durable prose, and this file is the first one written under
the loop it describes.

## Consequences

- `docs/agent/cairn-dev-workflow.md` becomes the canonical loop documentation.
- `CLAUDE.md` should reference it from the "Using cairn in this repo" section.
- Iterations should leave `cairn scan`, `cairn lint`, and `cairn hook all` green.
- Larger-than-atomic work must still go through a proposal/change directory, not
  be forced through the loop.
