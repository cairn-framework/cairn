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
dec.simplify-cli-subset-folds already cites a "query JSON schema
versioning convention" that has no artifact; this creates it. Needs a
change proposal (touches the external contract).

## Review note (2026-07-16)

Adversarial backlog review verified the json!() saturation (~40 ad-hoc literals across src/query_api; only MapSnapshot is a proper serde struct) but REFUTED the keystone claim: next-recommended-unification step 1, change-read-surface, symbol-locate-query, and bundle-real-gates do not require this landing first. Disposition: defer (L effort, preventive hygiene); pursue after the converged S/M wins, not as a prerequisite.

## Resolution

2026-07-17: Component schemas for `MapSnapshot`, `Finding`, and the shared
`WorkItem` projection landed, with schema validation/drift tests, the
`TOOL_REGISTRY` response-label allowlist gate, integration-contract docs, and
version-3 wire snapshots. The JSON-envelope increment remains open: registered
`StatusResponse`, `RemediateResponse`, and other response envelopes remain on
the explicit unschema'd allowlist. This todo stays `in_progress` until
full-envelope schemas land and the allowlist is burned down.

## Status

2026-07-17: Component schemas for `MapSnapshot`, `Finding`, and the shared
`WorkItem` projection landed, along with validation/drift tests and the
`TOOL_REGISTRY` response-label allowlist gate. The JSON envelope increment
remains open: the registered `StatusResponse`, `RemediateResponse`, and
other response envelopes still use the explicit unschema'd allowlist rather
than committed full-envelope schemas. Continue with envelope schema files
and burn down the allowlist before marking this todo done.
