---
id: src.query-api-draft-generation
file: src/query_api/mod.rs
verification: unverified
type: in-repo code read
date: 2026-07-28
---

# Draft generation refuses to run with the summariser disabled

In-repo code inspected while designing
`meta/changes/contract-node-shape-drift/`. Registered `unverified` on purpose:
this is a live source file under active development, so hash-pinning it would
turn any ordinary edit into a structural error. The claim below is the state at
`00c212a` on `main`, and a reader should re-read rather than trust the hash.

The draft-generation handler requires a node argument on the request, loads
`SummariserSettings`, and matches on `settings.mode`. `SummariserMode::Disabled`
returns a `QueryError` with code `CAIRN_SUMMARISER_DISABLED` and the remediation
"set summariser.mode to local_command or hosted_api", before any backend is
constructed and before `build_request` resolves the node against the graph.
Obtaining a fresh draft therefore requires a live summariser, whatever the state
of the node's existing drafts.
