---
node: cairn.root
status: done
created: 2026-07-06
---

# Simplify Architecture: One Spine, Verified Duplication Removal

Umbrella task tracking the architecture simplification programme derived
from the 2026-07-06 four-audit investigation (CLI surface, usage evidence,
subsystem weight, paradigm audit). Principle: every operation produces one
canonical JSON shape in `src/query_api`; every surface (CLI text, webui,
LSP, MCP, export) is a thin consumer of it. MCP already works this way
(`src/mcp/mod.rs:162`, dispatch through `query_api::execute`) and is the
template.

Verified findings this programme addresses:

- `src/sse.rs` (372 LOC): zero internal callers, self-described spike.
- `src/cli/format/util.rs:17-60`: verbatim copies of
  `src/query_api/serialise.rs` helpers.
- `src/ui/api.rs` + `src/ui/serialise.rs`: parallel reimplementation of
  ~10 query_api handlers (a few UI endpoints have no handler yet: beads,
  meta, blueprint; those get spine operations).
- `src/lsp/diagnostics.rs:101-123` + `src/lsp/server.rs:50`: own scan and
  rescan loop instead of the spine plus a shared watch loop.
- CLI: 52 flat commands over ~7 operation families; `check` is `lint`
  filtered by node, `depends`/`dependents` share one handler, `symbols`
  reads data `get` resolves (but does not yet expose on the wire); the
  `draft_*` family has no workflow/skill/script usage.
- Artefact registry: shared frontmatter parser but bespoke per-type
  loader/scaffold/serialiser for todo, decision, gap, research, source,
  review (~1,000 LOC collapsible into one `ArtefactKind` table).
- Hand-rolled state-file persistence across eight call sites with no
  shared `read_json`/`atomic_write`/version-peek helper.
- Hand-maintained CLI help/descriptions/suggestions
  (`src/cli/mod.rs:484-640`) mirroring `query_api::registry()`.

Scoped estimate: 2,500 to 3,500 LOC of verified duplication removable
with zero capability loss. The earlier "delete render/ entirely" idea is
rejected: `context_view.rs` (Mermaid, depth rollup) and `remediate.rs`
(plan composition) are genuine presentation logic and stay.

## Shared rules (every subtask points here)

1. Any CLI rename/removal moves `tests/command_reference_consistency.rs`,
   `docs/commands.md`, and `docs/integration-contract.md` in the same
   commit. Check `tests/phase_10_distribution.rs` for literal
   `cairn <cmd>` assertions too.
2. Registry cli_names are pinned on the /api/meta wire by
   `tests/snapshots/wire_format_snapshots__api_meta.snap`
   (`src/ui/api.rs:9-12` iterates the registry). Every rename/removal
   changes that wire shape: treat it as a schema decision per
   `meta/decisions/webui-json-schema-version.md` conventions, never a
   silent snapshot update.
3. MCP tool names never change, regardless of CLI spellings.
4. Clean cutover: no deprecation aliases.
5. Every subtask lands as its own feature branch and PR through
   `scripts/pre-archive-rust-gates.sh`.

## Subtasks and dependency order

Each subtask file's own "Depends on:" line is authoritative; the waves
below are the derived pick-up order.

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
- todo.simplify-render-canonical-json (after dedup-format-util AND
  registry-table; incremental, per command, measured)

This todo closes when all eleven subtasks are done or explicitly dropped
with a recorded reason.

## Status (2026-07-10)

Programme substantially complete. Landed: cut-sse, dedup-format-util,
persist-helper, artefact-kind-table (#224), draft family, change family
(#223), subset folds (#226), generic-language-reconciler (#227),
cli-registry-table (#228), ui-query-api slice 1 (#229),
render-canonical-json (#230, six commands stop-ruled with measured numbers),
lsp-spine (#231). Outstanding: the remaining 11 ui-query-api endpoint flips
(todo.simplify-ui-query-api, in_progress). This umbrella closes when that
lands.

## Status (2026-07-12)

Closed. All eleven subtasks are done; none dropped. Final outstanding item
(todo.simplify-ui-query-api) landed with PR #265: `/api/status` flipped to
the query_api spine and `src/ui/api.rs` + `src/ui/serialise.rs` deleted.
Verified against main: every `/api/*` endpoint routes through
`query_api::execute_with_scan`, `app.js` no longer fetches `/api/status`,
and the legacy UI serialisers are gone from `src/ui/`. The one-spine principle
(every surface a thin consumer of canonical query_api JSON) now holds for
CLI, webui, LSP, MCP, and export.
