---
node: cairn.kernel.query
status: open
created: 2026-08-09
---

# Remediate Code Filter Ignored

## Scope
`cairn remediate [finding-code]` documents the argument as "Optional finding
code to focus the plan", but the plan it returns is identical for every code,
including a code that does not exist. Make the argument filter the plan, or
remove it and the help text that promises it.

Evidence and reproduction: `res.authoreval-instrument-evidence` section 1.

## Dependencies
None.

## Acceptance
- `cairn remediate <code> --json` returns only actions attributable to that
  code, or the command no longer accepts a code argument.
- An unknown code is distinguished from a known one: it does not return a
  plan as if it were real.
- A test covers both a known code and an unknown one.
- `cairn --help` and `cairn remediate --help` describe whatever behaviour
  ships.

## Sizing
S.

## Non-goals
Do not widen the remediation catalogue itself. This unit is about the filter
honouring its argument, not about which actions exist.
