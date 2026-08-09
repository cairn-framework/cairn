---
node: cairn.coord
date: 2026-08-08
reviewer: anthropic/claude-fable-5/contestedness-alternatives
review_type: agent_cross_model
subject_hash: sha256:089be0e5b54d6c3d07b2dec06b430f57d96dd8eeadf4f60988316e76f8e84b12
lens_prompt_hash: sha256:1ceb131f531393b6d998c5641ce6741774cce8f6b0305d4fd2876f4db4179003
---

# Receipt review: coord fact write once (alternatives lens)

Receipt-grade review of `dec.coord-fact-write-once` under
`docs/agent/lenses/contestedness-alternatives.md`, run clause-by-clause with
read-only repository access.

Lens: ALTERNATIVES. Read-only verification of `dec.coord-fact-write-once` (meta/decisions/coord-fact-write-once.md) against /Users/george/repos/cairn-ov-spine at commit 8e2ac8d4. Method: steel-man the strongest competing choice for every load-bearing clause, then test whether the competitor survives contact with the tree.

## Claims verified

1. **Append uses the write-once helper and refuses an existing target.** src/coord/append.rs:148-157 calls `persist::atomic_write_once` and maps `ErrorKind::AlreadyExists` to a write-once refusal. src/persist.rs:75-85 writes a sibling temp file, fsyncs it, then `fs::hard_link`s it into place, so target creation is exclusive and the name never appears with partial bytes. VERIFIED.

2. **The mode sentence is descriptive and correct.** src/persist.rs:46-47 sets `0o666` on the tempfile builder for `atomic_write_bytes` and preserves an existing target's permissions at src/persist.rs:55-57. src/persist.rs:80 uses `NamedTempFile::new_in`, whose `0o600` mode survives the link. The decision states this as a property of the selected helper and not as a separate requirement, which matches. VERIFIED.

3. **Tier 1, content immutability.** src/coord/read.rs:86-107 recomputes `fact_id` via `fact_id_for` and rejects a mismatch, then reconstructs the expected filename from `compact_rfc3339(recorded_at)`, `kind`, and `fact_id` and rejects any disagreement. src/coord/envelope.rs:132-137 hashes the body with `fact_id` blanked. src/coord/verify.rs:47 applies the same `validate` to archived facts. Divergent bytes under one fact name fail closed on every read. VERIFIED.

4. **Tier 2, live-set uniqueness enforced.** The exclusive create in claim 1 is the enforcement point. Name and identity are in bijection: equal `fact_id` implies an equal serialized body, hence equal `recorded_at` and `kind`, hence an equal filename. So the filename-keyed append probe and the identity-keyed verify check cannot disagree. VERIFIED.

5. **Tier 3, cross-set uniqueness detected fail-closed.** src/coord/verify.rs:104-110 intersects live and archived `fact_id` sets and returns an error naming the duplicate identity. There is no atomic enforcement, which is exactly what the clause says. VERIFIED.

6. **Append reorder and rollback, and the any-failure window.** src/coord/append.rs:158-172 creates the live path first, then probes the archive, and removes the just-created live copy on either probe error or archive hit. src/coord/append.rs:114-123 ignores `NotFound` during rollback. There is no journal across the create-then-rollback interval, so the stated any-failure window is an accurate description of the code, not an understatement. VERIFIED.

7. **Compaction is exclusive and no-replace.** `move_once` at src/coord/verify.rs:82-85 is `hard_link` followed by `remove_file`, called at src/coord/verify.rs:226-227 with the error text "cannot archive ... without replacement". An occupied archive target fails the link and leaves the live fact in place. VERIFIED.

8. **Sidecar rule is the sidecar's own.** src/cli/commands/ruling_run.rs:267-269 writes `sidecars/preimage-<digest>-<compacted>.diff` with `atomic_write_once`, so a same-second retry for the same digest collides and fails closed, as described. VERIFIED.

9. **No parse-cache trust or regeneration rule remains.** src/coord/read.rs:1-2 and :116-156 fold and parse `facts/` in full on every read; the regression at src/coord/read_regressions.rs:246-249 asserts that a full read does not regenerate a parsed-envelope cache, and src/coord/verify.rs:230 removes a legacy cache left by older builds. VERIFIED.

