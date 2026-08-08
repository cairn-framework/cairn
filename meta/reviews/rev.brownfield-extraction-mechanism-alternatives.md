---
node: cairn.brownfield
date: 2026-08-08
reviewer: anthropic/claude-fable-5/contestedness-alternatives
review_type: agent_cross_model
subject_hash: sha256:8ee5c2d4e8ce9c8bd90cb3b210a1fa6dbcb1b1f253114ffe533e9df1e4b57d64
lens_prompt_hash: sha256:1ceb131f531393b6d998c5641ce6741774cce8f6b0305d4fd2876f4db4179003
---

# Receipt review: brownfield extraction mechanism (alternatives lens)

Receipt-grade review of `dec.brownfield-extraction-mechanism` under
`docs/agent/lenses/contestedness-alternatives.md`, run clause-by-clause with
read-only repository access.

## Claims verified

Access was read-only. Executed in `/Users/george/repos/cairn-ov-spine` at commit
`7feb541`, clean tree (`git log --oneline -1`; `git status --porcelain` returned
zero lines): `./target/debug/cairn lint`, plus file reads, `grep`, `sed`, and two
`diff` byte comparisons. `cairn scan` was NOT run, and no writing command was run.

`cairn lint`: zero Error lines, zero Warning lines. Info findings only
(`CAIRN_DECISION_REFINED_AUTHORITY`, `CAIRN_RESEARCH_ORPHAN`,
`CAIRN_SOURCE_UNVERIFIED`, `CAIRN_DECISION_ACCUMULATION`,
`CAIRN_REVIEW_SUBJECT_UNMATCHED`), plus 1 finding deferred by
`dec.revisit-trigger-correlator-deferred`.

Frontmatter:

- The `affects:` list at `meta/decisions/brownfield-extraction-mechanism.md:16-34`
  has EXACTLY 18 entries. All 18 were enumerated and resolved against the working
  tree; all 18 exist on disk. Checked by enumeration, not sampling. The two
  not-yet-written reference files are correctly absent, per the driver
  adjudication that is out of scope here.
- The subject bytes contain no U+2014 and no U+2013.

Clause 1, deterministic Cairn surface:

- `cairn onboard` loads the project through the scanner and passes findings to
  `brownfield::onboard::analyze` (`src/cli/commands/onboard.rs:28-37`); `analyze`
  groups the `CAIRN_RECONCILE_ORPHANED_FILE` code (`src/brownfield/onboard.rs:9`,
  `src/brownfield/onboard.rs:63-66`). VERIFIED.
- The command synthesises a temporary stub blueprint when the requested file is
  absent (`src/cli/commands/onboard.rs:13-26`), writing a single node with id
  `stub` and no owned files (`src/cli/commands/onboard.rs:24`). VERIFIED, and this
  is what justifies the fail-closed precondition: under that stub every path
  resolves unbound.
- `run_onboard_command` never reads `parsed.command_args` anywhere in the function
  (`src/cli/commands/onboard.rs:6-58`), so a positional subcommand is silently
  ignored today and falls back to the orphan report. VERIFIED exactly as stated.
- `--json` already exists on this path (`src/cli/commands/onboard.rs:31-34`), so
  `decisions --json` reuses an established renderer pair. VERIFIED.
- `eligible_owners` (`src/reconcile/generic.rs:382`) and `most_specific_owner`
  (`src/reconcile/generic.rs:405`) are plain `fn`, private to the module.
  `map::paths::is_component_prefix` is `pub(crate)` (`src/map/paths.rs:15`) and is
  the primitive `most_specific_owner` calls (`src/reconcile/generic.rs:407`).
  VERIFIED.
- Exit code 2 for a usage error is the documented contract
  (`docs/integration-contract.md:36-38`, row "2 | Usage error (bad arguments,
  unknown command)"), is echoed in help text (`src/cli/mod.rs:1221`), and has an
  exact literal precedent of the mandated shape at `src/cli/mod.rs:526`,
  `err(2, "usage: cairn change new <change-id>")`. VERIFIED: the mandated error
  contract follows convention rather than inventing one.
- `help.commands.onboard` exists with `usage = "cairn onboard [options]"` and no
  `args` key (`docs/design-system/copy.toml:840-841`). The help renderer prepends
  its own label before the usage value (`src/cli/help/mod.rs:382-388`), confirming
  the rule that the copy value stays unprefixed and only the error path adds
  `usage: `. `args` is a documented optional key (`src/cli/help/mod.rs:16-18`) and
  the renderer emits the Arguments section only when one was authored
  (`src/cli/help/mod.rs:400-405`). VERIFIED.
- `docs/commands.md:146` and `docs/integration-contract.md:88` both carry the
  onboard row worded "Suggest blueprint entries for orphaned files", so both are
  genuinely invalidated. VERIFIED, and line 88 is the exact cited line.
- `tests/kernel.rs:1554-1608` covers the human and the `--json` onboard paths, so
  the named behaviour coverage exists. VERIFIED.
