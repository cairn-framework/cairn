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
  directory only after it has at least three source files, stops below depth
  four, skips directories such as `target`, `node_modules`, `.git`, `meta`,
  `dist`, and `build`, derives path-based candidate ids, records sorted
  evidence paths, and derives directed edges from imports that it can observe.
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
- `src/cli/commands/onboard.rs` currently loads a scanner result and passes its
  findings to `brownfield::onboard::analyze`. `src/brownfield/onboard.rs`
  groups orphaned files by parent directory and deterministically classifies
  each group as an ignore or node suggestion, with human and JSON renderers.
  This is already a read-only brownfield analysis surface, but it currently
  sees scanner orphan findings rather than ADR-like documents.
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
- `dec.decision-ratification-tiers` makes a decision binding when it changes a
  shipped pack, an artefact schema, a registry, a spec invariant, or another
  protected surface. A single-container brownfield command with no
  supersession could otherwise fit the local shape. The same decision says
  shipped pack content is binding. `dec.reviewer-panel-ratification` permits a
  convergent binding ruling to be accepted on panel receipts, while retaining
  the proposed state until that panel runs.

The command half must therefore own only facts that can be reproduced from the
same checkout: candidate ids, paths, bounded source evidence, recognised ADR
files and headings, and stable ordering. The harness agent must own the
interpretation of those facts and the prose of a decision. Neither half may
silently set `status: accepted`.

## Candidate comparison

### Candidate A: shipped agent skill as the whole mechanism

A skill such as
`tools/agent-pack/content/skills/cairn-brownfield-decision-extraction/SKILL.md`
can ask the harness agent to inspect the repository, identify invariant prose,
and draft decisions. This is the only candidate that can write useful
narrative without adding model inference to Cairn. It also fits the existing
pack convention: canonical content is distributed through adapter-specific
pack roots, and a skill can invoke the existing `cairn decision new` command.

The determinism boundary is weak if the skill is the whole mechanism. File
walking, document selection, candidate ordering, and evidence citation would
be instructions interpreted by a model or by ad hoc shell commands. A changed
agent, prompt, or shell implementation can therefore change the evidence set
before the prose is written. The skill can be reviewed, but review is not a
stable evidence index that a later run can compare.

Maintenance is split across skill prose, pack manifests, adapter destinations,
and whatever commands the harness happens to use. Updating the skill requires
pack distribution work. Because pack content is a binding surface under
`dec.decision-ratification-tiers`, this candidate is binding-tier even when it
only changes one brownfield node. It has no Rust command schema to maintain,
but it still has a wide distribution surface and a model-dependent evidence
boundary.

**Result:** reject as the complete mechanism. Retain the skill as the prose
writer in the hybrid because that is its actual strength.

### Candidate B: a new deterministic Cairn command as the whole mechanism

A new top-level noun such as `cairn brownfield extract` could reuse
`discovery::discover`, add deterministic scans for ADR-like files, and emit a
candidate/evidence report. It could also validate each proposed node against
the loaded graph. A command-owned JSON contract would be reproducible and
would be easy for a harness to consume.

The command cannot infer that a paragraph expresses a decision, resolve
ambiguous ownership, or write maintainable decision prose. `discovery.rs`
proves structural facts, while the summariser only delegates to an external
backend and is disabled by default. A command that promises complete drafts
would either emit a sterile template or smuggle non-deterministic inference
into Rust, violating `docs/agent/principles.md` and the verified fact in the
todo.

A new noun also adds a dispatch branch, help and copy entries, JSON snapshots,
command documentation, and a second brownfield vocabulary beside the existing
`onboard`, `init --from-code`, and `refine` family. A command-only change on
`cairn.brownfield` could be local-tier if its affected paths stay outside the
binding allowlist, but that cheaper tier does not solve the prose problem.

**Result:** reject as the complete mechanism. The deterministic evidence
operation is needed, but it belongs on the existing `onboard` surface rather
than as a new top-level noun.

### Candidate C: extend `cairn onboard` as the whole mechanism

`cairn onboard` already owns a deterministic report over brownfield orphan
findings. Its report groups paths, sorts them through `BTreeMap` traversal,
classifies ignored directories, and renders both human and JSON forms. Adding
an `onboard decisions` subcommand is a smaller grammar and maintenance change
than adding a new top-level noun. The extension can reuse
`discovery::discover` for source candidates and add a bounded, sorted scan of
`docs/adr`, `docs/decisions`, README sections, and long-lived invariant
comments. Its output can index candidate ids, the evidence paths and headings,
and the code paths that support each candidate without claiming semantic
intent.

