# CAIRN Integration Contract

Stable interface specification for external tools, orchestrators, and AI agents consuming cairn.

## Transport modes

| Mode | Entry point | Use case |
|---|---|---|
| CLI + JSON | `cairn --json <command>` | Shell scripts, CI pipelines, simple agents |
| MCP server | `cairn-mcp` (stdio) | AI agents with MCP support |
| Library | `cairn::query_api::execute()` | Rust-native integrations |

All three modes share the same query API and produce identical response shapes.

## JSON envelope

Every `--json` command produces:

```json
{"command":"<name>","status":"ok|error","data":{...}}
```

| Field | Type | Description |
|---|---|---|
| `command` | string | The command name that was executed |
| `status` | `"ok"` or `"error"` | Whether the operation succeeded |
| `data` | object | Command-specific response payload |

Error responses from the MCP/query path:

```json
{"error":{"code":"<ERROR_CODE>","message":"<description>"}}
```

## Exit codes

| Code | Meaning | Action |
|---|---|---|
| 0 | Success, no blocking findings | Proceed |
| 1 | Blocking findings (Error severity), or command failed | Inspect and resolve before proceeding |
| 2 | Usage error (bad arguments, unknown command) | Fix invocation |

`--strict` extends exit 1 to Warning findings as well as Error.

## Command taxonomy

### Read-only queries (safe to call anytime)

| CLI | MCP tool | Returns |
|---|---|---|
| `get <node>` | `cairn_get` | Node record (id, name, description, state, children, files) |
| `neighbourhood <node>` | `cairn_neighbourhood` | Node + inbound/outbound edges |
| `contract <node>` | `cairn_contract` | Contract body text |
| `files <node>` | `cairn_files` | File paths owned by the node |
| `locate <symbol>` | `cairn_locate` | All exact public symbol definitions with owning node ids and source locations |
| `bundle <node>` | `cairn_bundle` | Contract, decisions, dependency interfaces, and gates composed for one node |
| `deps <node>` | `cairn_depends` | Outbound dependency edges |
| `deps <node> --direction in` | `cairn_dependents` | Inbound dependency edges |
| `order` | `cairn_order` | Topological sort of all nodes |
| `islands` | `cairn_islands` | Disconnected graph components |
| `frontier` | `cairn_frontier` | Buildable-now (ready) and blocked ghost nodes, tiered by dependency depth |
| `roadmap` | `cairn_roadmap` | Live todos in dependency tiers from `blocked_by`, grouped by `parent` |
| `graph` | `cairn_graph` | Full structural graph: nodes and edges |
| `lint` | `cairn_lint` | All findings (errors + warnings + info) |
| `health` | `cairn_health` | Comprehensive health assessment (clean flag, counts, findings) |
| `remediate` | `cairn_remediate` | Ordered action plan from current findings |
| `status` | `cairn_status` | Project summary (node count, finding count, etc.) |
| `pending [<id>]` | `cairn_pending` | Proposed decisions with a plain ruling summary, rubric, local review evidence, stale-reviewed-material marker, computed reverse provenance edges, and exact next action; omit the id for the oldest-first list |
| `next` | n/a | The next ready unit of work from the backlog |
| `brief [<id>]` | n/a | Fused next-unit brief: task, binding decisions, contract, gates |
| `rationale <node>` | `cairn_rationale` | Provenance chain (decisions, research, sources) |
| `todos <node>` | `cairn_todos` | Todo artefacts linked to the node |
| `backlog <node>` | n/a | Beads (issues) linked to the node via its `cairn-node:<id>` label |
| `beads <node>` | `cairn_beads` | Backlog beads (issues) linked to a node |
| `decisions <node>` | `cairn_decisions` | Decision artefacts linked to the node |
| `research <node>` | `cairn_research` | Research artefacts linked to the node |
| `sources <node>` | `cairn_sources` | Source artefacts linked to the node |
| `change list` | `cairn_changes` | Active change directories |
| `change show <change>` | `cairn_show_change` | Change details (proposal, delta, contracts) |
| `context` | `cairn_context` | Full structured project overview |
| `docstring <node>` | `cairn_docstring` | Generate a docstring for a node |
| `export` | `cairn_export` | Export project data (JSON or Markdown) |
| `onboard` | `cairn_onboard` | Suggest blueprint entries for orphaned files |
| `ui_meta` | `cairn_ui_meta` | Available query commands and their request/response schemas |
| `blueprint` | `cairn_blueprint` | Raw blueprint file content |
| `ui` | - | Launch the web UI server |
| `watch` | `cairn_watch` | Watch for finding changes and emit events |
| `workspace <status\|lint\|frontier>` | - | Aggregate status, lint, and frontier queries across a `cairn.workspace`'s member projects |

