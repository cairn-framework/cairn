---
node: cairn.reconcile
status: open
created: 2026-08-09
---

# Node Symbol Coverage Evaluation


## Goal

After the ruling, transient extractor, and query wiring land, measure the
result against the same evidence that motivated
`res.node-symbol-coverage.investigation`.

## Scope

Exercise deterministic Rust and TypeScript fixtures containing exported and
private definitions. Rust coverage must include an all-private file with no
`pub ` marker, which currently exits through the pre-parse fast path, and a
mixed file that reaches `rust_is_exportable`. Re-run the frozen context-bundle
evaluation harness from
`archive/strongholds/agent-context-bundle-evaluation/evidence.tar.gz` against
the pinned ripgrep manifest and report the ripgrep recall delta. Keep the
sealed confirmation split unopened. Verify that interface hashes are unchanged
for the repository and that query output contains the expected definition
sites.

## Acceptance

- The fixture tests fail before the query-visible implementation and pass
  afterward.
- The report names the exact manifest, binary, commands, and recall values.
- No private symbol enters the interface hash, dependency bundle, contract
  check, or persistent map snapshot.
- The strict scan, hook, and applicable Rust gates pass.