# Design: Phase 5 Edge Validation and Docstrings

## References

- `docs/spec.md` section 10 for edge divergence and docstring drift.
- `docs/spec.md` section 12 for `cairn docstring`.
- `docs/spec.md` section 16 for resolved edge and docstring design choices.

## Semantic Dependency Extraction

The Rust code reconciler SHALL extend its Tree-sitter pass to collect observed dependency candidates:

- `use` paths.
- `mod` declarations.
- Fully-qualified public item references when Tree-sitter captures them reliably.
- Intra-workspace path relationships resolved through claimed module paths.

The reconciler SHALL map observed dependencies to Cairn node IDs by file ownership and symbol origin when available. Ambiguous dependencies SHALL be reported as low-confidence observations and SHALL NOT create structural errors.

## Edge Divergence

The scanner SHALL compare declared blueprint edges with observed dependencies:

- Declared edge missing from observed dependencies: rationale tension.
- Observed dependency missing from declared edges: rationale tension.
- Ambiguous observed dependency: informational finding unless it becomes resolvable.

Edge divergence SHALL NOT block hooks by default.

## Docstring Fact Extraction

The reconciler SHALL extract authored module-level docstrings and parse lightweight Cairn fact lines when present:

- Module name.
- Declared dependency IDs.
- Tags.
- Contract path or section reference.

Fact parsing SHALL be deliberately narrow. Free-form prose SHALL remain free-form and SHALL NOT be linted for semantic truth.

Supported fact lines SHALL be case-sensitive and SHALL use these exact keys after doc-comment marker stripping:

```text
Cairn-ID: saas.api.auth
Cairn-Name: JWT authentication
Cairn-Depends: saas.db
Cairn-Depends: saas.crypto
Cairn-Tags: auth, api
Cairn-Contract: ./meta/contracts/api/auth.md#Public interface
```

The Rust extractor SHALL inspect module-level inner doc comments (`//!`) in `lib.rs`, `main.rs`, and `mod.rs`, plus outer doc comments (`///`) immediately attached to `mod` declarations. Multiple `Cairn-Depends` lines SHALL be allowed and combined. `Cairn-Tags` SHALL parse comma-separated tag names after trimming ASCII whitespace. Unknown Cairn fact keys SHALL produce informational findings. Unknown node IDs in `Cairn-ID` or `Cairn-Depends` SHALL produce docstring drift rationale tensions with source spans.

## Docstring Command

`cairn docstring <node> [--language <lang>]` SHALL emit a template grounded in:

- Node ID, name, and description.
- Declared dependencies.
- Tags.
- Contract section headings.

Supported output languages SHALL be Rust, Python, TypeScript, and Go. Unknown languages SHALL produce a clear error listing supported values.

## Testing

Tests SHALL cover observed dependency extraction, declared-edge missing observations, observed-edge missing declarations, ambiguous observations, exact fact-line parsing, module-level Rust doc discovery, multiple dependency lines, case sensitivity, unknown IDs, docstring template output for all supported languages, and docstring drift findings.
