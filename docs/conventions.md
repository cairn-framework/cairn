# Cairn OpenSpec Conventions

Cross-cutting conventions for all phases of the Cairn implementation. Every phase implementor MUST read this document before beginning work. These conventions are normative; violations are implementation defects.

---

## 1. Error Code Registry

### Format

Every Cairn error code follows the format **`CXNNN`** where:

- **C** is the literal prefix character (for "Cairn").
- **X** is a single uppercase letter identifying the subsystem category.
- **NNN** is a zero-padded three-digit number (001--999) within that category.

### Category Letters

| Letter | Subsystem        | Owning Phase(s) |
|--------|------------------|-----------------|
| P      | Parser (blueprint)     | 1               |
| K      | Kernel/Map  | 1               |
| A      | Artefacts        | 2               |
| C      | Changes          | 3               |
| H      | Hooks            | 4               |
| E      | Edges            | 5               |
| T      | Targets          | 6               |
| M      | MCP              | 7               |
| S      | Summariser       | 8               |
| O      | CLI output / I/O | 7.8 onwards     |
| B      | Brownfield       | 9               |
| D      | Distribution     | 10              |

### Registry Location

The canonical registry lives at `docs/registries/error-codes.md`. That file is the single source of truth for which codes are allocated.

### Rules

1. Before adding any error code, the implementor MUST read the current registry to avoid collisions.
2. New codes MUST be appended to the registry file as part of the same commit that introduces the code in Rust source.
3. Error codes are **stable once assigned**. A code SHALL NOT be reused, reassigned, or renumbered after it appears in the registry.
4. Each registry entry records: code, one-line description, and the phase that introduced it.
5. Within a category, numbers MUST be allocated sequentially starting at 001. Gaps left by removed features are not backfilled.

### Usage in Code

All error types that surface to users MUST carry an error code from the registry. The `CairnError` type SHALL expose the code via a `.code()` method that returns the string form (e.g., `"CP001"`). JSON error output SHALL include a `code` field with this value.

---

## 2. Module Size and Code Quality Guards

### File Size

- Maximum file length: **500 lines** (including tests, doc comments, and blank lines).
- When a module reaches **300 lines** and contains a clear structural seam, the implementor MUST split it proactively rather than waiting for the 500-line limit.

### Type Density

- Each `.rs` file SHALL expose **one primary public type** (struct, enum, or trait).
- Helper types that exist solely to support the primary type (e.g., a builder, an iterator adapter) MAY coexist in the same file.
- If a helper type grows its own public API surface or is used outside the parent module, it MUST move to its own file.

### Function Length

- Maximum function body: **50 lines** (excluding doc comments and attributes).
- Extract helpers at natural seams. Prefer small, named functions over inline closures longer than 10 lines.

### Nesting Depth

- Maximum nesting depth: **4 levels** (counting `fn`, `if`, `match`, `for`, `while`, `loop`, and closure bodies).
- Prefer early returns and guard clauses to reduce nesting.

### Visibility

- `pub` is reserved for items that form the crate's external API.
- For cross-module but crate-internal access, use `pub(crate)` or `pub(super)`.
- Default to private. Promote visibility only when a consumer outside the current module requires it.

### Module Tree Structure

The module tree MUST mirror the conceptual architecture:

```text
src/
  blueprint/          # Phase 1: parser, lexer, AST
  map/     # Phase 1: graph, build, query, integrity
  artefacts/    # Phase 2: contract, todo, decision, etc.
  reconcile/    # Phase 1: trait, code reconciler, fingerprint
  scanner/      # Phase 1: scan orchestration, outputs, state
  changes/      # Phase 3: delta parser, archive, rename
  hooks/        # Phase 4: hook runner, commit/task hooks
  edges/        # Phase 5: edge validation
  targets/      # Phase 6: multi-target resolution
  mcp/          # Phase 7: MCP server, tool registry
  summariser/   # Phase 8: summariser trait, impls
  brownfield/   # Phase 9: extraction engine
  distribution/ # Phase 10: packaging, output formats
  cli/          # Phases 1+: command definitions, output rendering
  error.rs      # Shared CairnError type
  lib.rs        # Crate root, re-exports
  main.rs       # Thin binary entrypoint
```

Each phase adds its own subtree. A phase MUST NOT modify another phase's module subtree unless the change is a bugfix or an explicitly designed cross-cutting extension documented in the phase's design.

