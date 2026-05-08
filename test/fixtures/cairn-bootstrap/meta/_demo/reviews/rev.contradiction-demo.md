---
node: cairn.demo.nonexistent
date: 2026-05-08
reviewer: bootstrap-fixture-demo
review_type: human
---

# DEMO: blocking-finding trigger (Error severity)

This review deliberately references a node ID (`cairn.demo.nonexistent`) that does not exist in `cairn.blueprint`. When the scanner loads it (via `cairn --file cairn-with-demo.blueprint scan`), it produces a `CAIRN_REVIEW_UNKNOWN_NODE` finding at Error severity. The hook command exits with code 1.

This stands in for the *blocking finding class* in the bootstrap fixture. See `meta/_demo/README.md` for the gap between this demo and the literal "interface contradiction" the kernel emits via `.cairn/state/interface-hashes.json` mismatch (`CAIRN_INTERFACE_HASH_CHANGED`); both are Error-severity, but they differ in origin and the kernel does not yet parse interface signatures from contract bodies.

It is NOT a real review of any node. Do not action.
