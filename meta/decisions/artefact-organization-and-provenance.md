---
id: dec.artefact-organization-and-provenance
nodes:
  - cairn.root
status: accepted
date: 2026-06-26
informed_by: []
---

# Artefact organization and provenance links

## Context

The cairn spec (section 16, Q-03) left the `meta/` directory layout as an open question:
artefact-type-first (`meta/decisions/`, `meta/research/`) versus node-first
(`meta/kernel/decisions/`). The question was deferred for real usage to surface a winner.

By mid-2026 cairn's own practice had converged without a formal rule: 26 decisions in a flat
`meta/decisions/`, research grouped by topic slug in `meta/research/`, 2 sources flat in
`meta/sources/`. A four-critic adversarial review (schema rigour, information architecture,
dogfooding practicality, provenance completeness) pressure-tested a proposed codification and
produced 15 adjustments before ratification.

Key findings from the review: sources carry no `nodes:` anchor (the Source schema does not model
one; existing source files with `nodes:` lines are inert); research uses `sources: [...]` not
`informed_by:`; and the provenance up-chain is mechanically DETECTED (advisory warnings), not
mechanically ENFORCED (blocking errors). A formal research artefact for this analysis has been
deferred to the dark-store triage change.

## Decision

Adopt flat artefact-type-first layout as the normative convention for cairn provenance artefacts.
The full normative text, including the shipped error-code inventory and per-type link-model fields,
lives in docs/conventions.md section 10 ("Artefact organization and provenance links").

Summary of the ratified rules:

1. Scanner-loaded artefacts are FLAT under `meta/decisions/`, `meta/research/`,
   `meta/sources/`. The loader is non-recursive; a file in a subfolder is silently ignored.
   Subfolders are not permitted. Topical grouping uses slug namespacing in the id and filename
   (for example `res.gas-city.analysis`, `res.gas-city.issue-slate`).
2. The typed id prefix (`dec.`/`res.`/`src.`) lives only in the `id:` frontmatter field, never
   in the filename.
3. Provenance links are per-type: decisions use `informed_by: [...]` (up to research or source
   directly; research is not a required intermediary); research uses `sources: [...]`; sources use
   `file:` with `verification`. Sources anchor transitively through citations and carry no
   `nodes:` field.
4. Todos are deliberately node-partitioned (`meta/todos/<node>/`): an explicit exception to flat
   artefact-type-first.
5. The genesis exception (conventions.md section 9) is exempt across all axes.
6. Provenance dangling (unresolved `informed_by`/`sources`) is mechanically detected as advisory
   warnings and does not fail the gate. Id uniqueness and typed-prefix conformance are
   author-side policy only; neither is gated today.

## Rationale

Artefact-type-first is the de-facto winner in cairn's own repo. Node-first requires knowing a
node's identity before locating its decisions, and node renames would cascade through directory
structure. Flat layout keeps `cairn decisions <node>` and `cairn research <node>` as the
discovery surface; directory walking is not the query interface.

Slug namespacing is strictly more powerful than single-level subfolders: it allows arbitrary
grouping depth in the id without any filesystem path that could fool the non-recursive loader.

The convention documents what the scanner actually enforces rather than asserting mechanical
guarantees the code does not provide. The schema-rigour critique was decisive: "mechanically
verifiable" MUST NOT be claimed for any rule without an Error-tier gate.

## Consequences

1. Spec section 16 Q-03 is closed in favour of flat artefact-type-first. The spec MUST be amended
   to remove the stale `meta/decisions/kernel/` example and align the `informed_by`/`sources`
   field assignment with the per-type model.
2. Three net-new scanner gates are recommended and tracked as open questions in
   docs/conventions.md section 10: `CAIRN_ARTEFACT_DUPLICATE_ID`, id-prefix-conformance check,
   and unwired-artefact-under-pointer-root detection.
3. The phrase "mechanically verifiable" MUST NOT be applied to any provenance rule until its
   Error-tier gate exists.
4. The dark-store triage (G1) archives `docs/strongholds/` and `docs/research/` to `archive/`,
   preserving the originals as referenceable history. Load-bearing material is promoted to a
   native artefact only where it earns it (for example `res.cairn-domain-expandability`);
   external/competitor material (the getcairn.dev study) stays in the archive and is cited as a
   `source` if and when a decision needs its provenance, never inlined as research.
