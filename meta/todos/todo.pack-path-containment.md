---
node: cairn.kernel.cli
status: done
created: 2026-07-27
---

# Pack Path Containment

## Priority

P0. `cairn pack uninstall` could delete a file outside the project root,
`cairn pack install` could publish the ownership ledger outside it, and
`cairn init --wire` could scaffold the agent guide outside it. All are
reachable with no privileges: an ordinary symlink in the target repository.

## Scope

Three surfaces write into a user's project during install and campaign work:
the pack lifecycle, the campaign lock, and the `cairn init` scaffolding that
runs before them. Each must act only on a project-relative path reached without
traversing a symlink, and must never read a file type whose read can block.
Commands outside those three were not audited here.

A directory at a destination is deliberately out of scope. Reading one fails
immediately and loudly, which is the behaviour every verb already had and which
`tests/pack_lifecycle.rs` pins; turning that into a reported drift would be a
separate product change.

Found by auditing cairn against mechanism 5 (manifest integrity) of
`res.harness-engineering`, whose `validate_manifest.py` reference rejects a
symlinked leaf, resolves strictly, and requires a regular file. The author-time
renderer in `tools/agent-pack/src/containment.rs` already did all three. The
shipped runtime applied only the symlink rule, and only on writes, so lifecycle
reads, ledger publication, removal, and scaffolding were unguarded.

## Acceptance

- Reproduced before the fix, and each case is a regression test in
  `tests/pack_path_containment.rs`.
- No pack lifecycle verb, campaign-lock operation, or init scaffold write
  reads, writes, or removes outside the project root.
- A FIFO, socket, or device at a destination cannot block a verb.

## Outcome

Fixed on 2026-07-27. `src/cli/commands/wire.rs` now owns the single containment
policy, beside the `check_symlink_containment` it builds on, because it is not
a pack concern: `contained_path` rejects any path component that is not a
normal one and any component reachable through a symlink, and `readable_path`
adds the blocking-file-type refusal while distinguishing absent from unreadable
through `symlink_metadata` rather than `Path::exists`. Applied to the ledger
read and write, asset writes, asset classification, uninstall, campaign
resolution, the campaign lock, and init scaffolding. The ad-hoc
`check_symlink_containment` call sites that used to stand in for it are gone.

Five defects were reproduced against the shipped binary first:

1. `pack install` published the ledger outside the root through a symlinked
   `.cairn/state`.
2. `pack uninstall` deleted a file outside the root through a symlinked parent
   whose target matched the recorded hash.
3. `pack campaign end` deleted a snapshot outside the root the same way. It is
   the one verb the ledger's adapter-root check cannot protect, because it runs
   before the ledger is consulted.
4. `init --wire` wrote `AGENTS.md` and `state/` outside the root through a
   symlinked `.cairn`, before any pack guard could refuse.
5. A symlink at an owned destination was counted pristine.

A sixth case, a FIFO at an owned destination, hung `pack status` indefinitely;
its regression test drives the real binary with a deadline so a reintroduced
hang fails the test rather than wedging the suite. All six fail or hang without
the fix and pass with it.

A seventh test covers a ledger row containing `..`. That one already passed
before the fix, because the dispatcher's adapter-root check refuses such a
ledger, and it is kept as defence in depth; the lexical half of the policy is
pinned directly by unit tests in `wire.rs` instead.

Cases 3 and 4 were found by the two pre-submit review lenses, not by the
original audit: the first asked whether one policy really covered every path,
and the second traced what ran before the pack lifecycle did.

Two narrower hardening fixes came from the same review. `write_pinned` sets the
read-only bit through its open handle rather than re-resolving the path, and
the Windows campaign cleanup refuses nested links instead of following them. A
bounded parent-component TOCTOU remains: the check and the act are not atomic,
which would need `openat`-style machinery the repository does not have. That is
materially narrower than the static escapes fixed here.
