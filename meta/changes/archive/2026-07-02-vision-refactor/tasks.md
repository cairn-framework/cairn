# Tasks: Vision refactor

## Phase 0 — Governance

- [x] Create change directory (`proposal.md`, `design.md`, `tasks.md`, `specs/`)
- [x] Create research artefact `res.vision-refactor-audit`
- [x] Create five decision artefacts
- [x] Amend `docs/spec.md` to v0.8
- [x] `cairn lint` clean (no Error findings; new artefacts load)

## Phase 1 — Symbol records

- [x] `src/reconcile/symbol.rs`: `SymbolRecord`, `SymbolKind`
- [x] Extend Rust/TypeScript/Python/Go reconcilers to build records
- [x] Thread records through `ReconcileReport`/`TargetReport`; bump cache version
- [x] Attach `NodeRecord.symbols` via `build_graph`
- [x] `cairn symbols <node>` query_api tool (CLI + MCP + docs)
- [x] Webui: symbols endpoint + `ModuleInspector` section
- [x] Tests: per-language symbol assertions; `symbols()` query test; fingerprint-hash stability

## Phase 2 — `map.json`

- [x] `src/scanner/snapshot.rs`: `MapSnapshot`
- [x] Write `map.json` in `scan()`
- [x] Commit this repo's own `map.json`
- [x] Tests: determinism (identical bytes on re-scan), schema_version

## Phase 3 — Structured contracts

- [x] `Contract.interface: Vec<String>` + frontmatter parsing
- [x] Promote `normalize_symbol` to `src/reconcile/symbol.rs`
- [x] `CAIRN_CONTRACT_INTERFACE_DRIFT` check
- [x] Register rule in `docs/registries/spec-rules.md`
- [x] Dogfood one contract's `interface:` block
- [x] Tests: parsing + drift-detection fixture

## Phase 4 — Generative direction

- [x] `cairn bundle <node>` query_api tool + handler
- [x] `cairn gap <node> --question` CLI-only command (7 surfaces)
- [x] `CAIRN_GAP_UNRESOLVED` lint rule
- [x] Tests: bundle fields, gap file creation + collision suffix, gap lint

## Phase 5 — `cairn frontier`

- [x] `query::frontier` (tiered ready/blocked over ghost nodes)
- [x] CLI/query_api/MCP + docs + snapshot updates (7 surfaces)
- [x] Write phase-5 tests

## Phase 6 — Workspace

- [x] `src/workspace/mod.rs` + `cairn.workspace` format
- [x] `cairn workspace <status|lint|frontier>` CLI family
- [x] Tests: two-member aggregate, missing-member error

## Phase 7 — Change-system trim

- [x] Delete `BeadsStateBackend` workflow methods + call sites
- [x] Reduce `src/state/mod.rs` to thin shell; delete `beads.rs`
- [x] Remove `state_backend` config key + docs mentions
- [x] Full `cargo test` green

## Phase 8 — Cleanup and landing

- [x] Update `AGENTS.md`, `README.md`, `docs/commands.md`
- [x] Regenerate root `map.json` fixture
- [x] File-size gate (zero new violations introduced this session; 4
  pre-existing violations unrelated to vision-refactor remain, confirmed
  via `git diff main`)
- [x] Final battery: `make check`, `dogfood.sh`, `cairn accept` (`gate_outcome: passed`)
- [x] `cairn archive vision-refactor`
- [ ] Land via feature branch + PR
