# CAIRN Domain Expandability

## Status: done

## Question

> "with CAIRN, right now we doing for software engineering, but of course could do all sorts. Like planning organogram, mapping company process, job descriptions, tasks, roles responsibilites etc. assuming our cairn framework is easily expandable by its architecture."

## TL;DR

The kernel is genuinely domain-neutral by design: the blueprint grammar, artefact type system, two-chain model, change system, and query interface contain no software-specific assumptions and would survive a transplant to org modeling without structural changes. The reconciler interface is explicitly designed to accept non-code implementations. However, the only reconciler that exists today is the code reconciler, which is deeply software-shaped. The UI vocabulary (MODULE, CONTRACT, synced/ghost/orphaned, `path` as filesystem path) uses software engineering framing throughout. The spec names non-code domain extension as a Phase 10 capability, meaning it is Declared but not yet Designed. Someone trying to model a 200-person company in CAIRN today would succeed at the blueprint and artefact layers and hit a hard wall at reconciliation, where CAIRN's core enforcement value proposition lives.

## What the spec actually says

CLAUDE.md states: "Extending beyond code to non-code domains (orgs, research, processes) is in-scope for future phases." That line is the strategic intent. Its precise meaning under the spec's own maturity model (`docs/spec.md` §0.1) is that this is a Declared capability: purpose and rough shape are described, but no interface, schema, or behaviour is yet specified. It is not Designed and not Implemented.

`docs/spec.md` §0 Vocabulary defines the reality layer as: "whatever the framework reconciles against. For v1 this is source code; the reconciler interface allows others (org structure, product BOMs, research programmes) as later additions." The parenthetical examples are not accidental; they reflect genuine design intent. But they appear only in the vocabulary definition and in Phase 10, not in any specified interface.

`docs/spec.md` §6 states: "The kernel is generic; the reconcilers and artefact types it supports are configured separately." The reconciler interface is defined as "a domain-agnostic contract for 'given a node, produce a fingerprint of its current state and a list of claimed sub-elements.'" That definition contains no code-specific assumptions.

`docs/spec.md` §14 Phase 10 lists: "Additional reconcilers for non-code domains (org structure, product BOMs, research programmes)." This is the final named phase, landing after the kernel, full artefact system, change system, hooks, edge validation, multi-target, MCP, summariser, and brownfield phases are all complete.

The gap between stated goal and current support: the kernel is architecturally ready for non-code extension; no non-code reconciler exists; the spec has not yet designed what one would look like.

## Kernel artefact types: domain-neutral or software-flavored?

**Contract (authority): `would need extension`.**
The default schema ships with sections "Purpose", "Public interface", "Invariants", "Dependencies", "Tests" — software engineering vocabulary. However, the contract is a human-authored markdown file whose schema is declared in project config, not hardcoded in the kernel (`docs/spec.md` §6 project config block shows `artefact_types` as a configurable key with "Overrides and additions to the v1 defaults"). A project could declare a contract schema for a role description with sections "Accountabilities", "Reporting line", "Measures of success". The integrity rule (interface hash drift triggers contradiction) would not apply without a reconciler that computes hashes from org reality. The container is domain-neutral; the default v1 schema is software-shaped.

**Todo (authority): `already supports it`.**
Schema is generic: node reference, status (open/in_progress/done/blocked), created date, optional `satisfies` pointer. Nothing assumes code. A todo on a job-description node with `satisfies: "Accountability > Budget approval"` parses and validates correctly today.

**Decision / ADR (provenance + authority): `already supports it`.**
Schema fields (id, nodes, status, date, revisited, revisit_triggers, informed_by, supersedes, refines, related) assume nothing about the domain. A decision about org structure citing interview research and a source document fits the schema without modification. The hinge semantics (provenance upward, authority downward) work equally well for "why we structured the engineering org this way" as for "why we chose this crypto library."

**Review (authority): `already supports it`.**
Three subtypes: human, agent_introspective, agent_cross_model. All generic. A human review of an org restructuring proposal fits without modification.

**Research (provenance): `already supports it`.**
Schema (id, nodes, date, sources, tags, question, synthesis sections) is fully generic. Research into compensation benchmarks feeding a decision about salary bands is structurally identical to research into cryptography feeding a decision about shared modules.

**Source (provenance): `already supports it`.**
Schema handles document, interview, transcript, link, research_paper, conversation source types. An interview with a department head as a source for org design research is exactly what this type was built for. The verification states (verified/external/unverified) are generic to any source material.

