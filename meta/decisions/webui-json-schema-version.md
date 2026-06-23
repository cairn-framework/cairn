---
id: dec.webui-json-schema-version
nodes:
  - cairn.ui
status: accepted
date: 2026-06-23
---

# Webui /api/* JSON responses carry a uniform schema_version

## Context

The webui HTTP surface (`src/ui`) emitted inconsistent JSON. Only `/api/meta`
and `/api/status` carried a top-level `schema_version`; `/api/graph`,
`/api/lint`, `/api/blueprint`, `/api/node/*` (and its `contract`, `decisions`,
`todos`, `research`, `sources`, `beads`, `rationale` suffixes), `/api/depends/*`,
and `/api/dependents/*` carried no version. A consumer could branch on the
handshake endpoints but had no version to branch on for the data endpoints.

`dec.query-json-schema-version` standardized the sibling `query_api` surface and
explicitly scoped the webui out as "a distinct unit of work, not taken here",
flagging it as a maintainer decision because handshake-only versioning could be
deliberate. The maintainer chose to standardize: stamp `schema_version` on every
`/api/*` response. This decision completes that deferred unit; it does not
contradict `dec.query-json-schema-version`.

## Decision

Every webui `/api/*` JSON response carries a top-level `schema_version` field,
currently `1`, stamped at a single choke point: the `server::json` `Response`
constructor. `json` is the sole builder of `application/json` responses for the
API surface, so a `versioned()` helper splices `schema_version` as the first key
of the (always-object) body inside `json`. The redundant inline stamps in
`meta_json` and `status_json` were removed so the choke point is the single
source of truth.

The webui keeps its own `ui::SCHEMA_VERSION` constant, independent of
`query_api::SCHEMA_VERSION`: these are separate wire surfaces with separate
versioning lifecycles, and coupling them would force a lockstep bump where none
is warranted.

## Rationale

Stamping at the `json` constructor, with one constant, beats per-handler stamps:
it cannot drift, every endpoint is covered automatically (including future ones
and error envelopes), and there is exactly one place to bump when the contract
changes. This mirrors the `query_api` choke-point philosophy on a separate
surface.

Only the top-level envelope is versioned. Nested objects (the node records
inside `/api/graph`) are built by `node_json` as plain strings that never pass
through `json`, so they stay unversioned: the version describes the response
contract, not every embedded record.

Error envelopes (404/500 `finding` bodies, the blueprint read-error body) flow
through `json` too and so are versioned uniformly, which is desirable: a consumer
parsing an error still gets a version to branch on. Plain-text fallbacks
(`text(404, ...)`) are not JSON and carry no version.

## Consequences

- Every webui `/api/*` JSON response now contains `"schema_version": 1`
  (serialised first, sorted alphabetically by the snapshot harness). Bumping the
  wire contract means bumping `ui::SCHEMA_VERSION`.
- `server::json` is now coupled to the API version contract. This is acceptable
  because `json` is `pub(super)` and is used exclusively to build `/api/*`
  responses; there is no non-API JSON consumer of it.
- The `wire_format_snapshots` golden fixtures were regenerated to include the
  stamp, and the test now asserts every endpoint carries a numeric
  `schema_version`, so a future endpoint added without the stamp fails the gate.
- `query_api::SCHEMA_VERSION` and `ui::SCHEMA_VERSION` remain independent
  constants for independent surfaces; they are not required to move together.
