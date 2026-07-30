---
node: cairn.kernel.cli
informed_by:
  - type: decision
    id: dec.blueprint-as-current-state
---

# Contract: cairn.kernel.cli

The CLI is the primary user surface: it maps subcommands onto queries, scans, and hook checks, and formats their results for terminals and scripts.

## Interface

- **Input.** A parsed argument vector naming one subcommand and its flags.
- **Output.** Human-readable text on stdout by default; stable JSON under `--json`. Exit code 0 on success, non-zero when a blocking condition is found.
- **Errors.** Unknown subcommands, unresolvable ids, and missing files report a single actionable message; the CLI never prints a stack trace at a user.

## Invariants

- Every query the CLI exposes reflects the blueprint as current-state truth.
- JSON output is additive-stable: fields may be added, never silently renamed or removed.
- The CLI owns formatting only; no graph rule is enforced here that the kernel does not enforce itself.

## Out of scope

- Query semantics. The Query module answers; the CLI presents.
- Enforcement. Hooks decide blocking classes; the CLI reports their verdict.