**Summary:** Four of six artefact types are domain-neutral today. Contract requires a schema extension for non-code contexts, but the extension mechanism exists in project config. Todo is borderline only because its `satisfies` field assumes a contract exists to point at, which remains valid if non-code projects also use contracts (and they should).

## Authority chain: where is the software assumption baked in?

The authority chain is: decision, blueprint, contract, code. The leaf is "code." That is the load-bearing software assumption.

`docs/spec.md` §3.2: "Contracts constrain what code can do. Every link is mechanically checkable." The mechanical checkability of the final link depends entirely on the code reconciler. The reconciler walks the filesystem, parses source via Tree-sitter, extracts symbols and signatures, and computes an interface hash.

If the leaf authority were "role description" instead of "code", the chain becomes: decision, blueprint, contract, role description. The contract prescribes accountabilities and reporting lines. A reconciler reads the org's actual structure and computes a fingerprint. The chain topology is identical. What changes is the reconciler implementation, not the chain structure.

The blueprint grammar itself contains no software assumption. System, Container, Module, Actor are named for C4 software architecture but their semantics are purely structural: nodes with stable IDs, tags, edges, and artefact pointers. A Module named "SalesOrg" with a path pointing to a structured data file would parse and resolve correctly. The naming is software-flavored; the graph semantics are domain-neutral.

Verdict: `would need extension`. The authority chain topology is domain-neutral. The leaf assumption ("code") is load-bearing in the reconciler, not in the kernel. Replacing it requires a new reconciler, not a kernel redesign.

## Reconciler: what does it reconcile against?

Today's code reconciler does six things that are all software-specific:

1. Walks a filesystem path and finds source files.
2. Parses them via Tree-sitter.
3. Extracts symbols and function signatures.
4. Computes an interface hash from the extracted interface surface.
5. Detects intra-module imports and calls for edge validation.
6. Parses Rust `//!` comment blocks looking for `Cairn-ID:`, `Cairn-Depends:`, `Cairn-Tags:`, `Cairn-Contract:` fact lines for docstring drift detection. (`openspec/specs/reconciliation/spec.md`)

Items 1-4 would be replaced wholesale for a non-code domain. Item 5 could be reconceived as "does the org chart confirm the declared reporting relationship?" Item 6 has no analogue in org modeling.

The reconciler interface is defined abstractly: given a node, produce a fingerprint and a list of claimed sub-elements. An org reconciler implementing this interface would read a structured data file representing the org chart, extract roles within a declared scope, produce a fingerprint from role accountabilities and reporting line, and return claimed sub-roles. This is implementable against the existing interface contract. The interface is generic. The implementation does not yet exist.

The unresolved question: what is the reality layer format for non-code domains? For code it is filesystem paths to source files. For an org chart it must be a structured data format (YAML, JSON, CSV export from an HRIS) or a live system API. The spec does not specify this. It is the first design question that must be answered before a non-code reconciler can be built.

## What the UI/webui assumes

`src/ui_assets/assets/app.js` hardcodes the following software engineering vocabulary that would not survive a transplant to org modeling without changes:

- Node kind labels rendered in caps in SVG nodes: "SYSTEM", "CONTAINER", "MODULE", "ACTOR". An org modeler would want "DIVISION", "DEPARTMENT", "ROLE".
- Blueprint syntax highlighter keywords: `System`, `Container`, `Module`, `Actor`; keyword fields `path`, `contract`, `decisions`, `research`, `sources`, `todos`, `reviews`, `id`. The `path` field semantics (filesystem path to source directory) do not apply to non-code nodes.
- Query bar placeholder text: "search modules, containers, decisions".
- Inspector stat cells: hardcoded labels "decisions", "contracts", "todos", "research". The "contracts" label is software vocabulary.

These elements are domain-neutral and would survive the transplant unchanged:

- The chain-balance widget labels "Provenance" and "Authority" with "Hinge" midpoint. Fully domain-neutral; this is CAIRN's two-chain topology, not software vocabulary.
- Reconciliation state badges "synced", "ghost", "orphaned". "Ghost" (declared but not yet real) and "orphaned" (real but undeclared) are meaningful for any declared system, including org nodes.
- The breadcrumb default label "map". Neutral.
- The two-chain fill bars on each node card (left side = provenance strength, right side = authority strength). This concept transfers directly to any domain.

