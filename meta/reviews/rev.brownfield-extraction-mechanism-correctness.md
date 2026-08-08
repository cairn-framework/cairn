---
node: cairn.brownfield
date: 2026-08-08
reviewer: anthropic/claude-fable-5/contestedness-correctness
review_type: agent_cross_model
subject_hash: sha256:8ee5c2d4e8ce9c8bd90cb3b210a1fa6dbcb1b1f253114ffe533e9df1e4b57d64
lens_prompt_hash: sha256:288d695e09e8f9c922e07c0349c2870f887b817b9a39eac777c501f90c70f6c5
---

# Receipt review: brownfield extraction mechanism (correctness lens)

Receipt-grade review of `dec.brownfield-extraction-mechanism` under
`docs/agent/lenses/contestedness-correctness.md`, run clause-by-clause with
read-only repository access.

## Claims verified

Subject bytes are identical to commit 7feb541, and the working tree carried no modification before or after this review.

Frontmatter and affects list. The `affects:` list occupies lines 17 to 34 of `meta/decisions/brownfield-extraction-mechanism.md` and has exactly 18 entries. Each of the 18 was existence-checked individually against the tree, and all 18 exist: `meta/decisions/brownfield-extraction-mechanism.md`, `meta/research/brownfield-extraction-mechanism.comparison.md`, `src/cli/commands/onboard.rs`, `src/cli/mod.rs`, `src/brownfield/onboard.rs`, `docs/commands.md`, `docs/integration-contract.md`, `docs/design-system/copy.toml`, `tools/agent-pack/content/skills/cairn-dev/SKILL.md`, `.claude/skills/cairn-dev/SKILL.md`, `tools/agent-pack/content/skills/cairn-dev/references/command-reference.md`, `.claude/skills/cairn-dev/references/command-reference.md`, `tools/agent-pack/manifest.toml`, `src/cli/commands/pack_assets.rs`, `tests/kernel.rs`, `.gitattributes`, `tools/agent-pack/tests/determinism_drift_tests.rs`, `tools/agent-pack/tests/router_route_tests.rs`. Both bound nodes resolve: `cairn get cairn.brownfield` and `cairn get cairn.kernel.cli` each return a node with contracts and decisions.

Graph health. `./target/debug/cairn lint` exits 0 with Info-level findings only plus one deferred finding, and reports no finding naming `dec.brownfield-extraction-mechanism`.

Context clauses. `src/brownfield/discovery.rs:1-7` and `:90-128` confirm a bounded walk, path-derived candidate ids, sorted evidence paths, and import-only edges. `src/cli/commands/decision.rs:26-34` validates a kebab slug, and `decision_stub` at `:119-141` writes `status: proposed` with the standard sections. `src/cli/commands/gap.rs:3-5` and `:19` confirm the `gap: true` writer, and `src/artefacts/registry/validate/mod.rs:207-215` confirms `CAIRN_GAP_UNRESOLVED` stays open while proposed. No `cairn brownfield` noun exists: the help registry declares `spec("onboard", ...)` at `src/cli/help/mod.rs:162` and `spec("refine", ...)` at `:168`, and no dispatch branch matches a `brownfield` command. The inference constraint holds: `SummariserMode` defaults to `Disabled` at `src/summariser/backend/mod.rs:13-19`, and `HostedBackend` at `:290-320` is documented as a placeholder that returns an unsupported-backend error on every invocation.

Clause 1. `src/cli/commands/onboard.rs:6-58` contains no reference to `command_args`, so today an unrecognised positional falls through to the orphan report, exactly as the ruling states. Lines 14 to 26 synthesise a temporary stub blueprint when the requested file is absent. Line 30 calls `crate::brownfield::onboard::analyze(&result.graph.findings)` after `scanner::load_project`, and `analyze` groups `CAIRN_RECONCILE_ORPHANED_FILE` (`src/brownfield/onboard.rs:9`, `:63`), confirming that the path reads scanner findings and not `discovery.rs`. The binding-rule description matches the reconciler: `eligible_owners` at `src/reconcile/generic.rs:382` is private and sorts owners most-specific first by reversed path length; `collect_owner` at `:392-401` admits leaf nodes or `owns_files` nodes; `most_specific_owner` at `:405-411` is private and delegates to `crate::map::paths::is_component_prefix`, declared `pub(crate)` at `src/map/paths.rs:15`. The prefix split is consistent with the tree: `docs/design-system/copy.toml:840-841` stores the unprefixed value `cairn onboard [options]`, while exit code 2 with a literal `usage: ` string is the established error convention (`src/cli/mod.rs:288`, `src/cli/commands/gap.rs:19`, `src/cli/commands/archive.rs:23`). `src/copy.rs:22-40` confirms that `copy::lookup` returns the key itself when a value is absent, and `src/cli/help/mod.rs:382-403` confirms that the renderer builds `help.commands.<key>.usage` and `.args` dynamically.

