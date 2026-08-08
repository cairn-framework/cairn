---
node: cairn.brownfield
date: 2026-08-08
reviewer: anthropic/claude-fable-5/contestedness-alternatives
review_type: agent_cross_model
subject_hash: sha256:41872c8908bfeeb9e1dc94cfa964d3d36b3f980691f30beba54e7b09bd4cec5d
lens_prompt_hash: sha256:1ceb131f531393b6d998c5641ce6741774cce8f6b0305d4fd2876f4db4179003
---

# Receipt review: brownfield extraction mechanism (alternatives lens)

Receipt-grade review of `dec.brownfield-extraction-mechanism` under
`docs/agent/lenses/contestedness-alternatives.md`, run clause-by-clause with
read-only repository access.

## Claims verified

Read-only verification against `/Users/george/repos/cairn-ov-spine` at commit `90c57c1`. Subject bytes were compared against `meta/decisions/brownfield-extraction-mechanism.md` in full (379 lines) and match.

### Frontmatter

| Claim | Status | Evidence |
|---|---|---|
| `nodes: [cairn.brownfield, cairn.kernel.cli]` are real and are the right two | VERIFIED | `cairn.blueprint:84-86` — `Module CLI ... id "cairn.kernel.cli"` declares `path "./src/cli"` **and** `path "./tools/agent-pack"`; `cairn.blueprint:119-120` — `Module Brownfield ... id "cairn.brownfield"` declares `path "./src/brownfield"`. Together they own every affects path that any node owns, including the binding-registry path `tools/agent-pack/content/`. Dropping `cairn.kernel.cli` would leave the binding surface bound to no listed node. |
| All 19 `affects:` paths exist | VERIFIED | Existence check over the full list: 19/19 present. Consistent with the driver ruling that the two future reference files stay off the list because `compute_subject_hash` errors on missing paths. |
| Derived artefacts are correctly omitted from `affects:` | VERIFIED | `map.json:1945` and `harness/fixtures/api/graph` both name the existing cairn-dev references and will change, but `scripts/merge-map-json.sh:2-3` declares `map.json` "a derived, rebuildable" snapshot under `dec.persistent-map-snapshot`, and no decision in `meta/decisions/` lists `map.json` or `harness/fixtures/` in `affects:`. Precedent supports the omission. |
| Artefact passes the gates | VERIFIED | `./target/debug/cairn lint` → exit 0, zero Error/Warn lines; `cairn scan` → exit 0. No finding names `brownfield-extraction`. |

### Clause 1 — deterministic Cairn surface

