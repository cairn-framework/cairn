# Design: Phase 9 Brownfield Extraction

## References

- `docs/spec.md` section 15 for brownfield approach.
- `docs/spec.md` section 12 for `init --from-code` and `refine`.
- Phase 3 for change directory archive semantics.
- Phase 8 for summariser backend and draft safety.

## Candidate Extraction

The reconciler SHALL scan the codebase and produce structural candidates:

- Top-level source roots.
- Major subdirectories with cohesive files.
- File clusters with strong internal coupling.
- Observed dependency edges between candidates.

Candidates SHALL include confidence and evidence paths. Low-confidence candidates SHALL remain in generated output but be marked for review.

## Summariser Role

The summariser SHALL name and describe candidates, suggest tags, and draft stub contract content. It SHALL receive structural candidates and bounded code samples, not the entire repository. If the summariser is disabled, Cairn SHALL generate mechanical names and descriptions from paths.

## Init Flow

`cairn init --from-code` SHALL create `meta/changes/brownfield-init/` containing:

- `proposal.md`.
- `dsl.delta` with added nodes and edges.
- Stub contracts under mirrored `contracts/`.
- Optional generated research notes explaining extraction evidence.

The command SHALL fail if `meta/changes/brownfield-init/` already exists unless `--force` is provided.

## Refine Flow

`cairn refine` SHALL compare current code reality with current DSL and produce a new change directory containing only proposed additions, removals, renames, or modifications. It SHALL not replace the whole DSL.

## Testing

Tests SHALL use fixture repositories. Coverage SHALL include disabled summariser fallback, deterministic fake summariser output, init change generation, refine delta generation, force behavior, and archive compatibility.
