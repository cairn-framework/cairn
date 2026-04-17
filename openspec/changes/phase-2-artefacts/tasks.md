# Tasks: Phase 2 Artefacts

## 1. Registry and Shared Parsing

- [x] 1.1 Refactor the Phase 1 contract loader into a typed artefact registry.
- [x] 1.2 Implement shared Markdown frontmatter parsing with typed error reporting.
- [x] 1.3 Add reusable artefact IDs, node references, source references, and finding types.

## 2. Artefact Loaders

- [x] 2.1 Implement todo loading and status filtering.
- [x] 2.2 Implement decision loading with ADR status, provenance links, and ADR-to-ADR links.
- [x] 2.3 Implement review loading with all three review subtypes.
- [x] 2.4 Implement research loading with node and source references.
- [x] 2.5 Implement source loading with verification states and SHA-256 checks.

## 3. Integrity Rules

- [x] 3.1 Validate node references across todos, decisions, reviews, and research.
- [x] 3.2 Validate research-to-source and decision-to-source links.
- [x] 3.3 Validate decision `supersedes`, `refines`, and `related` references.
- [x] 3.4 Report source orphan warnings, unverified source tensions, and SHA-256 structural errors.
- [x] 3.5 Add tests for every integrity rule and every finding class used by Phase 2.

## 4. Queries and Output

- [x] 4.1 Implement `todos`, `decisions`, `research`, `sources`, `rationale`, and `status`.
- [x] 4.2 Extend `neighbourhood` default output to include contracts and accepted decisions.
- [x] 4.3 Add include flags for todos, research, reviews, deprecated decisions, and active changes.
- [x] 4.4 Add stable JSON schemas and human-readable output snapshots for all new commands.

## 5. Documentation

- [x] 5.1 Document each artefact schema with a valid example.
- [x] 5.2 Document finding classes for artefact integrity failures.
- [x] 5.3 Document default and opt-in artefact inclusion for `neighbourhood`.

## 6. Required Verification

- [x] 6.1 `cargo build` passes with zero warnings.
- [x] 6.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 6.3 `cargo fmt --check` passes.
- [x] 6.4 `cargo test` passes.
- [x] 6.5 `cargo test --locked` passes.
- [x] 6.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-2-artefacts --strict` passes.
