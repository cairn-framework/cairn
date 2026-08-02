---
node: cairn.kernel.hooks
status: open
created: 2026-07-31
---

# Ratification Candidate Discovery Ignores The Decisions Pointer

## Problem

`candidate_accepted_local` (`src/hooks/ratification/git.rs`) enumerates
candidate decisions from the hardcoded path `meta/decisions`, but the
decisions directory is a configured artefact pointer, not a constant. An
adopting repository whose blueprint points `decisions` elsewhere gets a
ratification gate that sees no candidates at all: every local acceptance in
such a repository passes the hook unexamined, silently, with no finding to
say the gate did nothing.

This repository is unaffected today (its pointer is the default), which is
exactly why the gap survived review: the dogfood path cannot show it.

## Scope

Derive candidate discovery from the configured artefact pointers the registry
already resolves, rather than a literal. Both modes are affected (index and
head), and both the enumeration and the per-file `git show` spec build on the
same resolved directory.

Test with a fixture whose decisions pointer is NOT the default: an accepted
local decision there must still trigger the gate, and the existing
default-pointer tests must keep passing unchanged.

## Acceptance

- A fixture repository with a non-default decisions pointer refuses an
  accepted local decision that omits a changed path from `affects:`, proving
  the gate sees candidates through the pointer.
- No literal artefact directory remains in the hook's candidate discovery.

## Origin

Adversarial review of PR #544 (`dec.decision-ratification-tiers`
implementation), recorded 2026-07-31 immediately after that merge.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It keeps pending ratification work discoverable and actionable.
