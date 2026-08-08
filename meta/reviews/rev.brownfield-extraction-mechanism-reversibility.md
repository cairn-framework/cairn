---
node: cairn.brownfield
date: 2026-08-08
reviewer: anthropic/claude-fable-5/contestedness-reversibility
review_type: agent_cross_model
subject_hash: sha256:8ee5c2d4e8ce9c8bd90cb3b210a1fa6dbcb1b1f253114ffe533e9df1e4b57d64
lens_prompt_hash: sha256:45136bbc19a4732ebacc4bd194791674e1266a4ae11c8fd51bfcfae9c7c4d698
---

# Receipt review: brownfield extraction mechanism (reversibility lens)

Receipt-grade review of `dec.brownfield-extraction-mechanism` under
`docs/agent/lenses/contestedness-reversibility.md`, run clause-by-clause with
read-only repository access.

## Claims verified

Scope: commit 7feb541, clean tree (`git status --porcelain` returned 0 lines,
`git rev-parse HEAD` = 7feb541536c3f92b0a97d6ebec22d3d9665d8224), so every file
read below is the committed blob. Executed: `./target/debug/cairn lint` (twice),
`git log`, `git status`, `git rev-parse`, `git diff --stat`, plus read-only
`grep`, `sed`, `diff`, and file reads. `cairn scan` was NOT run and no repository
file was modified. Disclosure: one lint invocation was redirected to
`/tmp/lint.txt` outside the repository; no repository bytes were written, and no
later step used redirection.

Lens: reversibility and blast radius.

1. Frontmatter `affects` (lines 17 to 34) has EXACTLY 18 entries, and all 18
   paths exist at 7feb541. Verified by enumerating the block and stat-ing each
   path: meta/decisions/brownfield-extraction-mechanism.md,
   meta/research/brownfield-extraction-mechanism.comparison.md,
   src/cli/commands/onboard.rs, src/cli/mod.rs, src/brownfield/onboard.rs,
   docs/commands.md, docs/integration-contract.md, docs/design-system/copy.toml,
   tools/agent-pack/content/skills/cairn-dev/SKILL.md,
   .claude/skills/cairn-dev/SKILL.md,
   tools/agent-pack/content/skills/cairn-dev/references/command-reference.md,
   .claude/skills/cairn-dev/references/command-reference.md,
   tools/agent-pack/manifest.toml, src/cli/commands/pack_assets.rs,
   tests/kernel.rs, .gitattributes,
   tools/agent-pack/tests/determinism_drift_tests.rs,
   tools/agent-pack/tests/router_route_tests.rs. Count: 18, existence: 18 of 18.
2. Reconciler helpers are private, so the reimplementation plus parity test is
   the honest route: `fn eligible_owners` at src/reconcile/generic.rs:382 and
   `fn most_specific_owner` at src/reconcile/generic.rs:405 carry no `pub`.
   The prefix rule they use, `crate::map::paths::is_component_prefix`
   (src/reconcile/generic.rs:407), is `pub(crate)` at src/map/paths.rs:15, so
   the new resolver can call the same primitive rather than fork it.
3. The stub-blueprint precondition claim holds: src/cli/commands/onboard.rs:13
   to 26 writes a temporary `System Stub` blueprint when `parsed.file` is
   absent, then src/cli/commands/onboard.rs:28 loads the project and
   src/cli/commands/onboard.rs:30 calls `brownfield::onboard::analyze`.
   `ORPHAN_CODE = "CAIRN_RECONCILE_ORPHANED_FILE"` at src/brownfield/onboard.rs:9
   confirms the grouping source is scanner findings.
4. The silent-fallback claim holds: `command_args` appears zero times in
   src/cli/commands/onboard.rs; every use is in src/cli/mod.rs (for example
   lines 284, 396, 512), and the onboard dispatch at src/cli/mod.rs:402 to 403
   passes `parsed` without inspecting positional arguments.
5. Copy surface: `[help.commands.onboard]` with `usage = "cairn onboard
   [options]"` at docs/design-system/copy.toml:840 to 841. The renderer builds
   `help.commands.<key>.usage` at src/cli/help/mod.rs:382 and
   `help.commands.<key>.args` at src/cli/help/mod.rs:400, and treats a missing
   `args` key as optional (src/cli/help/mod.rs:16 and the key-fallback guard at
   src/cli/help/mod.rs:402 to 404). The unprefixed usage value claim is
   consistent with the renderer prepending its own label at
   src/cli/help/mod.rs:386 to 388.
6. Registry default: absent `ratification` maps to `Binding` at
   src/artefacts/registry/kinds.rs:138 to 141, exactly the cited 138 to 140
   span, and the behaviour is pinned by
   src/artefacts/registry/kinds/tests.rs:117 to 126.
7. Writer claims: `fn decision_stub` at src/cli/commands/decision.rs:119 emits
   `status: proposed` at src/cli/commands/decision.rs:131 and no `ratification`
   line, so extracted drafts default to binding as stated. The shared
   `fn write_new_artefact` at src/cli/commands/decision.rs:61 is kind-agnostic.
8. Gap exclusion is correct: src/cli/commands/gap.rs:107 to 109 writes
   `gap: true` with `status: proposed`, and `CAIRN_GAP_UNRESOLVED` lints every
   open gap per src/artefacts/validate/mod.rs:207 to 211.
