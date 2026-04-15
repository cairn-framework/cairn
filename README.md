# Cairn Kernel MVP

Cairn reads a human-authored architecture DSL and answers structural graph queries over it. This MVP implements the phase 1 kernel: parser, in-memory ontology, query layer, and CLI.

## Install

```bash
pnpm install
pnpm build
```

The CLI entry point is `bin/cairn.js` after build.

## Commands

Use `--file` to point at a DSL file. If omitted, Cairn reads `./cairn.dsl`.

```bash
node bin/cairn.js get cairn.kernel.parser --file test/fixtures/cairn.dsl
node bin/cairn.js neighbourhood cairn.kernel.reconciliation --file test/fixtures/cairn.dsl
node bin/cairn.js order --scope cairn.kernel. --file test/fixtures/cairn.dsl
```

Every command supports `--json` for a stable machine-readable shape:

```bash
node bin/cairn.js dependents cairn.kernel.reconciler --file test/fixtures/cairn.dsl --json
```

## Development

```bash
pnpm test
pnpm lint
```

The package declares `commander` and `vitest` as requested by the MVP tasks. The CLI parser is intentionally tiny for the MVP, so the runtime path remains straightforward while the public command shape stabilizes.
