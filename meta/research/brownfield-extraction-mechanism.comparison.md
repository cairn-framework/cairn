---
id: res.brownfield-extraction-mechanism.comparison
nodes: [cairn.brownfield]
date: 2026-08-08
method: primary
---

# Brownfield decision extraction mechanism comparison

## Question

`todo.brownfield-decision-extraction` asks for a guided path that mines an
existing codebase and ADR-like material into proposed decision artefacts. Its
implementation unit leaves open which product owns the path: a shipped agent
skill, a new deterministic Cairn command, an extension of `cairn onboard`, or a
hybrid. The choice must preserve real code evidence, let an agent write the
narrative that Cairn cannot infer, bind each draft to real blueprint nodes, and
leave every draft at `status: proposed`.

This is a primary code audit of the current brownfield, CLI, summariser, and
agent-pack surfaces. The recommendation below is a forced choice. It does not
assume a model backend that the repository does not provide.

## Method and evidence boundary

The relevant implementation facts are directly visible in the following
surfaces:

- `src/brownfield/discovery.rs` defines the existing filesystem traversal.
  It accepts Rust, TypeScript, JavaScript, Python, and Go files, records a
  directory only after it has at least three source files, does not recurse
  beyond depth four, and skips directories such as `target`, `node_modules`,
  `.git`, `meta`, `dist`, and `build`. It derives path-based candidate ids,
  records sorted evidence paths, and derives directed edges from imports that
  it can observe.
  This is a deterministic structural boundary. It does not read ADR files,
  README sections, or invariant comments, and it does not infer a decision
  from prose.
- `src/cli/commands/decision.rs` implements `cairn decision new`. It validates
  a kebab-case slug, writes a slug-only file below `meta/decisions`, emits
  deterministic `id`, `nodes`, `status: proposed`, `date`, and optional
  `informed_by` frontmatter, and supplies `Context`, `Decision`, `Rationale`,
  and `Consequences` sections. Its shared `write_new_artefact` helper refuses
  to overwrite an existing file. This is the existing decision-writing entry
  point the extraction flow can reuse.
- `src/cli/commands/gap.rs` implements a different write surface. `cairn gap`
  resolves one existing node, writes a `gap: true`, `status: proposed`
  decision containing the question, and leaves `CAIRN_GAP_UNRESOLVED` until the
  question is answered and the artefact is accepted or deleted. It records an
  implementation-time underspecification, not a normal extracted decision,
  and it has no evidence index or prose-drafting contract.
- `src/cli/commands/onboard.rs` currently loads a scanner result and passes
  its findings to `brownfield::onboard::analyze`. `src/brownfield/onboard.rs`
  groups orphaned files by parent directory and deterministically classifies
  each group as an ignore or node suggestion, with human and JSON renderers.
  This is already a read-only brownfield analysis surface, but it currently
  sees scanner orphan findings rather than ADR-like documents.
- The command change must also update `src/cli/mod.rs`'s onboard dispatch and
  `CliOnlyCommand` description, the Brownfield onboarding rows in
  `docs/commands.md` and `docs/integration-contract.md`, and
  `help.commands.onboard.usage` plus `help.commands.onboard.args` in
  `docs/design-system/copy.toml`. The shipped Cairn-dev
  `references/command-reference.md` in both canonical and `.claude` trees is
  another invalidated command surface and must describe the new subcommand.
- `src/brownfield/init.rs` calls `discovery::discover` and
  `src/brownfield/mod.rs::write_change` writes a proposal, a blueprint delta,
  and contract stubs under `meta/changes/brownfield-init`. The `init
  --from-code` family is therefore a map bootstrap and change-application
  lifecycle, not a decision-drafting lifecycle. Its proposal explicitly asks
  a reviewer to correct grouping, paths, edges, and contracts before applying.
- `src/brownfield/summarise.rs` can build a bounded request from a discovered
  candidate, but `enrich_candidate` invokes an injected
  `SummariserBackend` and falls back to the path-derived candidate and built-in
  stub when invocation fails. `src/summariser/backend/mod.rs` makes the
  boundary explicit: `Disabled` is the default, `LocalCommand` delegates JSON
  over stdin and stdout to an external process, and `HostedBackend` is a
  placeholder that returns an unsupported-backend error. `generate` persists
  the returned text as a pending draft but does not create a decision
  artefact. Nothing here gives Cairn first-party inference of decision prose.
- `dec.build-and-extension` keeps Brownfield and Summariser as distinct
  first-class modules and describes Summariser as LLM-assisted. It also places
  LLM outputs in the suggested-edge and summariser workflows rather than in
  deterministic graph enforcement. `docs/agent/principles.md` states the
  positive boundary: AI may propose narrative summaries, while the reconciler
  and its reality fingerprint remain mechanically checkable.


