---
node: cairn.kernel.parser
informed_by:
  - type: decision
    id: dec.two-chain-authority
  - type: decision
    id: dec.module-path-mapping
---

# Contract: cairn.kernel.parser

The Parser consumes a `cairn.blueprint` source file and emits an in-memory abstract syntax tree representing the declared graph. The AST is the input to every downstream reader (scanner, query, hooks, summariser).

## Interface

- **Input.** UTF-8 text of a `.blueprint` file.
- **Output.** An `Ast` value containing nodes (System, Container, Module, Actor) keyed by stable ID, edges (each carrying `from`, `to`, a description, and a source span), and per-node raw fields including `path`, `contract`, `decisions`, `research`, `todos`, `reviews`, `sources`.
- **Errors.** Malformed syntax surfaces as a parse-error finding (registry category P) with file, line, and column. The parser does not attempt recovery; the first parse error stops processing.

## Invariants

- Every node carries a stable ID drawn from the source text. The parser never invents IDs.
- Edge endpoints reference nodes by ID, never by display name.
- Path strings are preserved verbatim. Normalisation (resolving `./`, trimming trailing slashes) belongs to the scanner.
- Comments (lines beginning with `#`) are dropped during AST construction. They have no semantic effect.

## Out of scope

- File-system reconciliation. The parser produces a graph; the scanner determines whether declared paths exist on disk.
- Contract validation. The parser records contract pointers; the contract loader (`src/artefacts/contract.rs`) enforces frontmatter shape and node-match invariants.
- Cross-blueprint validation. Each parse call is over a single blueprint file.
