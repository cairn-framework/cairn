---
node: cairn.brownfield
date: 2026-08-08
reviewer: anthropic/claude-fable-5/contestedness-reversibility
review_type: agent_cross_model
subject_hash: sha256:41872c8908bfeeb9e1dc94cfa964d3d36b3f980691f30beba54e7b09bd4cec5d
lens_prompt_hash: sha256:45136bbc19a4732ebacc4bd194791674e1266a4ae11c8fd51bfcfae9c7c4d698
---

# Receipt review: brownfield extraction mechanism (reversibility lens)

Receipt-grade review of `dec.brownfield-extraction-mechanism` under
`docs/agent/lenses/contestedness-reversibility.md`, run clause-by-clause with
read-only repository access.

Lens: **reversibility and blast radius**. Verification against `/Users/george/repos/cairn-ov-spine` at commit `90c57c1`. Subject bytes confirmed byte-identical to `meta/decisions/brownfield-extraction-mechanism.md` (379 lines, exact prefix comparison). Per round rules I did not re-litigate the driver adjudications (the `affects` exclusion of the two not-yet-existing reference files, or the reference-hosting verdict under `dec.cli-agent-workflow-consolidation:74-75`).

**Read-only disclosure.** Early in this review I invoked `./target/debug/cairn scan`, before being reminded that the scanner writes interface state and map outputs (`src/scanner/mod.rs:556-584`) and therefore falls outside the read-only constraint. I do **not** cite its output as evidence anywhere below. I confirmed the run left no footprint: `git status --porcelain` is empty, and `find . -maxdepth 2 -newermt '-40 minutes' -type f` (excluding `target/` and `.git/`) returns nothing, including under the ignored `.cairn/` directory. No cleanup was required. Every claim below rests on file reads and greps only.

## Claims verified

### Context clauses

| Claim | Status | Evidence |
| --- | --- | --- |
| `cairn onboard` loads via scanner and passes findings to `brownfield::onboard::analyze`, which groups `CAIRN_RECONCILE_ORPHANED_FILE` | VERIFIED | `src/cli/commands/onboard.rs:28-30`; `src/brownfield/onboard.rs:9` (`ORPHAN_CODE`), `:1-2` module doc |
| The current command synthesises a temporary stub blueprint when the requested file is absent | VERIFIED | `src/cli/commands/onboard.rs:13-26` (temp dir + `System Stub` write) |
| `run_onboard_command` silently ignores `command_args` | VERIFIED | `src/cli/commands/onboard.rs:6-59` contains no reference to `command_args`; contrast `src/cli/commands/decision.rs:10-20`, which dispatches on `command_args.get(1)` and errors on an unknown subcommand |
| Brownfield CLI surfaces are `onboard`, `refine`, `init --from-code`; no `cairn brownfield` noun | VERIFIED | `src/cli/mod.rs:402` (onboard), `:382` (refine), `:288`/`:210` (`--from-code`, `meta/changes/brownfield-init`); no `"brownfield"` command string exists in `src/cli/`; no `interview` dispatch despite `src/brownfield/interview.rs` |
| `init --from-code` writes `meta/changes/brownfield-init` with proposal/delta/contract stubs, `--apply` archives | VERIFIED | `src/cli/mod.rs:210,288,1592,1656` (`cairn change apply brownfield-init`) |
| Discovery walks bounded dirs, derives path-based candidate ids, records evidence paths, derives only proven import edges | VERIFIED | `src/brownfield/discovery.rs:1-6,19-25,43-52` |
| Summariser delegates to an external local command, disabled by default | VERIFIED | `src/summariser/config.rs:5` ("summariser defaults to disabled mode"), `:28` (`local_command: Option<LocalCommandConfig>`) |

### Clause 1 — deterministic Cairn surface

