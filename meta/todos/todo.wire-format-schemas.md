---
node: cairn.kernel.query
status: in_progress
created: 2026-07-16
---

# Wire Format Schemas

Externally consumed wire formats have version stamps but no shape
definitions: `docs/integration-contract.md` promises envelope stability
with no mechanism behind it; SCHEMA_VERSION is stamped
(`src/query_api/mod.rs:54`, `src/scanner/snapshot.rs:17`) but nobody can
tell when to bump it; the registry's response_schema fields
(`src/query_api/registry.rs`) are bare string labels advertised to
clients via ui_meta, and only 2 of about 30 named response types exist as
Rust structs, the rest ad-hoc `json!` literals in serialise.rs and
handlers/. A field rename ships silently; existing tests pin individual
fields, not shapes. Consumers are real: MCP clients, the webui SPA,
harness/capture-fixtures.mjs, LSP, and the jq-based CI patterns in the
integration contract.

Scope to what external consumers parse, in increments:
1. `schemas/map.schema.json` for MapSnapshot (already proper serde
   structs, so schemars works with no refactor) plus a test that builds
   the dogfood snapshot and validates serialized output against the
   schema (jsonschema crate as dev-dependency only).
2. `schemas/finding.schema.json` for the Finding wire shape (shared by
   map.json, lint --json, and watch events), then the JSON envelope.
3. A registry test requiring every response_schema label in TOOL_REGISTRY
   to resolve to a schema file or an explicit allowlist entry, extending
   the single-registry discipline instead of creating a parallel
   structure.

Motivation: `res.a2ui-analysis` finding 8. Note
dec.query-json-schema-version carries the query JSON schema
versioning convention whose shape artefact this todo creates. Needs a
change proposal (touches the external contract).

## Review note (2026-07-16)

Adversarial backlog review verified the json!() saturation (~40 ad-hoc literals across src/query_api; only MapSnapshot is a proper serde struct) but REFUTED the keystone claim: next-recommended-unification step 1, change-read-surface, symbol-locate-query, and bundle-real-gates do not require this landing first. Disposition: defer (L effort, preventive hygiene); pursue after the converged S/M wins, not as a prerequisite.

## Resolution

2026-07-17: Component schemas for `MapSnapshot`, `Finding`, and the shared
`WorkItem` projection landed, with schema validation/drift tests, the
`TOOL_REGISTRY` response-label allowlist gate, integration-contract docs, and
version-3 wire snapshots. The JSON-envelope increment and first allowlist slice
landed `schemas/envelope.schema.json` plus full schemas for `StatusResponse`
and `RemediateResponse`. This todo stays `in_progress` while the remaining
40 response labels are burned down.

## Status

2026-07-17: Added `schemas/envelope.schema.json` for the query API MCP
envelope, requiring `data.schema_version` while permitting heterogeneous
tool-specific data. Added full post-dispatch schemas and validation tests for
`StatusResponse` and `RemediateResponse`, and refactored both handlers to
serialise typed structs without changing wire snapshots. The allowlist now has 40 remaining labels: NodeResponse, NeighbourhoodResponse, ContractResponse, DocstringResponse, FilesResponse, BundleResponse, DependencyResponse, OrderResponse, IslandsResponse, FrontierResponse, GraphResponse, LintResponse, RationaleResponse, TodosResponse, DecisionsResponse, ResearchResponse, SourcesResponse, ChangesResponse, ShowChangeResponse, HookReport, HealthResponse, UiServerResponse, ScanResponse, ArchiveResponse, RenameResponse, InitResponse, ContextResponse, InitFromCodeResponse, RefineResponse, DraftsResponse, DraftShowResponse, DraftDiscardResponse, DraftEditResponse, DraftAcceptResponse, SummariseResponse, WatchResponse, UiMetaResponse, BlueprintResponse, BeadsResponse, and LocateResponse. Continue burning it down before marking this todo done.

2026-07-29: The Finding wire is no longer one shared shape.
`dec.loop-selection-deferred-findings` added `deferred_by` to the query
findings wire (`lint --json` / `scan --json`, hand-built in
`query_api::serialise::findings_json`, `schema_version` 5), while the
serde-derived consumers (`map.json`, watch events) deliberately keep
`serde(skip)` and omit the field. `schemas/finding.schema.json` documents the
serde-derived shape only; the `LintResponse` / `ScanResponse` labels in the
burn-down must model the query findings entry as that shape plus a required
nullable `deferred_by`, not by reusing the component schema unchanged.

2026-07-29 (later): `dec.loop-selection-strict-green-fold` added a top-level
`strict_green` boolean to the lint/scan `data` payload, computed by the shared
strict predicate (`map::graph::strict_green`); query and webui envelopes are
now `schema_version` 6. The `LintResponse` / `ScanResponse` labels in the
burn-down must model that required boolean alongside the findings array; the
envelope schema itself is unchanged (tool-specific data properties stay open).

2026-07-29 (parked classification): `todo.lint-selection-folding` item 1a added
a required nullable `parked_by` to the query findings wire alongside
`deferred_by` (the `blocked` todo whose `defers:` reference parks a live Info
finding, or null); query and webui envelopes are now `schema_version` 7, and
`schemas/envelope.schema.json` pins the field on `EnvelopeFinding`. The
`LintResponse` / `ScanResponse` labels in the burn-down must model the query
findings entry with both required nullable fields; the serde-derived component
shape (`schemas/finding.schema.json`) still omits both by design.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It preserves the wire-format contract while campaign units serialise against it.
