---
node: cairn.provenance
---

# Contract: cairn.provenance

## Purpose
Typed schemas and readers for the per-archived-change trace sidecar (`.cairn-trace.json`), the immutable AI-provenance record introduced in phase 7.6. The cairn library defines the schema and reads it; an external producer (orchestrator or another backend) writes it. Holds only immutable trace records; the mutable suggested-edges queue lives in the `suggested_edges` module.

## Public interface
- `mod.rs`: re-exports the trace surface: `TraceSidecar`, `StageRecord`, `TraceStage`, `TraceError`, `TRACE_SIDECAR_VERSION`, and `read_sidecar`.
- `trace.rs`:
  - `TRACE_SIDECAR_VERSION: u32` (currently `1`), the wire schema version.
  - `TraceStage`: enum of `Propose`, `Apply`, `Accept`, `Archive`, serialised lowercase.
  - `StageRecord`: per-stage payload with optional `model_id`, `tokens_in`, `tokens_out`, `error_message`, plus required `latency_ms`, `success`, `started_at`, `ended_at`.
  - `TraceSidecar`: top-level payload carrying `version`, `phase`, a `BTreeMap<TraceStage, StageRecord>` of stages, and a reserved untyped `prompts` vector.
  - `TraceError`: `Io`, `Parse`, `UnsupportedVersion { found, expected }`, implementing `Display` and `std::error::Error`.
  - `read_sidecar(path) -> Result<TraceSidecar, TraceError>`: reads and parses a sidecar from disk.

## Invariants
- Read-only over the sidecar: this module never writes trace files; the producer owns writing.
- `read_sidecar` rejects any sidecar whose `version` exceeds `TRACE_SIDECAR_VERSION` with `UnsupportedVersion`.
- Token and model fields are `Option` so backends that omit them round-trip cleanly; `#[serde(default)]` keeps deserialisation tolerant of absent fields.
- Timestamps (`started_at`, `ended_at`) are RFC 3339 UTC strings.
- `prompts` is reserved (always empty today) and stored untyped so future producers evolve without a version bump.

## Dependencies
Leaf node: no outgoing blueprint edges. Depends only on external crates `serde` and `serde_json` plus `std` (filesystem and `BTreeMap`).

## Tests
Unit tests in `trace.rs` under `#[cfg(test)]` (lines ~123 onward) cover sidecar parsing, version gating (rejecting newer versions), I/O and parse error paths, and round-tripping records with optional token and model fields absent.