| Claim | Status | Evidence |
|---|---|---|
| Existing no-subcommand path reads scanner findings, not `discovery.rs` | VERIFIED | `src/cli/commands/onboard.rs:28-37` — `scanner::load_project` → `brownfield::onboard::analyze(&result.graph.findings)`; `src/brownfield/onboard.rs:9` `ORPHAN_CODE = "CAIRN_RECONCILE_ORPHANED_FILE"`, `:63 pub fn analyze`. Both renderers (`render_json`, `render_human`) already exist. |
| Discovery is bounded, path-id-derived, import-edge-only | VERIFIED | `src/brownfield/discovery.rs:1-7` module doc ("Edges are derived only from imports observed ... an edge discovery cannot observe in the code is not proposed"), `:20 SOURCE_EXTS`, `:23 MIN_FILES = 3`, `:26 MAX_DEPTH = 4`, `:16 sanitised_path_derived_id`, `:44 pub evidence: Vec<String>`. |
| `eligible_owners` / `most_specific_owner` are private; `is_component_prefix` is reusable | VERIFIED | `src/reconcile/generic.rs:382` and `:405` are bare `fn` (private); `src/map/paths.rs:15` is `pub(crate) fn is_component_prefix`. Reimplementation plus a parity test is the only route that does not widen a private reconciler API. |
| The most-specific-prefix rule is described accurately | VERIFIED | `generic.rs:392-398` — leaf (`children.is_empty()`) **or** `owns_files`, paths normalized via `map::paths::trim_dot`; `:387` `sort_by_key(Reverse(path.len()))` = most-specific first; `:405-411` first `is_component_prefix` match wins. The decision's prose matches line-for-line. |
| Onboard currently synthesises a stub blueprint when the file is absent | VERIFIED | `src/cli/commands/onboard.rs:13-26` — writes `System Stub "onboard stub" id "stub"` into a temp dir when `parsed.file` does not exist. |
| `run_onboard_command` silently ignores `command_args` | VERIFIED | `src/cli/commands/onboard.rs` never references `command_args` at all; `src/cli/mod.rs:730` pushes every unmatched token into it. `cairn onboard nonsense` therefore prints the orphan report today. |
| Exit code 2 + literal `usage: ` is the repo convention | VERIFIED | `src/cli/mod.rs:288`, `:526`, `:560`, `:599`, `:611`; `commands/gap.rs:19,31`; `commands/archive.rs:23`; `commands/workspace.rs:19,25`. |
| The usage copy value must stay unprefixed for the help renderer | VERIFIED | `src/cli/help/mod.rs:386-389` emits `help.usage-label` then the raw value; a prefixed value would render "Usage: usage: cairn onboard …". The decision's guard is correct and necessary. |
| `help.commands.onboard.usage` exists | VERIFIED | `docs/design-system/copy.toml:840-841` — `[help.commands.onboard] usage = "cairn onboard [options]"`. |
| `help.commands.onboard.args` exists | CONTRADICTED (non-load-bearing) | No `args` key under `[help.commands.onboard]` (copy.toml:840-842). `args` is optional — `src/cli/help/mod.rs:400-403` falls back to the key name and suppresses the section. See Finding 1. |
| No `cairn brownfield` noun exists | VERIFIED | `src/cli/mod.rs` dispatch matches only `refine` (:382) and `onboard` (:402); registry entry at `:1130`. `docs/commands.md:145-146` lists `cairn refine` and `cairn onboard`. |
| `docs/integration-contract.md:88` is the Brownfield onboarding row | VERIFIED | Line 88 is exactly `| \`onboard\` | \`cairn_onboard\` | Suggest blueprint entries for orphaned files |`. |
| `tests/kernel.rs` carries onboard behaviour coverage | VERIFIED | `tests/kernel.rs:1554` `test_onboard_groups_orphans_and_classifies`, exercising both `onboard` (:1579) and `--json onboard` (:1602). |

### Clause 2 — cairn-dev authoring reference

| Claim | Status | Evidence |
|---|---|---|
| `BASE_ASSETS` uses `include_str!` per reference; `all_assets` rewrites the adapter root | VERIFIED | `src/cli/commands/pack_assets.rs:61-118` (`BASE_ASSETS`, `include_str!` per row), `:158-170` (`replacen(CLAUDE_ROOT, pack_root, 1)`), `:20 OMP_ROOT = ".omp/"`. |
| `LOOP_ASSETS` is opt-in and reserved for loop mode plus its closure | VERIFIED | `pack_assets.rs:120-153` — doc comment states the router reads absent `loop-mode.md` as "loop mode is unavailable in this repository", so default installation "would make that signal a lie". The BASE-not-LOOP placement is forced by that invariant. |
| Manifest needs canonical + Claude + OMP rows | VERIFIED | `tools/agent-pack/manifest.toml:60-62` (canonical `source`), `:177-179` (`.claude` destination), `:309-311` (`.omp` destination) for the existing `command-reference`. Three rows per reference is the established shape. |
| Both router files exist and the proposed route row matches the table shape | VERIFIED | `tools/agent-pack/content/skills/cairn-dev/SKILL.md:52-61` is a two-column `| task | \`references/x.md\` |` table. |
| Canonical and `.claude` copies are byte-identical today | VERIFIED | `diff` of both `SKILL.md` and both `references/command-reference.md` → identical. |
| `.gitattributes` marker is invalidated | VERIFIED | `.gitattributes` carries one `linguist-generated=true` row per `.claude` mirror file (22 rows); a new mirror needs one. |
| `determinism_drift_tests.rs` arrays are size-pinned | VERIFIED | `:128 const EXPECTED_CANONICAL: [(&str, &str, &str); 21]`, `:228 const EXPECTED_CLAUDE: [(&str, &str, &str); 21]`. |
| `router_route_tests.rs` enforces route reachability | VERIFIED | `:78 every_router_route_resolves_to_a_shipped_reference`, `:100 every_shipped_reference_is_reachable_from_the_router`. Both directions break on an unlisted reference. |

### Clause 3 — artefact writer