9. Tier anchor: `tools/agent-pack/content/` is the exact directory row at
   docs/registries/binding-surface.md:7, and the governing consolidation rule
   is verbatim at meta/decisions/cli-agent-workflow-consolidation.md:74 to 75
   ("Future pack promotions are judged on marginal lift over the current pack
   and merge non-overlapping value into the owning skill before adding a new
   skill.").
10. Pack plumbing: `BASE_ASSETS` at src/cli/commands/pack_assets.rs:61 to 118
    holds one `include_str!` row per shipped cairn-dev reference (for example
    line 80 for command-reference.md); `LOOP_ASSETS` at
    src/cli/commands/pack_assets.rs:124 to 153 holds only loop-mode assets;
    `all_assets` at src/cli/commands/pack_assets.rs:158 to 164 rewrites the
    adapter root. Placing ordinary guidance in `BASE_ASSETS` matches the
    existing table shape.
11. Invalidated gates are real: size-pinned
    `EXPECTED_CANONICAL: [(&str, &str, &str); 21]` at
    tools/agent-pack/tests/determinism_drift_tests.rs:128 and
    `EXPECTED_CLAUDE` at line 228, both compared against manifest.toml at lines
    337 and 352; route reachability runs both directions at
    tools/agent-pack/tests/router_route_tests.rs:78 and 100, with git tracking
    checked at line 260. `.gitattributes` lines 2 to 21 mark each shipped
    `.claude` asset `linguist-generated=true`.
12. Router mirrors are byte-identical today (`diff` of
    tools/agent-pack/content/skills/cairn-dev/SKILL.md against
    .claude/skills/cairn-dev/SKILL.md produced no output), and the required row
    shape matches the existing table row at line 53 of both files.
13. Surfaces named for update exist and currently carry the pre-change text:
    docs/commands.md:146, docs/integration-contract.md:88, and the CLI-only
    registry entry at src/cli/mod.rs:1129 to 1132.
14. The "no `cairn brownfield` noun" claim holds: zero occurrences of the
    quoted command name `"brownfield"` in src/cli/mod.rs.
15. Lifecycle-neutral prose confirmed at lines 44 to 47 of the subject: the
    Status section defers state and receipts to frontmatter.
16. `./target/debug/cairn lint` exits 0 at this commit with 31 output lines,
    all Info severity, none naming `dec.brownfield-extraction-mechanism`.

## Findings

Reversibility assessment, with the concrete undo path for each half.

1. CLI half is additive and state-free. The ruling adds a subcommand branch and
   forbids mutation: no `status: accepted`, no blueprint write, no persisted
   format, no migration. Undo in six months is deleting one branch plus its copy
   rows, docs rows, and tests, which returns the surface to the behaviour
   verified at src/cli/commands/onboard.rs:6 to 31. Nothing durable is left
   behind in a user repository because the command only reads and prints.
2. One genuine back-compat change, and it is cheap. Today `cairn onboard <junk>`
   silently renders the orphan report because `command_args` is never read in
   src/cli/commands/onboard.rs; after this ruling it exits 2. The affected caller
   population is small: onboard is CLI-only with no argument shape in the MCP
   row at docs/integration-contract.md:88. Reverting is a one-branch change.
   Recorded as an accepted, low-cost risk, not a fork.
3. The expensive-to-retract option was avoided. A new top-level noun would enter
   the command registry (src/cli/mod.rs:1129 style rows), help, copy, docs, and
   the integration-contract table, and retiring a published top-level noun is a
   user-visible break. There is no `cairn brownfield` noun today (finding 14
   above), so hosting under `onboard` keeps the retreat path short.
4. Pack half has one real stickiness cost, disclosed here for the record. In
   `run_apply`, previously owned ledger rows are re-carried at
   src/cli/commands/pack.rs:196 (`carry_owned`), and the write loop at
   src/cli/commands/pack.rs:157 to 194 only iterates current assets. A withdrawn
   reference therefore stays on disk in an already-installed repository until
   `cairn pack remove` deletes ledger-tracked files
   (src/cli/commands/pack.rs:395 to 428). The residue is inert guidance prose,
   not executable behaviour, and this property is identical for the ten
   cairn-dev references already shipped, so the ruling adds no new class of
   irreversibility.
5. Blast radius is bounded and mostly non-code. Of the 18 affects entries, 4 are
   Rust source, 2 are meta artefacts, and 12 are docs, copy, mirrors, manifest,
   and tests. No entry is a data format, database, wire schema, or on-disk state
   file, so there is no migration to unwind.
6. Uncertainty is bounded by queryable escape hatches. The four
   `revisit_triggers` (frontmatter lines 36 to 39) name the exact failure modes
   this lens worries about: a first-party inference backend, external validation
   showing the deterministic index misses material ADR-like locations or cannot
   preserve a node binding, an ownership or mutation contract change on onboard,
   and a change to the pack distribution model. Each trigger maps to a
   reconsideration rather than a stranded surface.
7. Observation, no byte change required. The subject says clause 1 invalidates
   "the `help.commands.onboard.usage` and `help.commands.onboard.args` values in
   `docs/design-system/copy.toml`", but only `usage` exists today
   (docs/design-system/copy.toml:840 to 841); `args` must be authored, and the
   renderer treats it as optional until then (src/cli/help/mod.rs:16 and 402 to
   404). The operative requirement, that both keys name the supported form and
   stay in copy rather than becoming parallel hardcoded strings, is unambiguous
   and implementable as written, so this is a wording nuance rather than a
   defect. It has no reversibility consequence.

No defects. No live alternative on this lens: the two halves are additive,
revertible by deleting the same rows they add, and the one durable-looking
surface (pack distribution) already has a sanctioned removal verb.

## Verdict

PASS

Reversibility is cheap on both halves, the blast radius is bounded to 18 verified
paths with no migration or persisted state, and the sole stickiness (installed
pack assets clearing only on `cairn pack remove`) is a pre-existing property of
the pack, so this ruling is convergent on this lens.
