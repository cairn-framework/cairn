---
name: cairn-dev
description: "Activate when working in a repo that contains cairn.blueprint, when the user says 'add a module', 'update the blueprint', 'check cairn', 'cairn lint fails', 'add a decision', 'run cairn scan', or when CLAUDE.md references cairn. Also activate when navigating architecture, adding files to an existing cairn project, or interpreting cairn findings."
---

# Cairn Development Guide

How to develop with and navigate a cairn-managed codebase. Cairn is a graph-based architecture map: you author a `.blueprint` file declaring your system's structure, and cairn reconciles it against actual code, surfacing drift, orphans, and provenance gaps.

## Orientation (run first)

Before making changes, understand the project's current state:

```bash
cairn context              # Structural overview: nodes, edges, artefacts, findings
cairn lint --json           # Current findings (errors block hooks, warnings are advisory)
cairn neighbourhood <node>  # Scope around the node you're about to change
```

If `cairn context` shows findings, triage them before adding new ones.

## Command quick-reference

| Use case | Command | Key flags |
|---|---|---|
| **Orientation** | `cairn context` | `--json` |
| **Full scan** | `cairn scan` | `--json` |
| **Lint findings** | `cairn lint` | `--json` (exit 1 on errors) |
| **Non-blocking lint** | `cairn check [<node>]` | always exit 0 |
| **Inspect node** | `cairn get <node>` | `--json` |
| **Node + neighbours** | `cairn neighbourhood <node>` | `--json`, `--include-todos`, `--include-changes`, `--include-orphans` |
| **Node files** | `cairn files <node>` | `--json` |
| **Dependency graph** | `cairn depends <node>` / `cairn dependents <node>` | `--json`, `--transitive` |
| **Build order** | `cairn order` | `--json` |
| **Provenance trail** | `cairn rationale <node>` | `--json` |
| **Decisions** | `cairn decisions <node>` | `--json`, `--status accepted` |
| **Todos** | `cairn todos <node>` | `--json`, `--status open` |
| **Research** | `cairn research <node>` | `--json` |
| **Sources** | `cairn sources <node>` | `--json` |
| **Contracts** | `cairn contract <node>` | `--json` |
| **Project status** | `cairn status` | `--json` |
| **Commit gate** | `cairn hook <structural\|interface\|tension\|all>` | `--json` |
| **Active changes** | `cairn changes` | `--json` (required) |
| **Change details** | `cairn show <change-id>` | `--json` (required) |
| **Brownfield onboard** | `cairn onboard` | `--json` |
| **Record cairn friction** | `cairn feedback "<message>"` | `--json`; logs to `.cairn/feedback.md`, prints upstream issue link |
| **Brownfield extract** | `cairn init --from-code` | `--force` (overwrite existing) |
| **Brownfield refine** | `cairn refine` | writes timestamped change |
| **Disconnected islands** | `cairn islands` | `--json` |
| **Acceptance gate** | `cairn accept [<change-id>]` | `--json` (gate_outcome in data) |
| **Export** | `cairn export --format <json\|dot> --output <path>` | full graph export |
| **Bootstrap** | `cairn init` | creates blueprint, config, meta dirs |
| **Web explorer** | `cairn ui` | `--port <N>` |

Node IDs use dotted notation (e.g. `cairn.kernel.scanner`). Run `cairn get <id>` to verify a node exists.

## The development loop

### Before coding

1. `cairn context` to see the full graph state
2. `cairn neighbourhood <node> --include-todos` for the node you're about to modify
3. `cairn rationale <node>` to understand why it's shaped the way it is

### While coding

When adding new source files:
1. Check if the file falls under an existing node's `path` declaration in `cairn.blueprint`
2. If not, either extend an existing node's `path` or declare a new Module
3. Run `cairn scan` to verify zero orphans

When adding a dependency between modules:
1. `cairn depends <target> --transitive` to check for cycles
2. Add the edge in `cairn.blueprint`: `from.id -> to.id "relationship label"`
3. `cairn scan` to verify

### Before committing

```bash
cairn scan          # Zero findings is the target
cairn hook all      # Exit 0 means commit is safe
```

If `cairn hook` exits 1, read the findings. Error-severity findings block; fix them before committing.

## Blueprint syntax

```
System <TypeLabel> "<description>" id "<dotted-id>" [@tag...] {

    Container <TypeLabel> "<description>" id "<dotted-id>" [@tag...] {

        Module <TypeLabel> "<description>" id "<dotted-id>" [@tag...] {
            path "<relative-path>"        # multiple path lines allowed
            contract "<path>"
            decisions "<dir>"
            todos "<dir>"
            research "<dir>"
            sources "<dir>"
            reviews "<dir>"
        }
    }
}

# Edges (outside blocks)
from.id -> to.id "relationship label"
```

**Node types:** System (top-level), Container (grouping), Module (leaf with code), Actor (external)
**Tags:** `@tag` annotations are informational, not structural
**Paths:** relative to repo root, can point to files or directories

## Artefact authoring

### Decision (most common for architecture changes)

```yaml
---
id: dec.<short-name>
nodes: [<node.id>, <another.node.id>]
status: accepted          # proposed | accepted | deprecated | superseded
date: 2026-05-11
informed_by: [res.<id>]   # optional: research that informed this
supersedes: [dec.<old>]   # optional: prior decision this replaces
---

Body text explaining the decision rationale.
```

Place in the node's declared `decisions` directory, or in `meta/decisions/` by convention.

### Todo

```yaml
---
node: <node.id>
status: open              # open | in_progress | done | blocked
created: 2026-05-11
satisfies: <change-id>    # optional: links to a change
---

Description of the work item.
```

### Research

