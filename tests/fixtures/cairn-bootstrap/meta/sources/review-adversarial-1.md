---
id: src.review-adversarial-1
file: null
sha256: null
verification: unverified
type: conversation
date: 2026-04-13
tags: [review, critique, structural-feedback]
description: Adversarial review of Cairn spec v0.4 by an external reviewer. Identified the evidence/authority conflation, missing stable IDs, underspecified ownership rules, contradiction overloading, and missing source checksums. Load-bearing input into v0.5 revisions. Content was provided in conversation rather than as a standalone artefact; verification pending if the review is saved to a local file and hashed.
---

# Adversarial review of Cairn v0.4

Seven structural recommendations, all adopted in v0.5: split authority hierarchy into provenance + authority chains, add stable IDs, define ownership resolution precisely, separate machine state from authored files, classify contradiction severity, add source checksums, decide blueprint-vs-ADR current-state authority.

`file:` is null because no standalone transcript exists: the review arrived in
conversation and the body above is the evidence. It previously named
`./meta/sources/review-adversarial-1.md`, which is this artefact's own path once
the filename follows `dec.artefact-filename-rule`, so a source record would have
cited itself. Set it to the transcript path and `verification: verified` with a
`sha256` if the conversation is ever saved to a file.