### Mutating commands (modify filesystem)

| CLI | MCP tool | Effect |
|---|---|---|
| `baseline <record\|drop> <node>` | n/a | Write or prune a node's contract baseline. CLI-only |
| `scan` | `cairn_scan` | Re-scan project, update interface hashes |
| `rename <old> <new>` | `cairn_rename` | Rename a node ID across all files |
| `change apply <change>` | `cairn_archive` | Apply a completed change |
| `change new <id>` | - | Scaffold a new change directory |
| `init` | `cairn_init` | Scaffold new cairn project |
| `init --from-code` | `cairn_init_from_code` | Brownfield extraction from existing code |
| `refine` | `cairn_refine` | Re-run brownfield discovery |
| `import-openspec` | `cairn_import_openspec` | Migrate openspec changes to meta/changes |
| `feedback "<message>"` | - | Record cairn friction in `.cairn/feedback.md`, print upstream issue link |
| `decision new <slug>` | - | Scaffold a new decision artefact (frontmatter + sections) |
| `gap <node> --question "<text>"` | - | Log an unresolved question as a `gap: true`, `status: proposed` decision artefact |

### Draft lifecycle (semi-stable)

| CLI | MCP tool | Effect |
|---|---|---|
| `draft list` | `cairn_drafts` | List pending draft proposals |
| `draft show <draft>` | `cairn_draft_show` | Show a draft proposal |
| `draft edit <draft>` | `cairn_draft_edit` | Open a draft in your editor |
| `draft discard <draft>` | `cairn_draft_discard` | Discard a draft proposal |
| `draft accept <draft>` | `cairn_draft_accept` | Accept a draft and apply it |
| `draft create <node>` | `cairn_summarise` | Generate a contract summary for a node |

### Gate commands

| CLI | Purpose | Exit semantics |
|---|---|---|
| `hook <kind>` | Pre-commit gate | Exit 0 = pass, Exit 1 = blocked |
| `hook install` | Install a raw Git hook when no pre-commit framework owns it | Exit 0 = installed or already installed |
| `hook status` | Report Cairn hook ownership | Exit 0 = reported |
| `hook uninstall` | Remove only a Cairn-owned hook | Exit 0 = removed or absent |
| `change accept [<change>]` | Full verification battery | JSON: `data.gate_outcome` = passed/failed/blocked |
| `change accept --dry-run [<change>]` | Preview the battery, running nothing | JSON: `data.gate_outcome` = preview; steps are `planned`, except a configured gate with a blank command, which is `failed` with exit 1 as it would be live |
| `check [<node>]` | Non-blocking inspection | Always reports, never blocks |
| `pack install` | Install the agent pack, recording an ownership manifest | Exit 0 = installed or already current |
| `pack update` | Refresh pristine pack files, backfill missing ones | Exit 0 = reported; modified files are never overwritten |
| `pack status` | Report installed pack versions and per-file state | Exit 0 = reported; drift is information, not failure |
| `pack uninstall` | Retire owned, pristine pack files | Exit 0 = removed or absent |
| `pack resolve` | Resolve an entry to its prompt bytes and required asset closure | Exit 0 = resolved; Exit 1 = unreadable, unowned, drifted, or concurrently mutated |
| `pack campaign` | Pin resolved bytes, verify them, or release them | Exit 0 = pinned, matched, or released; Exit 1 = halt before work |

