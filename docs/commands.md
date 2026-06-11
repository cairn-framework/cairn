# Command Reference

Complete reference for the `cairn` CLI.

## Installation

```bash
cargo install --git https://github.com/cairn-framework/cairn.git
```

This installs `cairn`, `cairn-mcp`, and `cairn-lsp`.

## Global flags

| Flag | Description |
|------|-------------|
| `--file <path>` | Blueprint file path (default: `cairn.blueprint`) |
| `--json` | Output JSON instead of human-readable text |
| `--strict` | Exit non-zero on Warning findings (scan/lint only) |
| `--help` | Show help text |

## Commands

### Orientation

| Command | Description |
|---------|-------------|
| `cairn context` | Structured project overview for agents |
| `cairn status` | Show project status summary |
| `cairn health` | Comprehensive health check: lint, hooks, and module state |
| `cairn remediate` | Generate an ordered action plan from current findings |
| `cairn scan` | Scan the project and report findings |
| `cairn check [<node>]` | Inspect findings for a node or project (non-blocking) |
| `cairn lint` | Lint the blueprint and report findings (blocking) |
### Node inspection

| Command | Description |
|---------|-------------|
| `cairn get <node>` | Inspect a node by ID |
| `cairn files <node>` | List files owned by a node |
| `cairn contract <node>` | Show the contract for a node |
| `cairn neighbourhood <node>` | Show a node and its neighbours |
| `cairn depends <node>` | List nodes a given node depends on |
| `cairn dependents <node>` | List nodes that depend on a given node |
| `cairn rationale <node>` | Show rationale chain for a node |
| `cairn order` | Topological order of all nodes |
| `cairn islands` | Show connected components of the map graph |

### Artefacts

| Command | Description |
|---------|-------------|
| `cairn decisions <node>` | List decisions linked to a node |
| `cairn todos <node>` | List todos linked to a node |
| `cairn research <node>` | List research linked to a node |
| `cairn sources <node>` | List sources linked to a node |

### Changes

| Command | Description |
|---------|-------------|
| `cairn changes` | List active changes |
| `cairn change new <change-id>` | Scaffold a new change directory |
| `cairn change tasks <change-id>` | List task beads for a change |
| `cairn change apply <change-id>` | Claim a change and all its open tasks |
| `cairn show <change-id>` | Show details of a change |
| `cairn accept [<change-id>]` | Run acceptance gate for a change |
| `cairn archive <change-id>` | Archive a completed change |

### Brownfield

| Command | Description |
|---------|-------------|
| `cairn init` | Scaffold a new cairn project |
| `cairn init --from-code` | Discover modules from existing code |
| `cairn refine` | Re-run brownfield discovery and write a timestamped change |
| `cairn onboard` | Suggest blueprint entries for orphaned files |

### Hooks and gates

| Command | Description |
|---------|-------------|
| `cairn hook structural` | Run structural verification hook |
| `cairn hook interface` | Run interface verification hook |
| `cairn hook tension` | Run tension verification hook (advisory) |
| `cairn hook all` | Run all verification hooks |
| `cairn hook architecture-decision` | Run architecture decision gate |

### Summariser and drafts (JSON-only)

These commands require the `--json` flag.

| Command | Description |
|---------|-------------|
| `cairn summarise <node>` | Generate a contract summary for a node |
| `cairn drafts` | List pending draft proposals |
| `cairn draft_show <draft-id>` | Show a draft proposal |
| `cairn draft_edit <draft-id>` | Open a draft in your editor |
| `cairn draft_accept <draft-id>` | Accept a draft and apply it |
| `cairn draft_discard <draft-id>` | Discard a draft proposal |

### Other

| Command | Description |
|---------|-------------|
| `cairn export` | Export project data |
| `cairn ui` | Launch the web UI |
| `cairn rename <old-id> <new-id>` | Rename a node ID across the project |
| `cairn watch` | Watch for finding changes and emit JSON events |
| `cairn import-openspec` | Migrate openspec changes to meta/changes/ |
| `cairn docstring <node>` | Generate a docstring for a node |
| `cairn feedback "<message>"` | Record cairn friction in `.cairn/feedback.md` and print a prefilled upstream issue link |

### MAS orchestration
| Command | Description |
|---------|-------------|
| `cairn health` | Report project health (JSON only) |
| `cairn remediate` | Suggest remediation actions for findings (JSON only) |

The `health` command produces a comprehensive health report combining lint findings, hook results, and module state counts. Use it to verify the project is in a clean state before merging.

The `remediate` command analyzes findings and produces a prioritized list of actions. Run it when `cairn scan` or `cairn lint` reports issues. The actions guide you toward a clean state:

1. `fix_blueprint` - Parse or integrity errors must be fixed manually first.
2. `init_from_code` or `refine` - Reconcile blueprint drift with code.
3. `summarise` - Update contracts after interface changes.
4. `add_decision` - Record decisions for blueprint changes.
#### Clean-state workflow

```bash
# 1. Check current health
cairn health --json
# 2. If not clean, get remediation actions
cairn remediate --json
# 3. Execute suggested actions (e.g., refine, summarise)
cairn refine
# 4. Verify clean state
cairn health --json

```

A project is considered clean when `health` reports `clean: true` (zero errors, warnings, and hook pass).

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success (or check with no blocking findings) |
| 1 | Success with findings or operational error |
| 2 | Usage error |

## JSON envelope

All `--json` output uses a consistent envelope:

```json
{"command":"<name>","status":"ok|error","data":{...}}
```

## MCP server

The `cairn-mcp` binary exposes Cairn queries as MCP tools. See `docs/mcp.md` for the full tool list.

## Examples

```bash
# Project overview
cairn context

# Check for blockers before committing
cairn lint --json

# Inspect a module
cairn get cairn.kernel.scanner
cairn neighbourhood cairn.kernel.scanner

# CI verification gate
cairn scan --strict

# Browse the graph
cairn ui --port 3000
```

## Startup

After installation, the three binaries are available:

```bash
# Main CLI
cairn --version

# MCP server (stdio transport)
cairn-mcp

# LSP server (stdio transport)
cairn-lsp
```

All binaries support `--version` and print a version label on startup.
