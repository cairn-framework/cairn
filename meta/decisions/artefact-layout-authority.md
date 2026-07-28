---
id: dec.artefact-layout-authority
nodes:
  - cairn.kernel.artefacts
status: accepted
date: 2026-07-28
informed_by:
  - res.decision-accumulation-cairn-root
  - res.artefact-filename-drift-audit
supersedes:
  - dec.artefact-organization-and-provenance
  - dec.artefact-filename-rule
related:
  - dec.product-perimeter
  - dec.source-file-never-self
---
# Artefact layout authority: flat type-first folders, id-derived filenames

## Context

Two accepted decisions described one rule in two instalments.
`dec.artefact-organization-and-provenance` (2026-06-26) settled where provenance
artefacts live and how they link, as author-side policy.
`dec.artefact-filename-rule` (2026-07-27) made the filename half mechanically
enforceable and migrated the corpus. The earlier one was anchored on
`cairn.root` alone; the later one named both `cairn.root` and
`cairn.kernel.artefacts`. The artefact registry is where the rule actually
binds.

`res.decision-accumulation-cairn-root` holds the measurement and the precedent.
This decision consolidates the pair, re-anchors it on
`cairn.kernel.artefacts`, and changes no behaviour.

## Decision

### 1. Flat, type-first folders

Scanner-loaded artefacts live flat under `meta/decisions/`, `meta/research/`,
and `meta/sources/`. No subfolders: the loader is non-recursive and silently
ignores a file placed in one. Todos live flat under `meta/todos/` on the same
grounds.

Topical grouping is expressed through slug namespacing in the id and filename,
for example `res.gas-city.analysis` in `meta/research/gas-city.analysis.md`,
never through directory structure. The genesis exception in
`docs/conventions.md` section 9 is exempt across all axes.

`docs/conventions.md` section 10 remains the home of the full normative text,
the shipped error-code inventory, and the per-type link-model fields.

### 2. Filenames derive from the id

1. For decisions, research, and sources, the filename stem equals the artefact
   `id` with its typed prefix stripped: `id: dec.no-orchestrator` lives in
   `meta/decisions/no-orchestrator.md`.
2. Slug namespacing is preserved intact; only the typed prefix and the
   extension mapping change.
3. The typed prefix appears only in the `id:` frontmatter field, never in the
   filename.
4. Todos are the exception: `meta/todos/todo.<slug>.md`. A todo has no `id`
   field, and `cairn todo new <slug>` and `cairn todo set <slug> <status>`
   resolve the slug through that exact path.
5. Both halves are enforced during artefact loading by
   `CAIRN_ARTEFACT_FILENAME_DRIFT` (CA038) at Warning. The whole filename is
   compared against the `id`, so `bar.md` declaring `id: dec.foo` is caught, not
   just an absent prefix.
6. Warning, not Info, is deliberate: `cairn scan --strict` exits non-zero on
   Warning, so drift is gated, and an adopting repository fails its first
   strict scan until its filenames conform.
7. Reviews and contracts are out of scope; contracts are keyed by node rather
   than by typed id. Slug charset is also out of scope: `todo.Bad.md` satisfies
   this rule but is not addressable through `cairn todo set`, which is a
   separate rule and a separate finding, not a filename remediation.
8. Archived material under `meta/changes/archive/` stays at its historical
   paths. No gate resolves prose paths.

### 3. Provenance links

Decisions link up through `informed_by: [...]`, to research or directly to a
source; research is not a required intermediary. Research cites sources through
`sources: [...]`. Sources carry `file:` with `verification` and no `nodes:`
field: a source anchors transitively through the artefacts that cite it.

Unresolved `informed_by` and `sources` links are mechanically detected advisory
warnings and do not fail the gate. Id uniqueness remains author-side policy;
the recommended `CAIRN_ARTEFACT_DUPLICATE_ID` gate and unwired-artefact
detection under a pointer root are still recorded as open in
`docs/conventions.md` section 10. The phrase "mechanically verifiable" is not
applied to a provenance rule until an Error-tier gate exists for it.

### 4. Dark-store triage, as settled

`docs/strongholds/` and `docs/research/` are archived under `archive/`, their
originals preserved as referenceable history. Load-bearing material is promoted
to a native artefact only where it earns it. External or competitor material
stays in the archive and is cited as a `source` when a decision needs its
provenance; it is never inlined as research.

## Rationale

The two decisions were consolidated because the second exists to enforce the
first: read separately, the earlier one still records the filename rule as
ungated policy and still describes todos as node-partitioned at
`meta/todos/<node>/`, which the later decision and the current convention
replaced with a flat `todo.<slug>.md`. A reader who found the earlier decision
first got a stale answer to a question the graph can answer exactly.

Re-anchoring on `cairn.kernel.artefacts` alone follows the same reasoning:
`cairn.root` is the crate boundary module, and this rule binds the artefact
registry that loads and validates the files. The registry is where CA038 is
implemented and where the loader's non-recursive behaviour lives.

## Consequences

- Both named decisions are `superseded`. They keep their provenance and still
  count as provenance coverage for the nodes they name, `cairn.root` included.
- Six live pointers named `dec.artefact-filename-rule` as the authority for
  the rule and its enforcement: `docs/conventions.md` twice, both tracked
  copies of `finding-codes.md` (canonical under `tools/agent-pack/content/`,
  rendered under `.claude/skills/`), the doc comment at
  `src/artefacts/registry/validate/filenames.rs`, and the test contract comment
  at `tests/fixtures_smoke.rs`. Unlike the prose citations the earlier
  consolidations left alone, these are shipped guidance, an enforcement site,
  and a test contract, so they name this decision instead. Remaining mentions
  elsewhere are historical (completed or blocked todos, research, an archived
  fixture body) and are left naming the decision that made the rule.
- Completed work is historical and is not restated as an obligation: the rename
  of the 41 non-conforming files, the removal of the false "matches every
  existing artefact" claim from section 10, and the closing of the spec's
  `meta/` layout question, which now points here.
- `cairn.kernel.artefacts` nets to the same accepted-decision count; `cairn.root`
  sheds two.
