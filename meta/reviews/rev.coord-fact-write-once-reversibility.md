---
node: cairn.coord
date: 2026-08-08
reviewer: anthropic/claude-fable-5/contestedness-reversibility
review_type: agent_cross_model
subject_hash: sha256:089be0e5b54d6c3d07b2dec06b430f57d96dd8eeadf4f60988316e76f8e84b12
lens_prompt_hash: sha256:45136bbc19a4732ebacc4bd194791674e1266a4ae11c8fd51bfcfae9c7c4d698
---

# Receipt review: coord fact write once (reversibility lens)

Receipt-grade review of `dec.coord-fact-write-once` under
`docs/agent/lenses/contestedness-reversibility.md`, run clause-by-clause with
read-only repository access.

Lens: reversibility and blast radius. Read-only verification in /Users/george/repos/cairn-ov-spine at commit 8e2ac8d4. `cairn lint` was run and reports Info-tier findings only; `scan` was not run.

## Claims verified

1. Write-once append. `src/coord/append.rs:148` calls `persist::atomic_write_once` and maps `ErrorKind::AlreadyExists` to a write-once refusal at lines 149 to 153. `src/persist.rs:75-85` writes a temp file, syncs it, and links it into place, so target creation is exclusive and readers never observe partial bytes. The replace-capable path at `src/persist.rs:36-58` is not on the fact write route.

2. Reversal cost. Undoing the rule is a helper swap at one call site plus removal of the archive probe and rollback (`src/coord/append.rs:98-123` and `158-172`), roughly forty lines. The rule is purely restrictive: it introduces no on-disk artefact, no envelope field, no index, and no lockfile. Any store written under write-once is already a valid store under a later replace-capable regime, so reversal carries no data migration. The costly direction is the opposite one, and the ruling takes it now while the live store holds nine facts.

3. Mode sentence is descriptive and accurate. `src/persist.rs:80` uses `tempfile::NamedTempFile::new_in` with no permissions builder, so the target inherits the temp file's restrictive mode, whereas `src/persist.rs:44-49` sets `0o666` on unix for `atomic_write_bytes`. Declaring this descriptive rather than normative leaves the mode free to change without a further refining decision, which lowers reversal cost.

4. Tier 1, content immutability. Fact filenames are content-derived at `src/coord/append.rs:139-144` from the compact timestamp, kind, and `fact_id`. Different bytes cannot share a name, so every residual failure state in this design is two byte-identical copies. This is the property that makes each recovery path lossless.

5. Tier 2, live-set uniqueness enforced. The `fs::hard_link` at `src/persist.rs:83` is the enforcement point, scoped to one directory entry.

6. Tier 3, cross-set uniqueness detected, not atomically enforced. `src/coord/verify.rs:104-110` intersects live and archived `fact_id` sets and returns a hard error on any overlap. The decision's wording matches the code: detection and refusal, not prevention. Committing to detection rather than to a locking protocol is the reversibility-preferred choice, because the named advisory-locking follow-up will not have to unwind a mechanism, an artefact, or a wire format.

7. Append order and rollback. Live reservation precedes the archive probe (`src/coord/append.rs:148` then `158`); rollback at `114-123` and `161-172` ignores `NotFound` and removes only the path this call created. A failed append therefore leaves the store byte-for-byte as it was found.

8. Compaction is exclusive. `src/coord/verify.rs:82-85` implements `move_once` as `hard_link` then `remove_file`, which cannot replace an existing archived path, and `compact` routes through it at `226-227`.

9. Scope fence. The rule names `facts/` and `archive/` and excludes `leases/`, `singleton/`, `cache/observed.json`, and `sidecars/`. The sidecar rule is separately sited at `src/cli/commands/ruling_run.rs:268`. Three other persistence regimes stay untouched, so a future reversal cannot cascade beyond the fact store.

10. Timestamp claim and migration obligation. `src/coord/time.rs:62-65` requires a 20-byte, Z-terminated, whole-second spelling, with the fractional rejection covered by the test at `time.rs:251-254`. Counted the live store independently: nine fact files, zero fractional timestamps in filenames or bodies. The no-migration claim holds, and no migration means nothing to unwind.

11. Archive-inclusive consumed check. `src/coord/verify::archived_fact_has_target` (`src/coord/verify.rs:55-66`) is consumed by `src/cli/commands/ruling_run.rs:91`, so the consumed probe reads archived facts as well as live ones.

12. Regression cover. `src/coord/append.rs:357-372` covers rejection of an archived duplicate with live rollback, and `374-399` injects a probe that simulates compaction winning the interval, asserting zero live copies and exactly one archived copy after refusal.

## Findings

1. Non-blocking. `move_once` at `src/coord/verify.rs:82-85` has its own crash window between the `hard_link` and the `remove_file`. A crash there leaves the fact in both sets with no concurrent append involved, and `verify` will then refuse the store at `verify.rs:106-110` until a human removes the live copy. The decision attributes the dual-copy state to a failure "between create and rollback", which is the append-side origin; the compaction-side origin produces the same state. The operative tier 3 claim, that cross-set uniqueness is detected and not atomically enforced, remains true either way, and tier 1 guarantees the two copies are byte-identical so either can be deleted without information loss. Recovery is cheap and local; documenting it is a prose improvement, not a correction.

2. Non-blocking. Write-once turns a crash-retry of an identical append into a hard refusal rather than a silent idempotent success. This is the desirable failure mode under this lens: the failure is loud, the message at `src/coord/append.rs:150-153` names the rule, and no state is committed. Worth knowing operationally, but it does not raise reversal cost.

3. Non-blocking, code doc drift in an affected file. The `compact` doc comment at `src/coord/verify.rs:169` still says "a rename failure" after the move to `hard_link` plus `remove_file`. Wording only; behaviour matches the decision.

## Reversibility judgement

No live alternative under this lens. The imaginable alternative, keeping the replace-capable helper and relying on detection alone, is not merely worse on the stated evidence; it is also the strictly harder direction to adopt later, since it accumulates replaceable published paths that a future write-once rule would have to reconcile. Genuine uncertainty does remain in the cross-set race window, but that uncertainty is attached to a rule whose undo is a one-line helper swap, a scope fence that keeps three other persistence regimes out of the blast radius, zero migrated data, and a nine-fact live store. Cheap to reverse under uncertainty is convergent by the rubric. The governance cost the decision names, a further refining decision plus an explicit replacement for the append-only guarantee, is the ordinary cost of any refining decision here and does not make the ruling irreversible in the blast-radius sense.

## Verdict

PASS