Clause 2. `src/cli/commands/pack_assets.rs:61-119` shows `BASE_ASSETS` using `include_str!` on each `.claude` cairn-dev reference, `:124-152` shows `LOOP_ASSETS` as the opt-in loop closure, and `all_assets` at `:158-171` rewrites `CLAUDE_ROOT` to the requested pack root, which is how the `.omp` destination is produced. `tools/agent-pack/manifest.toml:84-92`, `:205-215`, and `:337-347` show the canonical entry plus Claude and OMP adapter rows per reference, matching the required additions. The proposed route row matches the live router table shape at `tools/agent-pack/content/skills/cairn-dev/SKILL.md:51-61`, and a byte comparison proves the canonical and `.claude` copies of both `SKILL.md` and `references/command-reference.md` are identical today. The stated invalidations are real: `tools/agent-pack/tests/determinism_drift_tests.rs:128` and `:228` pin arrays of length 21 asserted at `:337-376`; `tools/agent-pack/tests/router_route_tests.rs:78` and `:100` assert route resolution and reference reachability; `.gitattributes` carries one `linguist-generated=true` row per shipped mirror; `command-reference.md:90` currently lists only `cairn onboard`; `docs/commands.md:146` and `docs/integration-contract.md:88` carry the onboarding rows, and line 88 is precisely the `onboard` row; `tests/kernel.rs:1554-1608` covers human and JSON onboard behaviour; `src/cli/mod.rs:402-403` is the dispatch and `:1129-1131` the CLI-only description.

Clause 3. `write_new_artefact` at `src/cli/commands/decision.rs:61-81` is kind-agnostic: it refuses an existing target before creating the directory, then writes supplied bytes, owning no typed frontmatter. `decision_stub` at `:119-141` emits no `ratification` field. The registry default sits exactly where the ruling cites it: `src/artefacts/registry/kinds.rs:138-140` maps an absent `ratification` to `RatificationTier::Binding`.

Clause 4. `docs/registries/binding-surface.md:7` is the `tools/agent-pack/content/` directory row, and no other allowlist row (lines 5, 6, 8, 9) matches any of the 18 affects entries. `dec.decision-ratification-tiers` is accepted and states at lines 25 to 33 that `local` requires an `affects:` list wholly outside the allowlist, so binding is the only reachable tier here. `dec.reviewer-panel-ratification` is accepted and supports the convergent-receipts versus contested-debate routing at lines 54 to 65. The governing consolidation rule is quoted accurately: `meta/decisions/cli-agent-workflow-consolidation.md:74-75` states that future pack promotions are judged on marginal lift over the current pack and that non-overlapping value merges into the owning skill before adding a new skill, and that decision is `status: accepted`.

Rejected alternatives. `init --from-code` does write a change proposal under `meta/changes/brownfield-init`, a blueprint delta, and templated stub contracts (`src/brownfield/init.rs:4`, `:13`, `:28`; `src/brownfield/templates.rs:1-55`), so the coupling argument rests on fact rather than assertion. The `cairn gap` rejection rests on the verified `gap: true` plus open-finding lifecycle cited above.

## Findings

No defects. One precision observation that requires no change: the Clause 2 sentence listing invalidated surfaces names both `help.commands.onboard.usage` and `help.commands.onboard.args`, but `docs/design-system/copy.toml:840-841` currently declares only `usage` for onboard, so `args` would be authored rather than amended. This is not an error in substance, because `src/cli/help/mod.rs:16-18` documents `args` as an optional per-command key and `:400-403` renders an Arguments section only when a real string is authored, so the Clause 1 instruction that both values must name the supported form is achievable and unambiguous. Nothing turns on the word choice.

The previously contested standalone-skill option was re-examined only for the correctness of its stated grounds, not re-adjudicated. Its refutation rests on an accepted decision quoted correctly at `cli-agent-workflow-consolidation.md:74-75`, so it is a refuted option rather than a live alternative.

## Verdict

PASS
Executed for this receipt: read-only file reads and greps, `git log`, `git show --stat`, a `git diff` of the subject against HEAD, `git status --short`, `./target/debug/cairn lint` twice, and two `cairn get --json` queries; `cairn scan` was never run, no repository file was created, modified, or deleted, and the only write anywhere was the second lint invocation redirecting its stdout to `/tmp/lint.txt`, a temporary file outside the repository.
