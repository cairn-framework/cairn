# Tasks: Cairn kernel MVP

## 1. Project setup

- [ ] 1.1 Initialise TypeScript project with pnpm, strict tsconfig
- [ ] 1.2 Add dependencies: `commander`, `vitest`
- [ ] 1.3 Set up `bin/cairn.js` entry point
- [ ] 1.4 Configure build (tsc) and dev (tsx) scripts
- [ ] 1.5 Initialise OpenSpec in this repo (`openspec init`) and archive this proposal when MVP is complete

## 2. Lexer

- [ ] 2.1 Token types: keywords, identifiers, strings, IDs, tags, arrows, braces, comments
- [ ] 2.2 Position tracking (line, column) on every token
- [ ] 2.3 String literal handling (double-quoted)
- [ ] 2.4 Comment handling (`#` to end of line)
- [ ] 2.5 Unit tests: one per token type, plus mixed-token fixtures

## 3. Parser

- [ ] 3.1 AST types as discriminated union (SystemNode, ContainerNode, ModuleNode, Edge, ActorNode)
- [ ] 3.2 Parse top-level `System` declaration
- [ ] 3.3 Parse nested `Container` and `Module` declarations (and top-level Modules per v0.6)
- [ ] 3.4 Parse node attributes (`id`, `path` as string or list, `contract`, `todos`, `decisions`, `research`, `reviews`)
- [ ] 3.5 Parse tags (`@tag`)
- [ ] 3.6 Parse edges (`nodeA -> nodeB "description"`)
- [ ] 3.7 Parse error reporting with source position and helpful messages
- [ ] 3.8 Unit tests: one per production, plus error-case tests including multi-target paths

## 4. Graph builder

- [ ] 4.1 Walk AST, produce flat node map keyed by ID
- [ ] 4.2 Build inbound and outbound edge indices
- [ ] 4.3 Build parent/child indices from nesting
- [ ] 4.4 Build name-to-ID lookup for CLI input resolution
- [ ] 4.5 Unit tests: graph shape for representative fixtures

## 5. Integrity checks

- [ ] 5.1 Duplicate ID detection
- [ ] 5.2 Duplicate path detection (leaf nodes only)
- [ ] 5.3 Edge endpoint validation (both sides must exist)
- [ ] 5.4 ID format validation (regex)
- [ ] 5.5 Missing required field detection
- [ ] 5.6 All errors surface with source position
- [ ] 5.7 Unit tests: one fixture per check, both passing and failing

## 6. CLI

- [ ] 6.1 `cairn get <node>`: print node metadata (ID, name, description, tags, path(s), artefact pointers, state)
- [ ] 6.2 `cairn neighbourhood <node>`: print node plus inbound and outbound edges with target metadata
- [ ] 6.3 `cairn dependents <node> [--transitive]`: print nodes that edge into this one; transitive walks inbound edges recursively
- [ ] 6.4 `cairn depends <node> [--transitive]`: inverse of dependents; print nodes this one edges into
- [ ] 6.5 `cairn order [--from <node>] [--scope <id-prefix>]`: print nodes grouped by dependency tier via topological sort; cycles are structural errors
- [ ] 6.6 Name-or-ID resolution for `<node>` argument
- [ ] 6.7 `--json` flag on all commands with stable schema
- [ ] 6.8 `--file` flag to override default DSL path
- [ ] 6.9 Exit code 1 on any parse or integrity error; 0 otherwise
- [ ] 6.10 CLI integration tests: snapshot output against fixture DSL files

## 7. Self-hosting

- [ ] 7.1 Place Cairn bootstrap's `cairn.dsl` in `test/fixtures/cairn.dsl`
- [ ] 7.2 Test: `cairn get cairn.kernel.parser` returns expected metadata
- [ ] 7.3 Test: `cairn neighbourhood cairn.kernel.reconciliation` returns the five modules it connects to
- [ ] 7.4 Test: `cairn dependents cairn.kernel.reconciler` returns `[cairn.reconcilers.code]`

## 8. Documentation

- [ ] 8.1 README with install instructions and three command examples
- [ ] 8.2 `docs/dsl.md` with the grammar summary
- [ ] 8.3 Notes file documenting what was deferred for phase 2

## Future Work

- 9.1 Pick a real project (MAG is the planned subject)
- 9.2 Hand-author a `cairn.dsl` for it (time-boxed: one hour)
- 9.3 Use Cairn queries during the next real coding session on MAG
- 9.4 Note which queries were useful, which were ignored, which were missing
- 9.5 Decide: continue to phase 2 (scanner + contracts), or abandon
- 10.1 Write up findings: what worked, what didn't, what the spec got wrong
- 10.2 If continuing: draft the phase 2 change proposal (scanner + contract artefact type)
- 10.3 If abandoning: publish the Cairn spec and findings as a ReavesHQ content piece anyway
