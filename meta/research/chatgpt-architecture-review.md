---
id: res.chatgpt-architecture-review
nodes:
  - cairn.coord
  - cairn.kernel.query
sources: [src.chatgpt-architecture-review]
date: 2026-08-07
---

# Architecture review of the coordination substrate (external, verified in part)

An external model architecture review at head 9edfdac, received
in-session on 2026-08-07 while running todo.roadmap-assumption-audit.
The full report is preserved via src.chatgpt-architecture-review; its
verdict in one line: harden and deepen coordination before
todo.driver-in-repo makes the driver its second adapter. This artefact
records only what this session verified and where each finding was
routed. Evidence, never authority.

## In-session verification (2026-08-07)

Confirmed against the tree:

- `recorded_at` reaches the fact filename unvalidated:
  `compact_rfc3339` (src/coord/time.rs:63) strips only `-` and `:`.
  The review overstates the neighbouring claim: `kind` IS constrained to
  sanctioned families in `append_fact`, and console writes of lease and
  singleton facts are rejected.
- Lease fail-open: `read.rs` types `expires_at` as `Option<&str>` and
  the `held` predicate (src/coord/read.rs:177) requires
  `expires_at > at`, so a malformed or absent expiry folds to
  "not held".
- Facts are written with replace-capable `persist::atomic_write`
  (src/coord/append.rs:96); write-once is not enforced.
- Cursor filter is `named.name.as_str() > since`
  (src/query_api/handlers/coordination.rs:75); the same-second gap is
  plausible by inspection, not yet pinned by a test.
- Contract drift is real: meta/contracts/kernel.query.md:39 says 36
  tools; src/query_api/registry.rs:528 asserts 50.

Not verified in-session (recorded as claims): parse-cache envelope trust
without byte binding, `coord verify` skipping identity recomputation,
decline-preimage sidecars under `cache/`, uppercase-hex digest
acceptance, and the `wave stats` timestamp reading of `since`. One
external numeral is wrong by local count: the report says PR #589
carried 141 files; `git diff 9edfdac^1 9edfdac^2 --stat` shows 147.

## S8 follow-up verification (2026-08-09)

The two claims routed as unverified to `todo.coord-fact-store-hardening` were
confirmed against the implementation and closed with tests:

- Parse-cache envelopes were trusted by filename alone, and `coord verify`
  did not recompute `fact_id`. The cache now binds each entry to the SHA-256
  of the fact bytes, and every live or archived fact is checked against a
  recomputed identity. Regressions:
  `coord::read::regressions::parse_cache_entry_tampering_fails_even_when_fact_bytes_are_unchanged`
  and
  `coord::verify::tests::verify_recomputes_fact_identity_before_accepting_the_store`.
- Oversized decline preimage diffs were written below disposable `cache/`
  while immutable facts referenced those paths. The sidecar now lives below
  the immutable `sidecars/` subtree, keyed by digest and observation second,
  and is created write-once. Regression:
  `cli::commands::ruling_run::tests::repeat_oversized_declines_use_observation_sidecars`.
- Panel follow-up also closed path, timestamp, and filename gaps. Kinds now
  use a safe family-shaped path component, stored coordination timestamps are
  whole-second UTC, and live plus archived filenames must match their content
  identity. Regressions cover kind traversal, fractional lease and
  park/unpark inputs, renamed live facts, and tampered archived facts.
Content-hash binding for sidecar bytes is out of scope for this unit because
the sidecar is a regenerable human-readable diagnostic; revisit if that
regeneration guarantee changes.

## Units filed on this evidence

- todo.coord-fact-store-hardening (cairn.coord): defects 1, 2, 3, 6 plus
  the required refinement to the atomic_write selection.
- todo.coord-cursor-semantics (cairn.kernel.query): defects 4 and 5.
- todo.query-contract-volatile-facts (cairn.kernel.query): the drifted
  contract numerals.

## Deliberately NOT filed

- Deepening cairn.coord's public surface and moving wave planning out of
  query_api: worth-exploring recommendations, not defects. They are
  recorded here and noted in todo.driver-in-repo's body so the
  sequencing question (harden before driver) reaches the maintainer at
  signature time.
- Reviving a generic StateBackend: the review itself rules this out; one
  storage implementation does not justify a port.
