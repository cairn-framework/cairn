---
node: cairn.kernel.artefacts
status: open
created: 2026-07-28
---

# Source Tracked Verification Mode

Implement the `tracked` source verification mode ruled in
`dec.source-tracked-verification`: local path, no hash, resolution and
containment checked, no perpetual advisory.

## Scope

- `SourceVerification` in `src/artefacts/registry/types.rs` gains `Tracked`.
- `parse_source_verification` in `src/artefacts/registry/parse.rs:71` accepts
  `"tracked"`. An unknown value still raises
  `CAIRN_SOURCE_VERIFICATION_INVALID`.
- `source_verification` in `src/query_api/serialise.rs:320` gains
  `Tracked => "tracked"`. The match is exhaustive, so the build fails without
  it, and the string is a wire value.
- `validate_sources` in `src/artefacts/registry/validate/mod.rs` grows a
  `Tracked` arm: before touching the filesystem, require at least one
  `Component::Normal` and reject anything else apart from an optional leading
  `./`, which rules out absolute paths, `..`, and a bare `./`; resolve it
  against the root with a
  metadata probe rather than `fs::read` (`src.summariser-accept-path` cites a
  directory, which `fs::read` rejects), and confirm the canonical result stays
  under the root so a symlink cannot leave the tree. `src/cli/commands/wire.rs:103-129`
  is the existing containment reference. Raise `CAIRN_SOURCE_READ_FAILED` at
  Error when the path does not resolve or escapes, emit nothing when it does,
  and generalise that code's description in `docs/registries/error-codes.md`
  (CA031) and its entry in the "Deterministic enforcement" list at
  `docs/conventions.md:441` from verified-source integrity to source path
  resolution. It has no `[findings.codes]` entry in
  `docs/design-system/copy.toml` to update.
- A `tracked` source declaring `sha256` raises the new
  `CAIRN_SOURCE_SHA256_UNEXPECTED` at Error, registered as CA040 (CA039 went
  to `CAIRN_DECISION_ACCUMULATION`) in `docs/registries/error-codes.md` with a
  `[findings.codes]` entry in `docs/design-system/copy.toml`, per clause 5. It
  is a distinct code, not a reuse of `CAIRN_SOURCE_VERIFICATION_INVALID`, and
  it joins the source-issue list in
  `src/query_api/handlers/remediate.rs:155-160` so it produces the `fix_sources`
  action rather than falling through. That list is reachable from the MCP
  `cairn_remediate` tool.
- `docs/spec.md`: amend the whole source-verification block per clause 6 of the
  decision (the immutability claim at :484, the enum comment at :491, the
  per-state list at :505-507, the integrity rule at :509, the freshness rule at
  :511). Re-grep line numbers afterwards and check individually the
  `docs/registries/spec-rules.md` rows the edit moved. Several source rows are
  already mis-anchored independently of this change; leave those to their own
  unit rather than shifting them here.
- `docs/registries/spec-rules.md` gains a row per clause 7 for each rule this
  makes Designed: tracked-path resolution, tracked-path containment, and
  hash-absence.
- `docs/conventions.md` section 10: add the mode to the `verification` field
  list and the flexibility bullets, and rewrite line 425 so the missing-mode
  clause goes and the sparing-citation advice attaches to `verified`.
- `docs/artefacts.md:108` and the shipped agent-pack references under
  `tools/agent-pack/content/skills/cairn-dev/references/` (`artefact-schemas.md`,
  `finding-codes.md`) teach the closed set of three modes and name only
  `verified` or `external` as resolutions. They ship to adopting repositories,
  so they move with the schema or they keep teaching the remediation this
  decision removes. `tools/agent-pack/content/` is canonical but
  `src/cli/commands/pack_assets.rs` compiles the rendered
  `.claude/skills/cairn-dev/references/` copies with `include_str!`, so re-render
  into `.claude/` in the same change or `cairn init` keeps shipping the old
  schema.
- Move `meta/sources/query-api-draft-generation.md` and
  `meta/sources/summariser-accept-path.md` to `verification: tracked` and trim
  the paragraphs in their bodies that exist only to explain the deliberate
  `unverified` state.
- `dec.source-file-never-self` binds every verification value, and its dependent
  `todo.source-self-reference-finding` writes that coverage as "all three"
  values. If it has landed, extend its check and tests to `Tracked`; if not,
  amend its Acceptance to four. A tracked self-pointer resolves, so the
  existence check alone would pass it.

## Depends on

- `dec.source-tracked-verification` reaching `status: accepted`. It changes a
  spec invariant and the artefact schema every adopting repository shares, so a
  loop iteration may not ratify it. Satisfied 2026-07-29 by acceptance of
  `dec.source-tracked-verification` (maintainer ratification, sheet of record
  PR #528, row W3).

## Acceptance

- The existing case table in `src/artefacts/registry/parse.rs` (the one at
  :229-247, which calls `parse_source_verification(value, path, set)`) includes
  `("tracked", Some(SourceVerification::Tracked))` and produces no finding.
- A `tracked` source whose `file:` resolves produces no finding. Covered for
  both a file path and a directory path, since the directory case is what rules
  out `fs::read`.
- A `tracked` source produces `CAIRN_SOURCE_READ_FAILED` at Error for each of:
  a missing path, an absolute path, a `..` traversal, a bare `./`, and a
  symlink inside the tree whose target is outside it. A leading `./` before a
  real path is accepted. Tests live in
  `src/artefacts/registry/validate/tests.rs`.
- A `tracked` source with a populated `sha256` produces
  `CAIRN_SOURCE_SHA256_UNEXPECTED` at Error, and `cairn remediate` emits the
  `fix_sources` action for it rather than nothing.
- An unknown `verification` value still produces
  `CAIRN_SOURCE_VERIFICATION_INVALID`.
- A source serialised through the query API reports `"tracked"`, covered by a
  serialiser or wire test.
- `verified` and `unverified` behaviour is unchanged, including
  `CAIRN_SOURCE_SHA256_MISSING` and the Info advisory.
- `docs/registries/error-codes.md` and `docs/design-system/copy.toml` register
  `CAIRN_SOURCE_SHA256_UNEXPECTED`, and CA031 plus the deterministic-enforcement
  list at `docs/conventions.md:441` describe source path resolution rather than
  verified-source integrity.
- `docs/spec.md`, `docs/conventions.md`, `docs/artefacts.md`, both canonical
  agent-pack references, and their rendered `.claude/skills/cairn-dev/references/`
  counterparts describe `tracked` consistently, and
  `docs/registries/spec-rules.md` carries the three clause 7 rows at anchors
  that match their rule text.
- Both affected source records read `verification: tracked`, no longer carry
  their deliberate-`unverified` explanations, and `cairn lint` on this
  repository reports no `CAIRN_SOURCE_UNVERIFIED` finding.
