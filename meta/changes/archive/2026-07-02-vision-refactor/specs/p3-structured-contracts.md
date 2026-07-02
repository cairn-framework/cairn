# Spec: Phase 3 — Structured contracts

## Acceptance criteria

- A contract with an `interface:` block whose entries all match extracted
  symbols lints clean.
- Appending a bogus entry to the dogfooded contract's `interface:` list makes
  `cairn lint` report exactly one `CAIRN_CONTRACT_INTERFACE_DRIFT` warning
  naming the unmatched entry; reverting the entry returns lint to clean.
- Symbols not listed in a contract's `interface:` block are never findings.
- The rule is registered in `docs/registries/spec-rules.md` as `enforced`.
