# CLI Capability Spec

## ADDED Requirements

### Requirement: Provide a cairn check inspection subcommand

The CLI SHALL expose a `cairn check` subcommand that renders the existing `query::lint` finding stream with inspection semantics. The subcommand SHALL always exit with code zero regardless of finding severity, distinguishing it from `cairn lint` which retains gate semantics for hooks. The subcommand SHALL accept an optional positional `node` argument that filters the finding stream to findings whose `node` field equals the requested node ID, providing scope-toggle parity with the webui Findings panel.

#### Scenario: Whole-map inspection without arguments

- **GIVEN** a working directory with a parsed blueprint and a non-empty finding stream
- **WHEN** the user runs `cairn check`
- **THEN** the CLI prints labelled human-readable output covering every finding in the stream
- **AND** the process exits with code zero regardless of whether any finding has severity `Error`, `Warning`, or `Info`

#### Scenario: Node-scoped inspection with a positional argument

- **GIVEN** a parsed blueprint with findings on multiple nodes
- **WHEN** the user runs `cairn check saas.api.auth`
- **THEN** the CLI prints only findings whose `node` field equals `saas.api.auth`
- **AND** the process exits with code zero

#### Scenario: Inspection delegates to the same library service as lint

- **GIVEN** the cairn library defines `query::lint(graph)`
- **WHEN** either `cairn check` or `cairn lint` runs
- **THEN** both commands call `query::lint(graph)` to obtain the finding stream
- **AND** the CLI rendering layer is the only point of semantic divergence between the two commands

#### Scenario: Inspection has no JSON mode in this phase

- **GIVEN** the user runs `cairn check --json`
- **WHEN** the CLI parses the arguments
- **THEN** the CLI rejects the unknown flag with a clear error pointing the user at `cairn lint --json` for JSON output

### Requirement: Empty-state CTAs name the next move on the CLI

The CLI SHALL render empty-state output that names the next move whenever a command would otherwise display zero-data silence. This applies to two specific surfaces: the no-arguments invocation (when no blueprint file is present in the working directory) and the clean-map result of `cairn lint` or `cairn check` (when the finding stream is empty). Empty-state copy SHALL be sourced from the centralised copy file at `docs/design-system/copy.toml` so the verbal language stays consistent across CLI and webui surfaces.

#### Scenario: No-blueprint invocation renders a CTA

- **GIVEN** a working directory that contains no `cairn.blueprint` file
- **WHEN** the user runs `cairn` with no arguments
- **THEN** the CLI renders the heading, body, and call-to-action sourced from `[empty-states.cli-no-blueprint]` in `docs/design-system/copy.toml`
- **AND** the call-to-action names a runnable next command (for example `cairn init --from-code` or instructing the user to author `cairn.blueprint` directly)

#### Scenario: Clean-map result renders a CTA

- **GIVEN** a parsed blueprint whose finding stream is empty
- **WHEN** the user runs `cairn check` or `cairn lint`
- **THEN** the CLI renders the heading, body, and call-to-action sourced from `[empty-states.cli-clean-map]` in `docs/design-system/copy.toml`
- **AND** the call-to-action names a runnable next command (for example `cairn neighbourhood <node>` to inspect a specific area)

#### Scenario: Empty-state copy is free of em-dashes

- **GIVEN** any `[empty-states.*]` entry consumed by the CLI
- **WHEN** the entry is loaded at compile time via `include_str!`
- **THEN** the rendered output contains no em-dash characters (U+2014)
- **AND** the rendered output uses plain English consistent with the design-system voice section
