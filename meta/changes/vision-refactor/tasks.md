# Tasks: Vision refactor

## Phase 0 — Governance

- [x] Create change directory (`proposal.md`, `design.md`, `tasks.md`, `specs/`)
- [x] Create research artefact `res.vision-refactor-audit`
- [x] Create five decision artefacts
- [x] Amend `docs/spec.md` to v0.8
- [ ] `cairn lint` clean (no Error findings; new artefacts load)

## Phase 1 — Symbol records

- [ ] `src/reconcile/symbol.rs`: `SymbolRecord`, `SymbolKind`
- [ ] Extend Rust/TypeScript/Python/Go reconcilers to build records
- [ ] Thread records through `ReconcileReport`/`TargetReport`; bump cache version
- [ ] Attach `NodeRecord.symbols` via `build_graph`
- [ ] `cairn symbols <node>` query_api tool (CLI + MCP + docs)
- [ ] Webui: symbols endpoint + `ModuleInspector` section
- [ ] Tests: per-language symbol assertions; `symbols()` query test; fingerprint-hash stability

## Phase 2 — `map.json`

- [ ] `src/scanner/snapshot.rs`: `MapSnapshot`
- [ ] Write `map.json` in `scan()`
- [ ] Commit this repo's own `map.json`
- [ ] Tests: determinism (identical bytes on re-scan), schema_version

## Phase 3 — Structured contracts

- [ ] `Contract.interface: Vec<String>` + frontmatter parsing
- [ ] Promote `normalize_symbol` to `src/reconcile/symbol.rs`
- [ ] `CAIRN_CONTRACT_INTERFACE_DRIFT` check
- [ ] Register rule in `docs/registries/spec-rules.md`
- [ ] Dogfood one contract's `interface:` block
- [ ] Tests: parsing + drift-detection fixture

## Phase 4 — Generative direction

- [ ] `cairn bundle <node>` query_api tool + handler
- [ ] `cairn gap <node> --question` CLI-only command (7 surfaces)
- [ ] `CAIRN_GAP_UNRESOLVED` lint rule
- [ ] Tests: bundle fields, gap file creation + collision suffix, gap lint

## Phase 5 — `cairn frontier`

- [ ] `frontier()` query + tool registration
- [ ] CLI rendering + docs + MCP + wire-snapshot updates
- [ ] Tests: ready/blocked/all-synced cases

## Phase 6 — Workspace

- [ ] `src/workspace/mod.rs` + `cairn.workspace` format
- [ ] `cairn workspace <status|lint|frontier>` CLI family
- [ ] Tests: two-member aggregate, missing-member error

## Phase 7 — Change-system trim

- [ ] Delete `BeadsStateBackend` workflow methods + call sites
- [ ] Reduce `src/state/mod.rs` to thin shell; delete `beads.rs`
- [ ] Remove `state_backend` config key + docs mentions
- [ ] Full `cargo test` green

## Phase 8 — Cleanup and landing

- [ ] Update `AGENTS.md`, `README.md`, `docs/commands.md`
- [ ] Regenerate root `map.json` fixture
- [ ] File-size gate
- [ ] Final battery: `make check`, `dogfood.sh`, `check-file-sizes.sh`
- [ ] `cairn archive vision-refactor`
- [ ] Land via feature branch + PR
