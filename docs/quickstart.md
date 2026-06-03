# Quickstart

Get Cairn running and scan your first project in under five minutes.

## Prerequisites

- **Rust toolchain.** Cairn uses edition 2024, which requires `rustc` 1.85 or later. Install or update via [rustup](https://rustup.rs/).
- **Git.** Needed for cloning the repo and for Cairn's own reconciliation.

Verify your Rust version:

```sh
rustc --version   # should print 1.85.0 or later
```

## Install

Cairn is not yet published on crates.io. Install from the GitHub repository using one of these methods.

### From Git (recommended)

```sh
cargo install --git https://github.com/George-RD/cairn.git
```

This builds three release binaries and places them in `~/.cargo/bin/`, which is on your `PATH` if you installed Rust through rustup:

- `cairn` -- the main CLI (scan, lint, check, ui, etc.)
- `cairn-mcp` -- MCP server for agent integration
- `cairn-lsp` -- language server protocol support (stub)

### Manual build

```sh
git clone https://github.com/George-RD/cairn.git
cd cairn
cargo build --release
```

When installing manually, copy (or symlink) the binaries you need:
```sh
cp target/release/cairn /usr/local/bin/
cp target/release/cairn-mcp /usr/local/bin/
cp target/release/cairn-lsp /usr/local/bin/
```

### Verify the installation

```sh
cairn --version
cairn-mcp --help
cairn-lsp --help
```

## First-run walkthrough

The steps below assume you have an existing project you want to describe with Cairn. If you want to explore Cairn itself, clone the repo and skip `cairn init` (the root `cairn.blueprint` is already in place).

### 1. Initialize a blueprint

Navigate to your project root and run:

```sh
cairn init
```

This creates a `cairn.blueprint` file with a skeleton structure. Open it in your editor.

### 2. Declare your modules

Edit `cairn.blueprint` to describe the systems, containers, and modules in your project. A minimal example:

```text
system my_project "My Project" {
  container backend "Backend Service" {
    module api "API Layer" {
      path "src/api"
      depends_on db
    }
    module db "Database Layer" {
      path "src/db"
    }
  }
}
```

Each `module` names a piece of your architecture. The `path` directive tells Cairn where the corresponding source files live. `depends_on` declares edges between modules.

For the full grammar reference, see [docs/blueprint.md](blueprint.md).

### 3. Scan and reconcile

```sh
cairn scan
```

Cairn reads your blueprint, walks the file system, and reconciles the two. It produces:

- `map.md` with a graph summary, node statuses (`synced`, `ghost`, `orphaned`), and any findings.
- `.cairn/log.md` with an appended scan event.
- `.cairn/state/interface-hashes.json` with Rust interface hash state (for Rust projects).

Review the output. Nodes whose declared paths match real files are `synced`. Nodes whose paths do not exist on disk are `ghost`. Files on disk that no module claims are `orphaned`.

### 4. Query the graph

Inspect a single module:

```sh
cairn get api --json
```

See a module's dependencies and dependents:

```sh
cairn neighbourhood api
```

List files owned by a module:

```sh
cairn files api
```

### 5. Check for drift

```sh
cairn lint
```

This reports findings across the project: structural errors, interface contradictions, rationale tensions, and provenance gaps. Add `--json` for machine-readable output.

### 6. Browse the graph visually

```sh
cairn ui --port 3000
```

Open `http://localhost:3000` in a browser to explore the reconciled graph interactively.

## What to do next

- Read the [blueprint grammar reference](blueprint.md) for the full syntax.
- Read the [specification](spec.md) for the conceptual model (two chains, artefact types, reconciliation).
- Run `cairn scan` before each commit to catch drift early. See [hooks](hooks.md) for Git hook integration.
- Use `cairn context` as an entry point for AI coding agents working in your repo.

## Other binaries

### cairn-mcp

The MCP server exposes Cairn's query API to agents:

```sh
cairn-mcp
```

See [docs/mcp.md](mcp.md) for the full tool list and configuration.

### cairn-lsp

Language server protocol support is planned but not yet fully implemented.
Start the stub to verify installation:

```sh
cairn-lsp
```
