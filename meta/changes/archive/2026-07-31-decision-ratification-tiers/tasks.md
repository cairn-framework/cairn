# Tasks: decision-ratification-tiers

- [x] 1. Manifest hasher: `src/artefacts/registry/manifest.rs` with
  governed-content extraction (line-based key removal), sorted manifest,
  canonical sha256; unit tests for ordering, byte sensitivity, and the
  keystone status-flip invariance.
- [x] 2. Schema fields: `ratification:`, `affects:`, `ratified_by:`,
  `receipts:` on Decision and `subject_hash`, `reviewer`,
  `lens_prompt_hash` on Review in `types.rs`/`parse.rs`, absent-means-binding
  default, per-field invalid-value findings with unit tests.
- [x] 3. Allowlist: `docs/registries/binding-surface.md` data file plus its
  parser (normalisation, prefix rules, escape-is-Error) and unit tests.
- [x] 4. Scanner checks: `src/scanner/ratification.rs` (span, supersession,
  allowlist, convergence, machine-on-binding), registered in scan; fixture
  tests one per refusal, including single receipt, duplicated reviewer
  identity, stale hash, and receipt path missing from `affects:`.
- [x] 5. Validate: receipt-link checks (unknown receipt id Error, unmatched
  subject hash Info) in `src/artefacts/registry/validate/`.
- [x] 6. Hook: `src/hooks/ratification.rs` affects-subset and
  manifest-equality refusals, registered in `hook all`; a test apiece.
- [x] 7. Wire: `SCHEMA_VERSION` 8, serialise arms, pending tier from the
  parsed field, decisions listing carries `ratified_by`; snapshots rebased,
  schema files updated if shape changed.
- [x] 8. Lens prompts: `docs/agent/lenses/correctness.md` and
  `docs/agent/lenses/simplicity.md` committed; `reviewer` id format
  documented beside them.
- [x] 9. Loop assets: tier-aware self-ratification rule in
  `cairn-loop-reconcile` clause 4 and `cairn-loop-scope` section 2, both
  `.claude` and `tools/agent-pack/content/` copies; pack-conformance tests
  stay green.
- [x] 10. Docs and registries: `docs/conventions.md` section 10,
  `docs/artefacts.md`, `docs/registries/spec-rules.md` rows,
  `docs/registries/error-codes.md` codes, `copy.toml` entries.
- [x] 11. Provenance: `dec.decision-ratification-tiers` (accepted, rubric,
  cites PR #528 sheet W8); tier frontmatter added to the two live proposed
  decisions (`binding` and `local` plus `affects:`); todo status updated via
  `cairn todo set`.
- [x] 12. Evidence: full gates (`scripts/pre-archive-rust-gates.sh`,
  `cargo test`, `cairn scan --strict`, `cairn hook all`), and the dogfood
  boundary run: `cairn pending --json` renders the two live tiers on this
  repository.
