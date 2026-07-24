# Blueprint Syntax Reference

Complete grammar for `.blueprint` files.

## Top-level structure

A blueprint file contains node declarations and edge declarations.

```
# Comments start with #

System <TypeLabel> "<description>" id "<dotted-id>" [@tag...] {
    <child nodes>
}

# Edges
from.id -> to.id "relationship label"
```

## Node declarations

Four node kinds, each nestable inside System or Container:

### System (top-level only)

```
System <TypeLabel> "<description>" id "<root-id>" [@tag...] {
    # Contains Containers, Modules, or Actors
}
```

The system node is the root. Its `id` is the prefix for all child IDs.

### Container (grouping)

```
Container <TypeLabel> "<description>" id "<parent-id>.<name>" [@tag...] {
    # Contains Modules, other Containers, or Actors
}
```

Containers have no code targets. They group related modules.

### Module (leaf with code)

```
Module <TypeLabel> "<description>" id "<parent-id>.<name>" [@tag...] {
    path "<relative-path>"          # required, can repeat for multiple paths
    contract "<path>"               # optional
    decisions "<directory>"         # optional
    todos "<directory>"             # optional
    research "<directory>"          # optional
    sources "<directory>"           # optional
    reviews "<directory>"           # optional
}
```

Modules are the primary leaf nodes. Each `path` line maps to files or directories that the reconciler scans.

### Actor (external entity)

```
Actor <TypeLabel> "<description>" id "<parent-id>.<name>" [@tag...] {
    # Usually no fields; represents external systems or users
}
```

## Edge declarations

Edges are declared outside any block, at file scope:

```
from.node.id -> to.node.id "description of the relationship"
```

Both node IDs must exist in the node declarations above. Edges form a directed graph used for dependency analysis, cycle detection, and topological ordering.

## ID conventions

- IDs use dotted notation: `system.container.module`
- The system ID is the root prefix
- Child IDs extend the parent: if the system is `myapp`, a kernel container is `myapp.kernel`, and a parser module inside it is `myapp.kernel.parser`
- IDs are case-sensitive

## Tags

Tags are prefixed with `@` and are informational annotations:

```
Module Parser "Parses input files" id "myapp.parser" @core @v2 {
    path "./src/parser"
}
```

Tags don't affect behavior. They're useful for filtering and documentation.

## Path declarations

- Paths are relative to the repository root
- A path can point to a file (`./src/main.rs`) or a directory (`./src/parser`)
- Directory paths claim all files recursively under that directory
- Multiple `path` lines are allowed per module
- Files not claimed by any module's path are reported as `CAIRN_RECONCILE_ORPHANED_FILE` (Info severity)
- Paths referencing nonexistent files produce a `NodeState::Ghost` node (not a lint finding; ghost nodes are surfaced by `cairn frontier`)

## Complete example

```
System MyApp "A web application with API and frontend" id "myapp" @webapp {

    Container Backend "Server-side services" id "myapp.backend" @server {

        Module API "REST API endpoints and routing" id "myapp.backend.api" @http {
            path "./src/api"
            contract "meta/contracts/api.md"
            decisions "meta/decisions/api"
            todos "meta/todos/api"
        }

        Module Database "Data access layer and migrations" id "myapp.backend.db" {
            path "./src/db"
            path "./migrations"
        }
    }

    Module Config "Shared configuration and environment" id "myapp.config" {
        path "./src/config.rs"
    }

    Actor ExternalPayment "Third-party payment processor" id "myapp.payment" @external {
    }
}

# Dependencies
myapp.backend.api -> myapp.backend.db "Queries and persists data"
myapp.backend.api -> myapp.config     "Reads configuration"
myapp.backend.api -> myapp.payment    "Processes payments"
myapp.backend.db  -> myapp.config     "Reads connection settings"
```
