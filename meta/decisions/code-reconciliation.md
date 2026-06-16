---
id: dec.code-reconciliation
nodes:
  - cairn.reconcile
status: accepted
date: 2026-06-16
---

# Code reconciliation

## Context

A blueprint declaration is only useful if it reflects real code. cairn needs to extract public interfaces from source files and compare them against the declared graph.

## Decision

Provide a single `cairn.reconcile` module that dispatches to language-specific tree-sitter reconcilers (Rust, TypeScript, Python, Go). Each reconciler returns a `ReconcileReport` of claimed files, public symbols, and per-node interface fingerprints.

## Rationale

One dispatch point keeps language parity manageable. Tree-sitter gives us parser reuse without shipping per-language compilers. Per-node interface hashes (not per-language global hashes) let the gate identify which module drifted.

## Consequences

- Adding a language means adding a new reconciler module and registering it in the scanner.
- Interface fingerprint changes block the `interface` and `all` hooks.
- The reconciler must attribute symbols to owner nodes so hashes are per-node.
