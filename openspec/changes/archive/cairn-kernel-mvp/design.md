# Design: Cairn kernel MVP

## Language choice: TypeScript

Rationale:

- **Faster iteration** for a prove-the-thesis MVP. The goal is to confirm the query model is useful, not to ship production infrastructure.
- **Tree-sitter ecosystem** is mature in both Rust and TypeScript; the MVP doesn't use Tree-sitter anyway (deferred to the scanner).
- **OpenSpec is TypeScript**, so if the experiment confirms Cairn's value, there's an easy path to publishing as an npm package that integrates with OpenSpec-based workflows.
- **Throwaway cost is lower.** If the MVP fails the side-by-side test, a TypeScript codebase is cheaper to abandon than a Rust one with the same functionality.
- **Claude Code generates high-quality TypeScript** with less hand-holding than Rust for parser work.

Defer the Rust port (if any) to after the experiment concludes. MAG stays Rust. Cairn starts TS.

## Parser strategy

Hand-written recursive descent, not a parser generator.

Reasoning:

- The grammar is small (roughly a dozen keywords, one nesting construct, one edge form).
- A hand-written parser produces vastly better error messages than generator output — critical for the UX of a tool humans author DSL for directly.
- Parser combinator libraries (e.g. `parsimmon`, `arcsecond`) are overkill for this grammar and hurt error reporting.
- No Tree-sitter: the DSL grammar is *authored* by humans, not *extracted* from code. Tree-sitter is for the scanner (phase 2, reading source code), not the DSL parser.

Token-level design: use a single-pass lexer producing a token stream with position info (line, column). Parser consumes tokens and builds an AST. AST is a discriminated union of node types (SystemNode, ContainerNode, ModuleNode, Edge).

## In-memory graph

After parse, walk the AST to produce:

```ts
interface Ontology {
  nodes: Map<NodeId, Node>;        // ID → full node
  nameToId: Map<string, NodeId>;   // Name → ID for CLI lookup
  outbound: Map<NodeId, Edge[]>;   // Node → edges leaving it
  inbound:  Map<NodeId, Edge[]>;   // Node → edges targeting it
  parents:  Map<NodeId, NodeId>;   // Child → parent (for nesting)
  children: Map<NodeId, NodeId[]>; // Parent → direct children
}
```

All queries read from this structure. Construction is O(n) in nodes + edges.

## CLI shape

Use `commander` or similar for argument parsing. Commands:

```
cairn get <node> [--json]
cairn neighbourhood <node> [--json] [--depth=1]
cairn dependents <node> [--json] [--transitive]
```

Default input file: `./cairn.dsl`. Override with `--file`.

Human output uses boxed sections with clear headers. JSON output is stable schema for agent consumption.

## Integrity checks at parse time

Fail fast on:

- Duplicate node IDs (structural error).
- Duplicate paths across leaf nodes (structural error).
- Edge referencing an unknown node ID (structural error).
- Missing required fields (ID, name, description) on a node (structural error).
- Malformed ID (must match `^[a-z][a-z0-9]*(\.[a-z][a-z0-9]*)*$`) (structural error).

All errors include source file, line, column, and a human-readable message. Exit code 1 on any structural error.

## Testing strategy

- Unit tests for the parser: one per grammar production, plus error cases.
- Integration tests: run the CLI against fixture `.dsl` files and snapshot the output.
- Self-hosting test: run the CLI against the Cairn bootstrap's own `cairn.dsl`; assert specific queries return expected results.

Use `vitest` (matches OpenSpec's choice). Aim for coverage on the parser and the integrity checks specifically — the CLI surface can be lighter.

## File layout

```
src/
  parser/
    lexer.ts
    parser.ts
    ast.ts
    errors.ts
  graph/
    ontology.ts
    builder.ts
  cli/
    index.ts
    commands/
      get.ts
      neighbourhood.ts
      dependents.ts
    output/
      human.ts
      json.ts
  index.ts          # library entry point
bin/
  cairn.js          # CLI entry point
test/
  parser/
  graph/
  cli/
  fixtures/
```

## Explicit non-decisions

- **Package name** is not decided. `@reaveshq/cairn` is one option; waiting until after the MVP proves out before registering anything on npm.
- **Licence** is not decided. MIT is the default unless there's a reason otherwise.
- **Publishing** is not decided. MVP runs locally via `pnpm link` for the duration of the experiment.

## Estimated effort

- Parser + AST: one focused session, ~3 hours with Claude Code driving.
- Graph builder + integrity checks: one session, ~2 hours.
- CLI + three commands + output formatters: one session, ~2 hours.
- Self-hosting test and debugging: ~1 hour.

Total: roughly one full day, split across two or three sittings. If it stretches past two full days, something is wrong with either the spec or the approach and we re-evaluate.