10. **Timestamp discipline and the no-migration finding.** src/coord/time.rs:63-66 requires exactly twenty bytes with separators and a trailing `Z`, rejecting fractional and numeric-offset spellings, with the regression at src/coord/time.rs:250-254. Read-only inspection of the live store under `<git-common-dir>/cairn/coord` shows nine fact files and four distinct `recorded_at` values, all whole-second `Z` spellings, so the "nine facts, zero fractional timestamps" finding and the absence of a migration obligation both hold. VERIFIED.

11. **Frontmatter references resolve and the surfaces are the governed ones.** `dec.rung-three-coordination-substrate` resolves to meta/decisions/rung-three-coordination-substrate.md (status accepted) and `res.chatgpt-architecture-review` to meta/research/chatgpt-architecture-review.md. `cairn.coord` and `cairn.persist` are declared at cairn.blueprint:33 and :24. `src/persist.rs` is correctly listed under `affects` as a governed surface even though `atomic_write_once` predates this commit range (added in 310f69e8). VERIFIED.

## Findings

No blockers and no defects. Findings below are steel-man outcomes, recorded so the sign-off means something. None of them block.

1. **Strongest competitor considered: keep the replace-capable `atomic_write`.** The best version of this argument is that content-derived naming plus congruence validation (claim 3) already guarantee a replacement would be byte-identical, so write-once buys nothing and adds a hard-fail on an otherwise idempotent retry. It fails on the accepted parent: meta/decisions/rung-three-coordination-substrate.md:100-128 fixes the store as append-only with exclusive create for tokens, so a replace-capable fact path contradicts an accepted rule rather than merely differing in taste. It also fails on direction of reversal: tightening later is additive, whereas history damage under a published path is unrecoverable. Refuted, not live.

2. **Second competitor: enforce cross-set uniqueness atomically now.** The best version is that a documented race in an audit store tends to survive forever, and a small advisory lock closes it. It fails on cost and on blast radius: the residual state is bounded to byte-identical duplicates, `verify` refuses it fail-closed at src/coord/verify.rs:104-110, compaction is a maintenance verb under the substrate's single-driver model, and a lock file imports a stale-lock and crash-recovery lifecycle into a store that has one writer by rule. The clause names advisory locking as the follow-up owner for the moment a second adapter exists, and upgrading tier 3 is additive. Refuted, not live.

3. **Third competitor: probe the archive before creating the live path.** The best version is that it avoids creating a copy you may have to delete, so no rollback and no dual-copy window. It is strictly worse: it is a check-then-create, and a compaction landing between probe and create produces a live duplicate of an archived fact that nothing detects at append time. The shipped order makes the same interleaving produce a state `verify` refuses. Refuted on evidence, and the decision already records this reasoning.

4. **Fourth competitor: O_EXCL create-and-write instead of tempfile plus hard_link.** It avoids a temp file, but publishes the final name before the bytes land, so a concurrent reader can observe a truncated envelope under a published path, which is the precise property the audit surface needs. Refuted.

5. **Observation, non-blocking: intra-document repetition.** The any-failure sentence and the advisory-locking follow-up sentence appear twice, once in the tier 3 bullet and once in the Concurrency assumption section. That is an artefact of stacking fixes across review rounds. Both copies are accurate and mutually consistent, so nothing is wrong; a future editorial pass could keep one.

6. **Observation, non-blocking: citation looseness.** The phrase "extending the substrate's disjoint-writer rule" paraphrases the parent's model (singleton driver plus per-wave disjoint write-sets at meta/decisions/rung-three-coordination-substrate.md:138-155) rather than quoting a clause literally named that way. The referent is defensible and the extension (one compactor at a time) is coherent with it.

7. **Observation, non-blocking: stale artefact on disk.** The live store still holds a `cache/parsed.json` left by pre-elimination builds. No code path reads it (claim 9), and src/coord/verify.rs:230 removes it on the next compaction, so the decision's "no parse-cache trust" statement is not contradicted by its presence.

## Verdict

PASS
