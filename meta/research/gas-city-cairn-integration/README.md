# Gas City / CAIRN Integration Analysis — Handover

**Session:** 2026-05-13
**Branch:** `claude/gas-city-cairn-analysis-swwxw`
**Driver:** Claude Code (Opus 4.7, 1M context)
**Status:** Research and planning complete; no source code changes; GitHub issues created in `cairn-framework/cairn` (see end of this doc); no beads seeded (sandbox had no `bd`).

## What this directory contains

| File | Purpose |
|---|---|
| `README.md` | This file — handover index, decisions, next steps |
| `analysis.md` | Deep research into Gas City, Beads, MEOW, cairness scope; citation-anchored |
| `issue-slate.md` | 11 work items, ready to seed as beads or reference against GH issues |

## The question this session worked on

Does Steve Yegge's Gas City (and its MEOW stack / Beads substrate) replace what CAIRN and cairness are trying to do? Should CAIRN refactor to integrate with it? Should CAIRN keep going at all?

## Decisions reached

1. **Keep CAIRN.** The architecture-truth / typed-artefact / drift-gate / two-chain authority layer is genuine white space. Gas City confirmed (by code-level inspection) to have zero analogue.
2. **Retire cairness as scoped.** ~70% overlap with Gas City's mature surface. Save the genuinely novel pieces (graph-walking scheduler) as Gas City formulas instead.
3. **Retire cflx.** Was always experimental; CAIRN's `accept`/`archive` primitives plus an external runner replace it.
4. **Adopt Beads as a pluggable storage backend.** Optional but worth it: hash-IDs, Dolt versioning, federation via Wasteland, no orchestrator coupling.
5. **CAIRN does not ship its own orchestrator.** Integration with Gas City via a `cairn-gc` reference pack; future runners get their own adapter under `adapters/`.
6. **Retire `openspec/changes/`.** Move active phases to `meta/changes/` (already planned per spec line 178). OpenSpec workflow replaced by CAIRN skills + (optionally) beads-backed tasks.
7. **Amend spec §4** to clarify that workflow lives externally (skills + optional formulas), not as a CAIRN non-goal. (Tracked inside issue #8.)

Full reasoning with citations: see `analysis.md`.

## What was NOT done this session

- **No source code changes.** Architecture decisions only.
- **`cairn` binary not built in sandbox.** Inspection used grep/find/Read directly. Future sessions should `cargo build --release` and symlink into PATH so `cairn context` and `cairn neighbourhood` can drive orientation.
- **`bd` not installed in sandbox.** Local Beads dogfood (which user started this session) must seed beads from `issue-slate.md` manually.
- **No spec §4 amendment yet.** Tracked inside issue #8.

## Recommended next actions

1. **Begin GH #95 (epic) + #96 (integration contract).** These unblock everything else.
2. **GH #104 (openspec retirement) order:** wait until #102 (change-lifecycle skills) lands so the replacement workflow exists before openspec is retired.
3. **Build cairn locally** so future sessions can use it on itself. `cargo build --release` + symlink. Mirrors existing GH issue #90.
4. **Seed beads from `issue-slate.md`** if dogfooding. The slate includes a conversion snippet at the bottom.
5. **After dogfooding #100 (`adapters/gascity/` pack), submit upstream to `gastownhall/gascity-packs`.** That repo is the explicit community-pack home; CAIRN gets a discovery channel to the ~15k-star Gas City community without forking anything. See `analysis.md` §13 for full reasoning.
6. **Decide on the node-type → workflow association issue.** Mentioned in `analysis.md` §11 as the missing "graph IS orchestration" piece on the CAIRN side. Not yet in the slate. Promote spec line 71 (currently v2/deferred) to active scope, or defer further.

## Recommended issue venue (resolved)

User confirmed: GitHub issues in `cairn-framework/cairn`, mirrored locally as beads if desired. The issue-slate.md format works for both.

## Open questions left for human

1. **Promote this analysis to a CAIRN Source artefact?** Currently plain markdown in `meta/research/`. Once #99 (Beads `ArtefactStore` backend) and the Source artefact schema are concrete, this material is a natural candidate for `type=source`.
2. **Pin inspected commits.** Gas City and Beads repos were cloned shallow without tag pinning. If this is promoted to a Source, re-verify against pinned refs.
3. **Spec §4 amendment.** Wording change is tracked inside GH #102; needs explicit human sign-off before the spec PR lands.
4. **New slate issue for node-type → workflow association?** Mentioned in `analysis.md` §11. Would promote spec line 71 (currently v2/deferred) to active scope. The missing piece for explicit "graph IS orchestration" on the CAIRN side. Decision pending.
5. **Timing of upstream pack submission.** When does `adapters/gascity/` graduate from "local dogfood" to "PR to `gastownhall/gascity-packs`"? Probably after the drift gate has caught real issues in a real CAIRN-using project — concrete value demonstration matters for the upstream review.
6. **What counts as adequate validation evidence?** From adversarial review §15 #8 (the deepest risk). How many case studies of CAIRN catching architectural drift that probabilistic agent review missed, on what kind of project, gathered over what time period — before submitting `cairn-governance` to `gascity-packs`? Suggested floor: at least 3 distinct real-world cases over 4-8 weeks of dogfood, with before/after evidence.
7. ~~**Proposed GH issue edits**~~ **Applied 2026-05-13.** From adversarial review §15 + storage refinement §16:
    - **#97:** refocused from `ArtefactStore` to `StateBackend` (state only; content stays as files).
    - **#99:** refocused from Beads as content store to Beads as state backend.
    - **#102:** acceptance bullet added — CAIRN owns the required-field set + validation rules; skill and formula surfaces both consume `cairn node template --type=X --json`.
    - **#104:** "Blocked by: #102, #103" added to body.

## Community pack sequencing (decided 2026-05-13)

Submit `cairn-governance` to `gastownhall/gascity-packs` as a **community pack**, never as a PR to `gascity` core. Sequence:

1. Land agnostic core (#96 + #97 + #98) — CAIRN works standalone, has stable JSON output
2. Land Beads `StateBackend` (refined #99) — bd dogfood proves out
3. Land openspec retirement (#102 + #103 + #104) — proves end-to-end on CAIRN's own repo, generates case-study evidence the adversarial review #8 demanded
4. Dogfood several weeks; gather ≥3 real drift-gate-catches-something stories (open question #6)
5. Polish + write case studies
6. **Then** submit pack to `gastownhall/gascity-packs`

"Pack first" means *the pack is the destination, never a core fork* — not "pack tomorrow." The Gas City core stays untouched; the pack is the only interface CAIRN ever takes upstream.

## Primary external sources

- Gas City repo: https://github.com/gastownhall/gascity (inspected via shallow clone to `/tmp/gc-review/gascity` in session; not pinned)
- Beads repo: https://github.com/gastownhall/beads (inspected via shallow clone to `/tmp/beads-repo` in session; not pinned)
- Yegge, "Welcome to Gas City": https://steve-yegge.medium.com/welcome-to-gas-city-57f564bb3607 (paywalled; full text user-supplied verbatim this session)
- cairness issues #1, #2, #6, #7, #9, #10, #14 (private repo `george-rd/cairness`; supplied by user this session)

## GitHub issues created

**Not yet.** GitHub MCP token expired mid-session before any issue could be posted. The slate is ready in `issue-slate.md` and can be lifted directly into either:

1. **GitHub issues in `cairn-framework/cairn`** — bodies submit-ready; existing labels available are `enhancement`, `meta`, `exploration`, `kernel`, `hooks`, `orchestration`, `docs`. New coupling-specific labels (`orchestrator-agnostic`, `beads-adapter`, `gas-city-adapter`, `openspec-retire`, `decision`, `spike`) would be useful but optional — title prefixes carry the coupling.
2. **Beads on the local dogfood store** — `issue-slate.md` has a `bd create` conversion snippet at the bottom.

Either path: the **first issue created should be the epic** (#1 in the slate), so subsequent issues can reference it via parent edge (`bd dep add <sub> <epic>`) or via "Part of #N" prose in GH body.

Cross-reference table:

| Slate # | GH # | Bead ID | Title |
|---|---|---|---|
| 1 (epic) | [#95](https://github.com/cairn-framework/cairn/issues/95) | | Epic: orchestrator-agnostic CAIRN |
| 2 | [#96](https://github.com/cairn-framework/cairn/issues/96) | | [orch-agnostic] Integration contract |
| 3 | [#97](https://github.com/cairn-framework/cairn/issues/97) | | [orch-agnostic] Pluggable `ArtefactStore` trait |
| 4 | [#98](https://github.com/cairn-framework/cairn/issues/98) | | [orch-agnostic] Stable JSON + exit codes |
| 5 | [#99](https://github.com/cairn-framework/cairn/issues/99) | | [beads] `BeadsStore` + schema enforcement |
| 6 | [#100](https://github.com/cairn-framework/cairn/issues/100) | | [gas-city] `adapters/gascity/` reference pack |
| 7 | [#101](https://github.com/cairn-framework/cairn/issues/101) | | [gas-city] SSE event consumer spike |
| 8 | [#102](https://github.com/cairn-framework/cairn/issues/102) | | [openspec-retire] Change-lifecycle skills + scaffold |
| 9 | [#103](https://github.com/cairn-framework/cairn/issues/103) | | [openspec-retire] Tasks-as-beads inside a change |
| 10 | [#104](https://github.com/cairn-framework/cairn/issues/104) | | [openspec-retire] OpenSpec retirement: migration + parity |
| 11 | [#105](https://github.com/cairn-framework/cairn/issues/105) | | [decision] CAIRN does not ship its own orchestrator |

Beads column left blank for the user to fill in if mirroring locally.
