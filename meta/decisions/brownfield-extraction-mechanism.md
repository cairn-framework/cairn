---
id: dec.brownfield-extraction-mechanism
nodes:
  - cairn.brownfield
status: proposed
ratification: binding
date: 2026-08-08
informed_by:
  - res.brownfield-extraction-mechanism.comparison
related:
  - dec.build-and-extension
  - dec.decision-ratification-tiers
  - dec.reviewer-panel-ratification
revisit_triggers:
  - "Cairn gains a first-party inference backend with a deterministic, reviewable contract for decision prose"
  - "the onboard evidence report misses a material ADR-like location or produces an unbound node claim in external validation"
  - "the agent pack stops shipping harness skills through the current canonical and adapter surfaces"
---

# Brownfield decision extraction uses a deterministic onboard index and a shipped authoring skill

## Status

This is a proposed ruling for the Decision governance panel. It deliberately
contains no acceptance, receipt, or machine-ratification marker.

## Context

Brownfield discovery already gives Cairn a deterministic structural starting
point. `src/brownfield/discovery.rs` walks bounded source directories, derives
path-based candidate ids, records evidence paths, and derives only import edges
that the code proves. It does not inspect ADR-like documents or infer which
prose expresses a decision.

The existing CLI has two relevant writing and analysis surfaces. `cairn
onboard` reports deterministic orphan-file clusters and node or ignore
suggestions. `cairn decision new` validates a slug and writes a proposal
scaffold with the required decision frontmatter and standard sections. `cairn
gap` writes a different `gap: true` decision for an unresolved implementation
question and leaves an unresolved-gap finding until that question is answered.
The brownfield `init --from-code` family writes a change proposal, blueprint
delta, and contract stubs for map bootstrap; it is not a decision-drafting
lifecycle.

The verified constraint is hard: Cairn performs no LLM inference of its own on
this path. The summariser delegates to an external local command when enabled,
is disabled by default, and has no concrete hosted backend. The agent may
interpret evidence and write narrative, but the deterministic Cairn layer must
remain mechanically checkable.

## Decision

Choose the hybrid mechanism: extend the existing `cairn onboard` surface with
a deterministic decision-evidence subcommand, and ship a harness skill that
writes the decision prose from that evidence.

### 1. Deterministic Cairn surface

The implementation unit `todo.brownfield-extraction-flow` will add exactly
this command surface:

```
cairn onboard decisions --json
```

The command reuses the bounded candidate and evidence facts from
`src/brownfield/discovery.rs`, adds a deterministic index of the flow's
explicit ADR-like locations and invariant comment forms, resolves candidate
ids against the user's blueprint, and emits stable JSON for the harness plus a
human-readable report. It reports evidence that has no node binding instead of
inventing one. It does not call a model, draft narrative, write
`status: accepted`, or mutate the blueprint.

This is an extension of `cairn onboard`, not a new `cairn brownfield extract`
noun. The existing surface is already a brownfield-specific, read-only report
with human and JSON renderers. The implementation must keep its existing
orphan-cluster behavior unchanged while adding the explicit `decisions`
subcommand.

### 2. Harness authoring surface

The implementation unit will ship this skill as canonical pack content:

```
tools/agent-pack/content/skills/cairn-brownfield-decision-extraction/SKILL.md
```

The normal pack manifest and adapter destinations install it under the active
harness pack root. The skill invokes `cairn onboard decisions --json`, asks the
harness agent to interpret the returned code and document evidence, records
the selected evidence in a primary research artefact, and writes the decision
body. It must preserve the evidence paths and resolved node ids in the draft
and must leave every extracted decision at `status: proposed`.

### 3. Existing artefact-writing entry point

For each selected decision, the skill reuses the existing command:

```
cairn decision new <slug> --node <id> --informed-by <research-id>
```

`src/cli/commands/decision.rs` and its shared `write_new_artefact` helper own
slug validation, slug-only filenames, typed frontmatter, proposal status, and
overwrite refusal. The skill edits only the generated body and any permitted
provenance fields after the command creates the file. It does not introduce a
second decision writer. It does not use `cairn gap`, because an extracted
decision is not an unresolved implementation question and must not create a
`CAIRN_GAP_UNRESOLVED` finding.

### 4. Ratification tier

The ruling is **binding**. The command half alone would fit the local shape of
`dec.decision-ratification-tiers`: it is anchored to one brownfield container,
supersedes nothing, and can avoid the binding-surface allowlist. The selected
mechanism also adds shipped agent-pack content, however, and shipped pack
content is binding by that decision. The pack skill therefore makes this
combined ruling binding even though its command half is local-shaped.

This draft stays `status: proposed` until the Decision governance panel runs.
Under `dec.reviewer-panel-ratification`, a convergent binding ruling may be
accepted on convergent panel receipts, while a contested clause needs the
recorded debate or maintainer path. This unit writes no receipts and does not
self-ratify.

## Rejected alternatives

### Shipped skill alone

Rejected as the complete mechanism. A skill can write useful prose, but if it
also owns repository walking and evidence selection, the evidence boundary
becomes prompt and harness dependent. It would not provide a stable report of
candidate ids, paths, headings, and node bindings that later runs can compare.
The skill is retained as the authoring half of the chosen hybrid.

### New deterministic `cairn brownfield extract` command

Rejected as the command host. A new noun would duplicate the existing
brownfield command family and add another dispatch, help, copy, JSON, and test
surface. More importantly, a deterministic command alone cannot infer
prose-level decisions. It would either emit a sterile template or violate the
hard no-inference boundary. The evidence operation belongs on `cairn onboard`
where an existing brownfield report already provides the read-only hand-off.

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
  candidate ids, bounded source evidence, recognised ADR-like paths and
  headings, node resolution, output ordering, and proposal-only status.
- The harness can perform the semantic work Cairn cannot: deciding which
  evidence expresses an architectural decision and writing readable context,
  rationale, consequences, and revisit triggers. Model variation is visible in
  the proposed body rather than hidden in a graph mutation.
- The flow has one sanctioned decision writer. `cairn decision new` remains the
  source of frontmatter and safe file creation, while the skill fills prose
  and links the selected research evidence.
- The selected command extends an existing brownfield report instead of adding
  a top-level noun or coupling extraction to the `init --from-code` apply
  lifecycle. The existing onboard orphan report must remain compatible.
- The skill is shipped pack content. Its canonical bytes, pack manifest,
  adapter destinations, and compiled pack assets become part of the binding
  distribution surface and require the corresponding pack conformance gates.
- The decision is binding-tier even though the Cairn command alone could have
  been local-tier. Panel acceptance, if warranted by contestedness review,
  must follow `dec.reviewer-panel-ratification`; this draft does not claim that
  acceptance occurred.
- The flow still needs external-repository validation and a behaviour test in
  `todo.brownfield-extraction-flow`. A green scan proves artefact integrity,
  not that a model selected the right prose, so the flow must retain the
  evidence report and proposed artefact for maintainer review.

## Revisit triggers

- Cairn gains a first-party inference backend whose input, output, and
  determinism or review contract can be tested without moving prose authority
  into the reconciler.
- External validation shows that the deterministic onboard index misses a
  material ADR-like location, cannot preserve a real node binding, or produces
  too much unrelated evidence for an agent to review.
- The `onboard` surface acquires a different ownership or mutation contract,
  making a decision subcommand ambiguous or unsafe.
- The pack distribution model changes so the selected skill is no longer
  shipped through the canonical content and adapter destinations named above.
