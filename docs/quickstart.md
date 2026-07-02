# Quickstart

Get Cairn running and scan your first project in under five minutes.

## Prerequisites

- **Git.** Needed for cloning the repo (source install only) and for Cairn's own reconciliation.

## Install

Cairn is not yet published on crates.io. Prebuilt binaries are the fastest path; building from source is the fallback.

### Prebuilt binary (recommended)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/cairn-framework/cairn/releases/latest/download/cairn-installer.sh | sh
```

No Rust toolchain required. Installs `cairn`, `cairn-mcp`, and `cairn-lsp` for macOS (arm64, x86_64) and Linux (x86_64, arm64). Windows is not yet supported.

### From source

Requires the Rust toolchain: Cairn uses edition 2024, which needs `rustc` 1.85 or later. Install or update via [rustup](https://rustup.rs/), then verify:

```sh
rustc --version   # should print 1.85.0 or later
```

#### From Git

```sh
cargo install --git https://github.com/cairn-framework/cairn.git
```

This builds three release binaries and places them in `~/.cargo/bin/`, which is on your `PATH` if you installed Rust through rustup:

- `cairn` -- the main CLI (scan, lint, check, ui, etc.)
- `cairn-mcp` -- MCP server for agent integration
- `cairn-lsp` -- language server protocol support (stub)

#### Manual build

```sh
git clone https://github.com/cairn-framework/cairn.git
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

This creates a starter `cairn.blueprint`, a `cairn.config.yaml`, and `.cairn/AGENTS.md`, an agent-facing guide you can append to your project's `CLAUDE.md` or `AGENTS.md` so coding agents use the map (and report Cairn friction with `cairn feedback`). Open the blueprint in your editor.

If the project already has a large codebase, `cairn init --from-code` discovers modules from the source tree and writes a reviewable proposal instead.

### 2. Declare your modules

Edit `cairn.blueprint` to describe the systems, containers, and modules in your project. A minimal example:

```text
System Project "My project" id "myproject" {
    Container Backend "Backend service" id "myproject.backend" {
        Module Api "API layer" id "myproject.backend.api" {
            path "./src/api"
        }
        Module Db "Database layer" id "myproject.backend.db" {
            path "./src/db"
        }
    }
}

myproject.backend.api -> myproject.backend.db "reads and writes"
```

Each `Module` names a piece of your architecture. The `path` line tells Cairn where the corresponding source files live (declare paths for test directories too, so test files are not flagged as orphans). Dependency edges are declared after the blocks as `from.id -> to.id "label"`.

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

Inspect a single module (IDs are the dotted `id` values from the blueprint):

```sh
cairn get myproject.backend.api --json
```

See a module's dependencies and dependents:

```sh
cairn neighbourhood myproject.backend.api
```

List files owned by a module:

```sh
cairn files myproject.backend.api
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
- Use `cairn context` as an entry point for AI coding agents working in your repo, and append `.cairn/AGENTS.md` to your agent instructions.
- When Cairn itself surprises you or gets in your way, run `cairn feedback "<what happened>"`. It records the friction in `.cairn/feedback.md` and prints a prefilled issue link for the upstream tracker. If `cairn` crashes, it prints the same kind of link on its own. Nothing is ever sent automatically: every report is a link you choose to open yourself.

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
