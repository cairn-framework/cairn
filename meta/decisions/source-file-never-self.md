---
id: dec.source-file-never-self
nodes:
  - cairn.kernel.artefacts
status: proposed
date: 2026-07-27
informed_by:
  - res.source-self-reference-unchecked
---
# A source's `file:` never resolves to the source artefact itself

## Context

A Source artefact records where external evidence lives. Its `file:` field is
that pointer, and `verification` says how much the project trusts it.

`dec.artefact-filename-rule` made an existing collision reachable. Under that
rule a source's filename is its `id` with `src.` stripped, so
`id: src.review-adversarial-1` must live at `meta/sources/review-adversarial-1.md`.
The bootstrap fixture's copy of that artefact already declared
`file: ./meta/sources/review-adversarial-1.md`, the exact path the rename gives
it. Applying the naming rule turned a stale pointer into a self-reference.

Nothing detects it. Measured on 2026-07-27, `validate_sources` reads `file:`
only in the `Verified` and `External` arms; the `Unverified` arm emits
`CAIRN_SOURCE_UNVERIFIED` at Info and ignores `file:` entirely
(`res.source-self-reference-unchecked`).

## Decision

A Source artefact's `file:` MUST NOT resolve to that artefact's own path, under
any `verification` value.

When the evidence is the artefact body itself, because the material arrived in
conversation and was never saved to a file, `file:` is `null` and the body
states why. `null` is the honest encoding: there is no external file, and
`verification: unverified` already says the record is not hash-backed.

This is `proposed`, not `accepted`. The rule is stated here so the fixture
change that prompted it does not carry an unrecorded judgment, but ratifying a
repository-wide artefact rule is the maintainer's call, not a loop iteration's.

## Rationale

A source exists to point outside itself. A pointer back at the record carries no
information, and it is worse than an absent pointer because it reads as though
provenance was established.

Three options were weighed for the artefact that surfaced this.

Pointing `file:` at the real transcript is the best answer whenever a transcript
exists. Here none does, and the artefact's own `description` has recorded that
since 2026-04-13.

Keeping the self-reference and annotating it was rejected because the annotation
can only say that the pointer means nothing, which argues for deleting the
pointer rather than documenting it.

Changing the artefact's slug so the two paths stay distinct was rejected because
the id is cited by four decisions in the fixture, and it makes the filename rule
the thing that dictates ids rather than the reverse.

`null` also required no new convention: three of the other four unverified
sources in that same directory already use it
(`res.source-self-reference-unchecked`).

## Consequences

- `tests/fixtures/cairn-bootstrap/meta/sources/review-adversarial-1.md` sets
  `file: null` and carries a body paragraph recording why. That edit stands on
  its own evidence and does not wait on this decision.
- The rule is currently enforced only where it was found:
  `test_bootstrap_fixture_sources_do_not_cite_themselves` in
  `tests/fixtures_smoke.rs`. That directory is covered by no reconciler, because
  the bootstrap blueprint declares no `sources` pointer, so a test is the only
  available gate there.
- A general reconciler check is filed as `todo.source-self-reference-finding`
  against `cairn.kernel.artefacts`, blocked until this decision is accepted.
  Its severity is deliberately left open.
- No existing decision is superseded. `dec.artefact-filename-rule` is unchanged;
  this decision governs a different field and was prompted by applying it.
