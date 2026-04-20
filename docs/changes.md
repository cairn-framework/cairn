# Cairn Change Directories

Change directories hold proposed map updates without changing current truth. Cairn discovers active changes under `meta/changes/<change-id>/` when the directory contains `proposal.md`. `meta/changes/archive/` is reserved for completed changes and is ignored by active-change discovery.

## Layout

```text
meta/changes/<change-id>/
  proposal.md
  design.md
  blueprint.delta
  contracts/
  todos/
  decisions/
  research/
  sources/
  reviews/
```

The artefact subdirectories mirror `meta/`. A file at `meta/changes/add-auth/decisions/auth.md` targets `meta/decisions/auth.md` when archived.

## Blueprint Deltas

`blueprint.delta` supports node and edge sections:

```markdown
## ADDED Nodes
Module Billing "billing" id "app.billing" {
    path "./src/billing"
}

## MODIFIED Nodes
Module Auth "auth" id "app.auth" {
    path "./src/auth"
}

## REMOVED Nodes
- app.old

## RENAMED Nodes
- app.auth -> app.identity

## ADDED Edges
app.identity -> app.billing "calls"

## REMOVED Edges
app.auth -> app.old "legacy"

## RENAMED Edges
app.auth -> app.billing "calls" => app.identity -> app.billing "calls"
```

Added and modified nodes use normal blueprint node declarations. Removed nodes name node IDs. Renamed nodes use `old -> new`. Edge operations use normal edge syntax, and renamed edges use `old edge => new edge`.

## Artefact Operations

Every changed artefact must include `operation` in frontmatter:

```markdown
---
operation: modified
id: dec.auth
nodes: [app.identity]
status: accepted
date: 2026-04-20
---
# Decision
```

Supported operations are `added`, `modified`, `removed`, and `renamed`. Renamed artefacts also require `renamed_from`, pointing to the source file in the main tree:

```markdown
---
operation: renamed
renamed_from: meta/decisions/auth.md
id: dec.identity
nodes: [app.identity]
status: accepted
date: 2026-04-20
---
# Decision
```

During archive, Cairn strips `operation` and `renamed_from` before writing artefacts to the main tree.

## Archive Guarantees

`cairn archive <change-id>` validates references against current truth, snapshots every file it will mutate, applies operations in this order, and rolls back on failure:

1. Renamed
2. Removed
3. Modified
4. Added

After applying the delta, Cairn runs a validation scan while the change is still active. If the resulting graph has structural errors, Cairn restores the snapshots and leaves the change directory in place. On success, the change directory moves to `meta/changes/archive/YYYY-MM-DD-<change-id>/`, generated outputs are refreshed, and `.cairn/log.md` receives an archive event.

## Rename Workflow

`cairn rename <old-id> <new-id>` creates `meta/changes/rename-<old-id>-to-<new-id>/`. It writes a node rename delta, rewrites affected edge endpoints in the delta, and copies artefacts whose frontmatter references the old ID into the change directory with modified frontmatter. The main tree is not changed until `cairn archive` succeeds.

## Queries

- `cairn changes` lists active change IDs, proposal titles, operation counts, and validation findings.
- `cairn show <change-id>` prints the proposal, blueprint delta summary, and artefact operation summary.
- `cairn neighbourhood <node> --include-changes` adds proposed operations related to the node and its direct neighbours.
- `cairn status` includes active changes alongside open todos and recent log entries.
