# UI Maintenance Contract

The graph explorer is a query consumer. It must render data obtained through
the embedded `/api/*` query bridge and must not introduce a second source of
truth for graph, artefact, status, or lint data.

## Schema Version

`GET /api/meta` returns `schema_version`. The bundled UI tracks the schema
version it was built for and shows a non-blocking warning when the server
reports a newer version. The UI must continue rendering with compatible fields.

## Forward Compatibility

- Unknown JSON fields are ignored.
- Missing expected fields render as `Not available`.
- Unknown artefact types use the generic artefact template: title,
  frontmatter fields, and body text.
- Query bridge responses must preserve the typed response shapes used by the
  CLI JSON output where a matching CLI command exists.

## Phase Addenda

Phase 3 must include a UI deliverable section for temporal navigation. The
deliverable must specify how proposed-vs-current graph state is displayed,
either as a split view or an overlay.

Phase 7 must include a UI deliverable section for the transport adapter switch
from embedded query bridge calls to the MCP transport where MCP is preferred.

Any future phase that changes `CairnQuery` or `CairnResponse` schema shape must
include a compatibility note in its acceptance criteria. The note must confirm
that existing graph explorer rendering is unaffected or specify the required UI
change.
