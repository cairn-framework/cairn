---
node: cairn.tests
date: 2026-08-03
reviewer: google/gemini-3.6-flash/simplicity
review_type: agent_cross_model
subject_hash: sha256:ea77b40ef2309aa53fe4c25e9407f29707460319be4b0214b4bb193dd8aaf299
lens_prompt_hash: sha256:a0eef6eaecc060f2e7502504fb87df1299d39d9f33ed6a8c3c2d3c1714e76e3a
---

# Receipt review: bootstrap fixture corpus split (simplicity lens)

Receipt-grade review of `dec.bootstrap-fixture-corpus-split` under
`docs/agent/lenses/simplicity.md`, performed by an independent
cross-model reviewer (Gemini); its verdict and verified claims are
transcribed verbatim below.

## Ruling in the reviewer's own words

The decision artefact dec.bootstrap-fixture-corpus-split ratifies keeping
meta/sources/ and meta/research/ as an unparsed evidence corpus while
loading authority artefacts (decisions, todos, reviews, contracts) in
tests/fixtures/cairn-bootstrap/cairn.blueprint. This separation prevents
scanner errors like CAIRN_RESEARCH_MISSING_SOURCES without resorting to
stripping source citations from research frontmatter or adding artificial
loader exceptions. The decision represents the simplest, most direct
option (Option A) relative to its alternatives, avoiding unneeded
abstractions or code changes while maintaining clean test coverage in
examples_gate.rs and fixtures_smoke.rs.

## Claims verified

1. The cairn-bootstrap blueprint explicitly declares pointers for todos,
   reviews, and decisions, while leaving sources and research unclaimed
   (tests/fixtures/cairn-bootstrap/cairn.blueprint:19-21).
2. tests/examples_gate.rs asserts that tests/fixtures/cairn-bootstrap
   scans clean with no findings (tests/examples_gate.rs:36-44).
3. tests/fixtures_smoke.rs hardcodes BOOTSTRAP_SOURCE_IDS (9 source IDs)
   and asserts meta/sources/ conforms directly without using the loader
   (tests/fixtures_smoke.rs:18-28,45-87).
4. tests/fixtures/cairn-bootstrap/meta/sources contains 9 markdown files
   matching the pinned source IDs (directory listing).

## Findings

No findings.

## Verdict

PASS

## Re-attestation (2026-08-03, same day)

The independent reviewer (Gemini) re-verified all four claims above
against the final tree after gitignored scanner runtime debris was
removed from the fixture directory (the only manifest change; decision
text and rule files byte-identical) and confirmed the log file is
absent. Verdict re-issued: PASS. This receipt attests the recomputed
subject hash in the frontmatter.
