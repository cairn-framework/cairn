---
id: dec.brownfield-extraction-mechanism
nodes: [cairn.brownfield]
status: proposed
ratification: binding
date: 2026-08-08
informed_by:
  - res.brownfield-extraction-mechanism.comparison
related:
  - dec.build-and-extension
  - dec.decision-ratification-tiers
  - dec.reviewer-panel-ratification
  - dec.cli-agent-workflow-consolidation
  - dec.agent-pack-packaging
  - dec.pack-adapter-roots
affects:
  - meta/decisions/brownfield-extraction-mechanism.md
  - meta/research/brownfield-extraction-mechanism.comparison.md
  - src/cli/commands/onboard.rs
  - src/cli/mod.rs
  - src/brownfield/onboard.rs
  - docs/commands.md
  - docs/integration-contract.md
  - docs/design-system/copy.toml
  - tools/agent-pack/content/skills/cairn-dev/SKILL.md
  - .claude/skills/cairn-dev/SKILL.md
  - tools/agent-pack/content/skills/cairn-dev/references/command-reference.md
  - .claude/skills/cairn-dev/references/command-reference.md
  - tools/agent-pack/manifest.toml
  - src/cli/commands/pack_assets.rs
  - tests/kernel.rs
  - .gitattributes
  - tools/agent-pack/tests/determinism_drift_tests.rs
  - tools/agent-pack/tests/router_route_tests.rs
revisit_triggers:
  - "Cairn gains a first-party inference backend with a deterministic, reviewable contract for decision prose"
  - "external validation shows that the deterministic onboard index misses a material ADR-like location, cannot preserve a real node binding, or produces too much unrelated evidence for an agent to review"
  - "the onboard surface acquires a different ownership or mutation contract, making a decision subcommand ambiguous or unsafe"
  - "the pack distribution model changes so the cairn-dev reference is no longer shipped through the current canonical and adapter surfaces"
---

# Brownfield decision extraction uses a deterministic onboard index and a cairn-dev authoring reference

## Status

This is a proposed ruling for the Decision governance panel. It deliberately
contains no acceptance, receipt, or machine-ratification marker.

## Context

Brownfield discovery already gives Cairn a deterministic structural starting
point. `src/brownfield/discovery.rs` walks bounded source directories, derives
path-based candidate ids, records evidence paths, and derives only import edges
that the code proves. It does not inspect ADR-like documents or infer which
prose expresses a decision.

The existing CLI has three relevant writing and analysis surfaces. `cairn
onboard` reports deterministic orphan-file clusters and node or ignore
suggestions. `cairn decision new` validates a slug and writes a proposal
scaffold with the required decision frontmatter and standard sections. `cairn
gap` writes a different `gap: true` decision for an unresolved implementation
question and leaves an unresolved-gap finding until that question is answered.
The brownfield CLI surfaces are `cairn onboard`, `cairn refine`, and
`cairn init --from-code`; no `cairn brownfield` noun exists. The
`init --from-code` family writes a change proposal, blueprint delta, and
contract stubs for map bootstrap, so it is not a decision-drafting lifecycle.

The verified constraint is hard: Cairn performs no LLM inference of its own on
this path. The summariser delegates to an external local command when enabled,
is disabled by default, and has no concrete hosted backend. The agent may
interpret evidence and write narrative, but the deterministic Cairn layer must
remain mechanically checkable.

## Decision

Choose the hybrid mechanism: extend the existing `cairn onboard` surface with
a deterministic decision-evidence subcommand, and host the harness authoring
instructions as a reference under the existing `cairn-dev` skill.

### 1. Deterministic Cairn surface

The implementation unit `todo.brownfield-extraction-flow` will add this
command surface:

```
cairn onboard decisions
cairn onboard decisions --json
```

The default `decisions` form emits the human-readable evidence report; adding
`--json` emits the stable machine-readable index for the harness. The existing
no-subcommand `cairn onboard` path loads the project through the
scanner and passes its findings to `brownfield::onboard::analyze`, which
groups `CAIRN_RECONCILE_ORPHANED_FILE` paths. That path reads scanner findings,
not `src/brownfield/discovery.rs` directly. The new `decisions` branch keeps
the scanner-loaded graph as its source of real node bindings and uses the
deterministic discovery facts as bounded code evidence.

