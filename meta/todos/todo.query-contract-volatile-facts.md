---
node: cairn.kernel.query
status: open
created: 2026-08-07
related: [res.chatgpt-architecture-review]
---

# Query contract carries drifted volatile facts

The kernel.query contract asserts a registry size of 36 while the
registry test asserts 50; evidence and line cites in
res.chatgpt-architecture-review. The prose drifted through fourteen
tool additions and no gate fired, because contract-staleness machinery
does not model factual numerals in prose.

## Task

1. Remove volatile numerals (tool count, schema version) from contract
   prose, or generate/assert them from the registry so drift fails a
   test.
2. Sweep the other contracts in meta/contracts/ for the same class of
   asserted count or version.
3. Note whether a scanner finding should cover asserted-numeral drift;
   if yes, file it as its own unit rather than building it here.

## Acceptance

- No contract asserts a registry size or schema version that a test does
  not tie to the code; kernel.query.md matches reality.
