# Design: decision-ratification-tiers

## Approach

The todo is the ratified design; this file binds it to the code as it stands
at `8f0bbad`. Three new decision fields (`ratification:`, `affects:`,
`ratified_by:`, plus a `receipts:` list naming Review artefacts), three new
review fields (`subject_hash`, `reviewer`, `lens_prompt_hash`), one canonical
manifest hasher shared by scanner and hook, one committed allowlist data file,
scanner checks for the graph-provable half, a commit hook for the
evidence-bound half, and the wire, docs, and loop-asset surfaces that teach
the rule.

Key mechanical rulings, all fixed by the ratified todo text:

- **Tier values**: `local` and `binding`; absent parses as `binding`. The only
  legal `ratified_by:` value is `machine`; absent means human-signed.
  `ratified_by: machine` on a `binding` decision is an Error.
- **Governed content**: the decision's own bytes with the ratification
  frontmatter keys removed line-wise (`status`, `ratification`, `ratified_by`,
  `receipts`, each key line plus its indented continuation lines). No YAML
  re-serialisation: extraction is line-based and byte-deterministic. Receipt
  artefacts are excluded from the manifest BY IDENTITY: any path under the
  Review artefact directory (`meta/reviews`) never enters a manifest, listed
  in `affects:` or reached by a directory walk, existing or not. Exclusion
  never depends on the `receipts:` reference list, or the candidate hash
  would be uncomputable before receipts exist and the proposed-to-accepted
  invariance would break. Every other `affects:` path hashes raw bytes.
- **Manifest**: sorted `path` plus sha256 lines, hashed once
  (`subject_hash: sha256:<64 hex>`), built by `registry/manifest.rs` beside
  the existing `registry/sha256.rs` source-hashing infrastructure. One
  function, unit-tested, consumed by both scanner and hook so results are
  byte-identical by construction.
- **Allowlist**: `docs/registries/binding-surface.md`, an ordered list of
  exact repository-relative paths or `dir/` prefix rules, parsed from the
  list rows only. No globs. Normalise both sides (no `.` or `..` segments,
  symlinks resolved); a path escaping the repository is an Error, not a miss.
  Starting rules: `docs/spec.md`, `docs/registries/`,
  `tools/agent-pack/content/`, `src/artefacts/registry/`, `cairn.blueprint`.
  The file lives inside `docs/registries/`, so extending it is binding by
  construction.
- **Container span**: a `local` decision's `nodes:` must resolve inside one
  container, computed from the graph's node kinds (a node's container is its
  nearest container-or-system ancestor). Supersession of anything disqualifies
  `local` outright.
- **Convergence**: exactly the todo's rule, applied to EVERY accepted `local`
  decision; `ratified_by` marks who signed, never which checks run, so human
  acceptance cannot silently bypass the protocol (a maintainer who wants a
  receipt-free signature declares the decision `binding`). Two receipts
  referenced from the decision's `receipts:` list, `review_type:
  agent_cross_model`, clean verdict, `reviewer` values
  (`<model-id>/<lens-id>`) differing in model id or lens id, identical
  `subject_hash` equal to the recomputed manifest, receipt paths covered by
  `affects:`. Two receipts with identical `reviewer` and `lens_prompt_hash`
  are one round. Stale-hash receipts are kept for audit, never counted, and
  surface as an Info, not an Error.
- **`affects:` entry forms**: an exact repository-relative file path, or a
  directory rule ending in `/` (matching the allowlist syntax). A directory
  entry prefix-matches in the hook's subset check and contributes every file
  under it (sorted recursive walk) as individual manifest entries. The
  ratified dogfood example (`dec.bootstrap-fixture-corpus-split`, whose
  rubric lists `tests/fixtures/cairn-bootstrap/`) requires the form.
- **Verdict grammar, exact and tested**: no new schema field (the ratified
  todo enumerates the Review additions exhaustively). The already-required
  `## Verdict` body section gets a precise parsed grammar: the first line
  matching exactly `## Verdict` opens the section, and the first non-blank
  line after it must start with `PASS` or `BLOCKING` at column zero
  (uppercase; trailing prose free). Clean means `PASS`. A missing heading,
  an empty section, or any other first token is not clean and fails the
  convergence check with a leg-naming message; the grammar parser is one
  function with its own unit tests.
- **Hook, range-based against the squash rider**: the checked diff is always
  a cumulative range, never the staged commit alone, in two explicit modes:
  pre-commit mode compares merge-base(origin/main, HEAD) to the index (prior
  branch commits plus staged changes); CI mode (explicit flag) compares
  merge-base(origin/main, HEAD) to HEAD. Trigger: the range flips any
  decision to `status: accepted` at tier `local` (todo, Surfaces list: not
  qualified by `ratified_by`); a commit after the flip re-triggers because
  the range still contains it. Checks: every changed path in the range is
  inside the decision's `affects:`, and the manifest recomputed from the
  checked tree equals the receipts' `subject_hash`. Per-commit checking
  would let an earlier branch commit smuggle ungoverned paths into the
  squash-merge that lands the acceptance; the keystone forbids exactly
  that. No merge base (missing origin/main): the hook FAILS CLOSED with a
  message naming the missing ref, never a narrower check, so the security
  property does not vary by environment. Fail closed, decision named.
  Registration follows the existing `hook all` battery
  (`src/hooks/mod.rs:97-114`); the architecture gate
  (`src/hooks/architecture.rs`) is the structural precedent. The dogfood CI
  job must invoke the CI mode on the PR checkout (a clean checkout has no
  staged acceptance to inspect); if the workflow lacks it, this change adds
  it.