The binding rule is path-to-blueprint, never candidate-id-to-blueprint. For
each discovery code target or ADR-like evidence path, the branch normalizes
the path relative to the project root and resolves it against the loaded
blueprint's declared ownership. The existing reconciler's
`eligible_owners` and `most_specific_owner` helpers in
`src/reconcile/generic.rs` are private, so the flow unit will reimplement the
same most-specific-prefix rule in its onboard resolver and add a parity test
against the existing reconciler fixture expectations. Eligible leaf or
`owns-files` nodes contribute normalized declared paths, those paths are
considered most-specific first, and `map::paths::is_component_prefix` selects
the first matching owner. The onboard report validates that the resulting
owner id exists in the loaded graph before emitting a bound candidate. A
path-derived discovery candidate id is evidence only and is never equated
with a blueprint node id. If no owner matches, the report records unbound
evidence and does not invent a node binding.

The flow requires an onboarded `cairn.blueprint` to exist and load
successfully. The current onboard command synthesises a temporary stub
blueprint when its requested file is absent; the `decisions` branch must fail
with a clear error instead, because a draft without a real `nodes:` binding
does not meet this flow's contract.

The branch adds a deterministic index of a closed set of evidence sources:
files under `docs/adr/` and `docs/decisions/`, README sections headed
`Decision`, `Rationale`, or `Invariant`, and source comments carrying the
literal `// invariant:` or `# invariant:` markers. It applies the
path-to-blueprint binding rule above and emits stable JSON for the harness
plus the default human-readable report. It reports evidence that has no node
binding instead of inventing one. It does not scan arbitrary prose, call a
model, draft narrative, write `status: accepted`, or mutate the blueprint.

With no subcommand, the existing orphan report remains unchanged. `decisions`
selects the evidence index. Any other positional subcommand must return exit
code 2 with the literal error text
`usage: cairn onboard [decisions] [options]`, using the supported usage value
from `copy::lookup("help.commands.onboard.usage")`; it must not silently fall
back to the orphan report, as the current `run_onboard_command` does by
ignoring `command_args`. The `usage` and `args` values in
`docs/design-system/copy.toml` must name the supported form,
`cairn onboard [decisions] [options]`, and explain that omitting `decisions`
keeps the orphan report. The usage copy value remains unprefixed for the help
renderer; only the error path adds the literal `usage: ` prefix. These
`help.commands.onboard.usage` and `help.commands.onboard.args` values must not
become hardcoded parallel copy surfaces.

This is an extension of `cairn onboard`. There is no existing
`cairn brownfield extract` command to preserve or extend, and this ruling does
not add that new top-level noun.

### 2. Cairn-dev authoring reference

The implementation unit will add the authoring instructions as this reference
under the existing owning skill:

```
tools/agent-pack/content/skills/cairn-dev/references/task-brownfield-decision-extraction.md
```

The checked-in `.claude` mirror at
`.claude/skills/cairn-dev/references/task-brownfield-decision-extraction.md`
is an input to `src/cli/commands/pack_assets.rs`: the `BASE_ASSETS` table
uses `include_str!` for each Cairn-dev reference, and `all_assets` rewrites
the adapter root for the `.omp` destination. This is ordinary cairn-dev
guidance, not loop-mode procedure, so it belongs in `BASE_ASSETS`, not
`LOOP_ASSETS`; `LOOP_ASSETS` is opt-in and reserved for the loop reference and
its required closure. The implementation must add the reference to
`tools/agent-pack/manifest.toml` as a canonical entry and add new Claude and
OMP adapter rows. It therefore ships through the same owning skill path, not
as a new standalone skill.

Both existing router files, `tools/agent-pack/content/skills/cairn-dev/SKILL.md`
and `.claude/skills/cairn-dev/SKILL.md`, must add this exact route row:

| Mine an existing codebase into proposed decisions | `references/task-brownfield-decision-extraction.md` |

The canonical and `.claude` router rows must remain byte-identical. The
existing shipped Cairn-dev `references/command-reference.md` in both the
canonical and `.claude` trees must also describe `cairn onboard
decisions --json` and the unchanged no-subcommand orphan report.

The manifest additions also invalidate the generated-file marker in
`.gitattributes`, the size-pinned `EXPECTED_CANONICAL` and `EXPECTED_CLAUDE`
arrays in `tools/agent-pack/tests/determinism_drift_tests.rs`, and the route
reachability checks in `tools/agent-pack/tests/router_route_tests.rs`.

