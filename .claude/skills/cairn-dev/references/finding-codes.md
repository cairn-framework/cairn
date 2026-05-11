# Finding Codes Reference

All cairn lint finding codes, their severities, and remediation steps.

## Error findings (block hooks)

These findings cause `cairn hook structural` and `cairn hook all` to exit 1 (fail).

### CAIRN_ORPHANED_FILE

**Severity:** Error
**Meaning:** A file exists on disk but no module in `cairn.blueprint` claims it via a `path` declaration.
**Remediation:**
1. Add the file's directory or path to an existing module's `path` declaration, OR
2. Declare a new Module in `cairn.blueprint` with a `path` covering this file, OR
3. Add the path to `exclude_paths` in `cairn.config.yaml` if it's intentionally outside the graph

### CAIRN_GHOST_FILE

**Severity:** Error
**Meaning:** A module's `path` declaration references a file or directory that doesn't exist on disk.
**Remediation:** Either create the missing file/directory or update the `path` in `cairn.blueprint` to the correct location.

### CAIRN_INTERFACE_HASH_CHANGED

**Severity:** Error
**Meaning:** A module's public interface (exported symbols, function signatures) has changed since the last `cairn scan` wrote `.cairn/state/interface-hashes.json`.
**Remediation:** Run `cairn scan` to update the baseline. If the interface change was intentional, the new hash becomes the baseline. If unintentional, revert the interface change.

### CAIRN_REVIEW_UNKNOWN_NODE

**Severity:** Error
**Meaning:** A review, contract, or artefact references a node ID that doesn't exist in the blueprint.
**Remediation:** Fix the `node:` or `nodes:` field in the artefact's frontmatter to reference a valid node ID. Run `cairn get <id>` to verify node existence.

### CAIRN_CYCLE_DETECTED

**Severity:** Error
**Meaning:** The dependency edge graph contains a cycle. Cairn requires a DAG (directed acyclic graph).
**Remediation:** Remove or redirect one of the edges in the cycle. Use `cairn depends <node> --transitive` to trace the dependency chain.

### CAIRN_CLI_MISSING_NODE

**Severity:** Error
**Meaning:** A CLI command that requires a node argument was called without one.
**Remediation:** Provide a node ID as the argument: `cairn get <node.id>`.

### CAIRN_BLUEPRINT_CHANGE_NO_DECISION (CA002)

**Severity:** Error
**Meaning:** The blueprint's structural shape changed for a node (module added, removed, or reassigned across containers) but no decision artefact has that node's ID in its `nodes` field.
**Remediation:** Author a decision artefact covering the changed node. The decision should explain why the structural change was made. Any decision status (proposed, accepted) satisfies the gate. First scan creates a baseline; the gate only fires on subsequent scans when a previous snapshot exists.

## Warning findings (advisory)

These findings are surfaced in `cairn hook tension` and in `cairn lint` output but do not block commits.

### CAIRN_PROVENANCE_NO_DECISION (CA001)

**Severity:** Warning
**Meaning:** A leaf node in the blueprint has no accepted decision artefact covering it (no decision has this node's ID in its `nodes` field).
**Remediation:** Author a decision artefact with `nodes: [<this-node-id>]` and `status: accepted` explaining why this module exists and how it's shaped. Only fires when at least one decision exists in the project (avoids noise in fresh projects).

## CLI errors (not scan findings)

These are reported when a command is invoked incorrectly. They are not produced by `cairn scan` or `cairn lint`.

### CAIRN_CLI_MISSING_NODE

See the Error findings section above.

## Registry format

Error codes are registered in `openspec/registries/error-codes.md` with the format:

```
| CXNNN | CAIRN_FULL_CODE_NAME | severity | description | issue # |
```

Categories:
- `CS` - Scanner/structural findings
- `CI` - Interface findings
- `CA` - Artefact findings
- `CH` - Hook findings
- `CC` - CLI findings
