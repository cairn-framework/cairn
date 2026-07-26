# Command reference

Every `query_api` command accepts `--json`.

## Orientation and query

| Use case | Command | Key flags |
|---|---|---|
| Orientation | `cairn context` | `--json`, `--depth <N\|all>`, `--scope <node>` |
| Project status | `cairn status` | `--json`, `--brief` |
| Inspect node | `cairn get <node>` | `--json`, `--symbols` |
| Node + neighbours | `cairn neighbourhood <node>` | `--json`, `--include-todos`, `--include-research`, `--include-changes` |
| Node files | `cairn files <node>` | `--json` |
| Locate a symbol | `cairn locate <symbol>` | `--json` (exact name match only) |
| Dependencies | `cairn deps <node>` | `--json`, `--direction in`, `--transitive` |
| Build order | `cairn order` | `--json` |
| Disconnected islands | `cairn islands` | `--json` |
| Graph dump | `cairn graph` | `--json` |
| Bundle (contract, decisions, interfaces, gates) | `cairn bundle <node>` | `--json` |
| Next unit brief | `cairn brief [target]` | `--json` |
| Next ready unit | `cairn next` | `--json` |
| Buildable ghost nodes | `cairn frontier` | `--json` |
| Health check | `cairn health` | `--json` |
| Web explorer | `cairn ui` | `--port <N>` |

## Provenance

| Use case | Command | Key flags |
|---|---|---|
| Provenance chain | `cairn rationale <node>` | `--json` (accepted decisions only) |
| Decisions | `cairn decisions <node>` | `--json`, `--status`, `--grep` |
| Research | `cairn research <node>` | `--json` |
| Sources | `cairn sources <node>` | `--json` |
| Contract | `cairn contract <node>` | `--json` |
| Todos | `cairn todos [node]` | `--json`, `--status open` |

## Authoring

| Use case | Command |
|---|---|
| New todo | `cairn todo new <slug> --node <id>` |
| Set todo status | `cairn todo set <slug> <open\|in_progress\|done\|blocked>` |
| New decision | `cairn decision new <slug> [--node <id>] [--informed-by <id>]` |
| Log an open question | `cairn gap <node> --question "<text>"` |
| Record friction | `cairn feedback "<message>"` |

`cairn todo set` is the sanctioned mutation verb for todo status
(`dec.todo-write-surface`); it rewrites only the frontmatter `status` field.

## Changes

| Use case | Command |
|---|---|
| List | `cairn change list` |
| Show | `cairn change show <change-id>` |
| New | `cairn change new <change-id>` |
| Apply to blueprint | `cairn change apply <change-id>` |
| Acceptance gate | `cairn change accept [<change-id>]` |
| Archive | `cairn change archive <change-id>` |
| Draft proposals | `cairn draft list\|show\|edit\|discard\|accept\|create` |

## Verification

| Use case | Command | Notes |
|---|---|---|
| Full scan | `cairn scan` | `--strict` also fails on Warnings |
| Lint findings | `cairn lint` | `--json`, `--node <id>`; not a commit gate |
| Commit gate | `cairn hook <structural\|interface\|tension\|all>` | correct blocking semantics |
| Remediation plan | `cairn remediate [finding-code]` | `--json` |

Hook semantics:

| Hook | Blocks on | Use |
|---|---|---|
| `structural` | Error findings, active-change conflicts | typical pre-commit |
| `interface` | Interface hash changes, conflicts | API changes |
| `tension` | never (advisory) | surface warnings |
| `all` | errors, interface changes, conflicts | strictest |

Use `cairn hook`, not `cairn lint`, to gate a commit. Do not use `cairn scan` as a
substitute for compiling; they check different things.

## Brownfield

| Use case | Command |
|---|---|
| Bootstrap | `cairn init` (`--wire` appends an orientation pointer) |
| Extract from code | `cairn init --from-code` (`--apply`, `--force`) |
| Re-discover | `cairn refine` |
| Triage orphans | `cairn onboard` |
| Import openspec | `cairn import-openspec` |

The brownfield flow generates a proposal, not final state. Review the generated
`blueprint.delta` and contracts before accepting.

## Other

| Use case | Command |
|---|---|
| Export graph | `cairn export --format <json\|md\|mermaid> --output <path>` |
| Rename a node | `cairn rename <old-id> <new-id>` |
| Raw blueprint | `cairn blueprint` |
| Docstring | `cairn docstring <node>` |
| Watch for finding changes | `cairn watch` |
| Query schemas | `cairn ui_meta` |
| Workspace aggregate | `cairn workspace <status\|lint\|frontier>` |
| Backlog beads | `cairn backlog <node>`, `cairn beads <node>` |
| Agent pack lifecycle | `cairn pack <install\|update\|status\|uninstall>` (`--harness <name>`, `--loop`) |
| Resolve pack bytes | `cairn pack resolve` (`--harness <name>`, `--loop`, `--json`) |
| Campaign lock | `cairn pack campaign <start\|verify\|end>` (`--harness <name>`, `--loop`, `--json`) |

## JSON contract

Every `query_api` command with `--json` returns a payload carrying a top-level
`schema_version` (currently `1`), stamped at one choke point in
`query_api::execute` (`dec.query-json-schema-version`). `cairn export` and the
summariser wire schemas version independently; they are not command envelopes.

Exit codes: `0` success, `1` blocking findings or command failure, `2` usage
error.
