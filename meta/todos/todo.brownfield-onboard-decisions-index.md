---
node: cairn.brownfield
status: done
created: 2026-08-10
parent: todo.brownfield-extraction-flow
---

# Build the deterministic `cairn onboard decisions` evidence index

Implementation unit split out of `todo.brownfield-extraction-flow` under the
sizing rule. This unit builds clause 1 of `dec.brownfield-extraction-mechanism`
and nothing else: the deterministic Cairn surface. The authoring reference and
the external-repository validation are separate sub-todos.

## Task

Add the command surface the ruling names:

```
cairn onboard decisions
cairn onboard decisions --json
```

The default `decisions` form emits the human-readable evidence report; `--json`
emits the stable machine-readable index for a harness. The no-subcommand
`cairn onboard` path keeps its existing orphan report unchanged.

Evidence sources are a closed set: files under `docs/adr/` and
`docs/decisions/`, README sections headed `Decision`, `Rationale`, or
`Invariant`, and source comments carrying the literal `// invariant:` or
`# invariant:` markers. The branch keeps the scanner-loaded graph as its source
of real node bindings and uses the deterministic `src/brownfield/discovery.rs`
facts as bounded code evidence.

Binding is path-to-blueprint, never candidate-id-to-blueprint. Normalise each
evidence path relative to the project root and resolve it against the loaded
blueprint's declared ownership: eligible leaf or `owns-files` nodes contribute
normalised declared paths, most-specific first, and `map::paths::is_component_prefix`
selects the owner. The reconciler's `eligible_owners` and `most_specific_owner`
in `src/reconcile/generic.rs` are private, so reimplement the same
most-specific-prefix rule in the onboard resolver and add a parity test against
the existing reconciler fixture expectations. Validate that the resolved owner id
exists in the loaded graph before emitting a bound candidate. Evidence with no
matching owner is reported as unbound; never invent a binding.

The flow requires an onboarded `cairn.blueprint` that loads. Where the current
onboard command synthesises a temporary stub blueprint for an absent file, the
`decisions` branch fails with a clear error instead.

Any other positional subcommand returns exit code 2 with the literal error text
`usage: cairn onboard [decisions] [options]`, built from
`copy::lookup("help.commands.onboard.usage")`. It must not fall back to the
orphan report the way `run_onboard_command` currently does by ignoring
`command_args`. The `help.commands.onboard.usage` and `help.commands.onboard.args`
values in `docs/design-system/copy.toml` name the supported form and say that
omitting `decisions` keeps the orphan report; the usage value stays unprefixed
for the help renderer, and only the error path adds the literal `usage: `
prefix. No parallel hardcoded copy surface.

The branch does not scan arbitrary prose, call a model, draft narrative, write
`status: accepted`, or mutate the blueprint.

Surfaces the ruling invalidates and this unit updates: `src/cli/commands/onboard.rs`,
the `src/cli/mod.rs` onboard description and help dispatch, the Brownfield
onboarding rows in `docs/commands.md` and `docs/integration-contract.md`,
`docs/design-system/copy.toml`, and the onboard coverage in `tests/kernel.rs`.
`src/brownfield/onboard.rs` is at 482 of the 500 permitted lines, so the
evidence index goes in a new module under `./src/brownfield`.

## Non-goals

The shipped `cairn-dev` authoring reference and its pack wiring belong to
`todo.brownfield-extraction-authoring-reference`. The external-repository run and
the end-to-end drafted-artefact assertion belong to
`todo.brownfield-extraction-external-validation`. Post-install pointer copy
belongs to `todo.brownfield-extraction-pointer`.

## Acceptance

- `cairn onboard decisions` on a fixture repository carrying `docs/adr/`
  material, a README `Decision` section, and an `// invariant:` comment reports
  every evidence path with the blueprint node id that owns it, and reports
  unbound evidence separately without inventing a binding.
- `cairn onboard decisions --json` emits the machine-readable index, asserted by
  a test on the wire rather than on the human report.
- `cairn onboard` with no subcommand still prints the orphan report, asserted by
  a regression test.
- An unsupported subcommand exits 2 with the literal
  `usage: cairn onboard [decisions] [options]` and prints no orphan report.
- A parity test shows the onboard owner resolver agrees with the existing
  reconciler fixture expectations on most-specific-prefix ownership.
- With no loadable `cairn.blueprint`, `cairn onboard decisions` fails with a
  clear error and synthesises no stub blueprint.
- Every added user-facing string lives in `docs/design-system/copy.toml`.
- `cairn scan --strict` exits 0.
- On landing, set `todo.brownfield-extraction-authoring-reference` to `open`.