### Binding rule verified against the reconciler

Discovery's path-derived candidate id is evidence metadata, not a blueprint
node id. For each candidate code target and each indexed ADR-like path, the
selected command must normalize the path relative to the project root and
resolve it through the loaded blueprint's declared ownership. The existing
reconciler implements this rule in `src/reconcile/generic.rs`: eligible leaf
or `owns-files` nodes contribute normalized declared paths, those owners are
sorted most-specific first, and `map::paths::is_component_prefix` selects the
first matching owner. The resolved owner id is then checked against the loaded
graph before it can populate a decision `nodes:` list. If no owner matches, the
report keeps the evidence unbound rather than equating a path-derived
candidate id with a graph node or manufacturing a binding.

`dec.decision-ratification-tiers` makes a decision binding when it changes a
shipped pack, an artefact schema, a registry, a spec invariant, or another
protected surface. A single-container brownfield command with no
supersession could otherwise fit the local shape. The same decision says
shipped pack content is binding. `dec.cli-agent-workflow-consolidation` adds a
more specific pack rule: judge a new promotion on marginal lift over the
current pack, and merge non-overlapping value into the owning skill before
adding a new skill. `dec.reviewer-panel-ratification` permits a convergent
binding ruling to be accepted on panel receipts, while retaining the proposed
state until that panel runs.

The command half must therefore own only facts that can be reproduced from the
same checkout: candidate ids, paths, bounded source evidence, recognised ADR
files and headings, and stable ordering. The harness agent must own the
interpretation of those facts and the prose of a decision. Neither half may
silently set `status: accepted`.

## Candidate comparison

### Candidate A: shipped standalone agent skill as the whole mechanism

A standalone skill such as
`tools/agent-pack/content/skills/cairn-brownfield-decision-extraction/SKILL.md`
could ask the harness agent to inspect the repository, identify invariant
prose, and draft decisions. It is the only candidate that can write useful
narrative without adding model inference to Cairn, and its name could make the
workflow directly discoverable to an agent.

The determinism boundary is weak if the skill is the whole mechanism. File
walking, document selection, candidate ordering, and evidence citation would
be instructions interpreted by a model or by ad hoc shell commands. A changed
agent, prompt, or shell implementation can therefore change the evidence set
before the prose is written. The skill can be reviewed, but review is not a
stable evidence index that a later run can compare.

Maintenance is split across skill prose and distribution surfaces: pack
manifests, adapter destinations, and whatever commands the harness happens to
use. Updating the skill requires pack distribution work. The only binding
registry path is the canonical content under `tools/agent-pack/content/`, the
exact directory row in `docs/registries/binding-surface.md:7`; the manifests,
adapter destinations, generated mirrors, and compiled assets are distribution
surfaces governed by that allowlist via the content path. Because that
canonical pack content is binding under `dec.decision-ratification-tiers`,
this candidate is binding-tier even when it only changes one brownfield node.

The standalone option also loses on the accepted workflow-consolidation rule.
`dec.cli-agent-workflow-consolidation` requires a marginal-lift judgement
before a new skill is promoted and says non-overlapping value belongs in the
owning skill first. No such marginal-lift judgement exists for this flow.

**Result:** reject as the complete mechanism and reject the standalone
distribution surface. Retain the authoring instructions as a reference under
the existing `cairn-dev` skill in the selected hybrid.

### Candidate B: a new deterministic Cairn command as the whole mechanism

A new top-level noun such as `cairn brownfield extract` could reuse
`discovery::discover`, add deterministic scans for ADR-like files, and emit a
candidate/evidence report. It could also validate each proposed node against
the loaded graph. A command-owned JSON contract would be reproducible and
would be easy for a harness to consume.

The prose limit applies to every deterministic command, including the
selected `onboard decisions` command. It is therefore not the differentiator
between these command hosts: the hybrid pairs either command with the
reference-hosted harness authoring half. A command that promises complete
drafts would still violate `docs/agent/principles.md` and the verified fact in
the todo.

A new noun adds a dispatch branch, help and copy entries, JSON snapshots,
command documentation, and a second brownfield vocabulary beside the existing
`onboard`, `init --from-code`, and `refine` family. The real differentiator is
ownership and marginal lift. `onboard` already owns a brownfield read-only
report with human and JSON renderers, so its `decisions` subcommand gives the
the harness a hand-off point without a second top-level namespace. A
command-only change on `cairn.brownfield` could be local-tier when its
`affects:` list stays wholly outside the binding-surface allowlist. The
selected hybrid is binding-tier because its cairn-dev reference changes the
allowlisted `tools/agent-pack/content/` path; manifest, adapter, and compiled
asset files are distribution surfaces governed through that path. A new noun
has no recorded marginal lift over the existing owner.

