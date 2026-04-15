# Proposal: Cairn kernel MVP

## Why

The v0.6 spec defines a framework with many components across ten phases. Before committing to a full implementation, we need a minimum viable kernel that proves the core thesis: **a structural graph query layer over a human-authored DSL is more useful for AI-assisted coding than unstructured capability specs alone**.

OpenSpec already handles the workflow side (propose, design, apply, archive) well enough that Cairn doesn't need to replicate it. What OpenSpec structurally cannot do — answer "what depends on this module?", "what does this rely on?", "what order should I build this in?" — is the entire justification for Cairn existing.

The MVP is phase 1 of the v0.6 build order. It must make the core query capability real. Nothing more.

## Scope (IN)

- Parser for the v0.6 DSL grammar: `System`, `Container`, `Module`, IDs, names, descriptions, tags, paths (single or list), artefact pointers, edges.
- In-memory graph representation with ID-based node lookup and edge traversal.
- Five CLI commands: `cairn get <node>`, `cairn neighbourhood <node>`, `cairn dependents <node>`, `cairn depends <node>`, `cairn order`.
- Both human-readable and `--json` output for each command.
- Accept either name or ID as the `<node>` argument; resolve to ID internally.
- Integrity checks at parse time: duplicate IDs, path ties, invalid edge references, missing required fields, cycle detection for `cairn order`.

## Scope (OUT, deferred to later phases per section 14)

- Scanner. No filesystem walking, no Tree-sitter, no code parsing. (Phase 1 still: the scanner's role in the MVP is just confirming path strings exist as directories.)
- Contract artefact type and interface hashing. (Deferred to phase 2 of full implementation.)
- All other artefact types (todos, decisions, research, reviews, sources). The DSL parser reads artefact *pointers* but does not open the files they point to.
- Change directories, archive command, rename operation. (Phase 3.)
- Hooks. (Phase 4.)
- Edge validation and docstring generation. (Phase 5.)
- Multi-target resolution beyond parsing the list. (Phase 6.)
- MCP wrapper, summariser, brownfield, LSP. (Phases 7 onward.)

Everything in this deferred list becomes its own subsequent change, once the MVP has proven the kernel works.

## Success criteria

The MVP is successful when, for a hand-authored `cairn.dsl` file describing a real project (MAG is the planned test subject), all of the following are true:

1. `cairn get mag.query` returns the node's metadata in under 50ms.
2. `cairn neighbourhood mag.query` returns the node plus its inbound and outbound edges with target node metadata, in a readable layout.
3. `cairn dependents mag.query` returns the set of nodes that depend on it, transitive optional via flag.
4. `cairn depends mag.query` returns the set of nodes this node relies on.
5. `cairn order` returns the project in dependency-tier order.
6. Parse errors on malformed DSL are human-readable and point to the offending line.
7. Running the tool against the Cairn bootstrap's own `cairn.dsl` file returns sensible results (self-hosting sanity check).

## Non-criteria

- Not required: MCP integration, file watching, pretty-printed diagrams, or any tool integration beyond a plain CLI.
- Not required: handling missing artefact files gracefully beyond reading the pointer. If a contract path is declared but the file doesn't exist, that's a phase 2 concern.

## Risk

The one risk that would kill the project: the queries return results but the results don't actually help an AI agent. This is the side-by-side test against OpenSpec-alone, to be run after the MVP is working.
