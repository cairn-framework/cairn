---
node: cairn.brownfield
date: 2026-08-08
reviewer: anthropic/claude-fable-5/contestedness-correctness
review_type: agent_cross_model
subject_hash: sha256:41872c8908bfeeb9e1dc94cfa964d3d36b3f980691f30beba54e7b09bd4cec5d
lens_prompt_hash: sha256:288d695e09e8f9c922e07c0349c2870f887b817b9a39eac777c501f90c70f6c5
---

# Receipt review: brownfield extraction mechanism (correctness lens)

Receipt-grade review of `dec.brownfield-extraction-mechanism` under
`docs/agent/lenses/contestedness-correctness.md`, run clause-by-clause with
read-only repository access.

## Claims verified

Read-only verification against `/Users/george/repos/cairn-ov-spine` at commit `90c57c1`. Every citation below was opened; nothing was edited. `./target/debug/cairn lint` was run (exit 0, no Error/Warn, no finding naming this artefact) and `git status --porcelain` was empty afterwards, so verification itself mutated nothing.

### Frontmatter

- All 18 `affects:` paths exist on disk (individually stat-checked). The two future files — `tools/agent-pack/content/skills/cairn-dev/references/task-brownfield-decision-extraction.md` and its `.claude` mirror — are absent and correctly excluded (driver adjudication; not re-litigated).
- `nodes: [cairn.brownfield, cairn.kernel.cli]` both resolve: `cairn.blueprint:84` (`Module CLI`, paths `./src/cli` at :85 **and** `./tools/agent-pack` at :86) and `cairn.blueprint:119` (`Module Brownfield`, `./src/brownfield`). The restored two-node list is not decorative — it is the minimum pair that owns every code and pack surface in `affects`.
- `informed_by: res.brownfield-extraction-mechanism.comparison` matches `meta/research/brownfield-extraction-mechanism.comparison.md:2`, `method: primary` (:4), consistent with clause 2's "primary research artefact".
- All six `related:` decisions exist; the two load-bearing ones are `status: accepted` (`cli-agent-workflow-consolidation.md:9`, `reviewer-panel-ratification.md:6`, plus `decision-ratification-tiers.md:7`, `agent-pack-packaging.md:5`).
- `affects` is a block list, satisfying the parser's list-not-scalar requirement (`src/artefacts/registry/kinds.rs:153-156`).

### Context

- "Deterministic structural starting point, no prose inspection": `src/brownfield/discovery.rs:20-26` bounds the walk (`SOURCE_EXTS`, `MIN_FILES 3`, `MAX_DEPTH 4`); `:103` derives ids from the relative path; `:106-115` records evidence paths; `:128` derives edges only through `import_edges::derive_import_edges`. The module never reads ADR-like documents. VERIFIED.
- "Cairn performs no LLM inference on this path": `src/summariser/backend/mod.rs:16-19` marks `Disabled` as `#[default]`; `:20-30` `LocalCommand` shells out to an external binary; `:290-320` `HostedBackend` is an explicit placeholder returning `unsupported hosted backend` on **every** invocation, and `src/query_api/mod.rs:590-596` is the only construction site. The strongest-sounding claim in the artefact is the one most literally true. VERIFIED.
- "Three relevant writing/analysis surfaces; no `cairn brownfield` noun": the command registry `src/cli/mod.rs:1078-1146` runs `backlog…workspace` with no `brownfield`; `refine` dispatches at `:382`; `init --from-code` at `:284`. VERIFIED.

### Clause 1 — deterministic Cairn surface

- "The current onboard command synthesises a temporary stub blueprint when its requested file is absent": `src/cli/commands/onboard.rs:13-26`. VERIFIED.
- "That path reads scanner findings, not `discovery.rs` directly": `:28-30` calls `scanner::load_project` then `brownfield::onboard::analyze(&result.graph.findings)`; `src/brownfield/onboard.rs:9,63` keys on `CAIRN_RECONCILE_ORPHANED_FILE`. VERIFIED.
- "as the current `run_onboard_command` does by ignoring `command_args`": the function body (onboard.rs:6-59) never touches `parsed.command_args`, so `cairn onboard decisions` today renders the orphan report silently. VERIFIED — this is the concrete defect the clause exists to close.
- "exit code 2 with the literal error text `usage: …`": convention, not invention — `src/cli/mod.rs:288, 526, 560, 599, 611` and `src/cli/commands/mod.rs:64`. VERIFIED.
- "The usage copy value remains unprefixed for the help renderer": `src/cli/help/mod.rs:381-389` emits `help.usage-label` then the raw usage value; `docs/design-system/copy.toml:841` is unprefixed today. VERIFIED.
- "Reimplement the most-specific-prefix rule": `src/reconcile/generic.rs:382` `eligible_owners` and `:405` `most_specific_owner` are private; `:387` sorts `Reverse(path.len())` (most-specific first); `:406-408` returns the first `crate::map::paths::is_component_prefix` hit (`src/map/paths.rs:15`); `:394` restricts contributors to leaf or `owns_files` nodes. The ruling's description of the algorithm is exact, including the leaf/`owns-files` qualifier. VERIFIED.