**Result:** reject as the complete mechanism. The deterministic evidence
operation is needed, but it belongs on the existing `onboard` surface rather
than as a new top-level noun.

### Candidate C: extend `cairn onboard` as the whole mechanism

`cairn onboard` currently loads a scanner result and passes its findings to
`brownfield::onboard::analyze`; that analysis groups
`CAIRN_RECONCILE_ORPHANED_FILE` paths and classifies them as ignore or node
suggestions. Adding an `onboard decisions` subcommand is a smaller grammar and
maintenance change than adding a new top-level noun. The new branch can keep
the scanner-loaded graph as the source of node bindings and use bounded
discovery facts as evidence, while adding a deterministic, sorted scan of the
closed set named by the flow: files under `docs/adr/` and `docs/decisions/`,
README sections headed `Decision`, `Rationale`, or `Invariant`, and source
comments carrying the literal `// invariant:` or `# invariant:` markers. Its
output can index candidate ids, evidence paths and headings, and supporting
code paths without claiming semantic intent.

The existing `onboard` command still cannot draft the prose. Its current
`analyze` function consumes findings and returns classifications; it has no
model backend and no decision writer. It also cannot use `cairn gap` as a
substitute, because a gap is an unresolved implementation question with its
own warning lifecycle. As a pure onboard extension, the result would stop at
a report or a template and would not satisfy the value promised by the parent
todo.

A command-only onboard extension could be local-tier when its `affects:` list
stays wholly outside the binding-surface allowlist. The selected hybrid is
binding-tier because the cairn-dev reference changes protected
`tools/agent-pack/content/`; it has a narrower deterministic CLI surface, but
its combined mechanism is not local.

**Result:** reject as the complete mechanism, and select it as the
deterministic evidence half of the hybrid.

### Candidate D: hybrid deterministic evidence plus a cairn-dev reference

The hybrid gives each layer one responsibility:

1. `cairn onboard decisions` extends the existing `onboard` surface. The
   default form emits the human-readable report; `--json` emits the stable
   machine-readable index for the harness. The existing command still reads
   scanner findings through `analyze`; the new branch keeps the
   scanner-loaded graph for real node bindings and uses deterministic
   discovery facts as bounded code evidence. It resolves each evidence path
   or code target through the verified most-specific blueprint ownership rule,
   never equating a path-derived discovery id with a graph node id. It indexes
   the closed ADR-like set named above and emits stable JSON and human output.
   It does not invent semantic decisions, call a model, or accept an
   artefact.
2. The authoring reference
   `tools/agent-pack/content/skills/cairn-dev/references/task-brownfield-decision-extraction.md`
   runs under the existing `cairn-dev` skill. Both canonical and `.claude`
   `cairn-dev/SKILL.md` routers add the row
   `Mine an existing codebase into proposed decisions | references/task-brownfield-decision-extraction.md`.
   It invokes the evidence command, asks the harness agent to interpret the
   returned evidence, and records the research narrative and proposed decision
   body. The reference is ordinary guidance and therefore belongs in
   `BASE_ASSETS`, not opt-in `LOOP_ASSETS`.
3. The reference is mirrored under `.claude/skills/cairn-dev/references/`,
   registered in `tools/agent-pack/manifest.toml` with a canonical entry and
   new Claude and OMP adapter rows, and added to `BASE_ASSETS` in
   `src/cli/commands/pack_assets.rs`, whose renderer maps the Claude asset to
   the OMP root. The manifest update also requires the generated marker in
   `.gitattributes`, the size-pinned arrays in
   `tools/agent-pack/tests/determinism_drift_tests.rs`, and the reachability
   checks in `tools/agent-pack/tests/router_route_tests.rs`.
4. The command change also updates `src/cli/mod.rs` onboard description and
   dispatch, `src/cli/commands/onboard.rs`, `docs/commands.md`,
   `docs/integration-contract.md`, `docs/design-system/copy.toml` usage and
   args, `tests/kernel.rs` onboard behavior coverage, and both shipped
   Cairn-dev `references/command-reference.md` copies.
5. For each selected decision, the reference reuses
   `cairn decision new <slug> --node <id> --informed-by <research-id>`.
   That command's `decision_stub` creates the slug-only decision file and
   `status: proposed` frontmatter. The onboard report validates bindings; the
   writer does not re-resolve or validate graph ownership. The reference fills
   the body and permitted provenance fields and leaves the maintainer to
   accept, reject, or edit it.