The structural conclusion: the UI's graph rendering, two-chain display, artefact inspector, and change-state display are all domain-neutral. The hardcoded vocabulary is limited to node kind labels, blueprint keyword highlighting, and a few label strings. A non-code domain needs configurable node kind labels and updated placeholder text; it does not need a new UI architecture.

## Hypothetical: modeling a 200-person company in CAIRN tomorrow

The three concrete obstacles in the order they appear:

**Obstacle 1: What does `path` point to?**

The blueprint grammar requires a `path` for leaf nodes that the code reconciler walks. For an org chart, there is no filesystem path to a "Marketing" department. A user would either omit `path` declarations (leaving all nodes ghost) or point `path` at a YAML file describing the department's roles. The scanner would run the code reconciler against that YAML, find no parseable source code, compute a null interface hash, and produce either orphaned nodes or a structural error.

The first concrete obstacle: no path convention exists for non-code nodes. The user must choose between (a) omitting paths and accepting all nodes are perpetually ghost, losing reconciliation value entirely, or (b) creating a structured data format and implementing a custom reconciler against an interface that is only Declared in the spec, meaning no design document exists yet to implement against.

**Obstacle 2: Contract schema has no org vocabulary.**

The v1 contract schema uses headings "Purpose", "Public interface", "Invariants", "Dependencies", "Tests". A user would author contracts with headings like "Accountabilities", "Reports to", "Direct reports", "Success measures". This is possible today since contract files are markdown and section headings are human conventions not mechanically enforced. A contract titled "Head of Sales" with custom headings parses and validates correctly. The blocker is future: when the summariser (Phase 8) proposes contract drafts, it uses a software-shaped prompt template. No org-domain prompt template exists. Not a today blocker, but it will surface the moment the summariser is enabled on a non-code project.

**Obstacle 3: No interface hash means no drift detection.**

This is the core value of CAIRN for software: when code drifts from its declared contract, the scanner detects drift and blocks commits. For an org chart today, there is no reconciler to compute an interface hash for any node. The scanner produces zero interface contradictions, not because the org is well-structured but because it cannot check. The map shows all nodes as ghost (no path) or synced-with-null-hash. Either way, the authority chain's mechanical enforcement property disappears entirely.

CAIRN degrades to a structured documentation system with change isolation and provenance tracking. That is still genuinely valuable: decisions are linked to research, changes are isolated and reviewable, the two-chain topology is visible. But it is not the product's primary claim. This is the single biggest obstacle. It does not block using CAIRN for org documentation today. It does block CAIRN from delivering its primary enforcement value for non-code domains.

## Comparison to getcairn.dev's domain rigidity

getcairn.dev is an MBSE platform for hardware systems engineering. Their domain shape is fixed: root system node, 4-7 subsystems, key interfaces, system brief. Their AI pipeline (Describe, Decompose, Inspect, Refine with 12 named lenses) is not pluggable. Their entity type hierarchy uses hardware-MBSE-specific ID schemes (SYS.01, SS.01, C.01.01). There is no reconciler interface; the AI pipeline is the reconciler and it is not extensible. (`docs/strongholds/external-cairn-docs-research.md`)

Their architecture is more domain-rigid than ours. A getcairn.dev project is always a hardware system with subsystems, interfaces, and requirements. You cannot redirect it to org modeling; you would get a hardware MBSE model with org-shaped labels attached.

Our architecture is less domain-rigid at the kernel layer but equally constrained at the implementation layer today. The difference is that our constraint lives in one swappable component (the code reconciler) rather than in the entire pipeline. Their twelve lenses, generation pipeline, and entity type system are all domain-specific and not abstracted behind any interface. Our blueprint grammar, artefact type system, two-chain model, change system, and query interface are domain-neutral.

The hypothesis "maybe ours is more flexible because we are not committed to any particular domain shape" is correct at the kernel design level and not yet proven at the implementation level. We have the right architecture for domain extension. We do not yet have the implementation. They have neither the architecture nor the implementation for domain extension. The gap between us and them is real and meaningful. The gap between our current state and genuine multi-domain operation is also real. Both things are true.

One further contrast: their provenance model ("Project Genesis, preserved as provenance") is a single QA-transcript snapshot taken before generation. Our provenance chain is a live directed graph. A project modeling org design reasoning in CAIRN would have a navigable chain from source interviews through research synthesis to decisions with formal downstream obligations. Their equivalent would be one frozen record. This is where our architecture is genuinely stronger even in a non-code domain context.

## Recommendations

