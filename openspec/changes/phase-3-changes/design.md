# Design: Phase 3 Changes

## References

- `docs/spec.md` section 9 for change directories, delta operations, archive order, and rename.
- `docs/spec.md` section 12 for change-aware queries.

## Change Discovery

A change directory SHALL be any direct child of `meta/changes/` containing `proposal.md`, excluding `meta/changes/archive/`. Change IDs SHALL be directory names. Discovery SHALL parse proposal title, optional design file, `dsl.delta`, and mirrored artefact subtrees.

## DSL Delta Parser

`dsl.delta` SHALL support these sections:

- `## ADDED Nodes`
- `## MODIFIED Nodes`
- `## REMOVED Nodes`
- `## RENAMED Nodes`
- `## ADDED Edges`
- `## MODIFIED Edges`
- `## REMOVED Edges`
- `## RENAMED Edges`

Added and modified nodes SHALL use full DSL node declarations. Removed nodes SHALL name IDs. Renamed nodes SHALL declare old and new IDs. Edge operations SHALL reference stable node IDs and descriptions.

## Artefact Operations

Artefacts inside a change directory SHALL mirror the main `meta/` tree. Every changed artefact file SHALL include `operation: added`, `operation: modified`, `operation: removed`, or `operation: renamed`. Renamed artefacts SHALL include `renamed_from`.

The archive engine SHALL reject artefact operations that lack required fields or target missing files.

## Archive Algorithm

`cairn archive <change>` SHALL:

1. Snapshot all files it will mutate.
2. Validate the change directory and all referenced IDs against current truth.
3. Apply `RENAMED`, `REMOVED`, `MODIFIED`, then `ADDED`.
4. Run `cairn scan`.
5. Abort and restore the snapshot if scan reports structural errors or unresolved interface contradictions.
6. Move the change to `meta/changes/archive/YYYY-MM-DD-<change-id>/`.
7. Append an archive event to `.cairn/log.md`.

The implementation SHALL use temp files and atomic rename operations for file writes.

## Rename Command

`cairn rename <old-id> <new-id>` SHALL create `meta/changes/rename-<old-id>-to-<new-id>/`. It SHALL generate a `dsl.delta` with the node rename and edge updates. It SHALL copy every artefact whose frontmatter references `old-id` into the change directory with modified frontmatter referencing `new-id`.

The command SHALL not mutate the main tree. Archive performs the mutation after review.

## Query Semantics

Default queries SHALL read current truth only. `--include-changes` SHALL add proposed additions, modifications, removals, and renames to supported query output with operation labels.

`cairn changes` SHALL show active change IDs, proposal titles, operation counts, and validation status. `cairn show <change>` SHALL show proposal text, DSL delta summary, and artefact operation summary.

## Testing

Tests SHALL cover delta parsing, artefact operation parsing, archive success, rollback on failure, dated archive path creation, log append, rename generation, default-query isolation, and `--include-changes`.
