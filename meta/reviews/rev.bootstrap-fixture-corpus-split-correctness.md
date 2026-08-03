---
node: cairn.tests
date: 2026-08-03
reviewer: anthropic/claude-fable-5/correctness
review_type: agent_cross_model
subject_hash: sha256:ea77b40ef2309aa53fe4c25e9407f29707460319be4b0214b4bb193dd8aaf299
lens_prompt_hash: sha256:f1040b7f79cf81c3813659f140080a2891d9776869feab464eaf4041f79eccf7
---

# Receipt review: bootstrap fixture corpus split (correctness lens)

Receipt-grade review of `dec.bootstrap-fixture-corpus-split` under
`docs/agent/lenses/correctness.md`, verifying the ruling's factual claims
against the repository at the subject hash above.

## Claims verified

1. The fixture keeps both corpora on disk while the blueprint declares
   pointers only for the authority side: `tests/fixtures/cairn-bootstrap/meta/`
   contains `sources/` and `research/` beside `decisions/`, `contracts/`,
   `reviews/`, and `todos/`, and `tests/fixtures/cairn-bootstrap/cairn.blueprint`
   declares no `sources` or `research` pointer.
2. The blueprint header carries the warning the decision's Consequences
   require (`cairn.blueprint` lines 11-15: the evidence corpus stays
   unclaimed and neither pointer may be added).
3. `tests/examples_gate.rs` asserts the fixture scans clean
   (`test_bootstrap_fixture_scans_clean`), which is the decision's recorded
   gate for the loaded corpus.
4. `tests/fixtures_smoke.rs` reaches `meta/sources/` directly without the
   loader, which is the smoke-test premise the ruling preserves.

## Findings

NON-BLOCKING meta/decisions/bootstrap-fixture-corpus-split.md:7: the
`affects:` list names the decision file itself; the subject manifest
already hashes the decision body, so the entry is redundant but harmless.

## Verdict

PASS

No blocking findings: the ruling matches the shipped repository state,
its gates exist and are exercised, and the recorded costs are accurate.

## Re-attestation (2026-08-03, same day)

The subject manifest was recomputed after removing gitignored scanner
runtime debris from the fixture (`tests/fixtures/cairn-bootstrap/.cairn/`:
an appended scan log and a blueprint snapshot; never tracked, never part
of any reviewed claim). The decision text and every rule file are
byte-identical to the originally reviewed bytes. All four verified
claims above were re-checked against the final tree (9 source files,
clean-scan gate present, pointers unchanged, runtime debris absent) and
hold. This receipt attests the recomputed subject hash in the
frontmatter.
