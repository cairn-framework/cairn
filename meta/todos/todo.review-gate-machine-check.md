---
node: cairn.root
status: open
created: 2026-07-30
---

# Review Gate Machine Check

## Problem

The two-lens pre-submit review (simplification pass, then adversarial pass)
is declaratively mandated by AGENTS.md and `cairn-loop-landing`, but nothing
machine-checks that it ran. The external driver verifies the terminal token,
todo status, and park state; all three attest that a session finished, not
that the mandated review happened. CodeRabbit is advisory in CI. A session
that skips the review and lands anyway is indistinguishable, on the wire,
from one that ran it.

## Scope

Design machine-checkable evidence that the mandated review workflow ran for
a landing, and decide where verification lives (driver, CI, or hook). This
sits inside the wider over-harness thread: first-class declarative workflow
definitions per project type, review workflow included, so the evidence
format should fall out of the workflow definition rather than being bespoke
to one review ritual.

## Relationship to todo.local-gate-attestation

`todo.local-gate-attestation` covers deterministic build and test receipts;
this todo covers evidence that a judgment workflow ran. Reuse any general
Phase 2 attestation substrate it produces rather than inventing a second
one.

## Acceptance

- A design names the review evidence, its producer, its verifier, and the
  failure mode for forged or missing evidence.
- The two-lens review obligation is verifiable by a machine for at least
  the loop landing path, or a decision accepts the trust-verified status
  quo and closes this todo.

## Origin

Maintainer conversation, 2026-07-30
(`src.maintainer-design-threads-2026-07-30`), thread b of
`res.overharness-design-threads`; captured via
`todo.overharness-research-capture`.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It makes the review gate repeatable and observable.

cairn.root anchor justified (2026-08-07): design todo; output is a decision, not owned source.
