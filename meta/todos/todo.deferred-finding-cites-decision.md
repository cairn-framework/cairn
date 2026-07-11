---
node: cairn.kernel.scanner
status: done
created: 2026-07-11
---

# Deferred Finding Cites Its Deferral Decision

`CAIRN_SPEC_RULE_UNIMPLEMENTED` for spec:634 (`ADR revisit_triggers appear
relevant to recent changes`) appears in every scan/lint/hook run. The rule
is deliberately deferred by `dec.revisit-trigger-correlator-deferred`, but
the finding message does not say so; every fresh-context session
re-investigates the registry prose before concluding it is
deferred-by-design.

Change: when a pending spec rule's registry row (or an adjacent mechanism)
records a deferral decision, include it in the emitted finding message,
e.g. `... is pending but names no enforcer (deferred by dec.<slug>)`.

Likely shape: add an optional `Deferred-by` column (or equivalent) to
`docs/registries/spec-rules.md`, parse it in the spec-rules reader, and
thread it into the finding emission. Update the format table in the
registry header and copy.toml if the finding text is centralised there.

Acceptance:
- `cairn lint` output for spec:634 names the deferral decision inline.
- A pending rule without a deferral decision renders unchanged.
- Test covering both rows.