The existing `onboard` command still cannot draft the prose. Its current
`analyze` function consumes findings and returns classifications; it has no
model backend and no decision writer. It also cannot use `cairn gap` as a
substitute, because a gap is an unresolved implementation question with its
own warning lifecycle. As a pure onboard extension, the result would stop at
a report or a template and would not satisfy the value promised by the parent
todo.

A command-only onboard extension remains a single-container brownfield change
and could be local-tier under the tier rules. It has a narrower distribution
surface than a skill and can be exercised deterministically in the CLI tests.

**Result:** reject as the complete mechanism, and select it as the
 deterministic evidence half of the hybrid.

### Candidate D: hybrid deterministic evidence plus harness prose

The hybrid gives each layer one responsibility:

1. `cairn onboard decisions --json` extends the existing `onboard` surface.
   Cairn walks the repository using the existing discovery boundary, indexes
   the bounded ADR-like evidence needed by the flow, resolves candidate ids
   against the user's blueprint, and emits stable JSON and human output. It
   does not invent semantic decisions, call a model, or accept an artefact.
2. The shipped skill
   `tools/agent-pack/content/skills/cairn-brownfield-decision-extraction/SKILL.md`
   invokes that command, asks the harness agent to interpret the returned
   evidence, and records the research narrative and proposed decision body.
   The skill is an authoring aid, not an acceptance path.
3. For each selected decision, the skill reuses
   `cairn decision new <slug> --node <id> --informed-by <research-id>`.
   That command's existing writer creates the slug-only decision file and
   `status: proposed` frontmatter. The skill fills the body and leaves the
   maintainer to accept, reject, or edit it. `cairn gap` is not used for this
   normal extraction path.

This boundary is deterministic where it needs to be. The evidence report has
stable candidate ids, paths, headings, and ordering, and the decision's
`nodes:` list comes from a resolved blueprint node rather than a model's guess.
The model can vary the explanation while the facts and artefact lifecycle stay
visible. Maintenance is also bounded: Rust owns the command's evidence
contract and tests, the skill owns the authoring instructions, and the
existing decision writer owns frontmatter and safe file creation.

The hybrid does require two distribution surfaces. The canonical skill belongs
under `tools/agent-pack/content/skills/`, with adapter registrations and the
installed `.claude` or `.omp` destination following the existing pack
manifest. That pack change makes the ruling binding-tier. The command half
would be local-tier by itself, but the combined mechanism is binding under
`dec.decision-ratification-tiers`. A panel may still accept a convergent
binding decision under `dec.reviewer-panel-ratification`; this draft does not
write receipts or self-accept.

**Result:** choose the hybrid.

## Deterministic evidence contract

The selected command is `cairn onboard decisions --json`, not a new
`cairn brownfield extract` noun and not an `init --from-code` mode. Its
observable contract for `todo.brownfield-extraction-flow` is:

- reuse the path-derived candidate ids and bounded source evidence produced by
  `src/brownfield/discovery.rs`;
- inspect only the documented ADR-like locations and comment forms that the
  flow names, with explicit paths and headings in the report;
- emit candidates and evidence in stable path and id order;
- report unresolved or unbound evidence instead of manufacturing a `nodes:`
  value;
- expose JSON for the harness and human output for an operator;
- never call a model, write `status: accepted`, or treat a prose guess as a
  graph fact.

The shipped skill consumes this report and may write a
`meta/research/<slug>.md` primary research artefact containing the evidence
selection and interpretation. It then invokes `cairn decision new` for each
draft. No new Cairn-side decision writer is introduced.

## Why the existing surfaces win

`onboard` is the right deterministic host because it is already a
brownfield-specific, read-only analysis report over a loaded project. It can
add a subcommand without creating a second top-level namespace, and its JSON
renderer gives the harness a stable hand-off point. `init --from-code` is not
selected because it creates a `meta/changes/brownfield-init` proposal and
blueprint delta, supports `--apply`, and is intentionally coupled to map
bootstrap and contract stubs. Adding semantic decision extraction there would
mix a proposal-application lifecycle with a proposal-only decision flow and
would make the first-run command carry two unrelated outputs.

`cairn decision new` wins the artefact-writing question because it already
validates slugs, derives the typed id, writes the required frontmatter, refuses
overwrite, and starts the standard decision sections. `cairn gap` loses
because its `gap: true` marker and unresolved-gap finding describe a missing
answer during implementation, not a mined decision with evidence.

## Recommendation

Choose candidate D: extend `cairn onboard` with the deterministic
`decisions` subcommand, ship the
`cairn-brownfield-decision-extraction` skill for harness-authored prose, and
reuse `cairn decision new` as the proposal writer. This is a hybrid, not a
hedge between candidates. Cairn owns reproducible evidence and node bindings;
the harness owns interpretation and prose; the maintainer or the ratification
panel owns acceptance.