- **Wire**: `SCHEMA_VERSION` 7 to 8 (`src/query_api/mod.rs:63`).
  `handlers/pending.rs` drops `RATIFICATION_DEFAULT` (line 14) for the parsed
  field, keeping `binding` for artefacts that omit it, and each local-tier
  row additionally carries `subject_hash`, the manifest hash recomputed from
  the current tree (null on manifest error or binding tier): receipts must
  be authored carrying H, so the queue is the surface that prints H for
  candidates. The decisions listing gains `ratification` and `ratified_by`.
  `ratified_by` is tri-state on the wire: `machine` whenever the marker is
  present, `maintainer` only when accepted without the marker, null for
  unaccepted decisions without it (a proposed decision has no signer yet).
  Affected wire snapshots rebase; `schemas/PendingResponse.schema.json`
  updates with the new optional field.
- **Finding codes**: indicative allocation, finalised by the implementing
  commit per `docs/conventions.md` registry rule 2: CA045
  `CAIRN_DECISION_RATIFICATION_INVALID`, CA046
  `CAIRN_DECISION_RATIFIED_BY_INVALID`, CA047
  `CAIRN_DECISION_AFFECTS_INVALID` (malformed or repository-escaping entry),
  CA048 `CAIRN_REVIEW_SUBJECT_HASH_INVALID`, CA049
  `CAIRN_REVIEW_REVIEWER_INVALID`, CA050
  `CAIRN_REVIEW_LENS_PROMPT_HASH_INVALID`, CA051
  `CAIRN_DECISION_TIER_SPAN`, CA052 `CAIRN_DECISION_TIER_SUPERSEDES`, CA053
  `CAIRN_DECISION_TIER_BINDING_PATH`, CA054
  `CAIRN_DECISION_CONVERGENCE_UNMET` (any accepted `local` decision), CA055
  `CAIRN_DECISION_RECEIPT_UNKNOWN`, CA056 `CAIRN_REVIEW_SUBJECT_UNMATCHED`
  (Info, audit pointer), CA057 `CAIRN_DECISION_MACHINE_BINDING`, plus two
  hook-section codes (CH004 affects-subset, CH005 manifest-mismatch; CH003
  is already allocated to `CAIRN_INTERFACE_HASH_CHANGED`) for the
  commit-time refusals on any `local` acceptance. Copy strings land in
  `docs/design-system/copy.toml` `[findings.codes]`.
- **Lens identity**: `reviewer` is `<model-id>/<lens-id>`; the lens id names a
  committed prompt file and `lens_prompt_hash` is that file's sha256. The two
  loop lenses land at `docs/agent/lenses/correctness.md` and
  `docs/agent/lenses/simplicity.md`.
- **Ratification provenance**: `meta/decisions/decision-ratification-tiers.md`
  (`dec.decision-ratification-tiers`, `status: accepted`, rubric included)
  records the W8 ratification this implementation executes, so the schema
  change carries its own decision chain.

## Changes

ADDED:
- `src/artefacts/registry/manifest.rs`: governed-content extraction, manifest
  builder, canonical hash; unit tests including the status-flip invariance.
- `src/scanner/ratification.rs`: tier span, supersession, allowlist, and
  convergence checks (shape follows `src/scanner/todo_defers.rs`), with
  fixture tests per refusal.
- `src/hooks/ratification.rs`: the two commit-time refusals plus tests.
- `docs/registries/binding-surface.md`: the allowlist data file.
- `docs/agent/lenses/correctness.md`, `docs/agent/lenses/simplicity.md`.
- `meta/decisions/decision-ratification-tiers.md`.

MODIFIED:
- `src/artefacts/registry/types.rs`, `parse.rs` (and `kinds.rs` if the kind
  table carries field metadata): the four decision fields, three review
  fields, per-field invalid findings.
- `src/artefacts/registry/validate/`: receipt-link checks (unknown receipt id,
  unmatched subject hash).
- `src/scanner/mod.rs`: register the new check module.
- `src/hooks/mod.rs`: register the ratification hook in `hook all`.
- `src/query_api/mod.rs`: `SCHEMA_VERSION` 8.
- `src/query_api/serialise.rs`: tier and `ratified_by` wire values (the
  exhaustive matches force completeness).
- `src/query_api/handlers/pending.rs`: parsed tier, default preserved.
- The decisions listing handler: `ratification` and `ratified_by` fields.
- `schemas/PendingResponse.schema.json` and `tests/schema_validation.rs`: only
  if the wire value set changes shape.
- `tests/wire_format_snapshots.rs` snapshots: rebase for schema_version 8 and
  new fields.
- `.claude/skills/cairn-loop-reconcile/SKILL.md` (clause 4, line 85 region),
  `.claude/skills/cairn-loop-scope/SKILL.md` (section 2, line 47 region), and
  their canonical copies under `tools/agent-pack/content/skills/`: the
  never-self-ratify rule becomes tier-aware (binding: never; local: only via
  the receipt protocol).
- `docs/conventions.md` section 10 and `docs/artefacts.md`: teach the new
  decision and review fields.
- `docs/registries/spec-rules.md`: one row per newly enforceable rule.
- `docs/registries/error-codes.md`: the new codes.
- `docs/design-system/copy.toml`: `[findings.codes]` entries.
- `meta/decisions/parked-deferral-composition.md` (`ratification: binding`)
  and `meta/decisions/bootstrap-fixture-corpus-split.md`
  (`ratification: local` plus `affects:`): the dogfood pickup B5 deferred;
  both stay `proposed`.
- `meta/todos/todo.decision-ratification-tiers.md`: status via
  `cairn todo set` at close.

REMOVED:
- `RATIFICATION_DEFAULT` hardcode in `handlers/pending.rs` (the constant, not
  the documented default behaviour).

RENAMED:
- Nothing.
