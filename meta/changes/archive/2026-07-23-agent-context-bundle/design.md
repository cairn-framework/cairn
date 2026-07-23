# Design: agent-context-bundle

## Approach

Append one dated primary-research entry to
`res.loop-efficiency-observations`. Keep the inventory separate from later
experimental results. The entry fixes the selector, captures byte-level run
records, defines grading units before any development-corpus candidate is
measured, names exact executable compositions, and labels the only new
projection as hypothetical.

The selector is corpus-dependent but precommitted. The evaluation unit builds
and freezes its manifest from the baseline corpus before candidate output. The
context-bundle evaluation uses development data; treatment alone uses
confirmation after opening the sealed split.

## Changes

ADDED:
- Acceptance requirements for inventory completeness, reproducibility, and the
  evaluation handoff

MODIFIED:
- `res.loop-efficiency-observations` gains the inventory, sample rule,
  accounting protocol, and candidate list
- `todo.agent-context-bundle` reaches `done` after acceptance

REMOVED:
- None

RENAMED:
- None

## Invariants

- No result, score, threshold, or recommendation is published in this change.
- Candidate requests never branch on candidate output.
- Confirmation prompts and ground truth remain sealed.
- Transport and argument vectors are part of each candidate.
- A hypothetical projection gains no authority or implementation status.
