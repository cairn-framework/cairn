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
| `--changes-dir <path>` | Changes directory (default: `meta/changes`) |
| `--depth <N\|all>` | `context`: cap structure depth (default 1) |
| `--scope <node>` | `context`: full detail for one subtree |
| `--version` | Print version |
| `--help` | Show help text |

## Commands

### Orientation

| Command | Description |
|---------|-------------|
| `cairn context` | Structured project overview for agents |
| `cairn status` | Show project status summary |
| `cairn health` | Comprehensive health check: lint, hooks, and module state |
| `cairn remediate` | Generate an ordered action plan from current findings |
| `cairn next` | Show the next ready unit of work |
| `cairn brief [<target>]` | Fused next-unit brief: task, binding decisions, contract, and gates. A `todo.<slug>` target names a native todo; any other target is a bead id |
| `cairn scan` | Scan the project and report findings |
| `cairn lint --node <id>` | Inspect findings for a single node (non-blocking) |
| `cairn lint` | Lint the blueprint and report findings (blocking) |
| `cairn ui_meta` | List available query commands and their request/response schemas |
| `cairn blueprint` | Show the raw blueprint file |
| `cairn graph` | Dump the full structural graph of nodes and edges |

### Node inspection

| Command | Description |
|---------|-------------|
| `cairn get <node>` | Inspect a node by ID |
| `cairn files <node>` | List files owned by a node |
| `cairn locate <symbol>` | Locate exact definitions of a public symbol across the project |
| `cairn get <node> --symbols` | Public symbols extracted from a node |
| `cairn bundle <node>` | Generate bundle: contract, decisions, dependency interfaces, and gates for a node |
| `cairn contract <node>` | Show the contract for a node |
| `cairn neighbourhood <node>` | Show a node and its neighbours |
| `cairn deps <node>` | List nodes a given node depends on (outbound) |
| `cairn deps <node> --direction in` | List nodes that depend on a given node (inbound) |
| `cairn rationale <node>` | Show rationale chain for a node |
| `cairn order` | Topological order of all nodes |
| `cairn islands` | Show connected components of the map graph |
| `cairn frontier` | Show buildable-now (ready) and blocked ghost nodes |

### Artefacts

| Command | Description |
|---------|-------------|
| `cairn decisions <node>` | List decisions linked to a node |
| `cairn todos [node]` | List todos for a node and its descendants, or project-wide when the node is omitted |
| `cairn research <node>` | List research linked to a node |
| `cairn sources <node>` | List sources linked to a node |
| `cairn decision new <slug>` | Scaffold a new decision artefact |
| `cairn todo new <slug> --node <id>` | Scaffold a new todo artefact |
| `cairn todo set <slug> <open\|in_progress\|done\|blocked>` | Set a todo's status via a surgical frontmatter edit (`dec.todo-write-surface`) |
| `cairn gap <node> --question "<text>"` | Log an unresolved question as a proposed decision artefact |

### Optional integrations

| Command | Description |
|---------|-------------|
| `cairn backlog <node>` | List beads (issues) linked to a node via its `cairn-node:<id>` label. Reads `.beads/issues.jsonl` when present; this repo's own task tracking uses native Todo artefacts (`cairn todo new`, above), not beads. |
| `cairn beads <node>` | List backlog beads linked to a node |

### Changes

| Command | Description |
|---------|-------------|
| `cairn change list` | List active changes |
| `cairn change new <change-id>` | Scaffold a new change directory |
| `cairn change show <change-id>` | Show details of a change |
| `cairn change accept [<change-id>]` | Run acceptance gate for a change |
| `cairn change apply <change-id>` | Apply a completed change (alias: `archive`) |

`cairn change accept` runs a language-aware verification battery, then (when a
change id is given) `cairn lint --strict <id>` and suggested-edge triage.

Language battery selection:

1. If `cairn.config.yaml` has a top-level `gates:` key, run exactly those steps
   (an empty list means zero language steps).
2. Else if the project language is Rust, run the cargo battery
   (`build`, `clippy -D warnings`, `fmt --check`, `test --workspace --locked`).
3. Else skip the language battery with an informational `skipped` finding
   (does not fail the gate). Configure `gates:` to run project-specific checks.

Example `gates:` block:

```yaml
gates:
  - name: typecheck
    command: tsc --noEmit
  - name: unit
    command: bun test
```

Each `command` is executed directly as argv: it is split on whitespace into
program + arguments. There is no shell, so quoting, pipes, redirects, and
operators are not supported. Prefer simple commands without spaces in args.

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
| `cairn hook install` | Install a Cairn-managed pre-commit hook |
| `cairn hook install --pre-push` | Install a Cairn-managed pre-push hook |
| `cairn hook status --pre-push` | Report whether the Cairn pre-push hook is installed |
| `cairn hook uninstall --pre-push` | Remove the Cairn-managed pre-push hook |
| `cairn hook status` | Report whether a Cairn hook is installed |
| `cairn hook uninstall` | Remove a Cairn-managed hook |

### Agent pack

| Command | Description |
|---------|-------------|
| `cairn pack install` | Install the agent pack and record an ownership manifest |
| `cairn pack update` | Refresh pristine pack files and backfill missing ones |
| `cairn pack status` | Report installed pack versions and per-file state |
| `cairn pack uninstall` | Retire pack files cairn owns and that are still pristine |

### Summariser and drafts (JSON-only)

These commands require the `--json` flag and are grouped under one `draft` command.

| Command | Description |
|---------|-------------|
| `cairn draft create <node>` | Generate a contract summary for a node |
| `cairn draft list` | List pending draft proposals |
| `cairn draft show <draft-id>` | Show a draft proposal |
| `cairn draft edit <draft-id>` | Open a draft in your editor |
| `cairn draft accept <draft-id>` | Accept a draft and apply it |
| `cairn draft discard <draft-id>` | Discard a draft proposal |

### Other

| Command | Description |
|---------|-------------|
| `cairn export` | Export project data |
| `cairn ui` | Launch the web UI |
| `cairn rename <old-id> <new-id>` | Rename a node ID across the project (JSON-only) |
| `cairn watch` | Watch for finding changes and emit JSON events |
| `cairn import-openspec` | Migrate openspec changes to meta/changes/ |
| `cairn docstring <node>` | Generate a docstring for a node (JSON-only) |
| `cairn feedback "<message>" [--area <area>] [--severity <level>]` | Record cairn friction in `.cairn/feedback.md` and print a prefilled upstream issue link; optional area and severity land in the log entry, the issue body, and `--json` output |
| `cairn workspace <status\|lint\|frontier>` | Aggregate status, lint, and frontier queries across a `cairn.workspace` |

### Clean-state workflow

The `health` command produces a comprehensive health report combining lint findings, hook results, and module state counts. Use it to verify the project is in a clean state before merging.

The `remediate` command analyzes findings and produces a prioritized list of actions. Run it when `cairn scan` or `cairn lint` reports issues. The actions guide you toward a clean state:

1. `fix_blueprint` - Parse or integrity errors must be fixed manually first.
2. `init_from_code` or `refine` - Reconcile blueprint drift with code.
3. `draft create --json` - Update contracts after interface changes.
4. `add_decision` - Record decisions for blueprint changes.

```bash
# 1. Check current health
cairn health --json
# 2. If not clean, get remediation actions
cairn remediate --json
# 3. Execute suggested actions (e.g., refine, draft create)
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