| Claim | Status | Evidence |
|---|---|---|
| `run_decision_new` validates the slug and combines two helpers | VERIFIED | `src/cli/commands/decision.rs:26-56` — `is_kebab_slug` guard (:32), legacy-duplicate guard (:37-46), `decision_stub` (:47), `write_new_artefact` (:48). |
| `decision_stub` owns typed frontmatter, `status: proposed`, date, sections | VERIFIED | `decision.rs:119-144` — `id:`, `nodes:`, `status: proposed` (:131), `date:` (:132), `informed_by:` (:133-138), `## Context` / `## Decision` sections. |
| `decision_stub` emits no `ratification` field | VERIFIED | No `ratification` write anywhere in `decision.rs:119-144`. |
| `write_new_artefact` is kind-agnostic | VERIFIED | `decision.rs:61-81` — refuses existing target (:71-73), `create_dir_all` (:74), `fs::write` (:77); messages are injected by the caller, no frontmatter knowledge. |
| Registry defaults absent `ratification` to `binding` at `kinds.rs:138-140` | VERIFIED | Exactly those lines: `let ratification = parsed.values.get("ratification").cloned().map_or(Some(RatificationTier::Binding), |value| parse_ratification_tier(...))`. Citation is precise. |
| `--node` / `--informed-by` are the real flags | VERIFIED | `decision.rs:15-16` `flag_values(&parsed.command_args, "--node")` / `"--informed-by"`. |
| `gap` writes `gap: true` and lints `CAIRN_GAP_UNRESOLVED` | VERIFIED | `src/cli/commands/gap.rs:108` template contains `gap: true`; `src/artefacts/registry/validate/mod.rs:207-215` emits `CAIRN_GAP_UNRESOLVED` for still-proposed gap decisions. |

### Clause 4 — tier, and the governing decisions

| Claim | Status | Evidence |
|---|---|---|
| `docs/registries/binding-surface.md:7` is the `tools/agent-pack/content/` row | VERIFIED | Line 7 is exactly `- tools/agent-pack/content/`. |
| Local tier requires an `affects:` list wholly outside the allowlist | VERIFIED | `meta/decisions/decision-ratification-tiers.md:32`; `:44-45` names `tools/agent-pack/content/` as inside the allowlist. This ruling's `affects:` is not wholly outside, so binding is forced, not chosen. |
| Convergent binding rulings are panel-acceptable; contested clauses need a recorded debate | VERIFIED | `meta/decisions/reviewer-panel-ratification.md:61-71` (§2) and `:75-82` (§3, "the For/Against/Verdict debate for every contested clause"). Status `accepted`, ratification `binding` (:6-7). |
| The consolidation rule is quoted accurately and is accepted | VERIFIED | `meta/decisions/cli-agent-workflow-consolidation.md:74-75` reads verbatim: "Future pack promotions are judged on marginal lift over the current pack and / merge non-overlapping value into the owning skill before adding a new skill." `status: accepted` at :9. |
| All six `related:` decisions are accepted | VERIFIED | `build-and-extension:9`, `cli-agent-workflow-consolidation:9`, `decision-ratification-tiers:7`, `reviewer-panel-ratification:6`, `agent-pack-packaging:5`, `pack-adapter-roots:5` — all `status: accepted`. |
| `init --from-code` writes `meta/changes/brownfield-init` | VERIFIED | `src/cli/mod.rs:210`, `:1592`, `:1656` (`cairn change apply brownfield-init`), `:79 BROWNFIELD_APPLIED_MARKER`. |
| Cairn runs no first-party inference | VERIFIED | `src/summariser/backend/mod.rs:13` "Disabled by default per phase-8 spec"; `:113` default backend "always refuses"; `src/summariser/config.rs:5` "summariser defaults to disabled mode"; only `SummariserMode::LocalCommand` (`config.rs:103-108`) delegates out. |

## Alternatives analysis (this lens)

The standalone-skill fork is adjudicated and out of scope. I steel-manned the remaining candidates on the same evidence.

**Strongest competitor: a dedicated top-level noun** (`cairn extract-decisions`, or reviving a `cairn brownfield extract`). Its best case is real and is not the one the decision answers in `## Rejected alternatives`: `onboard` today is documented as "Suggest blueprint entries for orphaned files" (`docs/commands.md:146`) and deliberately works **without** a blueprint — `onboard.rs:13-26` synthesises a stub precisely so a pre-blueprint repo gets a report. Clause 1 then requires the `decisions` branch to *fail* when no blueprint exists (subject lines 115-119). That is two contradictory preconditions under one noun, and a cohesion argument a competent maintainer could raise.

