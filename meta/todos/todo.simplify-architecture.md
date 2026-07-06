---
node: cairn.root
status: open
created: 2026-07-06
---

# Simplify Architecture: One Spine, Verified Duplication Removal

Umbrella task tracking the architecture simplification programme derived
from the 2026-07-06 four-audit investigation (CLI surface, usage evidence,
subsystem weight, paradigm audit). Principle: every operation produces one
canonical JSON shape in `src/query_api`; every surface (CLI text, webui,
LSP, MCP, export) is a thin consumer of it. MCP already works this way
(`src/mcp/mod.rs:7`) and is the template.

Verified findings this programme addresses:

- `src/sse.rs` (372 LOC): zero internal callers, self-described spike.
- `src/cli/format/util.rs:17-60`: verbatim copies of
  `src/query_api/serialise.rs` helpers.
- `src/ui/api.rs` + `src/ui/serialise.rs`: parallel reimplementation of
  ~10 query_api handlers.
- `src/lsp/diagnostics.rs:101-123` + `src/lsp/server.rs:50`: own scan and
  watch loop instead of `query_api` lint + `src/watch.rs`.
- CLI: 51 flat commands over ~7 operation families; provable strict
  subsets (`symbols` in `get`, `check` in `lint`, `depends`/`dependents`
  share one handler); `draft_*` family has zero external references.
- Artefact registry: shared frontmatter parser but bespoke per-type
  loader/scaffold/serialiser for todo, decision, gap, research, source,
  review (~1,000 LOC collapsible into one `ArtefactKind` table).
- Four file-persistence patterns with no shared
  `read_json`/`atomic_write`/version-peek helper.
- Hand-maintained CLI help/descriptions/suggestions
  (`src/cli/mod.rs:484-640`) mirroring `query_api::registry()`.

Scoped estimate: 2,500 to 3,500 LOC of verified duplication removable
with zero capability loss. The earlier "delete render/ entirely" idea is
rejected: `context_view.rs` (Mermaid, depth rollup) and `remediate.rs`
(plan composition) are genuine presentation logic and stay.

## Subtasks and dependency order

Wave 1 (independent, pick up in any order):

- todo.simplify-cut-sse
- todo.simplify-dedup-format-util
- todo.simplify-cli-draft-family
- todo.simplify-artefact-kind-table
- todo.simplify-persist-helper

Wave 2 (each depends on one wave-1 task):

- todo.simplify-ui-query-api (after dedup-format-util)
- todo.simplify-cli-change-family (after cli-draft-family)

Wave 3:

- todo.simplify-lsp-spine (after ui-query-api)
- todo.simplify-cli-subset-folds (after cli-change-family)

Wave 4:

- todo.simplify-cli-registry-table (after all CLI-surface tasks)
- todo.simplify-render-canonical-json (after registry-table; incremental,
  per command, measured)

Every subtask lands as its own feature branch and PR through
`scripts/pre-archive-rust-gates.sh`. CLI renames move
`tests/command_reference_consistency.rs`, `docs/commands.md`, and
`docs/integration-contract.md` in the same commit. MCP tool names stay
stable regardless of CLI renames. This todo closes when all eleven
subtasks are done or explicitly dropped with a recorded reason.