- No `cairn brownfield` noun exists: dispatch handles `refine`
  (`src/cli/mod.rs:382`) and `onboard` (`src/cli/mod.rs:402`), and
  `init --from-code` writes `meta/changes/brownfield-init` (`src/cli/mod.rs:210`,
  `src/cli/mod.rs:1592`, `src/cli/mod.rs:1656`). VERIFIED.

Clause 2, cairn-dev reference hosting:

- `BASE_ASSETS` uses `include_str!` on the `.claude` destinations
  (`src/cli/commands/pack_assets.rs:28-35`, `src/cli/commands/pack_assets.rs:61-63`)
  and `all_assets` rewrites the adapter root via
  `replacen(CLAUDE_ROOT, pack_root, 1)` (`src/cli/commands/pack_assets.rs:158-167`)
  with `OMP_ROOT = ".omp/"` (`src/cli/commands/pack_assets.rs:20`). VERIFIED.
- `LOOP_ASSETS` holds loop-mode content only
  (`src/cli/commands/pack_assets.rs:124-130`) and is chained only when `with_loop`
  is set (`src/cli/commands/pack_assets.rs:161`). VERIFIED, supporting placement of
  ordinary guidance in `BASE_ASSETS`.
- The manifest pattern the ruling must follow exists today: canonical entry
  (`tools/agent-pack/manifest.toml:59-62`), Claude adapter row
  (`tools/agent-pack/manifest.toml:175-179`), OMP adapter row
  (`tools/agent-pack/manifest.toml:307-311`). VERIFIED.
- Canonical and `.claude` copies of `SKILL.md` and of
  `references/command-reference.md` are currently byte-identical: two `diff` runs
  produced no output. VERIFIED, so the byte-identity requirement restates a
  property that holds now.
- The proposed route row matches the existing table format: rows at
  `tools/agent-pack/content/skills/cairn-dev/SKILL.md:52-61` are two-column pipe
  rows ending in a backticked `references/<file>.md`. VERIFIED.
- `command-reference.md:90` currently carries only a "Triage orphans" row for
  `cairn onboard`, so it is genuinely stale under the new form. VERIFIED.
- `EXPECTED_CANONICAL` and `EXPECTED_CLAUDE` are size-pinned arrays of length 21
  (`tools/agent-pack/tests/determinism_drift_tests.rs:128` and `:228`). The same
  file asserts that the `.claude/` `linguist-generated=true` rows in
  `.gitattributes` equal exactly the expected destination set
  (`tools/agent-pack/tests/determinism_drift_tests.rs:370-376`), and
  `.gitattributes:2-22` holds exactly 21 such rows. VERIFIED: the `.gitattributes`
  invalidation is backed by an enforcing gate.
- Route reachability is enforced both ways
  (`tools/agent-pack/tests/router_route_tests.rs:78-95` and `:100-114`). VERIFIED.

Clause 3, artefact writer:

- `run_decision_new` validates the slug, refuses a legacy prefixed twin, and
  delegates to `decision_stub` plus `write_new_artefact`
  (`src/cli/commands/decision.rs:26-56`); the flags are `--node` and
  `--informed-by` (`src/cli/commands/decision.rs:15-17`), matching the invocation
  the reference is told to use. VERIFIED.
- `write_new_artefact` is kind-agnostic: create directory, refuse existing target,
  write bytes (`src/cli/commands/decision.rs:61-81`). VERIFIED.
- `decision_stub` emits `status: proposed` and no `ratification` field
  (`src/cli/commands/decision.rs:119-139`). VERIFIED.
- The registry defaults an absent `ratification` to `Binding`
  (`src/artefacts/registry/kinds.rs:138-141`), the exact cited range. VERIFIED.
- `cairn gap` writes `gap: true` with `status: proposed`
  (`src/cli/commands/gap.rs:108`) and leaves `CAIRN_GAP_UNRESOLVED` open until
  accepted or deleted (`src/cli/commands/gap.rs:3-5`, `src/cli/commands/gap.rs:67`).
  VERIFIED, so rejecting `gap` as the writer rests on real behaviour.

Clause 4, tier:

- `tools/agent-pack/content/` is a literal directory row in the binding registry at
  `docs/registries/binding-surface.md:7`. VERIFIED, exact line.
- The consolidation rule cited in Against reads, at
  `meta/decisions/cli-agent-workflow-consolidation.md:74-75`: "Future pack
  promotions are judged on marginal lift over the current pack and merge
  non-overlapping value into the owning skill before adding a new skill."
  VERIFIED verbatim. Per instruction the reference-hosting adjudication is not
  re-litigated.

## Findings

Alternatives lens. Four competing designs were steel-manned. None is a live fork.

