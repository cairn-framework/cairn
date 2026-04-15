# Tasks: Cairn kernel MVP

## 1. Project setup

- [x] 1.1 Initialise TypeScript project with pnpm, strict tsconfig
- [x] 1.2 Add dependencies: `vitest`; CLI parsing is hand-rolled, so no runtime parser dependency is required
- [x] 1.3 Set up `bin/cairn.js` entry point
- [x] 1.4 Configure build (tsc), lint, and test scripts

## 2. Lexer

- [x] 2.1 Token types: keywords, identifiers, strings, IDs, tags, arrows, braces, comments
- [x] 2.2 Position tracking (line, column) on every token
- [x] 2.3 String literal handling (double-quoted)
- [x] 2.4 Comment handling (`#` to end of line)
- [x] 2.5 Unit tests: one per token type, plus mixed-token fixtures

## 3. Parser

- [x] 3.1 AST types as discriminated union (SystemNode, ContainerNode, ModuleNode, Edge, ActorNode)
- [x] 3.2 Parse top-level `System` declaration
- [x] 3.3 Parse nested `Container` and `Module` declarations (and top-level Modules per v0.6)
- [x] 3.4 Parse node attributes (`id`, `path` as string or list, `contract`, `todos`, `decisions`, `research`, `reviews`)
- [x] 3.5 Parse tags (`@tag`)
- [x] 3.6 Parse edges (`nodeA -> nodeB "description"`)
- [x] 3.7 Parse error reporting with source position and helpful messages
- [x] 3.8 Unit tests: one per production, plus error-case tests including multi-target paths

## 4. Graph builder

- [x] 4.1 Walk AST, produce flat node map keyed by ID
- [x] 4.2 Build inbound and outbound edge indices
- [x] 4.3 Build parent/child indices from nesting
- [x] 4.4 Build name-to-ID lookup for CLI input resolution
- [x] 4.5 Unit tests: graph shape for representative fixtures

## 5. Integrity checks

- [x] 5.1 Duplicate ID detection
- [x] 5.2 Duplicate path detection (leaf nodes only)
- [x] 5.3 Edge endpoint validation (both sides must exist)
- [x] 5.4 ID format validation (regex)
- [x] 5.5 Missing required field detection
- [x] 5.6 All errors surface with source position
- [x] 5.7 Unit tests: one fixture per check, both passing and failing

## 6. CLI

- [x] 6.1 `cairn get <node>`: print node metadata (ID, name, description, tags, path(s), artefact pointers, state)
- [x] 6.2 `cairn neighbourhood <node>`: print node plus inbound and outbound edges with target metadata
- [x] 6.3 `cairn dependents <node> [--transitive]`: print nodes that edge into this one; transitive walks inbound edges recursively
- [x] 6.4 `cairn depends <node> [--transitive]`: inverse of dependents; print nodes this one edges into
- [x] 6.5 `cairn order [--from <node>] [--scope <id-prefix>]`: print nodes grouped by dependency tier via topological sort; cycles are structural errors
- [x] 6.6 Name-or-ID resolution for `<node>` argument
- [x] 6.7 `--json` flag on all commands with stable schema
- [x] 6.8 `--file` flag to override default DSL path
- [x] 6.9 Exit code 1 on any parse or integrity error; 0 otherwise
- [x] 6.10 CLI integration tests: snapshot output against fixture DSL files

## 7. Self-hosting

- [x] 7.1 Place Cairn bootstrap's `cairn.dsl` in `test/fixtures/cairn.dsl`
- [x] 7.2 Test: `cairn get cairn.kernel.parser` returns expected metadata
- [x] 7.3 Test: `cairn neighbourhood cairn.kernel.reconciliation` returns the five modules it connects to
- [x] 7.4 Test: `cairn dependents cairn.kernel.reconciler` returns `[cairn.reconcilers.code]`

## 8. Documentation

- [x] 8.1 README with install instructions and three command examples
- [x] 8.2 `docs/dsl.md` with the grammar summary
- [x] 8.3 Notes file documenting what was deferred for phase 2

## Future Work

- 1.5 Initialise OpenSpec in this repo (`openspec init`) and archive this proposal when MVP is complete
  - Reason: `openspec init` is not available in the local tool cache and npm registry access is blocked by DNS (`getaddrinfo ENOTFOUND registry.npmjs.org`); archiving is also explicitly prohibited during apply mode.
  - Required action: Run OpenSpec init/archive from the orchestrator or a network-enabled environment after acceptance.
- 9.1 Pick a real project (MAG is the planned subject)
- 9.2 Hand-author a `cairn.dsl` for it (time-boxed: one hour)
- 9.3 Use Cairn queries during the next real coding session on MAG
- 9.4 Note which queries were useful, which were ignored, which were missing
- 9.5 Decide: continue to phase 2 (scanner + contracts), or abandon
- 10.1 Write up findings: what worked, what didn't, what the spec got wrong
- 10.2 If continuing: draft the phase 2 change proposal (scanner + contract artefact type)
- 10.3 If abandoning: publish the Cairn spec and findings as a ReavesHQ content piece anyway

## Acceptance #1 Failure Follow-up

- [x] Fix project setup reproducibility: task 1.1 claims a pnpm TypeScript project, but no `pnpm-lock.yaml` is present; `pnpm install --offline --frozen-lockfile` fails with `ERR_PNPM_NO_LOCKFILE`, and `npm run build` fails because `tsc` is not available from `node_modules/.bin`. Add the lockfile/installable dependency state needed for `pnpm build`, `pnpm lint`, and `pnpm test` to run from a clean checkout.
