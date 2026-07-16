---
node: cairn.kernel.query
status: open
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
