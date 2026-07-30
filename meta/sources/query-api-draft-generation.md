---
id: src.query-api-draft-generation
file: src/query_api/mod.rs
verification: tracked
type: in-repo code read
date: 2026-07-28
---

# Draft generation refuses to run with the summariser disabled

In-repo code inspected while designing
`meta/changes/contract-node-shape-drift/`. The claim below is the state at
`00c212a` on `main`; the path is cited `tracked` and read as it stands, so
re-read the file for the current behaviour.

The draft-generation handler requires a node argument on the request, loads
`SummariserSettings`, and matches on `settings.mode`. `SummariserMode::Disabled`
returns a `QueryError` with code `CAIRN_SUMMARISER_DISABLED` and the remediation
"set summariser.mode to local_command or hosted_api", before any backend is
constructed and before `build_request` resolves the node against the graph.
Obtaining a fresh draft therefore requires a live summariser, whatever the state
of the node's existing drafts.
