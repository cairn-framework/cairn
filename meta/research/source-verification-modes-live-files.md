---
id: res.source-verification-modes-live-files
nodes:
  - cairn.kernel.artefacts
date: 2026-07-28
method: primary
---

# The three source verification modes leave a live in-repo file with no honest home

Measured on 2026-07-28 at `d48851d` on `main`, while a loop iteration selected
the first `cairn lint` finding and found it unclearable.

## What was measured

`cairn lint --json` reports four findings, all Info, none of them clearable by
the loop:

- `CAIRN_SOURCE_UNVERIFIED` for `src.query-api-draft-generation`.
- `CAIRN_SOURCE_UNVERIFIED` for `src.summariser-accept-path`.
- `CAIRN_SPEC_RULE_UNIMPLEMENTED` for spec:634, which names its deferring
  decision inline.
- `CAIRN_SPEC_RULE_UNIMPLEMENTED` for the contract node-shape rule, which also
  names its deferring decision inline.

The two source findings are the subject here. Both sources were added at
`d48851d` by the `contract-node-shape-drift` change. Both are in-repo code
reads: `file: src/query_api/mod.rs` and `file: src/summariser/`. Both bodies
state that `unverified` was chosen on purpose, because the cited paths are live
source files under active development.

`validate_sources` (`src/artefacts/registry/validate/mod.rs:297-315`) branches
on `verification` in three arms:

- `Verified` calls `validate_verified_source`, which requires `sha256`, re-reads
  the file on every scan, and raises `CAIRN_SOURCE_SHA256_MISMATCH` at Error
  severity when the bytes moved.
- `External` requires `file:` to begin with `http://` or `https://` (`is_url`,
  `src/artefacts/registry/io.rs:194`), at Error severity.
- `Unverified` emits `CAIRN_SOURCE_UNVERIFIED` at Info unconditionally and never
  reads `file:` at all.

So the perpetual advisory is not a defect in those two records. It is what the
schema does with an honest declaration.

## The corpus confirms the shape of the gap

All 17 sources in `meta/sources/`, by mode:

- 11 `external`, every one of them a URL to a repository, spec site, or manual.
- 4 `verified`, every one of them a path under `archive/strongholds/`: archived
  evidence bundles and captured eval transcripts, content that is immutable by
  construction because it lives in the archive.
- 2 `unverified`, both live paths under `src/`.

No `verified` source in the repository points at a file that anyone edits. The
mode is used exactly as `docs/conventions.md:423` prescribes: "Use it only for
genuinely immutable content." A live source file pinned by sha256 would raise an
Error on the next ordinary edit, which converts routine development into a
re-pin treadmill. Dropping the pin does not escape it: `validate_verified_source`
checks `sha256` first and raises `CAIRN_SOURCE_SHA256_MISSING`, also at Error
(`validate/mod.rs:329-338`). The only escape is reclassifying the record to
`unverified`, which trades the Error for the perpetual Info.

## Why the advisory cannot be cleared under current authority

`docs/spec.md:509` is explicit: "Sources in `unverified` state persist as
rationale tensions until moved to `verified` or `external`." Both destinations
are wrong for a live in-repo file. `external` requires a URL and asserts the
project does not hold the bytes; `verified` asserts the bytes are frozen. There
is no third destination, and `docs/conventions.md:425` already records the gap
in prose: "There is no friction-free tracked-local-file mode today (see open
questions); until one exists, routine, frequently-edited internal docs SHOULD be
cited sparingly to avoid a re-pin treadmill."

At the measured revision, before this artefact chain was added, that prose note
had no owning node, no todo, and no decision. It was the only place in the
repository where the gap was written down.

The same gap appears at `docs/spec.md:484`, where the source section states
without qualification that "Local files are immutable, enforced by checksum": a
mutable local source has no home in that sentence either.

## Cost of leaving it

The cost is not the two lines of scan output. It is that loop-mode selection is
lint-first with no severity threshold, so every fresh iteration selects
`CAIRN_SOURCE_UNVERIFIED` first, re-derives that it is unclearable, and moves
on. That is the exact pathology `todo.deferred-finding-cites-decision` closed
for `CAIRN_SPEC_RULE_UNIMPLEMENTED`, by adding a `Deferred-by` cell to
`docs/registries/spec-rules.md` so the finding names its own deferral. Sources
have no equivalent, so the re-derivation has no end condition.

## What this does not settle

It does not decide which resolution to take; `dec.source-tracked-verification`
weighs the options and recommends one. It does not size the implementation, and
it does not claim any other repository has hit the gap. It also does not touch
`dec.source-file-never-self`, which constrains what `file:` may point at, not
which verification modes exist.
