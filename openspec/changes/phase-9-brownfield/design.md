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

Deterministic fallback heuristics SHALL apply before summariser naming:

- Source roots SHALL be directories containing at least three source files in supported languages after ignore rules are applied.
- Candidate directory depth SHALL be limited to four levels below the repository root unless explicitly configured.
- A directory SHALL become a module candidate when it contains at least three source files or at least two source files and one internal import edge to another candidate.
- File clusters SHALL be grouped by nearest common directory when no stronger candidate exists.
- Coupling score SHALL be `(internal_imports + 1) / (external_imports + 1)`. Candidates with score `>= 2.0` are high confidence, `>= 1.0` are medium confidence, and below `1.0` are low confidence.
- Observed edges SHALL be emitted when there are at least two import observations from one candidate to another, or one public API reference with high confidence.
- Summariser samples SHALL include at most five files per candidate and at most 4,000 bytes per file, preferring public interface files before implementation files.

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

## MCP Tools

Phase 9 SHALL register brownfield commands in the shared query tool registry so `cairn-mcp` exposes them through MCP when mutating tools are enabled. `cairn_init_from_code` and `cairn_refine` SHALL be `mutating` tools because they create change directories and write proposed artefacts.

## Testing

Tests SHALL use fixture repositories. Coverage SHALL include deterministic candidate thresholds, depth limits, coupling score bands, edge thresholds, sample byte limits, disabled summariser fallback, deterministic fake summariser output, init change generation, refine delta generation, force behavior, archive compatibility, and MCP registry exposure for brownfield mutation tools.