## Hook kinds

| Kind | Blocks on | Typical use |
|---|---|---|
| `structural` | Orphaned files, ghost files, missing paths | Pre-commit |
| `interface` | Interface hash changes | Pre-commit for API-sensitive repos |
| `tension` | Never (advisory) | Surface warnings post-merge |
| `architecture-decision` | Blueprint mutations without paired decisions | Pre-commit for architecture changes |
| `all` | All error-severity findings | CI gate |

## Subscription primitive

`cairn watch` emits newline-delimited JSON events on finding changes. Schema:

```json
{"event":"finding_added|finding_resolved","timestamp":"...","finding":{...}}
```

## Integration patterns

### CI pipeline gate

```bash
cairn scan --json > /dev/null
EXIT=$?
if [ $EXIT -eq 1 ]; then
  cairn lint --json | jq '.findings[] | select(.severity == "error")'
  exit 1
fi
```

### Agent context bootstrap

```bash
cairn context --json   # Full project overview in one call
```

### Pre-commit hook

```bash
cairn hook structural --json
```

### Brownfield onboarding (agent-driven)

```bash
cairn init --from-code           # Generate proposals
cairn islands --json             # Find disconnected components
cairn onboard --json             # Suggest classifications for orphans
```

## Stability guarantees

- **Stable**: JSON envelope shape, exit codes, command names, MCP tool names
- **Semi-stable**: `data` field contents per command (additions are non-breaking, removals are versioned)
- **Unstable**: Human-mode text output (not for machine parsing)

Version the integration by checking `cairn --version`. Breaking changes to the JSON contract will bump the minor version.

## Shape authority

The committed JSON Schemas under `schemas/` are the authority for externally
consumed wire shapes. `schemas/map.schema.json` defines the deterministic
`map.json` snapshot, and `schemas/finding.schema.json` defines the
serde-derived Finding component shared by map and watch payloads. The query
findings wire (`lint --json`, `scan --json`, and the envelope's `findings`
array) extends that component with two required nullable fields: `deferred_by`,
the accepted decision deferring the finding, or `null`
(`dec.loop-selection-deferred-findings`), and `parked_by`, the `blocked` todo
whose `defers:` reference parks the finding, or `null`. Parking applies to
Info findings alone and is report-level: the finding keeps printing and only
loop selection folds it (`todo.lint-selection-folding` item 1a). Both shapes
are pinned by `EnvelopeFinding` in `schemas/envelope.schema.json`.
The lint/scan `data` payload additionally publishes a top-level `strict_green`
boolean: `true` exactly when `--strict` would exit zero over the emitted
finding set (no Error and no Warning finding). This is the machine-visible
verdict that loop selection's Info fold keys on
(`dec.loop-selection-strict-green-fold`), and under `--strict` the CLI exit
code for `lint --json` / `scan --json` reads the published field itself, so
the wire verdict and the exit code cannot disagree. The hand-rendered CLI
findings payloads (`workspace lint --json` and JSON error envelopes) carry the
same field, computed by the same predicate over the findings they print.
`schemas/work-item.schema.json` defines
the `{source,title,node,command,rank}` projection shared by status, next, and
remediate JSON. `schemas/envelope.schema.json` defines the MCP
`{project_context,rules,data,findings}` envelope, requiring
`data.schema_version` while leaving tool-specific data properties open.
`schemas/StatusResponse.schema.json` and
`schemas/RemediateResponse.schema.json` define the post-dispatch status and
remediation payloads, including their stamped schema versions. The schema
drift and serialisation tests validate these files against the Rust types and
dogfood output. Registry response-schema labels which do not yet have a
dedicated full-envelope schema remain in the explicit burn-down allowlist
tested against `TOOL_REGISTRY`; new labels cannot bypass that check.