### Clause 2 — cairn-dev reference hosting

- "`.claude` mirror is an `include_str!` input to `pack_assets.rs`": `src/cli/commands/pack_assets.rs:61-116`, each `BASE_ASSETS` row `include_str!`s `../../../.claude/skills/cairn-dev/references/task-*.md`. VERIFIED.
- "`all_assets` rewrites the adapter root for `.omp`": `:158-171`, `template.path.replacen(CLAUDE_ROOT, pack_root, 1)`. VERIFIED.
- "`LOOP_ASSETS` is opt-in and reserved for loop mode and its closure": `:120-127` states this in the source comment, and `SKILL.md:73-79` treats the reference's absence as a signal. Placing ordinary task guidance in `BASE_ASSETS` alongside `task-bug-investigation`/`task-refactoring`/`task-architecture-discovery`/`task-feature-implementation` (`:88-100`) is the only consistent slot. VERIFIED.
- "Manifest needs a canonical entry plus Claude and OMP adapter rows": `tools/agent-pack/manifest.toml:69-72` (canonical), `:187-191` (claude), `:313-320` (omp) show the exact triple pattern for a sibling reference. VERIFIED.
- "Router rows must be byte-identical": canonical and `.claude` `SKILL.md` `diff` clean today; the route table at `SKILL.md:50-61` uses precisely the `| task | \`references/x.md\` |` shape the ruling prescribes. `command-reference.md` copies also `diff` clean, and the onboard row exists at canonical `command-reference.md:90`. VERIFIED.
- "Manifest additions invalidate the generated-file marker, the size-pinned arrays, and route reachability": `.gitattributes` carries one `linguist-generated=true` row per mirror; `tools/agent-pack/tests/determinism_drift_tests.rs:128` and `:228` declare `[(&str,&str,&str); 21]` fixed-length arrays (consumed at `:337,352`); `router_route_tests.rs:78,100` enforce both route directions. All three must be edited. VERIFIED.

### Clause 3 — artefact writer

- `src/cli/commands/decision.rs:9-21` parses `--node` / `--informed-by` via `flag_values`; `:32-55` validates the kebab slug and the legacy `dec.<slug>.md` collision, then calls `decision_stub` + `write_new_artefact`. VERIFIED.
- `decision_stub` (`:119-150`) emits `id`, `nodes`, `status: proposed`, `date`, optional `informed_by`, and the standard sections — and emits **no** `ratification` field, exactly as clause 3 asserts. VERIFIED.
- `write_new_artefact` (`:61-81`) is kind-agnostic: refuses an existing target, creates the directory, writes the bytes. VERIFIED (ordering nit below).
- "`gap` is the wrong writer": `src/cli/commands/gap.rs:108` writes `gap: true`, and `src/artefacts/registry/validate/mod.rs:207-215` lints `CAIRN_GAP_UNRESOLVED` for every still-`proposed` gap. VERIFIED.
- "The registry parser defaults an absent value to `binding` (`kinds.rs:138-140`)": `src/artefacts/registry/kinds.rs:139` is the `map_or(Some(RatificationTier::Binding), …)`. Line pin correct. VERIFIED.

### Clause 4 — tier

- `docs/registries/binding-surface.md:7` is exactly `- tools/agent-pack/content/`. Line pin correct. VERIFIED.
- The tier is forced, not chosen: `dec.decision-ratification-tiers:25-33` requires a `local` ruling's `affects:` to be *wholly* outside the allowlist. This `affects:` includes two `tools/agent-pack/content/skills/cairn-dev/**` paths, and includes none of the other allowlist rows (`docs/spec.md`, `docs/registries/`, `src/artefacts/registry/`, `cairn.blueprint` are all absent — `kinds.rs` is cited but not affected). `manifest.toml`, `pack_assets.rs`, the two test files and `.gitattributes` sit outside `content/`, matching the ruling's "distribution surfaces governed via the content row". VERIFIED.
- "A convergent binding ruling may be accepted on convergent panel receipts, while a contested clause needs the recorded debate or maintainer path": `dec.reviewer-panel-ratification:61-71` states this almost verbatim. VERIFIED.
- The artefact carries `status: proposed`, no `ratified_by`, no `receipts`, no acceptance marker — consistent with its own Status section. VERIFIED.