Clause 1 also invalidates the onboarding surfaces in
`src/cli/commands/onboard.rs`, the `src/cli/mod.rs` onboard description and
help dispatch, the Brownfield onboarding rows in `docs/commands.md` and
`docs/integration-contract.md:88`, the
`help.commands.onboard.usage` and `help.commands.onboard.args` values in
`docs/design-system/copy.toml`, and the onboard behaviour coverage in
`tests/kernel.rs`. Those surfaces must name the supported `decisions` form
rather than carrying stale or parallel command text.

The reference invokes `cairn onboard decisions --json`, asks the harness agent
to interpret the returned code and document evidence, records the selected
evidence in a primary research artefact, and writes the decision body. The
onboard report validates the binding; the `cairn decision new` writer does not
re-resolve or validate graph ownership. The reference must preserve the
report's evidence paths and resolved node ids in the draft and must leave
every extracted decision at `status: proposed`.


### 3. Existing artefact-writing entry point

For each selected decision, the authoring reference reuses the existing
command:

```
cairn decision new <slug> --node <id> --informed-by <research-id>
```

`run_decision_new` validates the slug and combines two existing helpers.
`decision_stub` owns the typed decision frontmatter, `status: proposed`, date,
and standard decision sections. The shared `write_new_artefact` helper is
kind-agnostic: it creates the directory, refuses an existing target, and
writes the supplied bytes. It does not own typed frontmatter or proposal
status. The reference edits only the generated body and permitted provenance
fields after the command creates the file. It does not introduce a second
decision writer.

The authoring reference may set these fields on a generated draft:

- `informed_by`, pointing to the primary research artefact selected from the
  evidence report;
- `revisit_triggers`, carrying queryable conditions for reconsideration;
- `ratification: local` or `ratification: binding`, or no `ratification`
  field at all.

`decision_stub` currently emits no `ratification` field. The registry parser
defaults an absent value to `binding` (`src/artefacts/registry/kinds.rs:138-140`),
so extracted drafts carry `binding` by default. An explicit `local` value is a
claim subject to the full tier shape rules, not a shortcut around them. The
flow reference must not set `status: accepted`, `ratified_by`, `receipts`, or
`supersedes`. It does not use `cairn gap`, because an extracted decision is not
an unresolved implementation question and must not create a
`CAIRN_GAP_UNRESOLVED` finding.

### 4. Ratification tier

The ruling is **binding**. The selected command is anchored to
`cairn.brownfield`, supersedes nothing, and its command paths are outside the
binding registry. The only selected implementation path
inside that registry is `tools/agent-pack/content/`, the exact directory row
in `docs/registries/binding-surface.md:7`, because the authoring reference is
shipped under the existing Cairn-dev skill. The manifest, adapter-root,
compiled-asset, and generated-mirror files are distribution surfaces the
allowlist governs via `tools/agent-pack/content/`; they are not themselves
binding-registry paths. The pack content is binding by
`dec.decision-ratification-tiers`, so reference-hosting does not turn this
combined ruling into a local decision. Extracted drafts also default to
`binding` because `decision_stub` omits `ratification` and the registry default
is binding.

This draft stays `status: proposed` until the Decision governance panel runs.
Under `dec.reviewer-panel-ratification`, a convergent binding ruling may be
accepted on convergent panel receipts, while a contested clause needs the
recorded debate or maintainer path. This unit writes no receipts and does not
self-ratify.


## For

A standalone `cairn-brownfield-decision-extraction` skill would make the
workflow directly discoverable to an agent that wants to mine decisions. Its
name could provide a clear trigger without requiring the harness to search
inside `cairn-dev`.

## Against

Accepted `dec.cli-agent-workflow-consolidation` states that future pack
promotions are judged on marginal lift over the current pack and that
non-overlapping value must be merged into the owning skill before a new skill
is added. The accepted consolidation rule governs this choice under
accepted-decision precedence. No marginal-lift judgement was recorded for a
standalone extraction skill, while a new skill would add another
distribution surface governed by the allowlist via
`tools/agent-pack/content/`.

## Verdict

**For:** standalone trigger discoverability.
**Against:** the accepted consolidation rule directly governs.
**Verdict:** choose reference-hosting; future recorded marginal-lift judgement
remains the sanctioned promotion path.