1. New top-level noun (`cairn extract`, or `cairn brownfield decisions`). This
   objection was pressed hardest, including the argument that `onboard` today means
   the orphaned-file report specifically, that the subcommand still costs dispatch,
   help, copy, JSON, and tests, and that CLI grammar is expensive to change once
   published. Steel-manned in full, it still loses on repository evidence.
   First, surface cost is not a wash, it is strictly ordered: a new noun needs
   everything the subcommand needs plus a new `CliOnlyCommand` registry row
   (`src/cli/mod.rs:1129-1132` is the shape), a new help spec row
   (`src/cli/help/mod.rs:236-243` is the table), and a new `[help.commands.<name>]`
   copy block, whereas the subcommand adds a branch inside an existing function
   (`src/cli/commands/onboard.rs:6-58`) and edits an existing copy block
   (`docs/design-system/copy.toml:840-841`). Both pay the same docs and test cost.
   Second, no wire consumer constrains the choice: `onboard` appears nowhere in
   `src/mcp/mod.rs` or `src/query_api/` (grep returned zero matches in both), so
   the documented `cairn_onboard` row at `docs/integration-contract.md:88` binds no
   implemented MCP tool schema.
   Third, and decisively, this repository has already performed the analogous
   grammar migration in this exact direction and kept it cheap: `accept`,
   `archive`, `changes`, and `show` were top-level commands and are now
   subcommands of `change`, retained as retired aliases with a `preferred` pointer
   (`src/cli/help/mod.rs:238-243`, `preferred_for` at `src/cli/help/mod.rs:440-441`,
   `help.retired-note` at `src/cli/help/mod.rs:395-397`, and the four `preferred`
   values at `docs/design-system/copy.toml:946-964`). So grammar changes here are a
   supported, precedented, non-removing operation, and the stability guarantee at
   `docs/integration-contract.md:196-198` treats additions as non-breaking with
   only removals versioned. Promoting `cairn onboard decisions` to a top-level noun
   later would be an addition plus an alias, not a migration. The choice therefore
   fails the costly-to-reverse half of the contested test, and the repository's own
   precedent runs toward subcommands under an owning noun rather than new nouns.
2. Fold the evidence index into the existing no-subcommand output. Strongest form:
   it removes the dispatch branch, the copy edits, and the unknown-subcommand error
   contract entirely. It loses because it mutates the `data` payload of the
   existing `onboard` envelope, whose shape is listed as Stable at
   `docs/integration-contract.md:196`, for every current consumer of
   `cairn --json onboard`, while the subcommand keeps the existing form byte-stable
   as pinned by `tests/kernel.rs:1554-1608`. Clearly worse, not a fork.
3. Host the index in the writer, for example `cairn decision new --from-code`.
   Strongest form: one command for the harness, no hand-off. It loses because
   `run_decision_new` is a scanner-free pure writer
   (`src/cli/commands/decision.rs:26-56`) and would have to grow project loading and
   ownership resolution, destroying the read-the-evidence-then-write property and
   the reviewable intermediate report. Clearly worse.
4. Allow the stub blueprint instead of failing. Strongest form: a brownfield
   repository is exactly the case with no blueprint, so a hard precondition blocks
   the headline use case. It loses because the synthesized stub is a single node
   with no owned files (`src/cli/commands/onboard.rs:24`), so every candidate would
   resolve unbound and the report would carry none of the bindings that give it
   value, while `init --from-code` (`src/cli/mod.rs:210`) is the sanctioned
   bootstrap for that state. Relaxing a precondition later is a compatible
   widening, so it is also cheap to reverse.

Examined and dismissed, recorded so the maintainer can see they were not missed:

5. Reimplementing the most-specific-owner rule versus widening the reconciler
   helpers to `pub(crate)` and calling them (`src/reconcile/generic.rs:382`,
   `src/reconcile/generic.rs:405`). A real preference, but not a contested fork:
   the matching primitive `is_component_prefix` is already shared
   (`src/map/paths.rs:15`), so only the owner-collection walk is duplicated; the
   ruling mandates a parity test against the existing reconciler fixture
   expectations, which bounds divergence; and swapping the reimplementation for a
   shared call later is a local refactor inside one implementation unit with no
   artefact, wire, or distribution consequence. The unit may take the
   visibility-widening route without amending this decision.
6. Wording precision, not an error. Clause 2 says clause 1 invalidates the
   `help.commands.onboard.usage` and `help.commands.onboard.args` values in
   `docs/design-system/copy.toml`, but `args` does not exist there today
   (`docs/design-system/copy.toml:840-841` carries only `usage`). The operative
   instruction in clause 1 is unambiguous, that both values must name the supported
   form, `args` is a documented optional key (`src/cli/help/mod.rs:16-18`), and the
   renderer skips the section when absent (`src/cli/help/mod.rs:400-405`), so the
   required end state is implementable exactly as written. No defect recorded.

No defects. No load-bearing claim was found false. Every path, line, code, and
count assertion checked above resolved as the decision states, including the
18-entry affects list and the cited ranges
`src/artefacts/registry/kinds.rs:138-140`, `docs/registries/binding-surface.md:7`,
`docs/integration-contract.md:88`, and
`meta/decisions/cli-agent-workflow-consolidation.md:74-75`.

## Verdict

PASS

Alternatives lens: convergent, because each steel-manned competitor loses on
recorded repository evidence and the command-host choice is demonstrably cheap to
reverse under the existing retired-alias mechanism.
