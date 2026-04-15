# Design: Phase 6 Multi-Target and Languages

## References

- `docs/spec.md` section 7 for path lists.
- `docs/spec.md` section 10 for multi-target interface divergence.
- `docs/spec.md` section 16 for resolved multi-target scope.

## Target Model

Every node path SHALL become a `Target`:

```rust
pub struct Target {
    pub node_id: NodeId,
    pub path: Utf8PathBuf,
    pub language: Language,
    pub reconciler_id: ReconcilerId,
    pub contract_role: ContractRole,
}
```

Single-path nodes SHALL have one target. Path-list nodes SHALL have one target per path.

## Language Detection and Dispatch

The scanner SHALL detect target language through explicit config first, then file extensions. Supported languages for Phase 6:

- Rust.
- TypeScript.
- Python.
- Go.

Each language SHALL implement the existing `Reconciler` trait. The shared scanner SHALL dispatch by target language and merge reports into the ontology.

## Interface Hash State

`.cairn/state/interface-hashes.json` SHALL store hashes by node ID and target path. Historical single-hash state from Phase 1 SHALL migrate to the new shape on first scan.

## Divergence Rules

When multiple targets claim the same contract role:

- Equal interface shapes produce no finding.
- Different interface shapes produce an interface contradiction unless an artefact or config marks the asymmetry intentional.
- Intentional asymmetry produces a rationale tension so humans can revisit it.

The implementation SHALL document the intentional-asymmetry marker it supports.

## CLI Changes

`get`, `files`, `lint`, `scan`, and JSON output SHALL include target-level state, language, reconciler ID, claimed files, interface hashes, and divergence findings.

## Testing

Tests SHALL cover single-target backwards compatibility, path-list dispatch, language detection, per-target hash persistence, migration from old state shape, divergence contradictions, intentional asymmetry tensions, and target-level output.
