# Cairn DSL

The DSL declares an architecture graph. Nodes describe systems, containers, modules, or actors. Edges describe dependencies between node IDs.

## Node Declarations

```cairn
System SaaS "Core product" id "saas" @product {
  Container API "Backend" id "saas.api" @backend {
    Module Auth "JWT authentication" id "saas.api.auth" @auth {
      path "./apps/api/auth"
      contract "./meta/contracts/api/auth.md"
      todos "./meta/todos/api/auth/"
      decisions "./meta/decisions/api/auth/"
      research "./meta/research/api/auth/"
      reviews "./meta/reviews/api/auth/"
    }
  }
}
```

Supported node keywords:

- `System`
- `Container`
- `Module`
- `Actor`

Every node requires a name, description, and `id`. Tags use `@tag`. IDs are dotted, lowercase, and may include hyphens inside a segment.

## Paths

`path` accepts a single string or a list. Downstream code always sees an array.

```cairn
path "./apps/core"
path ["./apps/core-rust", "./apps/core-ts"]
```

Leaf paths must be unique. Internal container paths are allowed, but only leaves claim paths for duplicate checking in this MVP.

## Artefact Pointers

The parser accepts these artefact pointer keys:

- `contract`
- `todos`
- `decisions`
- `research`
- `reviews`
- `sources`

Each pointer accepts a string or list of strings.

## Edges

Edges live after the node tree and reference stable IDs.

```cairn
saas.api.auth -> saas.db "Reads user records"
```

Both endpoints must exist. The description is required.

## Comments

Comments start with `#` and continue to the end of the line.
