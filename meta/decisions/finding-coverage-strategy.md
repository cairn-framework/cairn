---
id: dec.finding-coverage-strategy
nodes:
  - cairn.tests
  - cairn.root
status: accepted
date: 2026-07-17
informed_by: [res.a2ui-analysis]
---

# Finding codes are enforced by a meta-test, not a conformance corpus

## Decision

Every emitted CAIRN_* finding code must be a statically scannable literal
(or const) with an exact-token row in docs/registries/error-codes.md and a
test assertion, enforced by the tests/finding_code_coverage.rs meta-test.
Codes without tests live in an explicit allowlist burn-down, never
silently. A declarative conformance corpus for findings is rejected.

## Rationale

`res.a2ui-analysis` (finding 4, verified) found roughly a fifth of emitted
codes had drifted in untested (about 20 of 103; the implementation audit
later counted 35 of 99, roughly a third), and the registry's completeness claim had
silently broken. The adversarial verification refuted a2ui's
conformance-corpus shape as redundant for cairn; the meta-test is the
surviving kernel. Dynamically constructed codes defeat static scanning, so
emission sites must use literals.

## Consequences

New finding codes budget for three artefacts at once: emission site,
registry row, and asserting test (or a justified allowlist entry). Shipped
in v0.4.0 via todo.finding-code-test-coverage and
todo.error-codes-registry-completeness.