**Recommendation 1: No spec change. Create an org-domain fixture.**

Create `test/fixtures/org-bootstrap/` as a worked example of CAIRN modeling a small fictional organization. Use System, Container, and Module nodes to represent Division, Department, and Role. Omit `path` declarations intentionally (all nodes ghost by design). Use the existing decision, research, and source artefact types to model an org design decision with full provenance chain. Use the change system to show an org restructuring as a change directory with a proposal.md and blueprint.delta.

This proves that the artefact layer and two-chain topology work for non-code domains today with no code changes. It produces a concrete artifact the user can reference. It surfaces the ghost-node limitation honestly in the fixture's README, which documents it as a known constraint pending Phase 10 reconciler work rather than a design flaw.

Effort: low (authoring only, one day). Risk: none to the codebase. The honest limitation surfacing is a feature: it clarifies exactly what Phase 10 needs to deliver.

**Recommendation 2: Small spec extension. Design the non-code reconciler interface.**

Promote the non-code reconciler from Declared to Designed maturity by specifying: (a) what format the reality layer takes for non-code domains (YAML in the repo is the obvious starting point); (b) what the reconciler interface must return when reading a structured data file rather than parsed source; (c) how `path` declarations behave when there is no filesystem source path (introduce a `data-file` field as an alternative to `path` for non-code nodes, or clarify that `path` for non-code nodes points to a structured data file rather than a source directory).

This requires no code change. It requires one spec design phase adding a new section to `docs/spec.md` specifying the non-code reconciler contract. Once this design exists, someone can implement an org reconciler without further spec work.

Effort: medium (one spec phase, no code). Risk: low, purely additive.

**Recommendation 3: Architectural decision required. Decide whether node kind vocabulary is project-configurable.**

The blueprint grammar uses System, Container, Module, Actor as fixed node kind names. The UI renders these hardcoded. For genuine multi-domain use, a project modeling an org would want different vocabulary. The decision: fixed or configurable?

If fixed: non-code domain users use software vocabulary for their nodes. This works technically but creates friction for non-developer users, at odds with CLAUDE.md's direction ("broadening from career developers to people building with AI tools, including non-devs").

If configurable: the grammar accepts a `node_kinds` config mapping the four structural positions (system/container/module/actor) to project-specific labels. The parser, CLI, and UI read labels from config. The spec must define which grammar semantics are tied to kind names versus structural position (nesting depth, path ownership rules, leaf vs internal node distinctions).

This is a real architectural decision. It touches the parser, config schema, CLI output, UI rendering, and spec grammar section simultaneously. It should be decided before Phase 10 non-code domain work begins, not during it. Do not fold it into Phase 10 without a dedicated prior design phase.

Effort: high. Risk: cross-cutting. The right time to decide is as a standalone spec design before Phase 10; the wrong time is under Phase 10 implementation pressure.

## Open questions

The spec is currently silent on the following; they require a user decision before the architecture can commit:

1. **What is the reality layer format for non-code domains?** YAML in the repo, live HRIS API calls, and structured markdown directories are all plausible. Each has different implications for reconciler design and `path` semantics. This decision gates the design work in Recommendation 2.

2. **What does "interface hash" mean for a non-code node?** For code, the hash is computed from extracted symbols and signatures. For a department, what constitutes "the interface"? Its reporting-to chain? Its headcount? Its set of named accountabilities? Until this is defined, drift detection for non-code domains cannot be specified, and the authority chain's enforcement property is unavailable for non-code projects.

3. **Are node kind labels configurable?** See Recommendation 3. This is a user decision. The technical implications follow from it.

4. **Is the conventions surface framework-level or project-level?** `openspec/conventions.md` is specific to CAIRN's own Rust implementation. The em-dash ban in CLAUDE.md is a CAIRN-project writing convention. A future org-modeling project using CAIRN would not inherit these through the framework; they set their own conventions in their `AGENTS.md` and `cairn.config.yaml` `context`/`rules` blocks. This separation is already architecturally correct. The spec should explicitly confirm it so users do not think they must adopt CAIRN's internal implementation conventions to use the framework.

5. **When does non-code reconciler work begin?** The spec places it at Phase 10, after brownfield extraction (Phase 9). Given Phases 8 and 9 are not yet implemented, this is several phases away. The org-bootstrap fixture (Recommendation 1) is the right near-term move if the user wants to validate multi-domain claims before Phase 10. The phase order is correct and should not be resequenced.
