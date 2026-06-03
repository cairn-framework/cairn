# Agent Prompts for Cairn

Example prompts that coding agents can use to query a Cairn-managed project.
Each prompt is grounded in the Cairn CLI or MCP tool surface and produces
actionable, verifiable output.

## Orientation

### Understand the project structure

```
Run `cairn context` and summarise the system architecture in three sentences:
what the top-level system is, how many containers/modules there are, and
whether the map is clean or has findings.
```

### Check for blockers before editing

```
Run `cairn lint --json` and report any Error-severity findings. If there are
errors, list the affected node IDs and the remediation steps from the
findings' CTA fields.
```

## Scoped investigation

### Understand a module before changing it

```
Run `cairn get <node>` and `cairn neighbourhood <node> --include-todos` for
node <node>. Summarise: what this module does, what it depends on, what
depends on it, and any open todos or findings attached to it.
```

### Review the rationale for a module

```
Run `cairn rationale <node>` and summarise the provenance chain: what
decisions, research, and sources inform this module's existence and shape.
```

### Check dependency impact

```
Run `cairn depends <node> --transitive` and `cairn dependents <node>
--transitive`. Before I add a new dependency to <node>, confirm that the
target node exists and that adding the edge would not create a cycle.
```

## Before committing

### Verify zero orphans

```
Run `cairn scan --strict`. If it exits non-zero, list all Error and Warning
findings with their node IDs and remediation steps.
```

### Run the commit gate

```
Run `cairn hook structural` and report the outcome. If it fails, list the
blocking findings and the required fixes.
```

## Brownfield workflows

### Onboard orphaned files

```
Run `cairn onboard --json` and suggest which orphaned files should be added
to existing nodes versus which need new node declarations.
```

### Explore disconnected components

```
Run `cairn islands` and suggest edges that would connect the disconnected
components into a single graph.
```

## MCP tool usage (when available)

When the agent runtime supports MCP, prefer tool calls over shell commands:

- `cairn_get` to inspect a single node
- `cairn_neighbourhood` to explore a node's context
- `cairn_lint` to check for findings
- `cairn_context` for the project overview

Example MCP prompt:

```
Use the cairn_context MCP tool to get the project overview, then use
cairn_neighbourhood with node_id "<node>" to understand the scope around
the module I'm about to edit.
```