This boundary is deterministic where it needs to be. The evidence report has
stable candidate ids, paths, headings, and ordering, and the decision's
`nodes:` list comes from a resolved blueprint node rather than a model's guess.
The model can vary the explanation while the facts and artefact lifecycle stay
visible. Maintenance is bounded: Rust owns the command's evidence contract
and tests, the existing Cairn-dev reference owns the authoring instructions,
and the existing decision writer owns frontmatter and safe file creation.

The hybrid does require a shipped pack reference, but it does not invent a
new skill. The reference still changes canonical pack content under the
allowlisted `tools/agent-pack/content/` path; its adapter registration,
manifest, generated mirror, and compiled asset are distribution surfaces that
allowlist governs through that path. The combined mechanism is binding under
`dec.decision-ratification-tiers`. A panel may still accept a convergent
binding decision under `dec.reviewer-panel-ratification`; this draft does not
write receipts or self-accept.

**Result:** choose the hybrid with reference-hosted authoring.

## Deterministic evidence contract

The selected commands are `cairn onboard decisions` and
`cairn onboard decisions --json`, not a new `cairn brownfield extract` noun
and not an `init --from-code` mode. The default form emits human output and
`--json` emits the stable machine index. No `cairn brownfield` noun exists
today. Their observable contract for `todo.brownfield-extraction-flow` is:

- retain the current `cairn onboard` scanner-backed project loading and
  orphan-report behavior;
- use the loaded graph for node resolution and deterministic discovery facts
  only as bounded source evidence, never as an unbound replacement for a
  blueprint node;
- resolve every evidence path or code target by normalizing it relative to
  the project root and applying the existing reconciler ownership rule:
  eligible leaf or `owns-files` nodes contribute normalized declared paths,
  most-specific paths win, and `map::paths::is_component_prefix` selects the
  owner. Check that owner id in the loaded graph before emitting it in
  `nodes:`; a path-derived candidate id is never a graph node id;
- inspect only this closed set of ADR-like locations and comment forms:
  files under `docs/adr/` and `docs/decisions/`, README sections headed
  `Decision`, `Rationale`, or `Invariant`, and source comments carrying the
  literal `// invariant:` or `# invariant:` markers. Record explicit paths
  and headings in the report;
- emit candidates and evidence in stable path and id order;
- report unresolved or unbound evidence instead of manufacturing a `nodes:`
  value;
- require an existing onboarded `cairn.blueprint`, rather than accepting the
  temporary stub that the current no-blueprint onboard path synthesises;
- expose human output by default and stable JSON for the harness;
- return code 2 with the literal `usage: ` prefix followed by the supported
  `help.commands.onboard.usage` value for an unrecognised positional
  subcommand, instead of silently rendering the orphan report;
- never call a model, write `status: accepted`, or treat a prose guess as a
  graph fact.

The Cairn-dev reference consumes this report and may write a
`meta/research/<slug>.md` primary research artefact containing the evidence
selection and interpretation. It then invokes `cairn decision new` for each
draft. The reference may set `informed_by`, `revisit_triggers`, and
`ratification`, or omit those optional fields. Omission of `ratification`
means `binding`. No standalone extraction skill is added.

## Why the existing surfaces win

`onboard` is the right deterministic host because it is already a
brownfield-specific, read-only analysis report over scanner findings. It can
add a subcommand without creating a second top-level namespace, and its JSON
renderer gives the harness a stable hand-off point. `init --from-code` is not
selected because it creates a `meta/changes/brownfield-init` proposal and
blueprint delta, supports `--apply`, and is intentionally coupled to map
bootstrap and contract stubs. Adding semantic decision extraction there would
mix a proposal-application lifecycle with a proposal-only decision flow and
would make the first-run command carry two unrelated outputs.

`cairn decision new` wins the artefact-writing question because its
`decision_stub` owns typed frontmatter and proposal sections, while its
kind-agnostic `write_new_artefact` helper owns directory creation,
overwrite-refusal, and byte writing. `cairn gap` loses because its `gap: true`
marker and unresolved-gap finding describe a missing answer during
implementation, not a mined decision with evidence.

The authoring instructions belong under `cairn-dev` because the accepted
workflow-consolidation rule requires marginal-lift evidence before a new skill
is promoted. The existing pack assets already ship Cairn-dev references, so
reference-hosting provides the needed authoring surface without adding an
unjustified standalone asset.

## Recommendation

Choose candidate D: extend `cairn onboard` with the deterministic
`decisions` subcommand, host the harness-authored prose in
`tools/agent-pack/content/skills/cairn-dev/references/task-brownfield-decision-extraction.md`,
and reuse `cairn decision new` as the proposal writer. This is a hybrid, not a
hedge between candidates. Cairn owns reproducible scanner-backed evidence and
node bindings; the existing Cairn-dev reference owns the authoring
instructions; the harness owns interpretation and prose; the maintainer or
the ratification panel owns acceptance.