### Re-exports

Each module subtree SHALL have a `mod.rs` (or directory module) that re-exports only the public API. Internal submodules MUST be private (`mod internal;` not `pub mod internal;`). Consumers outside the subtree interact exclusively through the re-exported surface.

---

## 3. State Versioning

### Scope

Every file written to `.cairn/state/` MUST include a `version` field.

### Rules

1. The `version` field is an integer, starting at **1** for the initial schema.
2. The `version` field MUST be the **first field** in JSON state files, before all other keys, so that version inspection does not require a full parse.
3. When a phase changes the schema of an existing state file, it MUST:
   - Increment the version number by exactly 1.
   - Provide a migration function (`fn migrate_v{N}_to_v{N+1}`) that converts the previous version's data to the new schema.
   - Place the migration function in the same module that owns the state file's serialization.
4. State readers MUST check the `version` field before deserializing the payload. If the version is **higher** than the reader understands, the reader SHALL fail with a clear error message naming the expected and found versions, not silently ignore unknown fields.
5. If the version is **lower** than the current version, the reader SHALL apply the migration chain to bring the data forward.

### Example

```json
{
  "version": 1,
  "interface_hashes": { ... }
}
```

---

## 4. Shared Type Conventions

### Trait Derivations

All public types that cross module boundaries MUST derive or implement:

- `Debug`
- `Clone`
- `serde::Serialize` and `serde::Deserialize`

Types that are compared in tests SHOULD also derive `PartialEq` and `Eq`.

### Error Types

- All Cairn error types MUST use `thiserror::Error` for derivation.
- The top-level error type is `CairnError`, defined in `src/error.rs`.
- `CairnError` MUST carry an error code from the registry (section 1) and expose it via `.code() -> &str`.
- Subsystem error types MAY exist for internal use, but anything surfaced to users or returned from a public API MUST be convertible to `CairnError` via `From` implementations.

### ID Newtypes

Identifiers MUST be newtypes, not raw `String` values:

- `NodeId` -- stable blueprint node identifier.
- `ChangeId` -- change directory identifier.
- `ArtefactId` -- artefact file identifier.
- `ReconcilerId` -- reconciler identifier.

Each newtype SHALL wrap a `String`, implement `Display`, `FromStr`, `AsRef<str>`, and the standard derivations listed above. Constructors SHALL validate format invariants (e.g., `NodeId` segments are dot-separated identifiers).

### Path Handling

- All file-system paths in public APIs MUST use `camino::Utf8PathBuf` and `camino::Utf8Path`, not `std::path::PathBuf` or `std::path::Path`.
- This ensures cross-platform UTF-8 safety and ergonomic string conversion.
- The `camino` crate MUST be declared as a workspace dependency.

### Result Convention

All public APIs MUST return `Result<T, CairnError>`. Internal functions MAY use narrower error types, but the public boundary MUST unify on `CairnError`.

---

## 5. Testing Conventions

### Unit Tests

- Colocated in the same file, inside a `#[cfg(test)] mod tests { ... }` block.
- Unit tests verify a single module's logic in isolation. They MUST NOT touch the filesystem or depend on external state unless the module under test is a filesystem module.

### Integration Tests

- Located in the `tests/` directory at the crate root.
- One file per command or major feature (e.g., `tests/scan.rs`, `tests/lint.rs`, `tests/archive.rs`).
- Integration tests exercise the public library API or CLI binary against real fixture data.

### Test Fixtures

- Located in `tests/fixtures/`.
- Subdirectories per phase: `tests/fixtures/phase-1/`, `tests/fixtures/phase-3/`, etc.
- Shared fixtures (used by multiple phases) live in `tests/fixtures/shared/`.
- Fixtures MUST be committed to the repository. Tests MUST NOT generate fixtures at runtime except for temporary files in a `tempdir`.

### Snapshot Testing

- Use the `insta` crate for snapshot testing.
- Snapshot tests MUST cover CLI human-readable output and JSON output for every command.
- Snapshot tests MUST cover the serialized form of state files written to `.cairn/state/`.
- Public JSON wire formats SHALL be pinned via `insta` snapshot tests.
- Snapshot files are committed and reviewed like any other source file.

### Test-First Pre-Phase

Feature phases that introduce new acceptance criteria SHOULD be preceded by a paired pre-phase `phase-<N>.0-tests` whose apply task writes failing test assertions against the feature's acceptance criteria.