| Claim | Status | Evidence |
| --- | --- | --- |
| `eligible_owners` / `most_specific_owner` are private in `src/reconcile/generic.rs` | VERIFIED (but see Finding 1) | `src/reconcile/generic.rs:382,405` — bare `fn`, inside `pub mod generic` (`src/reconcile/mod.rs:5`) |
| Eligible leaf or `owns-files` nodes contribute normalized declared paths, most-specific first | VERIFIED | `src/reconcile/generic.rs:391-401` (`!is_internal \|\| node.owns_files`, `trim_dot`), `:387` (`sort_by_key(Reverse(path.len()))`) |
| `map::paths::is_component_prefix` selects the first matching owner | VERIFIED | `src/map/paths.rs:15-20`; called at `src/reconcile/generic.rs:407` |
| Unknown-positional error follows the established `err(2, "usage: …")` convention | VERIFIED | `src/cli/mod.rs:288,526,599,611`; `src/cli/commands/mod.rs:64` |
| Usage copy stays unprefixed for the help renderer | VERIFIED | `src/cli/help/mod.rs:382-387` prepends `help.usage-label` itself; `docs/design-system/copy.toml:841` is unprefixed |
| `docs/integration-contract.md:88` is the Brownfield onboarding row | VERIFIED | line 88 is exactly `\| \`onboard\` \| \`cairn_onboard\` \| Suggest blueprint entries for orphaned files \|` |
| `docs/commands.md` carries an onboard row | VERIFIED | `docs/commands.md:146` |
| `tests/kernel.rs` carries onboard behaviour coverage | VERIFIED | `tests/kernel.rs:1554-1608` |
| `help.commands.onboard.args` value exists today | CONTRADICTED (minor, see Finding 2) | `docs/design-system/copy.toml:840-841` has only `usage`; 58 other commands author `args` (e.g. `:722-724`) |

### Clause 2 — cairn-dev reference hosting

| Claim | Status | Evidence |
| --- | --- | --- |
| `BASE_ASSETS` uses `include_str!` per cairn-dev reference; `all_assets` rewrites the adapter root for `.omp` | VERIFIED | `src/cli/commands/pack_assets.rs:61-100`, `:158-171` (`replacen(CLAUDE_ROOT, pack_root, 1)`), `:20` (`OMP_ROOT`) |
| `LOOP_ASSETS` is opt-in and reserved for loop mode + closure | VERIFIED | `src/cli/commands/pack_assets.rs:120-151`, `:161` (`if with_loop`) |
| Manifest needs a canonical entry plus Claude and OMP adapter rows | VERIFIED | `tools/agent-pack/manifest.toml`: 21 `harness = "claude"` and 21 `harness = "omp"` rows (e.g. `:242-245`) |
| `EXPECTED_CANONICAL` / `EXPECTED_CLAUDE` are size-pinned | VERIFIED | `tools/agent-pack/tests/determinism_drift_tests.rs:128,228` — `[(&str, &str, &str); 21]` |
| `.gitattributes` generated marker is gated | VERIFIED | `determinism_drift_tests.rs:369-376` asserts the `.claude/` `linguist-generated=true` set equals the expected destination set |
| Router route reachability is gated both ways | VERIFIED | `tools/agent-pack/tests/router_route_tests.rs:78,100` |
| Canonical and `.claude` router files are byte-identical; the proposed row matches the existing table shape | VERIFIED | `diff` of the two `SKILL.md` files reports no difference; existing rows at `SKILL.md:52-55` use the identical `\| phrase \| \`references/x.md\` \|` form |
| Both `command-reference.md` copies describe `cairn onboard` | VERIFIED | canonical and `.claude` copies, line 90 (`\| Triage orphans \| \`cairn onboard\` \|`) |

### Clause 3 — existing artefact writer

| Claim | Status | Evidence |
| --- | --- | --- |
| `run_decision_new` validates the slug and combines `decision_stub` + `write_new_artefact` | VERIFIED | `src/cli/commands/decision.rs:26-57` |
| `--node` / `--informed-by` are the real flags | VERIFIED | `src/cli/commands/decision.rs:14-15` (`flag_values`) |
| `write_new_artefact` is kind-agnostic: creates dir, refuses existing, writes bytes | VERIFIED | `src/cli/commands/decision.rs:61-81` |
| `decision_stub` owns typed frontmatter, `status: proposed`, date, standard sections, and emits no `ratification` | VERIFIED | `src/cli/commands/decision.rs:119-150` |
| Registry defaults absent `ratification` to `binding` at `kinds.rs:138-140` | VERIFIED | exact lines: `parsed.values.get("ratification")…map_or(Some(RatificationTier::Binding), …)` |
| `cairn gap` marks `gap: true` and leaves `CAIRN_GAP_UNRESOLVED` until resolved | VERIFIED | `src/artefacts/registry/validate/mod.rs:207-215` |

