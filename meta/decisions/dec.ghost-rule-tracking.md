---
id: dec.ghost-rule-tracking
nodes:
  - cairn.root
status: proposed
date: 2026-06-27
informed_by: [res.spec-designed-audit]
---
# Tracking Designed-but-unimplemented spec rules (the "ghost rule" gap)

**Status: proposed.** This records a decision-ready recommendation for bead
cairn-iy2. It awaits maintainer ratification before any code lands; the loop
surfaced it on a timed escalation (no live ratification), so per the dev-loop's
unattended protocol the recommendation is persisted here rather than
self-executed.

## Problem

Cairn models two kinds of "declared but not yet real":

- **Ghost nodes** (`NodeState::Ghost`, spec:612): declared structure, no code yet.
- **Planned tests** (`#[cairn_planned(phase = N)]`): a test that should exist but
  is gated to a future phase.

There is **no** primitive for a Designed-but-unimplemented *rule / behaviour /
capability*. spec.md:24 mandates that implementation status live "in the
project's own Cairn state, not in the spec itself," but nothing turns a Designed
spec rule into tracked cairn state. Consequence: a Designed integrity rule can
rot in prose and pass scan. Proven twice: the leaf-contract rule (spec:318) sat
unimplemented until cairn-481, and the cairn-mqe audit
(`meta/research/res.spec-designed-audit.md`) found two more (research-orphan
spec:632, revisit-trigger relevance spec:634) that only a manual pass caught.

## Options

- **(a) Convention only.** Every Designed spec rule gets a tracking bead labelled
  `spec:<section>`. Cheapest, no code. But unenforced: it relies on a human
  remembering to file the bead, which is exactly the rot mode spec:24 warns
  about. cairn-mqe executed this once by hand; nothing prevents the next slip.
- **(b) Registry + scan check.** A machine-readable spec-rule registry
  (e.g. `docs/registries/spec-rules.md`, or a structured sibling) mapping each
  Designed rule (`spec:<line>`) to the `CAIRN_*` finding code expected to enforce
  it. A scan check warns when a registered Designed rule has no emitting code in
  non-test source. Deterministic, dogfoodable, and structurally identical to how
  cairn already gates declared-vs-real for nodes and edges. Cost: one check
  module plus the registry as a living surface that must be kept current.
- **(c) First-class "planned rule" artefact.** A new artefact type symmetric with
  ghost nodes: a planned-rule lifecycle with its own schema and validation. Most
  conceptually elegant, heaviest to build, and the registry in (b) achieves the
  tracking without minting a new artefact primitive.

## Recommendation

**Adopt (a) as standing convention AND build (b).** (a) is already in force (the
audit filed tracking beads). (b) is the durable enforcement that makes the
convention self-checking: cairn catching its own spec drift the same way it
catches node/edge drift. This is the bead author's recommendation and matches the
established pattern George has repeatedly ratified for self-gating
(cairn-481 leaf-contract, cairn-9v1 edge-drift). (c) is deferred as likely
premature.

Open sub-questions for the implementing bead, should (b) be ratified:

1. Registry format and home (`docs/registries/spec-rules.md` markdown table vs a
   structured TOML/JSON the check parses directly).
2. Severity (Warning, advisory, consistent with the other rationale tensions) and
   loose/strict modes (mirror the test-coverage and leaf-contract gates).
3. Whether a registered rule may be marked "Declared (deferred)" so Declared-level
   capabilities (edge validation, docstring drift) are listed but not flagged.

## Consequence if ratified

A new scan check and a living `spec-rules` registry. Each Designed rule has a row;
removing or never-adding an emitting code surfaces a finding. Declared-level items
are listed but exempt. This closes the meta-gap that cairn-mqe could only close by
hand.
