# Cairn blueprint

The blueprint declares an architecture graph. Nodes describe systems, containers, modules, or actors. Edges describe dependencies between node IDs.

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

Leaf paths must be unique. Internal container paths are allowed, but only leaves claim paths for duplicate checking in the kernel.

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

During Rust reconciliation, Cairn compares these declared edges with observed `use` paths and `mod` declarations. A declared edge with no matching observed dependency is reported as an advisory rationale tension. An observed dependency with no declared edge is also reported as a rationale tension and includes the source location when available.

Ambiguous observations, such as an import name that could map to more than one owning node, are informational warnings. They do not create structural errors.

## Docstring Fact Lines

Rust module docstrings may include exact Cairn fact lines. Cairn reads `//!` comments in `lib.rs`, `main.rs`, and `mod.rs`, plus `///` comments immediately attached to `mod` declarations.

Supported fact keys are case-sensitive:

```rust
//! Cairn-ID: saas.api.auth
//! Cairn-Name: JWT authentication
//! Cairn-Depends: saas.db
//! Cairn-Depends: saas.crypto
//! Cairn-Tags: auth, api
//! Cairn-Contract: ./meta/contracts/api/auth.md#Public interface
```

Multiple `Cairn-Depends` lines are combined. `Cairn-Tags` is a comma-separated list with ASCII whitespace trimmed. Free-form prose is ignored by drift detection. Unknown `Cairn-` keys are reported as informational warnings. Unknown node IDs and contradictions with map facts are reported as rationale tensions.

Use `cairn docstring <node> [--language <lang>]` to generate a grounded template. Supported languages are Rust, Python, TypeScript, and Go. Generated templates provide structural facts only; a human or agent still needs to complete the prose.

## Comments

Comments start with `#` and continue to the end of the line.
