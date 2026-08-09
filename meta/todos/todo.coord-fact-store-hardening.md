---
node: cairn.coord
status: done
created: 2026-08-07
related: [res.chatgpt-architecture-review, todo.driver-in-repo, dec.rung-three-coordination-substrate]
---

# Coordination fact store hardening (pre-driver)

Harden the coordination fact store before todo.driver-in-repo makes the
driver its second adapter. The defect dossier, file:line evidence, and
what was and was not verified in-session live in
res.chatgpt-architecture-review; this todo is the owner, not a second
copy. Fail-closed policy authority: dec.rung-three-coordination-substrate.

## Task

Verified defects (fix, test-first):

1. Reject malformed `recorded_at` before any filesystem mutation; it
   currently reaches the fact filename unvalidated.
2. A malformed `lease.grant` must make append or read fail, never fold
   to "not held" in `held` (src/coord/read.rs).
3. Facts are written replace-capable; moving to write-once semantics
   needs a refining decision recording the change from the
   `persist::atomic_write` selection, not silent cleanup.

Unverified claims (verify first, then fix or record why not):

4. Parse-cache envelopes trusted by filename without byte binding, and
   `coord verify` never recomputing fact identity.
5. Decline-preimage sidecars under `cache/` referenced from immutable
   facts.

## Resolution (2026-08-09)

1. `append_fact` validates the UTC `recorded_at` spelling before resolving or
   initialising the store. The append regression proves malformed input leaves
   no coordination directory.
2. Lease grants and renewals require a valid `unit_id` and `expires_at` on
   append and read. A malformed grant fails the read before `held` can fold it.
3. `persist::atomic_write_once` creates fact paths exclusively. The duplicate
   append regression proves existing bytes remain unchanged. The proposed
   ruling vehicle is `dec.coord-fact-write-once`; it remains `status: proposed`
   for the driver's ratification panel.
4. The cache and identity claims were resolved by elimination and direct
   validation. Full reads no longer create or trust a parsed-envelope cache,
   and live plus archived facts undergo identity recomputation. The dated
   evidence and regression names are recorded in
   `res.chatgpt-architecture-review`.
5. The decline-preimage sidecar claim was confirmed and fixed. Oversized
   diffs are write-once sidecars below `sidecars/`, not disposable `cache/`.
   The dated evidence and regression name are recorded in
   `res.chatgpt-architecture-review`.
Panel follow-up (2026-08-09) also closed kind path traversal, fractional
stored timestamps, renamed live facts, tampered archived facts, and valid
RFC 3339 instant comparison for fractional and offset `--at` values. Archive
re-append, cross-set duplicate, and no-replace compaction regressions are
included.

## Acceptance

- Regression tests encode the three verified behaviours (they fail if
  the guards are removed); items 4 and 5 are each either fixed with a
  test or recorded as refuted in the research; gates green.
