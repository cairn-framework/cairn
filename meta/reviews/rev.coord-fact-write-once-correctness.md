---
node: cairn.coord
date: 2026-08-08
reviewer: anthropic/claude-fable-5/contestedness-correctness
review_type: agent_cross_model
subject_hash: sha256:089be0e5b54d6c3d07b2dec06b430f57d96dd8eeadf4f60988316e76f8e84b12
lens_prompt_hash: sha256:288d695e09e8f9c922e07c0349c2870f887b817b9a39eac777c501f90c70f6c5
---

# Receipt review: coord fact write once (correctness lens)

Receipt-grade review of `dec.coord-fact-write-once` under
`docs/agent/lenses/contestedness-correctness.md`, run clause-by-clause with
read-only repository access.

Correctness lens receipt for `dec.coord-fact-write-once` at commit 8e2ac8d4. Read-only verification in /Users/george/repos/cairn-ov-spine. No edits were made and no scan was run.

## Claims verified

**Subject bytes match the tree.** The reviewed bytes are byte-identical to `meta/decisions/coord-fact-write-once.md` on disk (5106 bytes, frontmatter compared exactly). Every frontmatter reference resolves: `dec.rung-three-coordination-substrate` and `res.chatgpt-architecture-review` exist under `meta/decisions/` and `meta/research/`, node ids `cairn.persist` and `cairn.coord` are declared at `cairn.blueprint:24` and `cairn.blueprint:33`, and all three `affects` paths exist.

**The parent clause says what the Context says it says.** `meta/decisions/rung-three-coordination-substrate.md:102-103` reads "one atomically written file per fact", and `:126-128` speaks of "a fold of the append-only facts in filename order". Clause 6 at `:212` adds that `cairn coord compact` "moves facts to archives and rewrites nothing". The Context paraphrase is exact.

**The write-once gap was real.** `git show 49d6fdd6:src/coord/append.rs` line 96 is `persist::atomic_write`, and that helper's rename step is replace-capable (`src/persist.rs:58`, `tmp.persist(path)`). The informing research corroborates independently and by line at `meta/research/chatgpt-architecture-review.md:33-34`. The stated problem existed; this is not a ruling in search of a defect.

**The selected helper is exclusive.** `src/coord/append.rs:148` calls `persist::atomic_write_once`; `src/persist.rs:80-83` writes a `NamedTempFile` and then `fs::hard_link`s it into place, which fails `EEXIST` without touching an existing target. `src/coord/append.rs:149-153` maps `AlreadyExists` to the write-once refusal. Temporary bytes exist before the link, exactly as the ruling permits, and only the link makes the fact visible.

**The permissions sentence is accurate and correctly scoped.** `atomic_write_once` sets no permissions (`src/persist.rs:75-85`), so the target inherits the tempfile's owner-only mode, while `atomic_write_bytes` applies `0o666` at `src/persist.rs:47`. The ruling states this descriptively rather than as a requirement, which is the right call: the store is family-local and single-user under parent clause 2.

**Tier 1 holds on the write path.** `fact_id` is the first 12 hex of SHA-256 over the canonical envelope with `fact_id` blanked (`src/coord/envelope.rs:132-140`), and the filename is `recorded_at` plus `kind` plus `fact_id` (`src/coord/append.rs:139-144`), so the name is fully content-derived. Congruence is enforced on every read: `src/coord/read.rs:88-106` recomputes the identity and rejects a filename that does not match its content.

**Tier 2 holds.** Live-set uniqueness rests on the exclusive link above, and is pinned by `coord::append::tests::duplicate_fact_is_rejected_without_replacing_original_bytes` (`src/coord/append.rs:296-309`), which overwrites the target with `b"sentinel"` and asserts those bytes survive the refused second append.

**Tier 3 holds, including its honesty about the window.** Detection is real: `src/coord/verify.rs:104-110` intersects live and archived `fact_id` sets and fails closed, pinned by `coord::verify::tests::verify_rejects_a_fact_identity_present_in_live_and_archive`. Non-enforcement is equally real and correctly described against `write_fact_with_archive_probe` (`src/coord/append.rs:129-174`): create at `:148`, archive probe at `:158`, rollback at `:161` and `:168`. A failure anywhere in that window, with a pre-existing archived twin, leaves two identical copies until `verify` refuses them. The "any failure" phrasing covers the whole window rather than only the compaction race, which is the stronger and correct statement.