Pre-phase tests MUST be marked `#[cairn_planned(phase = <N>)]` so pre-phase archives pass `cargo test` cleanly. The proc-macro expands to `#[ignore = "cairn_planned: phase-<N>"]` underneath so `cargo test` keeps working without runner changes. Phase `N`'s first task group MUST remove the `#[cairn_planned]` attribute from the relevant test as the corresponding feature code lands.

The macro rejects combination with manual `#[ignore]`; a test cannot carry both attributes. If a planned test also needs an unrelated ignore reason, the planned attribute should be removed once the prerequisite phase lands, and a plain `#[ignore]` added for the orthogonal concern.

For decimal phase numbers (for example `phase-7.6`), encode the phase argument as a zero-padded integer: major * 100 + minor (`phase = 706`). This preserves injectivity (`phase-7.10` → `710`, `phase-71.0` → `7100`).

Verification states are modeled by the five-state `VerificationState` enum (`Draft`, `Planned`, `Passed`, `Failed`, `Blocked`) defined in `src/verification.rs`. See `archive/openspec/specs/testing-baseline/spec.md` for canonical scenarios and `docs/registries/error-codes.md` for the `CC001` error code used when a verification is `Blocked` by an upstream dependency.

### Coverage Requirements

- Every public function MUST have at least one test exercising its success path.
- Every error code in the registry MUST have at least one test that triggers it.
- Error path tests verify both the error variant and the error code string.

These coverage rules are enforced by review. A future gate may automate public-function coverage checking in `scripts/pre-archive-rust-gates.sh`.

### Naming Convention

Test names MUST follow the pattern:

```
test_{function_or_feature}_{scenario}_{expected_outcome}
```

Examples:
- `test_parse_valid_system_returns_ast`
- `test_scan_missing_path_reports_ghost`
- `test_archive_validation_failure_rolls_back`

---

## 6. Declared Items Tracker

### Location

The declared items tracker lives at `docs/registries/declared-items.md`.

### Purpose

The Cairn spec v0.6 contains items at the "Declared" maturity level -- capabilities that are in scope and named but not yet fully designed. When a phase encounters a Declared-level item that it cannot resolve or that creates ambiguity in its own implementation, it MUST NOT silently inherit the ambiguity.

### Rules

1. When an implementor encounters a Declared-level item from the spec that affects their phase, they MUST add a note to the declared items tracker.
2. Each tracker entry records: the spec section reference, the Declared item name, which phase encountered it, and a brief description of the ambiguity or dependency.
3. The tracker is append-only during implementation. Entries are resolved (not deleted) when the Declared item is promoted to Designed in a future spec revision.

---

## 7. Documentation Conventions

### Public Items

- Every public function, type, trait, and method MUST have a `///` doc comment.
- Doc comments explain **what** the item does and **why** it exists. They MUST NOT narrate implementation mechanics (the code shows how).
- Doc comments on functions MUST include a brief description of return value semantics and error conditions.

### Module-Level Documentation

- Every module file MUST begin with a `//!` comment explaining the module's role in the overall architecture and its relationship to neighboring modules.

### Private Items

- Private items MUST NOT have doc comments unless the logic is genuinely non-obvious or surprising (e.g., a workaround for a known platform bug).

### Crate-Level Documentation

- `src/lib.rs` MUST contain a crate-level `//!` doc comment that:
  - States the crate's purpose in one sentence.
  - Lists the top-level module tree with a one-line description of each module.
  - Explains how to navigate the crate for a newcomer.

### No Narrative Prose in Source

- Source files MUST NOT contain long narrative explanations. If a design rationale requires more than 3 lines, reference the relevant OpenSpec design document by path instead.

---

## 8. Git Hooks

