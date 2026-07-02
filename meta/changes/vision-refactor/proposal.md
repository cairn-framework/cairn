# Vision refactor: symbol graph, persistent map, generative direction, workspace

## Motivation

The maintainer's verdict: cairn was repeatedly made too small to live up to
its vision, and the shrinking (module-only graph data, throwaway
reconciliation output, check-only direction, prose-only contracts) is why
development keeps stalling. `res.vision-refactor-audit` grounds five concrete
gaps in the current code. This change brings cairn to the ratified vision:
**a fine-grained, persistent, traversable structural authority that external
orchestrators and agents walk deterministically, able to check code against
intent AND supply everything an agent needs to generate code from intent.**
`dec.no-orchestrator` holds throughout: cairn still never runs agents or picks
tasks; every new surface is a query or a format concern.

## Scope

Seven workstreams, ratified 2026-07-02:

1. **Symbol-level reality layer** (`dec.symbol-reality-layer`) — structured
   symbol records (name/kind/signature/file/line) survive extraction instead
   of being flattened into a hash input; queryable per node via
   `cairn symbols`.
2. **Persistent reconciliation layer** (`dec.persistent-map-snapshot`) — a
   committed, deterministic `map.json` measurement record at the project
   root, alongside the existing `map.md`.
3. **Structured contracts** — an `interface:` frontmatter block the
   reconciler verifies against extracted symbols
   (`CAIRN_CONTRACT_INTERFACE_DRIFT`).
4. **Generative direction** (`dec.generative-bundles-and-gaps`) —
   `cairn bundle <node>` emits a complete generation bundle; `cairn gap` logs
   decision-required underspecification instead of letting agents guess.
5. **Deterministic orchestrator surface** (`dec.frontier-query`) —
   `cairn frontier` answers "what is buildable now, in what order" over the
   graph, without running anything.
6. **Workspace model** (`dec.workspace-aggregation`) — `cairn.workspace`
   groups member projects; aggregate status/lint/frontier across them,
   enumeration only, no cross-project edges.
7. **Change-system trim to format-only** (`dec.change-format-only`) — remove
   workflow logic (beads epic/claim wiring) and the production-dead
   `StateBackend` machinery; the read-only backlog view is unaffected.

## Out of scope

- A versioned graph *store* (`dec.graph-root-fingerprint` ruling 1 stays
  rejected; `map.json` is a derived, rebuildable snapshot, not a store with
  its own write API).
- Cross-project blueprint edges in `cairn.workspace` (`dec.workspace-aggregation`).
- `NodeState::Planned` (tracked separately by
  `dec.native-task-state-and-agent-guidance` ruling 1; `cairn frontier` ships
  on `Ghost` only).
- Authoring `interface:` blocks on every leaf contract (one contract is
  dogfooded to prove the mechanism; the rest is follow-up work, filed as a
  bead).
- Reopening `dec.no-orchestrator`: every new query is read-only or a format
  concern; none schedules, claims, or runs anything.

## Acceptance criteria

See `specs/` for the per-pillar acceptance criteria (one `spec.md` per
workstream, criteria copied from this plan's verification steps). Final
battery: `make check`, `sh scripts/check-file-sizes.sh`,
`bash scripts/dogfood.sh` all green before archive.
