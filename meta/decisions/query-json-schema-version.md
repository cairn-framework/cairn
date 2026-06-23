---
id: dec.query-json-schema-version
nodes:
  - cairn.kernel.query
status: accepted
date: 2026-06-23
---

# Query-API JSON envelopes carry a uniform schema_version

## Context

The `query_api` command surface (consumed by the CLI `--json` flag and the MCP
server) emitted inconsistent JSON. `cairn islands --json` carried a top-level
`schema_version` (sourced from the map-domain `IslandsResponse` and
`ISLANDS_SCHEMA_VERSION`), while `order`, `contract`, `context`, `status`,
`lint`, `dependents`, and every other command emitted a `data` payload with no
version at all. A JSON consumer could branch on the islands version but had no
version to branch on for any other command.

The prior session handoff flagged this as the next candidate and marked it a
maintainer decision, because standardizing it is a user-facing output-contract
change. The maintainer chose to add `schema_version` everywhere.

## Decision

Every `query_api` command's JSON `data` payload carries a top-level
`schema_version` field, currently `1`. The stamp is applied at a single choke
point in `query_api::execute`: after `execute_data` returns, the data object is
stamped with `query_api::SCHEMA_VERSION`. Because the CLI prints `data`
directly and the MCP envelope wraps `data`, both surfaces share one versioned
contract from one constant.

The redundant per-handler stamp in `islands_json` was removed so the universal
stamp is the single source of truth. `ISLANDS_SCHEMA_VERSION` remains a
map-layer library concept (still constructed and tested on `IslandsResponse`)
but no longer drives the CLI islands envelope version. The live islands output
is byte-identical to before (its value was already `1`).

## Rationale

Versioning the `data` payload at one choke point, with one constant, beats
per-command stamps: it cannot drift, every command is covered automatically
(including future ones and the change-directory tools), and there is exactly one
place to bump when the contract changes. Per-command divergent versions were
explicitly rejected: the goal is a uniform contract a consumer branches on, not
a matrix of per-command versions.

Only JSON objects are stamped; every current command returns an object, so the
stamp is universal in practice.

## Consequences

- Every `query_api` `--json` command output now contains `"schema_version": 1`
  (serialised in alphabetical key position). Bumping the wire contract means
  bumping `query_api::SCHEMA_VERSION`.
- The webui HTTP surface (`src/ui/api.rs`) is a separate JSON surface with its
  own `SCHEMA_VERSION` and its own convention (only `/api/meta` and
  `/api/status` carry a version today). Standardizing `schema_version` across
  all webui `/api/*` endpoints is a distinct unit of work, not taken here.
- The `cairn export` envelope and the summariser request/response wire schemas
  keep their own independent version constants; they are not command envelopes.
