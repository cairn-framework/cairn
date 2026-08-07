---
node: cairn.coord
status: open
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

## Acceptance

- Regression tests encode the three verified behaviours (they fail if
  the guards are removed); items 4 and 5 are each either fixed with a
  test or recorded as refuted in the research; gates green.