```yaml
---
id: res.<short-name>
nodes: [<node.id>]
sources: [src.<id>]
date: 2026-05-11
---

Research findings and analysis.
```

### Source

```yaml
---
id: src.<short-name>
file: <path-or-url>
verification: verified    # verified | external | unverified
type: <free-text>
date: 2026-05-11
---
```

## Brownfield workflow (existing codebases)

For projects without a blueprint:

```bash
cairn init --from-code     # Discover modules, generate change proposal
# Review the generated blueprint.delta in meta/changes/<change-id>/
# Review generated contracts in meta/changes/<change-id>/contracts/
cairn scan                 # Verify the new blueprint
```

For iterative refinement after the initial extraction:

```bash
cairn refine               # Re-discover, writes timestamped change dir
cairn islands              # See disconnected components that may need edges
cairn neighbourhood <node> --include-orphans  # Find files near a node not yet claimed
cairn onboard              # Group orphaned files by directory with suggestions
```

The brownfield flow generates proposals (not final state). Review and edit the generated `blueprint.delta` and contracts before accepting.

## Understanding findings

Findings have three severities:

| Severity | Blocks hooks? | Action |
|---|---|---|
| `Error` | Yes (structural, interface, all) | Must fix before committing |
| `Warning` | No (surfaced in tension hook) | Should address; won't block |
| `Info` | No | Informational only |

Common finding codes:

| Code | Severity | Meaning |
|---|---|---|
| `CAIRN_RECONCILE_ORPHANED_FILE` | Error | File exists on disk but no node claims it via `path` |
| `CAIRN_INTEGRITY_DUPLICATE_ID` | Error | Same node ID appears more than once in the blueprint |
| `CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT` | Error | Edge references a node ID not in the graph |
| `CAIRN_INTERFACE_HASH_CHANGED` | Error | Module's public interface changed since last scan |
| `CAIRN_ORDER_CYCLE` | Error | Dependency cycle found in the edge graph |
| `CAIRN_REVIEW_UNKNOWN_NODE` | Error | Review/contract references a node ID not in the graph |
| `CAIRN_PROVENANCE_NO_DECISION` | Warning | Leaf node has no accepted decision covering it |
| `CAIRN_SOURCE_UNVERIFIED` | Info | Source artefact has not been verified |

Use `cairn lint --json | jq '.findings[] | select(.severity == "error")'` to filter for blockers.

## Hook semantics

| Hook kind | Blocks on | Use case |
|---|---|---|
| `structural` | Error findings + active-change conflicts | Pre-commit gate for code changes |
| `interface` | Interface hash changes + conflicts | Pre-commit gate for API changes |
| `tension` | Never (advisory) | Surface warnings without blocking |
| `all` | Errors + interface changes + conflicts | Strictest gate |

The pre-commit hook typically runs `cairn hook structural`. CI can run `cairn hook all` for stricter enforcement.

## Graph navigation patterns

**Before modifying a module:** `cairn rationale <node>` shows the provenance chain (decisions, research, sources) explaining why it's shaped the way it is. Respect existing decisions.

**Before adding a dependency:** `cairn dependents <node> --transitive` and `cairn depends <node> --transitive` reveal the full impact graph. Check for cycles.

**Understanding feature scope:** `cairn neighbourhood <node> --include-changes --include-todos` shows the node in context with its active work items.

**Topological ordering:** `cairn order` gives a valid build/dependency order for all nodes.

## When NOT to use cairn

- Don't use `cairn scan` as a substitute for `cargo build` or language-specific compilation. They check different things.
- Don't use `cairn check` to gate commits. Use `cairn hook` which has correct blocking semantics.
- Don't modify `cairn.blueprint` without running `cairn scan` afterward to verify.
- Don't add artefact files without ensuring the node declares the artefact directory in its blueprint entry.

## JSON envelope contract

All commands with `--json` produce a consistent envelope:

```json
{"command":"<name>","status":"ok|error","data":{...}}
```

- `status: "ok"` means the command succeeded (findings may still exist in data)
- `status: "error"` means the command failed or verification was incomplete
- `accept --json` includes `data.gate_outcome` ("passed", "failed", or "blocked")
- Exit codes: 0 = clean success, 1 = success with findings or operational error, 2 = usage error

## User-facing copy

All user-facing CLI strings live in `docs/design-system/copy.toml`. Do not hardcode messages in Rust source.

```rust
// In src/cli/ code:
use super::copy;
let msg = copy::lookup("empty-states.cli-no-blueprint");
```

Keys use dotted notation matching the TOML table structure. The `lookup()` function returns the key itself as fallback if missing. Finding-code entries use inline tables with `heading`, `body`, and `cta` fields:

```toml
[findings.codes]
CAIRN_RECONCILE_ORPHANED_FILE = { heading = "Orphaned file", body = "...", cta = "..." }
```

When adding new user-facing messages: add the entry to copy.toml first, then wire it via `copy::lookup()`.

## MCP integration

If your runtime supports MCP, `cairn-mcp` wraps the query API as MCP tools. Prefer MCP tool calls over shell invocations for `get`, `neighbourhood`, `lint`, `rationale` when available. Shell invocations are the fallback.

## References (load on demand)

Don't read these by default. Read only when the specific topic comes up:

- `references/blueprint-syntax.md`: full grammar with all node types and field declarations
- `references/artefact-schemas.md`: complete YAML frontmatter schemas for all artefact types
- `references/finding-codes.md`: all lint finding codes, severities, and remediation steps

## When this skill is wrong

If this skill gives incorrect guidance about cairn commands or behavior:

```
The cairn-dev skill told me to [X], but cairn actually [Y].
Please update the skill at .claude/skills/cairn-dev/SKILL.md
to correct [specific section].
```
