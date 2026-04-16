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

Single-path nodes SHALL have one target. Path-list nodes SHALL have one target per path. `ContractRole` SHALL be a normalized string identifying which contract obligation a target satisfies. Phase 6 SHALL default every target to `public_api` unless explicit target metadata is added in `cairn.config.yaml`.

Explicit target metadata SHALL use this shape:

```yaml
targets:
  - node: saas.api.auth
    path: crates/auth/src/lib.rs
    language: rust
    contract_role: public_api
```

Each entry SHALL include `node`, `path`, and `contract_role`, and MAY include
`language`. When present, `language` SHALL be one of `rust`, `typescript`,
`python`, or `go` and SHALL take precedence over file-extension detection.
Unknown node/path pairs and unsupported language values SHALL be configuration
errors.

## Language Detection and Dispatch

The scanner SHALL detect target language through explicit config first, then file extensions. Supported languages for Phase 6:

- Rust.
- TypeScript.
- Python.
- Go.

Each language SHALL implement the existing `Reconciler` trait. The shared scanner SHALL dispatch by target language and merge reports into the ontology.

## Interface Hash State

`.cairn/state/interface-hashes.json` SHALL store hashes by node ID and target path. Existing single-hash state from Phase 1 SHALL migrate to the new shape on first scan.

## Public Interface Extraction

Each language reconciler SHALL normalize public interface shapes into a sorted list of records containing `language`, `kind`, `name`, `signature`, and optional `members`. Hashes SHALL be computed from the canonical JSON encoding of this normalized list with deterministic key ordering and no source spans.

Language-specific extraction rules:

- Rust SHALL preserve Phase 1 semantics: public `pub` items exported from the target crate or module, including public functions, structs, enums, traits, type aliases, constants, and public fields or trait methods.
- TypeScript SHALL include exported declarations, re-exports, default exports using the canonical name `default`, exported class public members, exported interfaces, exported types, exported functions, exported constants, and exported enums. Non-exported declarations SHALL be excluded.
- Python SHALL use `__all__` when present. If `__all__` is absent, it SHALL include top-level functions, classes, constants, and variables whose names do not start with `_`; class methods whose names do not start with `_` SHALL be included as members.
- Go SHALL include package-level exported identifiers, exported functions, exported types, exported interfaces, exported struct fields, exported methods, constants, and variables according to Go's uppercase export convention.

All reconcilers SHALL sort records by `kind`, then `name`, then `signature`. Formatting-only changes, comments, private symbols, and source order SHALL NOT affect the interface hash.

## Divergence Rules

When multiple targets claim the same contract role:

- Equal interface shapes produce no finding.
- Different interface shapes produce an interface contradiction unless config marks the asymmetry intentional.
- Intentional asymmetry produces a rationale tension so humans can revisit it.

Intentional asymmetry SHALL be marked only in `cairn.config.yaml` under `multi_target.intentional_asymmetry`:

```yaml
multi_target:
  intentional_asymmetry:
    - node: saas.api.auth
      contract_role: public_api
      targets:
        - crates/auth/src/lib.rs
        - packages/auth-client/src/index.ts
      reason: "The client intentionally exposes a narrowed interface."
```

Each entry SHALL include `node`, `contract_role`, at least two target paths, and a non-empty `reason`. A matching entry SHALL suppress the interface contradiction only for those listed targets and SHALL emit the rationale tension.

## CLI Changes

`get`, `files`, `lint`, `scan`, and JSON output SHALL include target-level state, language, reconciler ID, claimed files, interface hashes, and divergence findings.

## Testing

Tests SHALL cover single-target backwards compatibility, path-list dispatch,
language detection, explicit language override precedence, unsupported language
configuration errors, per-language public interface extraction rules, canonical
hash normalization, per-target hash persistence, migration from old state shape,
divergence contradictions, intentional asymmetry tensions, and target-level
output.