Hooks are managed via [prek](https://github.com/j178/prek) (Rust rewrite of pre-commit, drop-in `.pre-commit-config.yaml` compatible). After clone, run `make install-hooks` to install both pre-commit and pre-push stages. Pre-commit runs `cargo fmt --check` plus the em-dash detector. Pre-push runs `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --locked`, `cargo doc --no-deps` (with `RUSTDOCFLAGS=-D warnings`), and `cairn lint`. CI mirrors the pre-push battery as a server-side safety net.

---

## 9. Genesis Transcript

When proposal authoring captures an elicitation transcript (historically via the retired `cflx-proposal` skill, now the `cairn-propose` skill), the transcript lives at `meta/changes/<id>/research/genesis.md` with id `genesis-<change-id>`. The file lives in the change directory, archives with the change directory, and is not loaded by the cairn scanner. It provides human-readable and codex-readable provenance for the change's elicitation history.

The `nodes` field of the genesis artefact carries the change ID as a placeholder, not a blueprint node ID. The apply-stage codex agent SHALL NOT re-point or rewrite this field. The file SHALL NOT be moved to `meta/research/` or any other location during apply or archive. Rationale and the option-A/B/C debate that produced this verdict live in `archive/strongholds/oq3-genesis-lifecycle.md`.

When a future phase implements artefact-delta processing in `cairn archive` and a scanner-visible `meta/research/genesis/` subtree exists, this convention may be revisited (see the forward-compatibility note in the linked stronghold). Until then, the change-directory-relative path is the durable home.

---

## 10. Artefact organization and provenance links

This section is normative. It governs how cairn's typed provenance artefacts (decisions, research, sources) are laid out on disk and linked into the graph, and it documents the relationship between those rules and the shipped scanner. It is written to describe what the engine actually enforces. Where a rule is author-side policy that the scanner does not yet gate, this section says so plainly, so that the convention never asserts a mechanical guarantee the code does not provide. Reviews and todos are touched on where they intersect these rules; contracts participate in the separate two-chain (interface-hash) topology and are not provenance artefacts.

Artefact TYPE is bound by the blueprint pointer FIELD that reaches a file (a `decisions` pointer makes a file a decision, `research` makes it research, `sources` a source), not by its directory name. Every rule below rests on that coupling.

### Layout

The canonical, scanner-loaded provenance artefacts live artefact-type-first and FLAT:

- `meta/decisions/` : decision files, FLAT. id prefix `dec.`.
- `meta/research/` : research files, FLAT. id prefix `res.`.
- `meta/sources/` : source files, FLAT. id prefix `src.`.

Filenames MUST be slug-only (for example `no-orchestrator.md`, `beads-repo.md`). The typed prefix lives ONLY in the `id:` frontmatter field (`id: dec.no-orchestrator`). A filename MUST NOT carry a `dec.`/`res.`/`src.` prefix. This matches every existing artefact; a filename-prefix rule would flag the entire current corpus.

Decisions, research, and sources MUST NOT use topic subfolders. The loader's directory read is NON-RECURSIVE: a file placed in a subfolder of a pointer directory is silently ignored with no finding, so folder placement alone never makes a file an artefact. Topical grouping MUST instead be expressed by slug namespacing in the id and filename (for example `res.gas-city.analysis`, `res.gas-city.issue-slate`). The ONLY sanctioned way to load a file outside the top level of a pointer directory is an explicit per-file blueprint pointer to that exact file. There is no depth limit to enforce, because depth is not the failure mode: non-recursion is.

Other typed material has a defined home but is governed by its own schema:

- `meta/todos/<node>/` : todos, deliberately NODE-partitioned. This is an explicit exception to artefact-type-first flat layout.
- `meta/reviews/` : reviews, FLAT (loaded only when a `reviews` pointer is wired).
- `meta/contracts/` : per leaf-module contracts (two-chain topology; not a provenance artefact).
- `meta/changes/<change-id>/` : one directory per change proposal; change-scoped artefacts archive with the change.

The set of directories whose `.md` files are treated as artefacts is CLOSED and defined by the blueprint pointers. In cairn's own repository only `decisions`, `research`, and `sources` pointers are wired, so files under `meta/reviews/`, `meta/todos/`, and per-node contracts are present but not scanner-loaded. Operational and non-artefact material is EXEMPT and MUST NOT be treated as a typed artefact: `meta/campaigns/`, `meta/debates/` (including its subfolders), change directories (including the genesis transcript), and loose files such as `meta/ui-contract.md` and `meta/session-handoff.md`.

Genesis exception. The elicitation transcript lives at `meta/changes/<id>/research/genesis.md` and is exempt across ALL axes (see conventions.md section 9 and archive/strongholds/oq3-genesis-lifecycle.md): its id is `genesis-<change-id>` (not a `res.` prefix), its `nodes` field carries the change id as a placeholder rather than a graph node, and it is NOT loaded by the scanner. The apply and archive stages MUST NOT re-point or relocate it.

This section closes spec section 16 open question Q-03 in favour of flat artefact-type-first layout and supersedes the stale `meta/decisions/kernel/` example in that section; the spec MUST be amended to match.

### Link model

Every scanner-loaded artefact anchors to the graph and cites its provenance through TYPED, PER-TYPE fields. The fields differ by type and MUST be used as named; a field written on the wrong type is silently ignored by the loader.

Decisions (`meta/decisions/`) carry:

- `id` (REQUIRED), prefix `dec.`.
- `nodes: [<graph-node-id>...]` (REQUIRED, non-empty; the graph anchor).
- `status` (REQUIRED): one of `proposed`, `accepted`, `deprecated`, `superseded`.
- `date` (REQUIRED).
- `informed_by: [<id>...]` (the up-chain). Each entry MAY reference a research id OR a source id directly (skip-level is allowed; research is NOT a required intermediary). `informed_by` MAY be empty: not all decisions require written research.
- `supersedes`, `refines`, `related`: arrays of DECISION ids (lateral edges). A `supersedes` target SHOULD itself carry `status: superseded`.
- `orphaned` / `orphan_reason`: the sanctioned hatch for a decision whose nodes are all unknown (see below).

Research (`meta/research/`) carries:

- `id` (REQUIRED), prefix `res.`.
- `nodes: [...]` (REQUIRED, non-empty).
- `date` (REQUIRED).
- `sources: [<source-id>...]` (the up-chain to sources). Research MUST cite at least one source UNLESS `method: primary`.
- `method`: `primary` or `secondary` (default `secondary`). Research does NOT use `informed_by`.

Sources (`meta/sources/`) carry:

- `id` (REQUIRED), prefix `src.`.
- `file` (REQUIRED): a URL for `external` sources, or a repo-relative path for `verified`/`unverified`.
- `verification` (REQUIRED): `verified`, `external`, or `unverified`.
- `type`, `date` (REQUIRED); `sha256` (REQUIRED when `verified`).

Sources have NO loaded `nodes` field. The Source schema does not model one and the loader does not read it, so a `nodes:` line on a source is inert and MUST NOT be relied upon. A source anchors TRANSITIVELY by being cited (from `research.sources` or `decision.informed_by`); an un-cited source surfaces as an advisory orphan, not by a missing anchor.

Reviews use a singular `node:` plus optional `related_change:` and carry no `id` and no plural `nodes`. Todos use a singular `node:` and carry no `id`. The 'every artefact has id + nodes' shape applies ONLY to decisions and research.

Chain roots are SOURCES and PRIMARY research (both have no upstream). The provenance spine is `source -> research -> decision`; the lateral decision graph (`supersedes`, `refines`, `related`) and the decision status lifecycle express obsolescence and refinement that the spine alone cannot. Supersession is modelled as a `supersedes` edge together with `status: superseded` on the target.

The decision-to-change terminal hop is OUT OF SCOPE for this section: change-directory files are governed by the change/delta convention, not the provenance link model.

Typed id prefixes (`dec.`/`res.`/`src.`) MUST match the artefact type, and ids MUST be globally unique across the decision, research, and source union (the prefix is the only cross-type disambiguator, since `informed_by` resolves against that union). Both are AUTHOR-SIDE POLICY: neither prefix-conformance nor cross-artefact id uniqueness is gated by the scanner today (see Deterministic enforcement).

### Flexibility

The layout permits exactly the variation the engine can honour, and no more.

- Decisions, research, and sources are FLAT. Topical grouping uses slug namespacing, never subfolders. A subfolder artefact loads ONLY via an explicit per-file blueprint pointer.
- Todos are NODE-partitioned (`meta/todos/<node>/`), a deliberate exception to artefact-type-first.
- Non-artefact material (docs/spec.md, external PDFs, competitor sites, internal design notes) MUST NOT be modelled as a typed artefact by inlining its content. It MAY enter provenance by being cited as a `source`. This is a permission, not a mandate, and authors SHOULD choose the verification mode honestly:
  - `external` for content the project does not hold the bytes of; `file` MUST be a URL.
  - `verified` for project-controlled local files whose bytes should be frozen; the scanner re-hashes the file on every scan, so any edit raises a sha256 mismatch until the hash is re-pinned. Use it only for genuinely immutable content.
  - `unverified` for local files not yet pinned; it surfaces a perpetual advisory until resolved.
  There is no friction-free tracked-local-file mode today (see open questions); until one exists, routine, frequently-edited internal docs SHOULD be cited sparingly to avoid a re-pin treadmill.
- A genuinely cross-cutting decision (a project-wide policy with no natural module) MUST still carry a non-empty `nodes:`; the orphaned hatch does not waive that. Such a decision SHOULD anchor to the system root node, or set `orphaned: true` with an `orphan_reason` when its nodes are intentionally unknown.
- Orphan research (research never cited by a decision) is LEGITIMATE per spec section 8.5 and MUST NOT be treated as an error.

### Deterministic enforcement

The convention distinguishes three tiers. Authors and reviewers MUST NOT assume policy-tier rules are caught by the scanner.

Mechanically ENFORCED today (Error; fails `cairn scan`, `cairn lint`, and the commit/push gate):

- `CAIRN_DECISION_MISSING_NODES` : a decision has an empty `nodes`.
- `CAIRN_DECISION_ORPHANED` : a decision's nodes are all unknown and it is not validly orphaned.
- `CAIRN_ARTEFACT_MISSING_NODES` : research has an empty `nodes`.
- `CAIRN_ARTEFACT_UNKNOWN_NODE` : a research node id is unknown (raised per unknown node).
- `CAIRN_RESEARCH_MISSING_SOURCES` : secondary research cites no sources.
- `CAIRN_SOURCE_EXTERNAL_URL` : an `external` source's `file` is not a URL.
- `CAIRN_SOURCE_SHA256_MISSING` / `CAIRN_SOURCE_SHA256_MISMATCH` / `CAIRN_SOURCE_READ_FAILED` : verified-source integrity.
- `CAIRN_ARTEFACT_MISSING_FIELD` : a required frontmatter field is absent.
- `CAIRN_REVIEW_UNKNOWN_NODE` : a review's node is unknown (only when a `reviews` pointer is wired).

Note the multi-node asymmetry: a research artefact errors on EACH unknown node, whereas a decision errors only when ALL its nodes are unknown. This divergence is documented, and one consistent semantics SHOULD be chosen (see open questions).

Mechanically DETECTED but ADVISORY (Warning or Info; recorded but does NOT fail the gate, because the gate counts only Error findings):

- `CAIRN_DECISION_UNKNOWN_PROVENANCE` : an `informed_by` id resolves to no research or source.
- `CAIRN_RESEARCH_UNKNOWN_SOURCE` : a `sources` id resolves to no source.
- `CAIRN_DECISION_REFERENCE_UNKNOWN` / `CAIRN_DECISION_SUPERSEDES_STATUS` : lateral decision-edge integrity.
- `CAIRN_SOURCE_ORPHAN` : a source is cited by nothing.
- `CAIRN_TODO_ORPHAN_NODE` : a todo references an unknown node.
- `CAIRN_SOURCE_UNVERIFIED` : an unverified source.

The provenance up-chain is therefore MECHANICALLY DETECTED, not mechanically enforced: a dangling `informed_by` is flagged but does not block. Authors MUST NOT treat a clean gate as proof the up-chain resolves.

NOT yet gated (author-side POLICY only; the scanner is silent):

- Global id uniqueness across the decision/research/source union. Two files sharing an id both load and the duplicate is collapsed silently.
- Typed-prefix conformance. An id is read verbatim; a mismatched or missing prefix is not detected.
- Unwired or mis-placed artefacts. A file in a pointer-directory subfolder is dropped with no finding.
- Artefact-id renames. `cairn rename` renames graph NODE ids only, and its frontmatter rewriter is substring-based, so it can corrupt `informed_by`/`sources` references where a node id is a substring of an artefact id. Artefact ids are immutable-by-policy until a safe cascade exists; renames are manual and unguarded.

Codification. This section is the normative home of the rules above; it DOCUMENTS the shipped checks rather than proposing them. The `agent_guide.md` delivered by `cairn init` SHOULD carry only a short orienting note pointing here, not a restatement of the layout. To make the policy-tier rules mechanical, the following NET-NEW gates are recommended (tracked as open questions): `CAIRN_ARTEFACT_DUPLICATE_ID` (across the dec/res/src union), an id-prefix-conformance check, and an 'unwired artefact under a meta/ pointer root' check. The phrase 'mechanically verifiable' MUST NOT be applied to any rule until its gate exists; until then describe it as detected (advisory) or as policy.
