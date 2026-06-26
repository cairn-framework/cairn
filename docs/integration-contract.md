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
| `depends <node>` | `cairn_depends` | Outbound dependency edges |
| `dependents <node>` | `cairn_dependents` | Inbound dependency edges |
| `order` | `cairn_order` | Topological sort of all nodes |
| `islands` | `cairn_islands` | Disconnected graph components |
| `lint` | `cairn_lint` | All findings (errors + warnings + info) |
| `health` | `cairn_health` | Comprehensive health assessment (clean flag, counts, findings) |
| `remediate` | `cairn_remediate` | Ordered action plan from current findings |
| `status` | `cairn_status` | Project summary (node count, finding count, etc.) |
| `rationale <node>` | `cairn_rationale` | Provenance chain (decisions, research, sources) |
| `todos <node>` | `cairn_todos` | Todo artefacts linked to the node |
| `decisions <node>` | `cairn_decisions` | Decision artefacts linked to the node |
| `research <node>` | `cairn_research` | Research artefacts linked to the node |
| `sources <node>` | `cairn_sources` | Source artefacts linked to the node |
| `changes` | `cairn_changes` | Active change directories |
| `show <change>` | `cairn_show_change` | Change details (proposal, delta, contracts) |
| `context` | `cairn_context` | Full structured project overview |
| `docstring <node>` | `cairn_docstring` | Generate a docstring for a node |
| `export` | `cairn_export` | Export project data (JSON or Markdown) |
| `onboard` | `cairn_onboard` | Suggest blueprint entries for orphaned files |
| `summarise <node>` | `cairn_summarise` | Generate a contract summary for a node |
| `ui` | — | Launch the web UI server |
| `watch` | `cairn_watch` | Watch for finding changes and emit events |

### Mutating commands (modify filesystem)

| CLI | MCP tool | Effect |
|---|---|---|
| `scan` | `cairn_scan` | Re-scan project, update interface hashes |
| `rename <old> <new>` | `cairn_rename` | Rename a node ID across all files |
| `archive <change>` | `cairn_archive` | Archive a completed change |
| `init` | `cairn_init` | Scaffold new cairn project |
| `init --from-code` | `cairn_init_from_code` | Brownfield extraction from existing code |
| `refine` | `cairn_refine` | Re-run brownfield discovery |
| `import-openspec` | `cairn_import_openspec` | Migrate openspec changes to meta/changes |
| `feedback "<message>"` | — | Record cairn friction in `.cairn/feedback.md`, print upstream issue link |
| `decision new <slug>` | — | Scaffold a new decision artefact (frontmatter + sections) |

### Draft lifecycle (semi-stable)

| CLI | MCP tool | Effect |
|---|---|---|
| `drafts` | `cairn_drafts` | List pending draft proposals |
| `draft_show <draft>` | `cairn_draft_show` | Show a draft proposal |
| `draft_edit <draft>` | `cairn_draft_edit` | Open a draft in your editor |
| `draft_discard <draft>` | `cairn_draft_discard` | Discard a draft proposal |
| `draft_accept <draft>` | `cairn_draft_accept` | Accept a draft and apply it |

### Gate commands

| CLI | Purpose | Exit semantics |
|---|---|---|
| `hook <kind>` | Pre-commit gate | Exit 0 = pass, Exit 1 = blocked |
| `accept [<change>]` | Full verification battery | JSON: `data.gate_outcome` = passed/failed/blocked |
| `check [<node>]` | Non-blocking inspection | Always reports, never blocks |

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
