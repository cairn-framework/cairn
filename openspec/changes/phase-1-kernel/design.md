# Design: Phase 1 Kernel

## References

- `docs/spec.md` section 6 for kernel components and state layout.
- `docs/spec.md` section 7 for DSL grammar.
- `docs/spec.md` section 10 for scanner, reconciliation, node states, generated outputs, and contradiction classes.
- `docs/spec.md` section 12 for CLI query behavior.

## Module Layout

The implementation SHALL use these Rust modules:

```text
src/
  dsl/
    lexer.rs
    parser.rs
    ast.rs
    error.rs
  ontology/
    graph.rs
    build.rs
    query.rs
    integrity.rs
  artefacts/
    contract.rs
    frontmatter.rs
  reconcile/
    mod.rs
    code.rs
    fingerprint.rs
  scanner/
    mod.rs
    outputs.rs
    state.rs
  cli/
    mod.rs
    output.rs
```

`src/lib.rs` SHALL export library APIs used by integration tests and by `src/main.rs`. `src/main.rs` SHALL remain a thin CLI entrypoint.

## DSL Parser

The parser SHALL be hand-written recursive descent. It SHALL tokenize comments, braces, arrows, quoted strings, identifiers, tags, list delimiters, and path strings with line and column tracking.

Supported declarations:

- `System`
- `Container`
- `Module`
- `Actor`

Every node SHALL include name, description, stable `id`, zero or more tags, optional path, optional `owns-files: true`, and zero or more artefact pointers. `path` and artefact pointer fields SHALL accept a string or list of strings. Edges SHALL use node IDs and a required description.

The parser SHALL produce typed AST structures and source spans. Parser errors SHALL include file, line, column, expected token, and encountered token.

## Ontology Graph

Ontology construction SHALL transform the AST into:

- Node map keyed by stable ID.
- Name map for name-or-ID CLI resolution.
- Parent and child indexes from nesting.
- Inbound and outbound edge indexes.
- Claimed path index with most-specific ownership resolution, leaf-default ownership, and internal-node ownership opt-in.
- Attached contract pointer metadata.
- Node states: `synced`, `ghost`, and `orphaned`.

Only leaf nodes SHALL own files by default. An internal node with a path SHALL own files only when it declares `owns-files: true`; otherwise it acts as a grouping boundary for descendant ownership and orphan detection. Ownership SHALL resolve by most-specific matching path among eligible owning nodes. Ties between eligible owners are structural errors.

Integrity validation SHALL reject duplicate IDs, invalid ID format, missing required fields, path ties, invalid edge endpoints, broken contract pointers, and dependency cycles used by `order`.

## Reconciler Interface

The reconciler trait SHALL be domain agnostic:

```rust
pub trait Reconciler {
    fn id(&self) -> ReconcilerId;
    fn reconcile(&self, request: ReconcileRequest<'_>) -> Result<ReconcileReport, ReconcileError>;
}
```

`ReconcileReport` SHALL contain claimed files, extracted symbols, an interface fingerprint, and findings. The Phase 1 code reconciler SHALL target Rust using Tree-sitter. It SHALL identify files under claimed eligible owner paths, compute deterministic interface fingerprints for public Rust items, and report orphaned Rust source files under claimed containers that no leaf module or `owns-files: true` internal node owns.

## Contract Artefact

Phase 1 SHALL implement only the contract artefact type. Contract files are Markdown with frontmatter containing `node: <id>`. The loader SHALL parse frontmatter, validate the referenced node exists, and retain section text for `cairn contract <node>`.

Missing contracts SHALL be warnings for ghost nodes and structural errors when a synced leaf node declares a broken contract pointer.

## Scanner and Outputs

`cairn scan` SHALL:

1. Load configuration and ignore rules.
2. Parse `cairn.dsl`.
3. Load contract artefacts.
4. Run registered reconcilers.
5. Build the ontology and integrity findings.
6. Write `.cairn/state/interface-hashes.json`.
7. Regenerate `index.md`.
8. Append a scan event to `.cairn/log.md`.

`index.md` SHALL include generated frontmatter, `Synced`, `Ghost`, `Active changes`, and `Findings` sections. Phase 1 SHALL include an empty active changes section because the change system lands in Phase 3.

## CLI

The CLI SHALL provide:

- `cairn get <node>`
- `cairn neighbourhood <node>`
- `cairn contract <node>`
- `cairn files <node>`
- `cairn dependents <node> [--transitive]`
- `cairn depends <node> [--transitive]`
- `cairn order [--from <node>] [--scope <id-prefix>]`
- `cairn lint`
- `cairn scan`

All commands SHALL accept `--file <path>` for the DSL path and `--json` for a stable JSON schema. Human-readable output SHALL use labelled sections and no ANSI color unless stdout is a TTY.

## Error Model

Use typed error enums with `Result<T, E>`. CLI commands SHALL exit `1` for parse, integrity, IO, or query errors and `0` for successful queries. JSON errors SHALL include a stable `code`, `message`, and optional source span.

## Testing

Tests SHALL cover lexer tokens, parser productions, malformed DSL, `owns-files: true` parsing, leaf-default ownership, internal-node ownership opt-in, ontology indexes, integrity failures, contract loading, reconciler reports for Rust fixtures, scan output generation, and CLI snapshots for both human and JSON output.