### Clause 4 — ratification tier

| Claim | Status | Evidence |
| --- | --- | --- |
| `tools/agent-pack/content/` is the exact directory row at `docs/registries/binding-surface.md:7` | VERIFIED | line 7 is `- tools/agent-pack/content/` |
| A `local` tier with these affects would be a violation | VERIFIED | `src/hooks/ratification.rs:340-368` emits `CAIRN_DECISION_TIER_BINDING_PATH` ("local decision governs binding-surface path") |
| Manifest / adapter-root / compiled-asset / mirror files are not themselves registry paths | VERIFIED | full allowlist is `docs/spec.md`, `docs/registries/`, `tools/agent-pack/content/`, `src/artefacts/registry/`, `cairn.blueprint`; `manifest.toml`, `pack_assets.rs`, `.claude/`, `copy.toml` are all outside it |
| `dec.cli-agent-workflow-consolidation:74-75` states the marginal-lift / merge-before-new-skill rule | VERIFIED | exact lines quoted |
| `dec.reviewer-panel-ratification` permits acceptance of an all-convergent binding ruling on receipts | VERIFIED | `meta/decisions/reviewer-panel-ratification.md:54-66` |

### Artefact integrity (read-only checks)

| Claim | Status | Evidence |
| --- | --- | --- |
| Both `nodes:` exist and own the ruling's primary surfaces | VERIFIED | `cairn.blueprint:84-87` (`cairn.kernel.cli` owns `./src/cli` **and** `./tools/agent-pack`), `:119-121` (`cairn.brownfield` owns `./src/brownfield`) |
| Every `affects:` path exists on disk | VERIFIED | all 18 entries resolve (existence test per entry) |
| All six `related:` decisions and the `informed_by` research artefact exist | VERIFIED | files present under `meta/decisions/` and `meta/research/` |
| Implementation unit `todo.brownfield-extraction-flow` exists | VERIFIED | `meta/todos/todo.brownfield-extraction-flow.md` |
| No acceptance, receipt, or machine-ratification marker | VERIFIED | frontmatter lines 1-40: `status: proposed`, no `ratified_by`, `receipts`, `supersedes`, or `gap` keys |

## Findings

**1 (CONCERN, reversibility-cheap). The duplication justification in lines 103-107 states a choice as a constraint.** The clause reads "`eligible_owners` and `most_specific_owner` … are private, so the flow unit will reimplement the same most-specific-prefix rule". Both are indeed private (`src/reconcile/generic.rs:382,405`), but they live in `pub mod generic` (`src/reconcile/mod.rs:5`) in the *same crate* as the onboard resolver, and the underlying primitive `is_component_prefix` is already `pub(crate)` (`src/map/paths.rs:15`). Widening two `fn` to `pub(crate) fn` is a one-word change, so privacy is not a real barrier — the defensible reason to duplicate would be avoiding coupling the onboard resolver to reconciler internals, which the text does not state. Under this lens it does not escalate: duplicate-then-dedupe is a local refactor of ~20 lines, and the clause already mandates a parity test against the reconciler fixture expectations, which bounds the drift window. Minimal remedy: the flow unit should either record why sharing was rejected, or share.

**2 (minor, wording). `help.commands.onboard.args` is an addition, not an invalidation.** Line 188 lists "the `help.commands.onboard.usage` and `help.commands.onboard.args` values in `docs/design-system/copy.toml`" among surfaces clause 1 *invalidates*, but only `usage` exists today (`docs/design-system/copy.toml:840-841`); `args` must be authored. Harmless in direction — the operative requirement at lines 136-142 (both keys must name `cairn onboard [decisions] [options]` and explain that omitting `decisions` keeps the orphan report) is correct and implementable, `copy::lookup` returns the key rather than panicking when absent (`src/copy.rs:22-31`), the renderer emits the Arguments section only when a real string was authored (`src/cli/help/mod.rs:400-408`), and 58 other commands already carry `args` (e.g. `:722-724`). Not a defect; noted so the implementer does not hunt for a value to edit.