It still loses clearly, on three grounds I could verify:
1. The contradiction is explicit and loud, not silent — a `usage:`-prefixed exit-2 error, matching the convention at `mod.rs:288`/`gap.rs:19`. Sub-verbs with stricter preconditions than their parent already exist here (`change accept` vs `change list`, `mod.rs:511-613`).
2. Ownership is genuine, not nominal: `onboard` already owns brownfield read-only reporting with *both* renderers (`onboard.rs:31-37`) and is already the node-suggestion command. Decision evidence bound to node ids is the same job.
3. Cost asymmetry runs the wrong way for the competitor. The subcommand is purely additive and retractable. A new top-level noun immediately incurs registry, help, copy, `docs/commands.md`, and `docs/integration-contract.md` rows — and `tests/command_reference_consistency.rs:141-165` mechanically enforces both doc surfaces for **every** registered command. The reversible choice is also the cheaper one.

**Precondition fork (fail vs keep the stub).** Steel-man: the flow already has an unbound-evidence path, so a stub blueprint would just make everything unbound, which is arguably still useful pre-blueprint. It fails: with a stub, "unbound" collapses two distinct states (no declared owner vs no graph at all), and the flow's output is `cairn decision new --node <id>`, which a zero-binding run cannot feed. Relaxing a precondition later is backward compatible; tightening one is not. The strict choice is also the reversible one.

**Helper reuse fork.** Making `eligible_owners`/`most_specific_owner` `pub(crate)` instead of reimplementing with a parity test is imaginable, but it widens a private reconciler API for a read-only consumer. Taste, no material consequence — the parity test pins the behaviour either way.

**Tier, nodes list, and writer** are all mechanically determined by artefacts I read (`binding-surface.md:7`, `cairn.blueprint:84-120`, `decision.rs:26-56`), not by discretion. No live alternative remains.

## Findings

1. **Minor, wording** — subject line 188 lists `help.commands.onboard.args` among values that clause 1 "invalidates", but that key does not exist: `docs/design-system/copy.toml:840-841` has only `usage` under `[help.commands.onboard]`. The instruction is still correct and satisfiable (`args` is optional — `src/cli/help/mod.rs:400-403` suppresses the section when unauthored, and `[help.commands.pending]`/`[help.commands.remediate]` show the optional-positional pattern to copy). It must be **added**, not amended. Non-load-bearing; no re-issue warranted.
2. **Minor, implementation trap for `todo.brownfield-extraction-flow`** — `src/cli/mod.rs:712-731` funnels *unrecognised flags as well as* positionals into `command_args`, so a naive `command_args.get(1) != Some("decisions")` check would reject `cairn onboard --typo` with a subcommand usage error. The decision says "positional", which is the right specification; the implementer must filter `--`-prefixed tokens. Recorded so the flow unit does not rediscover it.
3. **Minor, pre-existing repo state** — `docs/integration-contract.md:88` advertises an MCP tool `cairn_onboard`, but that identifier appears nowhere outside that doc line (repo-wide grep). Not introduced by this ruling and not this ruling's to fix, but the implementer updating that row should not assume an MCP surface exists to extend in parallel.
4. **Observation, no action** — `cairn lint` already reports `CAIRN_DECISION_ACCUMULATION Node \`cairn.kernel.cli\` carries 12 accepted decisions (threshold 10)` (Info). Acceptance makes it 13. The node addition is nonetheless required by ownership (`cairn.blueprint:84-86`), so the finding is a symptom of a correct binding, not a reason to drop the node.

No defects. Nothing in the subject makes a false load-bearing claim, and no cited line number is wrong — including the three precise citations (`kinds.rs:138-140`, `binding-surface.md:7`, `integration-contract.md:88`), all of which resolve exactly.

## What survived

Every line-number citation, the private-helper and prefix-rule mechanics, the stub-blueprint and `command_args`-ignoring defects the ruling promises to fix, the full pack-surface invalidation list (manifest triple-row, `.gitattributes`, two size-pinned 21-element arrays, both router reachability tests), the writer-reuse argument, the tier derivation, the quoted consolidation rule, and the panel-path claim. Both gates are green (`cairn lint` exit 0 with zero Error/Warn; `cairn scan` exit 0). The two deltas since the last round — the restored `cairn.kernel.cli` node and the restored For/Against/Verdict block with the adjudication line — are exactly what the panel and batch contract demanded, and both check out: the node is required by `tools/agent-pack` ownership, and the debate block satisfies `reviewer-panel-ratification.md:79`.

Verdict: convergent — no live alternative survives steel-manning (the top-level-noun competitor is the strongest and loses on ownership plus an asymmetric, harder-to-retract cost), and no defects were found.
