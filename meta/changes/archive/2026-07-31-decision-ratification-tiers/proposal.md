# Proposal: decision-ratification-tiers

Implements `todo.decision-ratification-tiers` (node `cairn.kernel.artefacts`).
The tier design was ratified 2026-07-29 (PR #528 sheet W8); this change is the
implementation vehicle that todo prescribes.

## Motivation

The loop may never self-ratify a decision, so every ruling costs a maintainer
round trip regardless of blast radius. That is correct for rulings every
adopting repository inherits (spec invariants, artefact schemas, registries,
shipped pack content) and wrong for the long tail of local ones. Goal 3 of
`dec.north-star-continuous-loop` fixes the signature boundary at the binding
surface and names `todo.decision-ratification-tiers` as its implementation.

The guarded shape is evidence-driven, not convenience-driven. The 26 paired
review rounds audited on 2026-07-27 and 2026-07-28 (Evidence section of the
todo) showed that a single clean review round is not evidence of correctness,
that reviewers catch durable rule errors but also miss holes and block on
disproven findings, and that lens agreement is asymmetric. So machine
acceptance requires converged receipts from independent lenses, auditable lens
identity, and a content-bound subject hash. Never a bare clean pass, and never
prose in a transcript.

## Outcome

A decision artefact can declare `ratification: local` or
`ratification: binding` (absent means `binding`, so existing artefacts keep
their protection without migration), and the loop can set `status: accepted`
on a `local` decision on its own, with machine-checkable evidence: two
committed Review receipts with distinct reviewer identity, clean verdicts, and
a `subject_hash` equal to the canonical manifest hash of everything the
decision governs. Every machine acceptance carries `ratified_by: machine` and
is queryable. A `binding` decision is structurally impossible to
machine-accept: an out-of-shape `local` claim is an Error with no further
evidence needed, and the commit gate refuses an acceptance whose evidence does
not verify.

## Acceptance boundary

`cairn scan` findings, `cairn hook all` exit behaviour, and the query wire:

- Scanner: a decision claiming `local` whose `nodes:` span containers, that
  supersedes anything, or whose `affects:` names a path inside the
  binding-surface allowlist raises an Error on a fixture holding exactly that
  artefact.
- Scanner: a machine-accepted `local` decision that does not reference two
  valid receipts (distinct reviewer identity, same `subject_hash`, clean
  verdicts) raises an Error.
- Hook: a commit accepting a `local` decision while touching a path absent
  from its `affects:` list, or where the manifest recomputed from the tree
  differs from the receipts' `subject_hash` by one byte, is refused with the
  decision named.
- Wire: `cairn pending --json` renders the parsed tier instead of the
  hardcoded v1 default, and `cairn decisions <node> --json` carries
  `ratified_by` so every machine acceptance is listable in one command.

## Evidence

- Unit tests on the canonical manifest hasher: sorted order, governed-content
  extraction, and the keystone invariance that flipping `status` to `accepted`
  and adding receipt references does not change `subject_hash`.
- Scanner fixture tests, one per refusal: cross-container span, supersession,
  allowlist hit, single receipt, duplicated reviewer identity, stale
  `subject_hash`, receipt path missing from `affects:`.
- Hook tests: the affects-subset refusal (the fail-closed case) and the
  manifest-mismatch refusal (the keystone), each its own test.
- The full gate set: `scripts/pre-archive-rust-gates.sh`, `cargo test`,
  `cairn scan --strict` exit 0, `cairn hook all` exit 0 on this repository.
- Dogfood at the boundary: `cairn pending --json` on this repository renders
  the tiers the two live proposed decisions declare in frontmatter
  (`dec.parked-deferral-composition` binding,
  `dec.bootstrap-fixture-corpus-split` local), and the pack-conformance tests
  hold the updated loop assets and their shipped copies byte-identical.

## Out of scope (exclusions)

- Widening the tier: the two-container-span candidate stays unapproved; each
  widening is its own binding decision against a clean audit batch of ten
  machine acceptances (todo, "Widening the blast radius, with evidence").
- Machine-accepting any real decision inside this change: the tiering rule is
  `binding` by its own definition; it lands on the maintainer's W8
  ratification, recorded in `dec.decision-ratification-tiers`.
- Cut 1b (decision-side `defers:`) and folding item 3: separate units of
  `todo.lint-selection-folding`; 1b is unratified.
- The loop End-step pending-count print: a shipped-pack edit
  `todo.maintainer-pending-queue` excluded and reserved for its own word.
- Shipping the lens prompt files in the agent pack: this change updates the
  never-self-ratify rule in the shipped skill copies, but prompt distribution
  to adopters is a later ruling.
- Per-repo configuration of the binding-surface allowlist: the starting list
  is fixed data; extending it is a binding decision by construction.
