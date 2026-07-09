---
id: dec.simplify-cli-subset-folds
nodes:
  - cairn.kernel.cli
  - cairn.kernel.query
status: accepted
date: 2026-07-09
---

# Fold strict-subset CLI commands into flags of their parent command

## Context

`simplify-architecture` (wave 3) removes three CLI commands that were strict
subsets of an existing command, differing only by an opt-in flag or a boolean:

- `symbols <id>` read the same in-memory `NodeRecord` that `get` resolves and
  emitted a public-symbols list. `get`'s wire JSON (`node_json`) already owned
  that node but did not yet expose `symbols`.
- `check <node>` ran `lint` (same `scanner::load_project` data) filtered by a
  node, but as a separate top-level command.
- `depends <id>` and `dependents <id>` shared one handler
  (`dependency_json`) differing only by a boolean (outbound vs inbound).

Each had documented usage (spec, skills, onboarding, emitted init skills), so
the fold is a user-facing command-spelling change, not a free rename. The
MCP tool surface (`cairn_get`, `cairn_lint`, `cairn_depends`,
`cairn_dependents`) is explicitly out of scope and keeps its names.

## Decision

Three folds, clean cutover, no aliases:

1. **`symbols` -> `get --symbols`.** The `symbols` CLI command and the
   `cairn_symbols` MCP tool are removed. `get`'s `NodeResponse` (`node_json`)
   gains an OPTIONAL `symbols` field, present only when the request carries
   `QueryFlag::Symbols` (set by the `get --symbols` flag). This is a deliberate
   wire-shape addition and is recorded here per the query JSON schema
   versioning convention (`meta/decisions/query-json-schema-version.md`), not a
   silent rename. The webui `SymbolsResponse` is untouched; `src/ui/api.rs`
   still calls `query::symbols` directly.

2. **`check <node>` -> `lint --node <id>`.** The `check` CLI command is
   removed. `lint --node <id>` runs the same lint and filters findings by the
   target node, non-blocking (always exit 0), matching `check`'s prior
   semantics. The missing-blueprint preflight, the `cairn.dsl` rename guidance,
   and the `empty-states.cli-no-blueprint` copy survive unchanged on the new
   path. `cairn_lint` is unchanged.

3. **`depends`/`dependents` -> `deps <id> --direction in|out`.** Both CLI
   commands are removed and replaced by a single `deps` command (default
   direction `out`). Direction is carried by `QueryFlag::Inbound`; the CLI sets
   it from `--direction in`, and the `cairn_dependents` MCP tool sets it
   server-side. The two MCP tools keep their distinct names.

## Wire snapshot impact

`tests/snapshots/wire_format_snapshots__api_meta.snap` is a byte-identical
guard over the `/api/meta` tool list. Because `cli_name` values changed, this
snapshot is updated deliberately and recorded here:

- The `symbols` tool entry is removed (the `cairn_symbols` MCP tool is gone).
- The `depends` and `dependents` `cli_name` entries both become `deps`, so
  `/api/meta` now lists two `deps` entries (mcp `cairn_dependents`,
  mcp `cairn_depends`).

The webui route snapshots `wire_format_snapshots__api_depends_app_api.snap`
and `wire_format_snapshots__api_dependents_app_api.snap` are unchanged: the
`/api/depends` and `/api/dependents` routes call `query::depends` /
`query::dependents` directly and are unaffected by the registry rename.

## Rationale

One command per concept with opt-in flags beats a family of near-synonym
top-level commands: less surface to document, learn, and keep consistent, and
no duplicated handler logic. The MCP surface is held stable so external
integrations do not need to change.

## Consequences

- CLI spellings: `cairn symbols` -> `cairn get <id> --symbols`;
  `cairn check [<node>]` -> `cairn lint --node <id>`;
  `cairn depends <id>` -> `cairn deps <id>`;
  `cairn dependents <id>` -> `cairn deps <id> --direction in`.
- `docs/commands.md`, `docs/integration-contract.md`, `docs/spec.md`, the agent
  skills, and `README.md` are updated to the new spellings.
- MCP tools `cairn_get`, `cairn_lint`, `cairn_depends`, `cairn_dependents` keep
  their names and schemas.
- `node_json` now takes an `include_symbols: bool` parameter; callers that do
  not want symbols pass `false`.