### For / Against / Verdict and the restored adjudication line

- The Against paragraph's governing rule is real and accepted: `meta/decisions/cli-agent-workflow-consolidation.md:74-75` — "Future pack promotions are judged on marginal lift over the current pack and merge non-overlapping value into the owning skill before adding a new skill" (`status: accepted`, :9). The For paragraph states the genuine upside (standalone trigger discoverability) rather than a strawman. The adjudication sentence is phrased as a past event and asserts no acceptance. VERIFIED.

### Whole-artefact gate

`./target/debug/cairn lint` completes at exit 0 with only Info-severity findings, none referencing `dec.brownfield-extraction-mechanism`; specifically no `CAIRN_DECISION_MISSING_NODES`, `CAIRN_DECISION_ORPHANED`, `CAIRN_DECISION_REFERENCE_UNKNOWN`, or `CAIRN_DECISION_UNKNOWN_PROVENANCE`. The repository's own validators accept the artefact.

## Findings

None blocking. Four non-defect observations, in descending usefulness to the implementation unit:

1. **QUESTION (implementation, not ruling).** The stated reason for reimplementing the owner rule is that `eligible_owners`/`most_specific_owner` are private (`src/reconcile/generic.rs:382,405`) — true, but weak on its own, since widening them to `pub(crate)` is a one-line in-crate change. The stronger unstated reason is the module boundary: `src/reconcile` is its own node (`cairn.blueprint:91-93`) with no `cairn.kernel.cli -> cairn.reconcile` or `cairn.brownfield -> cairn.reconcile` edge (`:180-184`, `:199-200`). The conclusion holds either way, and the mandated parity test against the reconciler fixtures bounds the drift risk, so this is a rationale that under-sells itself rather than an error.
2. **Observation.** `docs/integration-contract.md:88` is a correct pin (the `onboard` command↔MCP row), but the same file carries a second stale-able surface at `:187-192` ("Brownfield onboarding (agent-driven)", naming `cairn onboard --json`). The file is in `affects`, so the implementer is covered; the single pin just under-describes the work.
3. **Observation.** `[help.commands.onboard]` in `docs/design-system/copy.toml:840-841` currently has `usage` only — no `args` key — so clause 1's phrasing "invalidates the `help.commands.onboard.usage` and `help.commands.onboard.args` values" describes one edit and one addition. `args` is an optional, supported slot (`src/cli/help/mod.rs:400-410`, missing keys fall back to the key name and the Arguments block is skipped), so the required end state is reachable and no gate breaks today. Not a defect in a proposed ruling describing a future flow.
4. **Nit.** Clause 3 lists `write_new_artefact` as "creates the directory, refuses an existing target, and writes the supplied bytes"; the code refuses first, then creates (`src/cli/commands/decision.rs:69-79`). The set of behaviours is right; only the recital order differs, with no consequence.

## Contestedness

The two forks a competent maintainer could have taken are both already closed on the record, not merely asserted:

- **Standalone skill vs reference-hosting** — adjudicated, with a recorded For/Against/Verdict and a governing accepted rule quoted at its true location (`cli-agent-workflow-consolidation.md:74-75`). Not re-litigated here.
- **Command host (new top-level noun, or `init --from-code`)** — refuted on repository evidence, not preference: no `brownfield` noun exists to extend (`src/cli/mod.rs:1078-1146`), and `init --from-code` owns a distinct map-bootstrap lifecycle that writes a change proposal, blueprint delta and templated contract stubs (`src/brownfield/init.rs:3-4,66-71`; `src/brownfield/templates.rs:1-35`) and can be archived via `--apply` (`src/cli/mod.rs:1656`). Coupling proposal-only decision drafting to that is a real cost, not a stylistic one.

Reversal cost is low on every axis: the ruling is proposal-only, adds one additive subcommand branch behind an explicit error path, adds one pack reference through the existing manifest/adapter machinery, mutates no blueprint, and writes no receipts. An alternative that is merely imaginable here — and both of the above are already refuted on stated evidence — is not a live alternative. Escalating this would spend the maintainer's signature on a ruling whose every load-bearing sentence I was able to confirm by opening the file it cites.

Verdict: convergent — every load-bearing claim verified against the tree at commit 90c57c1 (including all 18 `affects` paths, both node bindings, the four cited line pins, and a clean `cairn lint`), no live alternative survives the recorded adjudication and the repository evidence, and no defects; the four observations above are wording and implementation notes that need no signature.
