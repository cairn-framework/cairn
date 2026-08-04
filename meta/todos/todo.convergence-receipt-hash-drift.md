---
node: cairn.kernel.scanner
status: open
created: 2026-08-03
related: [dec.decision-ratification-tiers, dec.bootstrap-fixture-corpus-split, todo.release-next-milestone]
---

# Convergence receipt hash drift blocks every gate on main

`main` is red. This is a P0 release blocker: it fails milestone gate 1 of
`todo.release-next-milestone`, so no version can be cut until it is
resolved.

## Reproduction

On a clean checkout of `origin/main`, with a freshly built binary
(`cargo build --locked --release --bin cairn`):

```
cairn scan --strict   # exit 1
cairn hook all        # exit 1
cargo test --locked --test schema_validation   # 2 failures
```

The single Error finding:

```
CAIRN_DECISION_CONVERGENCE_UNMET
Accepted local decision `dec.bootstrap-fixture-corpus-split` has receipts
that fail the required convergence leg: receipt subject_hash does not
equal the recomputed manifest.
```

Two Info findings name the same cause from the receipt side:
`CAIRN_REVIEW_SUBJECT_UNMATCHED` on
`meta/reviews/rev.bootstrap-fixture-corpus-split-correctness.md` and
`rev.bootstrap-fixture-corpus-split-simplicity.md`. Both carry
`subject_hash: sha256:ea77b40ef2309aa53fe4c25e9407f29707460319be4b0214b4bb193dd8aaf299`.

The blast radius is wider than the gate. Because `src/query_api/mod.rs`
refuses any `requires_valid_map` tool while the graph carries an Error,
one stale receipt hash also disables ordinary read commands:
`cairn todos <node>` prints the finding list and exits 2 instead of the
node's todos. Whatever the fix, it should not leave a single artefact
integrity failure able to take out unrelated read queries.

Observed 2026-08-03: `git push` of an artefact-only branch from this
checkout was rejected by the pre-push hook (`scripts/dogfood.sh`, which
runs the working tree's own cairn via `cargo run`, rebuilding when
sources changed, never a PATH-installed binary), on this same Error.
Since the gate lints the tree rather than the diff, any branch carrying
the drifted receipts should fail it identically [INFERENCE]. If that
holds, the sanctioned push path is closed until this is fixed, and item
5 gains a sharper question: a source-current client-side gate would
have caught the drift at push time (barring a cargo fingerprint miss),
so how did it land (server-side merge with no client hooks, a CI range
skip, or a manifest input that changed after the receipts were pinned)?

## Why it was not caught

The failure is invisible to a stale binary. A `target/release/cairn`
built before the ratification checks reports zero errors on the same
tree, so a session that verifies with an unrebuilt binary sees a green
gate that does not exist. That can hide the failure locally; how the
drift actually landed on `main` is task 5's question, not something this
section asserts.

The two failing tests are not obviously about ratification:
`tests/schema_validation.rs::response_envelope_schema_accepts_real_query_output`
and `::response_schemas_accept_representative_outputs` fail because
`src/query_api/mod.rs` returns `findings_error(&scan_result.graph.findings)`
whenever `requires_valid_map` holds and the graph has any Error finding.
The reported `QueryError.code` is the FIRST finding's code, which is an
Info `CAIRN_DECISION_REFINED_AUTHORITY`, so the panic message names a
harmless Info code while the actual cause is the Error further down the
joined list. That misdirection cost real diagnosis time and is worth
fixing on its own: `findings_error` should key its code on the first
Error-severity finding, not the first finding.

## Task

1. Decide which side is wrong, with evidence, and record the reasoning:
   either the receipts were pinned against a manifest that has since
   changed legitimately (so the manifest must be recomputed and the
   receipts re-pinned or re-issued), or the canonical hasher in
   `src/artefacts/registry/manifest.rs` changed its governed-content
   extraction after the receipts were written (so the hasher, not the
   receipts, regressed and every existing receipt is suspect).
2. Apply the correct fix. Do not silence the finding, and do not
   downgrade its severity to get a green gate.
3. `findings_error` in `src/query_api/util.rs` picks its code from
   `findings.first()`. Change it to prefer the first Error-severity
   finding so the wire error names the blocking cause.
4. Add a regression test that fails when an accepted local-tier
   decision's receipts stop matching the recomputed manifest, so this
   surfaces as a named test rather than as two schema tests panicking on
   an unrelated Info code.
5. Establish why CI did not stop this. If `cargo test --locked
   --workspace` is green in CI while red locally, the divergence itself
   is the defect. Establish the observed CI path first; do not assume a
   mechanism before the evidence names one.
6. Stop one artefact integrity Error from disabling unrelated read
   queries. Today `execute_data_with_scan` refuses every
   `requires_valid_map` tool while any Error finding exists, which is
   what turned one stale receipt hash into `cairn todos` exiting 2.
   Decide and implement the boundary: which tools genuinely need a
   structurally valid map, and how an integrity Error is still reported
   without taking out reads that do not depend on it.

## Acceptance

- `cairn scan --strict` and `cairn hook all` both exit 0 on `main` with a
  freshly built binary.
- `cargo test --locked --workspace` passes with no failures.
- A query returning a findings error names the Error-severity code, not
  the first Info code in the list.
- A test fails if receipt subject-hash drift returns.
- `cairn todos <node>` answers while an integrity Error unrelated to map
  structure is live, and that Error is still reported.
- The CI-versus-local divergence in item 5 is explained in this todo's
  outcome section, or shown not to exist.

## Origin

Found 2026-08-03 while checking whether the repository was at a cuttable
0.10.0 milestone. It is the reason that answer is no.