**3 (QUESTION, naming). `cairn onboard decisions` sits beside the existing top-level `cairn decisions <node>` query.** `decisions` is already a registered top-level command (`src/cli/commands/mod.rs:278`; `docs/integration-contract.md:80` maps it to `cairn_decisions`, "Decision artefacts linked to the node"). There is no parser collision — dispatch keys on `parsed.command == "onboard"` (`src/cli/mod.rs:402`) — but two `decisions` surfaces with unrelated semantics is an ergonomics cost the ruling does not acknowledge. Reversibility keeps this out of escalation: renaming a brand-new subcommand is cheap while its only consumer is the reference the same unit ships, and the pack gates would catch a stale invocation. One sentence from the author on why `decisions` and not, say, `evidence`, would close it.

**4 (observation, pre-existing).** `docs/integration-contract.md:88` advertises MCP tool `cairn_onboard`, but no such tool exists: MCP tools are generated from `query_api::visible_tools` (`src/mcp/mod.rs:225-231`) and `cairn_onboard` appears nowhere in `src/`. This drift predates the ruling and is not caused by it, but the implementer edits exactly that line, so it will surface. The ruling correctly does not claim MCP parity for the new subcommand.

## Reversibility assessment

- **Nothing durable is written by the flow.** Its only artefact is a `status: proposed` markdown file created by the existing writer (`src/cli/commands/decision.rs:119-150`). No graph mutation, no acceptance, no receipts, no on-disk format, no migration. Reversing a bad extraction run is `rm`.
- **The one behaviour change to a live surface is provably unobserved.** Making an unknown positional exit 2 changes `cairn onboard foo` from silently running the orphan report to erroring. A repo-wide grep for `cairn onboard` shows every invocation — tests (`tests/kernel.rs:1579,1602`; `src/cli/mod.rs:2848`), docs (`docs/commands.md:146`, `docs/agent-setup.md:93`, `docs/integration-contract.md:192`), shipped references (`command-reference.md:90`, `graph-navigation.md:35`) — uses the bare or `--json` form. Blast radius zero; reversal is one match arm.
- **The highest-residue element picks the lower-residue direction.** Pack content is the only part of this ruling with a footprint outside the repo. Adding a reference under an existing skill leaves one file; adding a standalone skill leaves a whole installed skill directory that is strictly harder to withdraw. Both directions fail loudly under the gates (`determinism_drift_tests.rs:128,228,369-376`; `router_route_tests.rs:78,100`), and the sanctioned promotion path stays open (`dec.cli-agent-workflow-consolidation:74-75`).
- **The uncertain part is bounded by an additive escape hatch.** The genuine unknown — whether the closed evidence set (`docs/adr/`, `docs/decisions/`, `Decision`/`Rationale`/`Invariant` headings, `// invariant:` markers) is wide enough — is resolved by adding sources, never removing them; JSON `data` contents are semi-stable by existing policy (`docs/integration-contract.md:198-200`); and a revisit trigger already names exactly this miss.
- **Tier choice is the safe direction and mechanically enforced anyway.** `local` is not selectable here (`src/hooks/ratification.rs:340-368` against `docs/registries/binding-surface.md:7`), and over-classifying costs one review now rather than a retroactive ratification later.

## What survived

I attacked and could not break: the no-subcommand compatibility claim (no caller passes a positional, verified repo-wide); the pack gate inventory (every named gate exists and is load-bearing — fixed-size 21-element arrays, the `.gitattributes` set equality, both router reachability directions); the binding-tier rationale (the allowlist row is exactly line 7, and the local-tier gate is real code, not prose); the writer-reuse claims (every helper behaves exactly as described, flags included); the default-binding claim (`kinds.rs:138-140`, cited to the line); the node bindings (`cairn.kernel.cli` really does own both `./src/cli` and `./tools/agent-pack`, which is what makes the restored two-node list correct rather than merely convenient); and artefact integrity (all `affects`, `related`, and `informed_by` targets resolve; no acceptance marker present). The exclusion of `graph-navigation.md` from `affects` also survives scrutiny: its line 35 describes the unchanged no-subcommand behaviour and does not go stale.

Verdict: convergent — every clause is additive, gated, and cheap to unwind (the only downstream residue is a single pack file, and the ruling deliberately chose the lower-residue of the two hosting options), the tier is mechanically forced rather than chosen, no live alternative survives on this lens, and no defects were found; Findings 1-3 are non-blocking notes for the implementation unit.
