---
id: dec.decision-claim-cross-check
nodes:
  - cairn.kernel.artefacts
status: accepted
date: 2026-06-26
---
# Cross-check decision closure claims against the declared-items registry

## Decision

Cairn now emits `CAIRN_DECISION_CLAIM_UNRESOLVED` (error code CA004) when a
decision artefact's prose claims to close or resolve a spec open-question
(`Q-NN`) but `docs/registries/declared-items.md` still lists that question as
anything other than `resolved`, or omits it entirely. This turns the manual
ratify-a-convention-and-close-all-surfaces procedure into a scan gate
(cairn-zad).

The check runs in `validate_decision_claims` (src/artefacts/registry/validate),
called from `validate_integrity`, which already receives the project root and
reads files for source verification. It reads the registry once, parses the
`Q-NN` status column, and scans each decision body for a `Q-NN` token on a line
that also carries a `clos*` or `resolv*` verb.

## Severity: Error, so the hook gate enforces it

The finding is an `Error`. `cairn hook all` (the pre-push and CI gate) blocks
only on error-severity findings: warnings and info flow through the advisory
tension channel and do not set `ExitDecision::Block`. A `Warning` would fail
`cairn scan --strict` but pass the hook path the bead targets, so it would not
actually gate. Prose drift between a decision and the registry is a genuine
integrity contradiction that should block, so it is an error. No existing
decision in the repo makes such a claim, so the repo stays green.

## Scope: Q-NN registry status only

The cross-check covers the deterministic, well-anchored case: a `closes Q-NN`
claim versus the registry's status column. The bead also mentioned `supersede`
claims against spec presence, but a "supersedes the X example" claim has no
structured anchor in `docs/spec.md` to check against deterministically, so a
gate there would be guess-driven and false-positive-prone. That portion is
deliberately deferred until a structured anchor exists; the registry check
delivers the high-value, reliable half now.
