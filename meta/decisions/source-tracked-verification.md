---
id: dec.source-tracked-verification
nodes:
  - cairn.kernel.artefacts
status: proposed
date: 2026-07-28
informed_by:
  - res.source-verification-modes-live-files
related:
  - dec.source-file-never-self
---
# A fourth source verification mode for live files the repository tracks

## Context

A Source artefact declares how far the project trusts its `file:` pointer.
Three modes exist: `verified` (repo-relative path plus a `sha256` the scanner
re-checks on every run), `external` (a URL the project does not hold the bytes
of), and `unverified` (a perpetual Info advisory).

`res.source-verification-modes-live-files` measured what that costs and holds
the evidence: no `verified` source in this repository points at a file anyone
edits, both `unverified` sources point at live paths under `src/`, and the two
destinations the spec offers an unverified source are false statements about a
tracked working file. `docs/spec.md:509` says such a source "persist[s] as
rationale tensions until moved to `verified` or `external`", and
`docs/conventions.md:425` already records the missing third destination in prose
without an owning node, decision, or todo.

## Decision

Add a fourth value of `verification`: `tracked`.

1. `tracked` means the cited bytes live in this repository's working tree and
   the project reads them as they stand rather than freezing them. `file:` is a
   path of at least one normal component, optionally prefixed `./` because
   existing records are written that way, with no leading `/` and no `..`
   (`file: ./` cites the root, not evidence, and is rejected), resolving to
   an existing file or directory whose canonical path stays under the
   repository root, and `sha256` is absent. Containment is part of the rule,
   not an implementation detail: `root.join(file)` silently resolves an
   absolute component outside the root, as this repository already records at
   `src/cli/commands/wire.rs:103-129`, and a symlink can leave the tree after
   the join succeeds.
2. The scanner checks resolution and containment, no bytes, and must probe with
   metadata (`Path::try_exists` plus canonicalisation, or equivalent), not the
   `fs::read` that
   `validate_verified_source` uses: `src.summariser-accept-path` cites
   `src/summariser/`, a directory, which `fs::read` rejects. A `tracked` source
   whose `file:` does not resolve, or resolves outside the repository root, is
   an Error, reusing `CAIRN_SOURCE_READ_FAILED`
   rather than adding a code, so its descriptions in
   `docs/registries/error-codes.md` (CA031) and the "Deterministic enforcement"
   list in `docs/conventions.md:441` generalise from verified-source integrity
   to source path resolution. A resolving `tracked` source produces no
   finding: the tension the Info advisory exists to express is absent, because
   nothing is unresolved.
3. `unverified` keeps its present meaning exactly: not yet pinned, resolution
   pending, perpetual advisory. It is the honest mode for a record whose
   evidence is a conversation or an unsaved artefact, which is what
   `dec.source-file-never-self` already assumes when it pairs `file: null` with
   `unverified`.
4. `verified` stays mandatory for content that must be frozen: archived
   evidence, captured transcripts, anything cited as proof of a past state.
   `tracked` is not a licence to stop pinning; a reviewer seeing `tracked` on an
   archive path should read it as a defect.
5. A `tracked` source that declares `sha256` is an Error under a new code,
   `CAIRN_SOURCE_SHA256_UNEXPECTED`, registered at the next free CA number in
   `docs/registries/error-codes.md`. It does not reuse
   `CAIRN_SOURCE_VERIFICATION_INVALID`, whose subject is a malformed
   `verification` value, and it is not a silently ignored field: an author who
   pinned a hash asked for `verified`, and accepting both would let `tracked`
   erode the frozen mode by accident.
6. The whole normative source-verification block in `docs/spec.md` is amended,
   not one line of it: the introductory claim at :484 that "Local files are
   immutable, enforced by checksum", the enum comment at :491, the per-state
   list at :505-507 (which independently names `verified` or `external` as the
   only resolutions), the integrity rule at :509, and the freshness rule at
   :511, which must state that `tracked` sources carry an existence and
   containment check and no hash check.
7. Every rule this decision makes Designed gets a row in
   `docs/registries/spec-rules.md`, which claims to track all of them:
   tracked-path resolution, tracked-path containment, and hash-absence.

This decision is `proposed`. It changes a stated spec invariant and adds a value
to the artefact schema every adopting repository shares. A loop iteration cannot
make a ruling of that reach; `dec.source-file-never-self` stopped at the same
boundary for the same reason.

## Rationale

Four options were weighed.

**Do nothing.** The advisory is spec-sanctioned and costs two Info lines. It was
rejected because the cost is not the output: lint-first selection re-selects it
every iteration, and the record itself is misleading. `unverified` reads as
"someone should finish this", when the correct end state has already been
reached and cannot be improved.

**Annotate the deliberateness.** Add an `unverified_reason:` field, or cite an
accepting decision, and name it inline in the finding, mirroring the
`Deferred-by` cell that `todo.deferred-finding-cites-decision` added to the
spec-rule registry. This was rejected as the primary fix because it documents a
misclassification instead of correcting it. The spec-rule precedent annotates a
rule that genuinely is unbuilt and someday will be; a git-tracked source is not
waiting on anything.

**Pin the live files anyway**, either by `sha256` or by adding a `commit:` pin
resolved through git. The sha route is the re-pin treadmill `conventions.md:423`
warns against and would raise an Error on unrelated edits. The commit route is
defensible but expensive: it makes the scanner depend on git object resolution,
which nothing in the artefact registry does today, and it buys precision the
sources do not need. Both remain open if a future source must cite an exact
historical revision.

**`tracked`, chosen.** It is the mode the corpus already wants: the repository
holds the bytes, and the source record names the path and the date it was read.
Resolution inside the working tree is the strongest invariant a scanner can
check without either freezing the file or reading the git index, and it is the
one that actually protects the reader: a pointer into deleted or external code
is the failure worth catching, and it is the one the current `Unverified` arm
misses entirely because it never reads `file:`. It is deliberately not a claim
that git tracks the path. An ignored or untracked file inside the tree passes,
and enforcing otherwise would put index resolution in the artefact registry,
which is the cost this option exists to avoid.

## Consequences

- `SourceVerification` gains a `Tracked` variant. Every exhaustive match over it
  must gain an arm, including `source_verification` in
  `src/query_api/serialise.rs:320`, which puts `"tracked"` on the wire.
- `docs/conventions.md` section 10 gains the mode, and the sentence at line 425
  is rewritten: the "no friction-free tracked-local-file mode today" clause goes,
  and the sparing-citation advice it introduces becomes advice about `verified`
  specifically.
- `docs/spec.md` is amended per clause 6. Any `docs/registries/spec-rules.md`
  row anchored below an edit must be re-checked individually, never
  blanket-shifted, and clause 7's new rows are added.
- Guidance that teaches the closed set of three must move with the schema:
  `docs/artefacts.md:108` and the shipped agent-pack references under
  `tools/agent-pack/content/skills/cairn-dev/references/`.
- `dec.source-file-never-self` already binds every verification value, so its
  rule needs no amendment, but the coverage its dependent
  `todo.source-self-reference-finding` promises is written as "all three
  `verification` values". Whichever of the two lands second extends that to
  four; a tracked self-pointer resolves and would otherwise pass unchecked.
- `src.query-api-draft-generation` and `src.summariser-accept-path` move to
  `tracked`, clearing their `CAIRN_SOURCE_UNVERIFIED` findings.
- Adopting repositories gain a value, not a constraint: nothing existing
  re-classifies itself, and no current record becomes invalid.
- Implementation is tracked by `todo.source-tracked-verification-mode`, blocked
  until this decision is accepted.
