# Design: Phase 2 Artefacts

## References

- `docs/spec.md` section 3 for provenance and authority chains.
- `docs/spec.md` section 8 for artefact schemas and integrity rules.
- `docs/spec.md` section 12 for artefact query commands.

## Artefact Registry

The Phase 1 contract loader SHALL become one entry in a typed artefact registry. The registry SHALL expose a common interface:

```rust
pub trait ArtefactLoader {
    fn artefact_type(&self) -> ArtefactType;
    fn load(&self, request: ArtefactLoadRequest<'_>) -> Result<Vec<ArtefactRecord>, ArtefactError>;
}
```

Each loader SHALL parse Markdown frontmatter into typed Rust structs and return findings with one of these classes:

- Structural error.
- Interface contradiction.
- Rationale tension.
- Informational finding.

Phase 2 SHALL use structural errors and rationale tensions; interface contradictions remain driven by Phase 1 contract fingerprints.

## Artefact Schemas

The implementation SHALL support:

- `Todo`: `node`, `status`, `created`, optional `satisfies`.
- `Decision`: `id`, `nodes`, `status`, `date`, `revisited`, `revisit_triggers`, `informed_by`, `supersedes`, `refines`, `related`.
- `Review`: `node`, `review_type`, `date`, `reviewer`, optional `related_change`.
- `Research`: `id`, `nodes`, `date`, `sources`, optional `tags`.
- `Source`: `id`, `file`, optional `sha256`, `verification`, `type`, `date`, optional `tags`, `description`.

Contracts remain unchanged from Phase 1.

## Integrity Rules

The scanner SHALL enforce these rules:

- Todo references exactly one valid node ID; orphan todos are warnings.
- Decision references at least one valid node ID; decisions whose nodes are all deleted require reassignment, archive, or explicit orphan status.
- Decision `supersedes` targets exist and have `superseded` status.
- Review references one valid node ID and uses a valid subtype.
- Research references at least one valid node ID and at least one valid source ID.
- Source is referenced by at least one research artefact or decision, otherwise it is a warning.
- `verified` source files have a non-empty SHA-256 that matches the local file.
- `external` sources have a URL-valued `file`.
- `unverified` sources surface rationale tensions.

## Query Semantics

Add these commands:

- `cairn todos <node> [--status open]`
- `cairn decisions <node> [--status accepted]`
- `cairn research <node>`
- `cairn sources <node>`
- `cairn rationale <node>`
- `cairn status`

`rationale` SHALL return accepted decisions for the node and direct neighbours, plus linked research and sources. `sources` SHALL traverse from node decisions and research to source records. `status` SHALL compose active changes, open todos, and recent `.cairn/log.md` entries; before Phase 3 it SHALL return an empty active changes list.

`neighbourhood` SHALL include contracts and accepted decisions by default. Todos, research, reviews, deprecated decisions, and active changes SHALL require explicit include flags.

## Testing

Tests SHALL include schema parsing, required field failures, cross-reference validation, SHA-256 mismatch, external URL validation, query filtering, rationale traversal, and status composition.
