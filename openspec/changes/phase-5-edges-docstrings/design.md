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

The scanner SHALL compare declared DSL edges with observed dependencies:

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

## Docstring Command

`cairn docstring <node> [--language <lang>]` SHALL emit a template grounded in:

- Node ID, name, and description.
- Declared dependencies.
- Tags.
- Contract section headings.

Supported output languages SHALL be Rust, Python, TypeScript, and Go. Unknown languages SHALL produce a clear error listing supported values.

## Testing

Tests SHALL cover observed dependency extraction, declared-edge missing observations, observed-edge missing declarations, ambiguous observations, docstring template output for all supported languages, and docstring drift findings.
