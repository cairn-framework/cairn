# Design: source-tracked-verification-mode

## Approach

Follow `dec.source-tracked-verification` clauses 1 to 7 exactly; the decision
owns the semantics and this design only places them in code. The enum gains a
variant, so every exhaustive match breaks at compile time until each surface
(parse, serialise, validate) gains its arm. Validation probes with metadata
(`symlink-walk containment` in the style of `src/cli/commands/wire.rs:103-129`
plus canonicalisation), never `fs::read`, because `src.summariser-accept-path`
cites a directory.

Path rule (decision clause 1): at least one `Component::Normal`, an optional
leading `./` (CurDir) accepted, everything else (absolute roots, `..`, a bare
`./`) rejected before touching the filesystem. Then resolve against the root
and require the canonical path to stay under the canonical root, so a symlink
cannot leave the tree. Failures reuse `CAIRN_SOURCE_READ_FAILED` (CA031),
whose registry description generalises from verified-source integrity to
source path resolution.

Registry status ruling: the three clause 7 rows land in the Enforced rules
table with status `enforced`. The registry's status vocabulary is `enforced`,
`pending`, `declared`; "makes Designed" in the decision selects which rules
get rows, not a status value, and the registry's own semantics say a built
rule carries its code at `enforced`. Marking built rules `pending` would be a
false record.

## Changes

ADDED:
- `SourceVerification::Tracked` variant in `src/artefacts/registry/types.rs`.
- `Tracked` arm in `validate_sources` (`src/artefacts/registry/validate/mod.rs`)
  with a `validate_tracked_source` helper: lexical component check, metadata
  probe, canonical containment, `sha256` rejection.
- `CAIRN_SOURCE_SHA256_UNEXPECTED` finding code, registered as CA040 in
  `docs/registries/error-codes.md`, with a `[findings.codes]` entry in
  `docs/design-system/copy.toml`, and routed into the source-issue list in
  `src/query_api/handlers/remediate.rs` so it yields `fix_sources`.
- Three Enforced rows in `docs/registries/spec-rules.md`: tracked-path
  resolution, tracked-path containment, hash-absence.
- Unit tests in `src/artefacts/registry/validate/tests.rs` and a parse case in
  `src/artefacts/registry/parse.rs` tests; a serialiser case for `"tracked"`.

MODIFIED:
- `parse_source_verification` accepts `"tracked"`.
- `source_verification` in `src/query_api/serialise.rs` maps
  `Tracked => "tracked"`.
- `docs/spec.md` source-verification block (now at :489-:516): immutability
  claim, enum comment, per-state list, integrity rule, freshness rule, per
  decision clause 6. Rows in `docs/registries/spec-rules.md` that this edit
  moves are re-checked individually.
- `docs/conventions.md` section 10: `verification` field list, flexibility
  bullets, line 425 rewrite (sparing-citation advice attaches to `verified`),
  and the CA031 description in the Deterministic enforcement list.
- `docs/artefacts.md:108` and both canonical agent-pack references
  (`artefact-schemas.md`, `finding-codes.md`), re-rendered into
  `.claude/skills/cairn-dev/references/` (byte-identical copies today, so the
  render is a copy).
- `meta/sources/query-api-draft-generation.md` and
  `meta/sources/summariser-accept-path.md`: `verification: tracked`, bodies
  trimmed of the deliberate-`unverified` paragraphs.
- `meta/todos/todo.source-self-reference-finding.md`: Acceptance wording
  "all three" becomes "all four" (its check has not landed).

REMOVED:
- The deliberate-`unverified` explanation paragraphs in the two source bodies.

RENAMED:
- None.