This is the proposed recommendation for the scheduled Decision governance
panel. It asserts no panel outcome, acceptance, or receipt.

## Rejected alternatives

### Shipped standalone skill alone

Rejected as the complete mechanism and as a new distribution surface. A
standalone skill can write useful prose, but if it also owns repository walking
and evidence selection, the evidence boundary becomes prompt and harness
dependent. It would not provide a stable report of candidate ids, paths,
headings, and node bindings that later runs can compare. The authoring
instructions are retained as a reference under `cairn-dev`, where the accepted
pack-consolidation rule places non-overlapping value.

### New deterministic top-level command

Rejected as the command host. A new noun would duplicate the existing
brownfield surfaces and add another dispatch, help, copy, JSON, and test
surface. The real differentiator is ownership and marginal lift: `onboard`
already owns a brownfield read-only report with human and JSON renderers, so
its `decisions` subcommand gives the harness a hand-off point without a
second top-level namespace. A new noun has no recorded marginal lift over that
existing owner. The prose limitation is handled by the authoring reference in
both designs and is not the reason this command host loses.

### `cairn onboard` extension alone

Rejected as the complete mechanism. The current onboard analysis groups
orphan findings and produces node or ignore suggestions; even with a
candidate-evidence index it has no authoring model and no decision prose
lifecycle. The extension is selected as the deterministic half, not as the
whole product.

### `init --from-code` extension

Rejected as the host for this flow. `init --from-code` calls discovery and
writes `meta/changes/brownfield-init` with a proposal, blueprint delta, and
contract stubs, while `--apply` can archive that map change. Mixing decision
mining into that map bootstrap would couple proposal-only decision drafting to
blueprint application and give first-run output two unrelated lifecycles.

### `cairn gap`

Rejected as the artefact writer. `gap` is intentionally reserved for a genuine
underspecification encountered during implementation and marks its decision
with `gap: true`; open gaps remain lint findings until accepted or deleted.
That lifecycle does not represent evidence-backed extraction and would make a
normal draft look unresolved.

## Consequences

- Brownfield extraction has a reproducible evidence boundary. Cairn owns
  scanner-backed graph resolution, candidate ids, bounded source evidence,
  recognised ADR-like paths and headings, node bindings, output ordering, and
  proposal-only status.
- The harness can perform the semantic work Cairn cannot: deciding which
  evidence expresses an architectural decision and writing readable context,
  rationale, consequences, and revisit triggers. Model variation is visible in
  the proposed body rather than hidden in a graph mutation.
- The flow has one sanctioned decision writer. `cairn decision new` remains the
  source of proposal bytes and safe file creation, while the reference fills
  prose and links the selected research evidence.
- The command extension has an explicit blueprint precondition and explicit
  unknown-subcommand error. The existing `cairn onboard` orphan report remains
  compatible for the no-subcommand form, while the `src/cli/mod.rs`
  description and dispatch, `docs/commands.md` onboarding row,
  `docs/design-system/copy.toml` usage and args, and both shipped Cairn-dev
  `command-reference.md` copies must describe the supported subcommand.
- The authoring reference is shipped under `cairn-dev`. Its canonical content
  under `tools/agent-pack/content/` is the binding-registry path. The router
  files, command-reference copies, `.claude` mirror, pack manifest rows,
  adapter-root transformation, and compiled pack asset are distribution
  surfaces governed by that allowlist via the canonical content path, and
  require the corresponding pack conformance gates.
- Extracted drafts carry `binding` by default. An explicit `ratification`
  value is allowed only when the registry shape rules support it; no draft is
  accepted by the flow.
- The decision is binding-tier because the selected hybrid changes shipped
  Cairn-dev pack content under `tools/agent-pack/content/`, which is protected
  by the binding-surface allowlist. The command-only half could be local-tier
  only if its `affects:` list stayed wholly outside that allowlist; this
  combined mechanism does not. Panel acceptance, if warranted by contestedness
  review, must follow `dec.reviewer-panel-ratification`; this draft does not
  claim that acceptance occurred.
- The flow still needs external-repository validation and a behaviour test in
  `todo.brownfield-extraction-flow`. A green scan proves artefact integrity,
  not that a model selected the right prose, so the flow must retain the
  evidence report and proposed artefact for maintainer review.

## Revisit triggers

See the `revisit_triggers` list in this artefact's frontmatter; it is the sole source for reconsideration conditions.