**Scope exclusions are descriptive of code that exists.** Lease and singleton tokens acquire with `OpenOptions::create_new` (`src/coord/epoch.rs:50-53`); `cache/observed.json` is a derived snapshot rewritten through `write_json` (`src/coord/verify.rs:114-159`); sidecars are written with `atomic_write_once` under their own naming at `src/cli/commands/ruling_run.rs:267-268`. None of the four exclusions is invented here.

**Compaction is no-replace.** `move_once` is `hard_link` then `remove_file` (`src/coord/verify.rs:82-85`), called only from `compact` at `:226-227` with the error text "cannot archive `{name}` without replacement". The archive target is never overwritten.

**The cache elimination claim is true of this tree.** `src/coord/read.rs` contains no reference to `cache` or `parsed.json` at all, and `src/coord/verify.rs:230-232` best-effort removes the legacy `cache/parsed.json`, with a comment stating that current reads neither create nor trust it.

**The live-store figures are correct.** The store resolves to `/Users/george/repos/cairn/.git/cairn/coord`. `facts/` holds exactly nine files; their `recorded_at` values are `2026-07-01T00:00:00Z`, six copies of `2026-08-07T03:45:12Z`, `2026-08-07T11:41:32Z`, and `2026-08-07T11:41:33Z`. All are whole-second UTC. `archive/` is empty, so nine is the complete population and the no-migration conclusion follows. Fail-closed rejection of fractional and offset spellings is enforced by the length-20 check at `src/coord/time.rs:63-67` and pinned by `fractional_seconds_are_rejected_for_stored_timestamps` at `:251-254`.

**The regression evidence runs.** `cargo test --lib coord::` passes 50 tests and `cargo test --lib persist::` passes 11, zero failures in both. That includes `duplicate_fact_is_rejected_without_replacing_original_bytes`, `archived_duplicate_is_rejected_after_compaction` (`src/coord/append.rs:357-372`), `archive_race_rolls_back_reserved_live_copy` (`:374-399`), and the live-and-archive verify rejection. These tests assert observable refusals and surviving bytes, not plumbing, and each would fail if the helper reverted to `atomic_write`.

**Ratification state is honest.** Frontmatter is `status: proposed` with no `receipts` key, in contrast to the accepted parent, which carries three receipts and `ratification: binding` (`meta/decisions/rung-three-coordination-substrate.md:7-12`). The body claims no acceptance.

## Findings

Three non-blocking observations. None is a defect, and none affects the ruling's correctness as written.

1. A compaction-wins race can produce a failed append whose bytes nonetheless persist. If `compact` lists the live set after `append` has created its file and the name falls before the cutoff, `move_once` (`src/coord/verify.rs:82-85`) relocates that very file, the archive probe then reports it archived, `remove_fact_if_present` swallows `NotFound` (`src/coord/append.rs:114-123`), and the caller receives the archive write-once error while the observation is durably recorded in `archive/`. This violates nothing the ruling promises: the target's bytes are never replaced, `verify` sees a single valid copy, and the concurrency section already declares compaction non-concurrent with append. It is a caller-visible false negative worth naming when the advisory-locking follow-up lands.

2. `fact_id` truncates SHA-256 to 48 bits (`src/coord/envelope.rs:137-138`), so tier 1's "impossible" is exact only up to a truncation collision that would additionally have to land in the same second under the same kind. At nine facts this is not a practical concern, and the tier's ordering against tiers 2 and 3 is unaffected.

3. Tier 1 is a property of the write path, not of the filesystem. Out-of-band tampering can place different bytes under a fact name, as the regression test itself demonstrates; that case is caught by the read-time congruence check at `src/coord/read.rs:88-106` and fails closed. The clause reads correctly under its intended write-path scope.

On contestedness: I looked for a live alternative and did not find one. Retaining the replace-capable helper is refuted by the parent clause and by the research evidence. Enforcing cross-set uniqueness atomically today, rather than deferring to advisory locking, is a sequencing preference whose reversal costs one function's error path and whose risk the ruling names in the open. Accepting a byte-identical same-second re-append idempotently instead of failing closed is imaginable, but it contradicts the store's established fail-closed discipline and carries no material consequence either way. The ruling is obvious and binding, which the rubric classes as convergent.

## Verdict

PASS
