# Cairn: Spec v0.7

> Working draft. The name *Cairn* is a placeholder; the spec will stand if it changes. v0.6 is a scope revision following a frame correction: capability scope and implementation phasing are now separate concerns. Several items previously labelled "v2" or "deferred" were genuinely in-scope capabilities whose implementation happens later in the build order; labelling them as out-of-scope was mistakenly conflating "not yet built" with "not part of the product." v0.6 brings those back in: brownfield extraction, multi-target paths, edge validation, docstring generation, a rename operation, and cross-model peer review as a review subtype. The kernel architecture from v0.5 stands unchanged.

## 0. Vocabulary

Used consistently throughout this spec:

- **The framework (Cairn)**: the toolchain. Parser, scanner, reconciler interface, artefact type system, hooks, CLI, query interface. What you install and run.
- **The blueprint**: the grammar, and by extension the `.blueprint` file a project authors. The intent layer.
- **The artefacts**: typed markdown files attached to nodes (contracts, todos, decisions, reviews, research, sources).
- **The map**: the runtime graph produced by reconciling the blueprint, the artefacts, and the scanned reality layer. The unified queryable layer.
- **The reality layer**: whatever the framework reconciles against. For v1 this is source code; the reconciler interface allows others (org structure, product BOMs, research programmes) as later additions.
- **A change**: a proposed modification to the current state, isolated in its own directory until merged. Changes never touch the main tree directly.

The framework is the tool. The blueprint is what you write. The map is what the tool produces. Changes are how you modify it safely.

## 0.1 Spec maturity levels

Not every part of this spec is at the same level of readiness. Three levels:

- **Declared.** We've decided this is in scope and named it. Purpose, role, and rough shape are described. Enough to plan around and to know what's coming, but not yet enough for someone to implement against.
- **Designed.** Interfaces, behaviour, integrity rules, and artefact schemas are specified. Someone can implement against this section without needing more spec work.
- **Implemented.** Code exists that satisfies the design. (Tracked in the build order and in the project's own Cairn state, not in the spec itself.)

Each section in this spec sits at one of these levels. Sections at Declared level are marked as such at the top. Sections without a marker are at Designed level: the default.

Declared-level sections exist because we know a capability matters and we know roughly where it fits, but specifying its details now would either invent constraints we don't yet have evidence for, or require knowledge we can only gain by implementing earlier phases first. Promoting a section from Declared to Designed is itself a spec change, usually prompted by the implementation of an earlier phase surfacing the information the design needs.

## 1. Problem

Good AI-assisted projects today follow a pattern: write a spec, have an agent implement it, then spend ongoing effort keeping the spec, the code, and the agent's understanding of both in sync. Four failure modes recur:

1. The spec drifts from the code. The spec describes a system that no longer exists.
2. The agent loses context. On long sessions or large repos, the agent forgets architectural intent and produces locally correct but globally wrong code.
3. Cross-cutting changes are unsafe. Touching one module silently breaks distant assumptions, because nothing in the toolchain knows what depends on what.
4. Project metadata fragments across tools. Specs in one place, todos in another, architecture decisions in a wiki, research in browser tabs. No single artefact gives an agent (or a human) the full picture of "what is this module, what work is outstanding on it, why was it built this way, what evidence supports that, what changed recently."

Cairn addresses all four by making the architectural map the spine of a unified map, with every other type of project metadata attached to nodes as typed artefacts. The reality layer is reconciled against the map continuously. Agents query the map instead of scanning the repo or hunting through tools.

## 2. Framing: map, not procedure

Cairn produces an *map* for a project: a map of what exists (modules, dependencies, contracts, decisions, research, sources) and how the pieces relate. It is not a procedure manual for how to build the project.

This distinction dictates what belongs in the map and what does not. Anything that describes *what the project is*, a module, its interface, its dependencies, its invariants, its open work items, the decisions that shaped it, the research that informed those decisions, goes in the graph. Anything that describes *how to operate on the project*, run the tests, format the code, deploy to staging, does not.

The practical consequence is a hard ceiling on the blueprint's complexity. If a feature being proposed is "how to do X," it belongs in an agent prompt, a skill, or a script. If it is "what X is and how it relates to Y," it belongs in the map.

## 3. The two chains: provenance and authority

v0.4 presented a single six-layer hierarchy running from raw sources through code. That framing conflated two different kinds of truth. A source is evidence; a contract is a norm. The framework cannot mechanically verify that research faithfully reflects a source (that requires reading the source and forming a judgment), but it *can* mechanically verify that code matches a contract. Presenting both under one word overclaimed enforcement.

v0.5 splits the stack into two chains that meet at the decision layer.

### 3.1 The provenance chain

**source → research → decision**

This chain is about traceability of reasoning. Sources are raw material (documents, interviews, transcripts, links). Research synthesizes sources into understanding. Decisions cite research (and sometimes sources directly) as justification.

The framework surfaces this chain and checks link integrity (every research artefact must cite at least one source, every decision must cite at least one piece of research or source), but it does not enforce *correctness*. Whether a research artefact faithfully represents its sources, and whether a decision is actually justified by its cited research, are human judgment calls. The framework exposes the chain so the judgment can be made; it does not attempt to make the judgment itself.

Issues in the provenance chain surface as **rationale tensions** (see section 10): advisory findings that warn but do not block.

### 3.2 The authority chain

**decision → blueprint → contract → code**

This chain is about enforcement. Decisions constrain what the blueprint should look like. The blueprint constrains what contracts can prescribe. Contracts constrain what code can do. Every link is mechanically checkable:

- Decisions can declare the blueprint nodes they apply to; the framework can then flag when a change to those nodes appears to violate the decision (v2 capability, deferred).
- The blueprint declares nodes, edges, and paths; the scanner verifies every declared path exists and every declared edge is realizable.
- Contracts prescribe module interfaces and invariants; the scanner computes interface hashes from real code and raises a contradiction when they drift.
- Code is reconciled against contracts by the reconciler registered for its domain.

Issues in the authority chain surface as **structural errors** or **interface contradictions** (see section 10): mechanical findings that block commits until resolved.

### 3.3 The hinge

The decision layer belongs to both chains. It is the output of provenance (what the evidence justifies) and the input to authority (what the system must satisfy). This is the translation point from "why we think this" to "what must be true."

Treating decisions as the hinge has a concrete consequence: decisions are the one artefact type that has obligations in both directions. Upward (provenance) they must cite their justification. Downward (authority) they declare which blueprint nodes they constrain. This is why the ADR schema carries both `informed_by` and `nodes` fields.

### 3.4 Current-state authority

The blueprint is the current architectural truth. Decisions are rationale plus commitments.

When an accepted decision has not yet been reflected in the blueprint, the current architecture is still what the blueprint says. The decision represents an obligation to change the blueprint, not a change that has already happened. An agent asking "what does the system look like today?" reads the blueprint; an agent asking "what are we committed to changing?" reads the active change directories (see section 9).

This is the simpler operational model and matches how software projects actually work. It also eliminates the ambiguity of v0.4, which implicitly placed decisions above the blueprint without specifying what that meant in practice.

The framework is a fence around the authority chain and a navigator for the provenance chain. It tells you when the fence has been crossed; it helps you trace why the fence was placed where it was. It does not decide which side is right.

### 3.5 Layer ordering of enforcement, configuration, and AI

Cairn is deterministic-typed at the bottom, configurable-templated in the middle, AI-assisted at the top, and the layer ordering is non-negotiable. Enforcement value lives in the kernel's deterministic, typed, two-chain primitives. Flexibility is delivered above the kernel via templates, tags, project config, and queued AI assistance.

- **Bottom: deterministic-typed.** Artefact types are obligation-bearing, not decorative. Each direct type's place in the provenance or authority chain determines what the kernel can enforce about it. Flattening the taxonomy collapses the obligations into labels, and labels are not enforceable.
- **Middle: configurable-templated.** Cairn's authoring guidance is template-driven and tag-extensible, not enum-bound. The kernel ships generic types; projects compose domain vocabulary on top via templates (per `artefact_types` in §6) and tags. A closed enum would constrain cairn's domain scope at the kernel layer; templates and tags do not.
- **Top: AI-assisted at authoring only.** Cairn extends to new domains by adding deterministic reconcilers, not by leaning on AI to normalise reality. The reality layer must produce a content-addressable fingerprint; without it, drift detection is impossible and the authority chain collapses to documentation. AI may propose edges, draft contracts, and suggest narrative summaries, all reviewable through the change-isolation primitive. AI may not produce the deterministic record itself.

## 4. Related work

**Structurizr blueprint** is the closest prior art for the architectural layer. It models C4 systems as a declarative blueprint with strict model/view separation. Cairn borrows the C4 hierarchy and the declarative style but drops the view layer (rendering is downstream), simplifies the grammar, and adds artefact pointers, path ownership, stable IDs, and reconciliation.

**Karpathy's LLM Wiki pattern** (April 2026) describes a persistent, LLM-maintained markdown wiki sitting between raw sources and an agent. The substrate is the same (markdown files in a git repo), but the direction is opposite: Karpathy's wiki is *descriptive*, the LLM compiles it from sources, inventing schema as it goes. Cairn is *prescriptive*: the human authors architectural intent upfront, and the framework reconciles reality against it. Cairn borrows Karpathy's three-layer framing (raw sources → compiled layer → code) and generalizes it into the two-chain model in section 3.

**akash-r34's llm-project-wiki** applies Karpathy's pattern to codebases. Wiki pages per file, diff-based ingest, gap logging. Remains descriptive where Cairn is prescriptive. The diff-based ingest mechanics transfer directly to Cairn's change-directory model.

**OpenSpec (Fission-AI)** is AI-native spec-driven development with a workflow system (OPSX), change directories for isolated proposals, and delta operations (`## ADDED`, `## MODIFIED`, `## REMOVED`, `## RENAMED`) for semantic merging. Cairn and OpenSpec solve different problems (OpenSpec is a change-lifecycle workflow, Cairn is a structural reconciliation framework), but OpenSpec's change-isolation and delta-merging patterns are directly applicable and are adopted in sections 9 and 12. Cairn deliberately does not adopt OpenSpec's workflow layer; the two tools are complementary and could coexist in the same repo.

**Dual-Graph / Codex-CLI-Compact** builds a bottom-up index of a codebase and exposes it to agents via MCP. Descriptive and code-level where Cairn is prescriptive and module-level. Its `graph_neighbors` tool inspired Cairn's neighbourhood query shape.

**dependency-cruiser, arch-unit, Nx module boundaries** are existing tools for enforcing architectural rules in CI. Narrower scope than Cairn, but prove the boundary-enforcement pattern. Cairn's integrity hooks are conceptually the same.

**ADR tooling** (adr-tools, log4brains) established the Architecture Decision Record format. Cairn adopts the format for the decision artefact type, extended with four frontmatter fields: `nodes` (which nodes the decision applies to), `revisited` (last review date), `revisit_triggers` (what would make this worth reconsidering), and `informed_by` (provenance).

**Map-first / context-engineering patterns** (as articulated by the dltHub team and others) frame LLM skills as intent + skill + map, where the gap is almost always the map. Cairn produces a map for a software project. The framing sharpens why the grammar refuses to absorb procedural detail: the framework gives the agent a map, not a recipe.

## 5. Goals and non-goals

**Goals**

- A single source of truth for project architecture, plans, decisions, and their provenance. Queryable by humans and agents.
- Module-level prescriptive contracts that the reality layer must conform to.
- Mechanical enforcement of structural and interface integrity.
- Advisory surfacing of rationale tensions in the provenance chain.
- Lean context delivery to agents via typed neighbourhood queries.
- A graph substrate that is agnostic to what reconciler populates the reality layer.
- An map small enough for a human to read in one sitting.
- Full provenance from a line of reality to the source material that justified it.
- Safe change isolation: proposed modifications do not affect current truth until explicitly merged.
- Brownfield extraction: generating an initial blueprint and contracts from an existing codebase, refined by a human. (Declared, see section 17.)
- Multi-target modules: a single module with implementations across multiple languages or reconcilers.
- Docstring generation and drift detection: the framework emits docstring templates grounded in map facts, and surfaces drift when authored docstrings diverge from the map.
- Edge validation: the reconciler verifies declared edges are realised in the reality layer and surfaces discrepancies.
- Rename propagation: restructuring the blueprint produces a single atomic change that updates all references across artefacts.

**Non-goals**

- A visual dashboard. Rendering the map as an interactive visual graph is a distribution concern for a downstream tool that consumes Cairn's JSON output, not a kernel capability.
- Multi-agent orchestration. The framework serves agents; it does not run them. Workflow tools like OpenSpec and cavekit occupy this space and are complementary consumers of Cairn.
- Function-level mapping in the blueprint itself. The blueprint is for module-level intent. Below-module data lives in reconciler output and is accessible via `cairn files <node>`.
- Procedural workflows. The blueprint is a map, not a recipe. Workflow systems are complementary, not absorbed.
- Project management features. Todos are tied to architectural work; general project admin lives elsewhere.

**Deliberate non-features**

- **Search/RAG infrastructure over raw sources.** The framework treats sources as opaque files linked by path and verified by checksum. It does not index their contents, chunk them, or embed them. If a project needs search over sources, that is a separate tool the human operates.
- **Project-specific workflow conventions.** "Run the scanner before committing." "Always write an ADR for `@security` changes." These live in a project-root `AGENTS.md` (or equivalent) and in per-artefact `rules` blocks in project config (see section 6). The framework does not formalise workflow, but it does compose project-level conventions into agent-facing instructions.

## 6. The kernel

Eight components. The kernel is generic; the reconcilers and artefact types it supports are configured separately.

1. **The blueprint.** Declares modules and above. Names, stable IDs, tags, edges, paths, artefact pointers. Roughly a dozen keywords.
2. **The artefact type system.** Pluggable. v1 ships with six types. The kernel does not hardcode them; they are declared in project config.
3. **The reconciler interface.** A domain-agnostic contract for "given a node, produce a fingerprint of its current state and a list of claimed sub-elements." v1 ships with one reference implementation: the code reconciler.
4. **The reconciliation engine.** Joins the parsed blueprint, the validated artefacts, and the output of registered reconcilers into the map. Runs integrity rules.
5. **The hooks.** Enforce reconciliation at task or commit boundaries.
6. **The CLI.** Primary interface. Exposes every kernel capability as a command.
7. **The query interface.** A typed query layer over the map. Initially CLI-only. Later wrapped by MCP, then LSP.
8. **The change system.** Change directories, delta semantics, archive operation. Keeps proposed modifications isolated from current truth.

A **summariser** is a pluggable ninth component, invoked by hooks when interface contradictions are detected. Optional.

**Project state layout**

```
./cairn.blueprint                    # Authored blueprint
./meta/                        # Authored artefacts
    contracts/
    decisions/
    research/
    sources/
    reviews/
    todos/
./meta/changes/                # Active change directories
    <change-name>/
./meta/changes/archive/        # Merged changes, date-prefixed
./.cairn/                      # Machine state (gitignored by default)
    state/
        interface-hashes.json
        scan-cache.json
    log.md
./map.md                     # Generated: map snapshot
```

**Project config**

```yaml
# cairn.config.yaml
reconcilers:
  - id: code
    version: 1
    config:
      tree_sitter_languages: [rust, typescript, python]
      ignore:
        - "**/node_modules/**"
        - "**/dist/**"
        - "**/target/**"
        - "*.lock"

artefact_types:
  # Overrides and additions to the v1 defaults
  
context: |
  Agents working in this repo should default to paraphrasing contract
  intent rather than quoting it verbatim.

rules:
  decision: |
    When proposing a decision that would change a node tagged @security,
    require review from two humans before accepting.
```

The `context` block is prepended to every instruction emitted by the query interface. The `rules` section is composed with artefact-specific templates. This is the dynamic-instruction pattern borrowed from OpenSpec; it keeps project conventions out of the blueprint while making them discoverable by agents at query time.

### 6.1 Ignore list semantics

The code reconciler determines which files to include via a layered list:

1. **Built-in defaults.** `node_modules/`, `target/`, `dist/`, `build/`, `.git/`, `.venv/`, `__pycache__/`, `.pytest_cache/`, `.vscode/`, `.idea/`, `.DS_Store`, `*.lock`, `*.log`, `coverage/`, `.next/`, `.nuxt/`. Shipped with the framework.
2. **`.gitignore`** at the project root is respected by default (these files don't matter to version control and mostly don't matter to Cairn either).
3. **`.cairnignore`** at the project root overrides, for cases where the project wants Cairn to ignore something git tracks (e.g., generated code committed to the repo but not really source).
4. **`cairn.config.yaml`** ignore entries add or remove from the combined list.

**Hardcoded protected paths.** The following are never ignored, regardless of user configuration: `./cairn.blueprint`, `./cairn.config.yaml`, `./meta/`, `./.cairn/`. These are Cairn's own state and must always be readable.

**Init-time assistance.** `cairn init` scans the project and proposes an initial ignore list based on what it finds. If it sees a `package.json`, it suggests `node_modules`. If it sees a monorepo structure with multiple `dist/` directories, it suggests those. The human confirms before the list is written. This matches OpenSpec's interactive init and removes the "400 warnings on first run" failure mode.

## 7. The blueprint grammar (concrete syntax)

Top-level declarations are `System`, `Container`, and `Module`. `Actor` is optional in v1. Nesting expresses ownership. Tags express domain. Arrows express dependency. Paths bind the node to the reality layer. Artefact pointers bind the node to its typed metadata. Every node carries a stable ID.

Containers are optional grouping nodes. A System can contain Modules directly without wrapping them in a Container. Containers exist to group related Modules when that grouping is meaningful; forcing every Module to nest inside a Container when there is no real grouping adds ceremony without value. Small projects may omit Containers entirely.

```
System SaaSPlatform "Core product" id "saas" @saas {

    Container API "Go backend" id "saas.api" @backend @rest {
        path "./apps/api"

        Module Auth "JWT authentication" id "saas.api.auth" @auth {
            path "./apps/api/auth"
            contract  "./meta/contracts/api/auth.md"
            todos     "./meta/todos/api/auth/"
            decisions "./meta/decisions/api/auth/"
            research  "./meta/research/api/auth/"
        }

        Module Billing "Stripe webhooks and ledger" id "saas.api.billing" @billing {
            path "./apps/api/billing"
            contract  "./meta/contracts/api/billing.md"
            todos     "./meta/todos/api/billing/"
            decisions "./meta/decisions/api/billing/"
            research  "./meta/research/api/billing/"
        }
    }

    Container DB "PostgreSQL" id "saas.db" @database {
        path "./infra/db"
    }
}

saas.api.auth -> saas.db "Reads user records"
saas.api.billing -> saas.db "Writes ledger entries"
```

**Rules**

- Every node has a name, a description, a stable ID, zero or more tags, optionally a path, and zero or more artefact pointers.
- **Stable IDs are required.** IDs are dotted, lowercase, and must be unique across the project. Names can change freely; IDs cannot.
- **Containers are optional.** A System may contain Modules directly. Containers exist to group related Modules when that grouping is structural; do not introduce a Container that holds a single Module solely to satisfy nesting.
- **ID depth is advisory.** Three levels (`project.subsystem.module`) scan cleanly. Four or more start to grate and usually indicate the grammar wants a different carving. Treat deeper IDs as a smell worth refactoring, not an error.
- Edges reference nodes by ID, not by name.
- Artefact pointers reference either a single file (for single-file types) or a directory (for multi-file types).
- Nesting is structural. A `Module` inside a `Container` inherits the container's boundary.
- **Paths may be a single path or a list of paths.** A module with implementations in multiple languages or reconciled by multiple reconcilers declares a list: `path ["./apps/core-rust", "./apps/core-ts"]`. Each path is claimed independently by the appropriate reconciler. The module's interface hash is computed per-target and the contract is checked against each. When targets diverge in interface, the scanner raises an interface contradiction naming the divergent targets.
- Paths must be unique across nodes (no two nodes claim the same path, including within multi-target lists).
- **Only leaf nodes own files by default.** An internal node can opt into file ownership with `owns-files: true` in its declaration.
- Ownership is resolved by most specific matching path. Ties are illegal at parse time.
- The project config declares an ignore glob list (see section 6.1). Default list excludes `node_modules`, lockfiles, build outputs, `.git`, common IDE directories. The `.gitignore` file is respected by default.
- Tag names are free-form but should be consistent within a project.

The grammar deliberately omits: styling, layout, view definitions, function-level declarations, type declarations, language-specific config, procedural steps.

## 8. Artefact types

Each type is defined by a name, a file format, a frontmatter schema, an integrity rule, and a freshness rule. v1 ships with six types, grouped by which chain they primarily serve. All artefact references use stable IDs.

**Provenance artefacts:** source, research, decision. Feed the reasoning chain.

**Authority artefacts:** contract, todo, review. Feed the enforcement chain.

Decision belongs to both.

### 8.1 Contract (authority)

Single markdown file per leaf node. The spec layer. Captures intent, public interface, invariants, dependencies, and tests.

```markdown
---
node: saas.api.auth
---

# Auth

## Purpose
## Public interface
## Invariants
## Dependencies
## Tests
```

Contracts are purely human-authored. Machine state (interface hashes, scan results) lives in `.cairn/state/`, not in the contract file.

**Integrity rule.** Every leaf node should have a contract. Missing contracts are warnings, except when a node transitions from ghost to synced (then required).

**Freshness rule.** When the module's interface hash changes, the scanner raises an **interface contradiction** between the code and the contract. The contradiction must be resolved (updating the contract or reverting the change) before the next commit.

### 8.2 Todo (authority)

Directory of markdown files per node. One todo per file.

```markdown
---
node: saas.api.auth
status: open  # open, in_progress, done, blocked
created: 2026-04-10
satisfies: "Public interface > validate_token"
---

# Implement JWT validation

Use the shared crypto module. Return a structured error type.
```

**Integrity rule.** Every todo must reference exactly one valid node ID. Orphan todos (referencing deleted nodes) are warnings. Coverage is informational in v1.

**Freshness rule.** None.

### 8.3 Decision / ADR (provenance + authority)

Directory of markdown files. Extended ADR format.

```markdown
---
id: dec.use-shared-crypto
nodes: [saas.api.auth, saas.api.billing]
status: accepted  # proposed, accepted, deprecated, superseded
date: 2026-04-10
revisited: 2026-04-10
revisit_triggers:
  - "Adding a second payment provider"
  - "Crypto module becomes unstable or is replaced"
informed_by:
  - type: research
    id: "res.crypto-sharing-2026-04"
  - type: source
    id: "src.security-audit-2026-03"
supersedes: []         # IDs of ADRs this one replaces
refines: []            # IDs of ADRs this one extends or clarifies
related: []            # IDs of ADRs that share context but do not supersede or refine
---

# dec.use-shared-crypto: Use shared crypto module for all signing

## Context
## Decision
## Consequences
```

**Integrity rule.** Every decision must reference at least one valid node ID. A decision whose nodes are all deleted must be reassigned, archived, or explicitly orphaned. Decisions are sparse by design; the framework does not warn about nodes without decisions.

**Status model.** A decision is either `proposed` (lives in a change directory), `accepted` (merged into the main tree, active), `deprecated` (no longer applies), or `superseded` (replaced by a later decision, which references it). There is no separate "accepted but unrealized" state because proposed changes live in change directories until their implications are reflected in the main tree.

**Cross-references.** `supersedes`, `refines`, and `related` are optional arrays of ADR IDs that capture relationships between decisions. `supersedes` implies the target ADR's status should be `superseded`; the framework flags the inconsistency if not. `refines` and `related` are informational. These fields feed `cairn rationale` so agents can follow decision chains.

**Freshness rule.** None enforced mechanically. `revisited` and `revisit_triggers` are for discipline, not enforcement.

**Role.** Decisions are the canonical "why" layer. When an agent or human proposes a change to a node, they should consult the decisions attached to that node and its direct neighbours first. The framework surfaces this via `cairn rationale <node>` (see section 12) and by default-including accepted decisions in neighbourhood queries.

### 8.4 Review (authority)

Directory of markdown files per node. Review notes, captured after a change. Three subtypes, distinguished by `review_type` in frontmatter.

```markdown
---
node: saas.api.auth
review_type: human  # human, agent_introspective, agent_cross_model
date: 2026-04-10
reviewer: george        # human name, agent identifier, or cross-model reviewer identifier
related_change: commit:a3f2c1
---

# Review notes
```

**Subtypes.**

- **`human`** (default). A person's review of a change. Free-form notes. The original and most common form.
- **`agent_introspective`.** Captured by the agent that implemented a change, expressing something it would have done differently had it not been constrained by the spec, or noting uncertainty it couldn't resolve. Turns implementation sessions into a spec-improvement signal. When the framework detects clusters of introspective reviews on the same node, that node's spec is a candidate for revision.
- **`agent_cross_model`.** A different model reviews the first model's output. Typically paired with dual-model workflows like cavekit's Codex integration. Findings can include severity (`critical`, `advisory`) to match downstream enforcement.

**Integrity rule.** Each review must reference one valid node ID. `review_type` must be one of the three declared values; missing defaults to `human`.

**Freshness rule.** None.

**Role.** Reviews close the loop between intent and implementation. Human reviews capture lessons learned. Agent introspective reviews surface spec-vs-behaviour tensions that would otherwise be lost. Cross-model reviews provide an adversarial check single-model self-review cannot.

**Design note (Declared).** The exact schema for `agent_introspective` and `agent_cross_model` (when they are generated, how they promote to proposed decisions, how they interact with the change system) is specified at Declared level only. Initial implementations should use the schema above as-is; refinements to schema will land once real usage data exists. This is an intentional tradeoff: ship the capability now rather than wait for perfect design.

### 8.5 Research (provenance)

Directory of markdown files. Synthesized understanding drawn from raw sources. Feeds decisions.

```markdown
---
id: res.crypto-sharing-2026-04
nodes: [saas.api.auth, saas.api.billing]
date: 2026-04-08
sources:
  - "src.crypto-library-comparison-2026"
  - "src.security-audit-2026-03"
tags: [cryptography, shared-modules]
---

# Evaluating shared vs per-module crypto implementations

## Question
Should Auth and Billing share a crypto module, or each maintain their own?

## What we considered
...

## What we're leaning toward
...
```

**Integrity rule.** Every research artefact must reference at least one node ID and at least one source ID. Orphan research (not linked from any decision) surfaces as info-level.

**Freshness rule.** None. Research is a snapshot of thinking at a point in time.

**Relation to decisions.** Research is input; decisions are output. Not all research produces a decision. Not all decisions require written research. When a non-trivial decision is made, its research should be linked via `informed_by`.

### 8.6 Source (provenance)

A raw source file or external reference with a sidecar manifest. Local files are immutable, enforced by checksum. External references (URLs) are tracked but cannot be checksummed without copying.

```markdown
---
id: src.security-audit-2026-03
file: ./meta/sources/security-audit-2026-03.pdf
sha256: a3f2c1d8b5e9...
verification: verified  # verified, external, unverified
type: document  # document, interview, transcript, link, research_paper, conversation
date: 2026-03-15
tags: [security, audit]
description: Third-party security audit report covering authentication flows.
---

# Security audit report, March 2026

Optional free-form notes about the source: provenance, context, why it matters.
```

**Verification states.**

- **verified.** Local file with a populated `sha256` that matches the file's current hash. The default for content the project controls. Immutability is mechanical.
- **external.** A URL or external reference where the project does not hold the bytes. `file` is a URL, `sha256` may be null. Cannot be checksummed. Trust is conventional: the project accepts that the source exists at that URL and may change over time.
- **unverified.** A local file with no `sha256` yet, or a claimed source whose integrity the project has not yet confirmed. Allowed temporarily; surfaces as a rationale tension until resolved to verified or external.

**Integrity rule.** Every source must be referenced by at least one research artefact or decision. Orphan sources are warnings. Files without sidecars are not tracked as sources. Sources in `unverified` state persist as rationale tensions until moved to `verified` or `external`. Sources marked `external` must have a non-null `file` that parses as a URL.

**Freshness rule.** For `verified` sources, if the underlying file's hash changes, the scanner raises a structural error: either the change is reverted or the file is re-registered as a new source with a new ID. For `external` and `unverified` sources, no freshness check.

**What the framework does not do with sources.** It does not index their contents, chunk them, embed them, or search inside them. Sources are opaque. The framework tracks that they exist, that their integrity state is known, and that they are cited by downstream artefacts.

## 9. Changes and deltas

Proposed modifications to the map never touch the main tree directly. They live in isolated change directories until merged. Pattern borrowed from OpenSpec.

### 9.1 Change directory structure

```
./meta/changes/add-notifications-module/
    proposal.md              # Intent, scope, rationale
    design.md                # Optional: technical detail
    blueprint.delta                # blueprint operations, if any
    contracts/
        api/
            notifications.md  # ADDED contract
            auth.md           # MODIFIED contract
    decisions/
        dec.extract-notification-logic.md  # New decision
    research/
        res.notification-provider-comparison.md
```

A change directory is any subdirectory of `./meta/changes/` that contains a `proposal.md`. The directory name is the change ID.

The directory structure mirrors the main `./meta/` tree. Any file in the change directory represents a proposed addition, modification, or removal of the corresponding file in the main tree.

### 9.2 Delta operations

Each artefact file in a change directory declares its operation via a frontmatter field:

```markdown
---
operation: modified  # added, modified, removed, renamed
renamed_from: saas.api.auth_v1  # only for renamed
# ... rest of frontmatter
---
```

For blueprint changes, `blueprint.delta` uses section markers following OpenSpec's pattern:

```
## ADDED Nodes
Module Notifications "Email and SMS dispatch" id "saas.api.notifications" @notifications {
    path "./apps/api/notifications"
    contract "./meta/contracts/api/notifications.md"
}

## ADDED Edges
saas.api.auth -> saas.api.notifications "Triggers verification emails"

## MODIFIED Nodes
# Full replacement of the modified node
Module Auth "JWT authentication and session management" id "saas.api.auth" @auth {
    path "./apps/api/auth"
    contract "./meta/contracts/api/auth.md"
}
```

### 9.3 Merge order

At archive time, the archiver applies deltas in this order:

1. **RENAMED**: update IDs and references first, so subsequent operations can find their targets.
2. **REMOVED**: delete nodes, edges, and artefacts before modifications could accidentally re-add them.
3. **MODIFIED**: replace content in place.
4. **ADDED**: append new content last.

This order is not optional; it is required for semantic consistency. Adopted directly from OpenSpec.

### 9.4 Archive

```
cairn archive add-notifications-module
```

The archive command:

1. Validates the change directory (every delta file parses, every referenced ID resolves, no orphans introduced).
2. Applies deltas to the main tree in the order above.
3. Runs a validation scan with the archiving change excluded from active-change discovery; if any structural error or interface contradiction results, aborts with full rollback.
4. Moves the change directory to `./meta/changes/archive/YYYY-MM-DD-<name>/`.
5. Runs a final output scan so generated status no longer lists the archived change as active.
6. Appends an entry to `.cairn/log.md`.

Archiving is atomic: either the change lands cleanly or nothing moves.

### 9.5 Change-aware queries

Queries default to the main tree (current truth) but accept flags for looking into active changes:

- `cairn changes`: list active change directories with one-line summaries.
- `cairn show <change>`: show what a change proposes.
- `cairn neighbourhood <node> --include-changes`: show the node plus any modifications pending in active changes.

Agents reasoning about "what is the system today" read the main tree. Agents reasoning about "what are we about to change" read change directories explicitly.

### 9.6 Rename

```
cairn rename <old-id> <new-id>
```

Renaming a node's ID is a structural change that must propagate through every reference. Manual rename is error-prone: the blueprint references the old ID in edges, artefacts reference it in frontmatter (`nodes`, `informed_by`, `supersedes`, and so on), and state files reference it in `.cairn/state/`.

`cairn rename` packages all of this as a single atomic change. The command:

1. Creates a change directory `./meta/changes/rename-<old-id>-to-<new-id>/`.
2. Generates a `blueprint.delta` with a `RENAMED` operation and any edge updates.
3. Walks every artefact file that references `<old-id>` and produces a modified copy in the change directory with the new ID in frontmatter.
4. Opens the change directory for human review.
5. The human runs `cairn archive` to merge, same as any other change.

After archive, references to `<old-id>` in the main tree are a structural error: either the rename was incomplete (framework bug) or external references exist and must be updated manually.

This resolves the ID stability question. IDs remain readable (reflecting structural position), and restructuring is safe because the rename propagates atomically.

## 10. Scanner, reconciliation, and generated outputs

The scanner runs on demand or on filesystem change. It does nine things:

1. Parses the blueprint file into a graph of declared nodes (with stable IDs).
2. For every registered reconciler, invokes it against its claimed nodes. The code reconciler walks the filesystem, parses source via Tree-sitter (and optionally LSP for richer semantics), and extracts symbols, signatures, and intra-module references.
3. For every declared artefact pointer, parses the artefact files and validates them against their type's frontmatter schema.
4. Joins all of this into the map, resolving ID references.
5. **Validates declared edges against reality-layer dependencies.** The reconciler compares declared edges to observed dependencies and surfaces divergence as rationale tension. (Declared, see section 17.)
6. **Checks docstrings against map facts.** For modules with authored docstrings, the reconciler compares the facts claimed in the docstring (module name, dependencies, tags) to the map. Divergence surfaces as rationale tension. (Declared, see section 17.)
7. Runs every artefact type's integrity rules and the global integrity rules.
8. Regenerates `map.md` at the project root and appends events to `.cairn/log.md`.
9. Writes updated state (interface hashes, scan cache) to `.cairn/state/`.

### 10.1 Node states

- **synced**: declared in blueprint, path exists, reconciler found expected content.
- **ghost**: declared in blueprint, path does not yet exist (planned but unimplemented).
- **orphaned**: reality content exists but no blueprint node claims it (via the all-files-claimed rule for the code reconciler).

### 10.2 Contradiction classes

The word "contradiction" from v0.4 is split into three classes with different enforcement levels:

**Structural errors (block commits unconditionally).**
- Duplicate node IDs.
- Path ties between two leaf nodes.
- Broken artefact pointer (file declared but missing).
- Artefact referencing a non-existent node ID.
- Source file whose SHA-256 does not match its sidecar.
- Orphan file under a claimed container with no leaf ownership.

**Interface contradictions (block commits until explicitly resolved).**
- Module interface hash (from reconciler) differs from the hash recorded in `.cairn/state/interface-hashes.json`. Resolution is either: update the contract and re-record the hash, or revert the reality-layer change.

**Rationale tensions (advisory; surface but never block).**
- Decision cites research or source that has since been deleted.
- Research not linked from any decision.
- Source not cited by any research or decision (orphan source).
- ADR `revisit_triggers` appear relevant based on recent changes.
- **Edge divergence.** A declared edge in the blueprint is not reflected in the reality layer (the reconciler finds no import, call, or reference from source to target). Or conversely, the reality layer contains a dependency the blueprint does not declare. Advisory rather than blocking because architectural edges can legitimately exist at coarser granularity than mechanical dependencies.
- **Docstring drift.** An authored docstring on a module claims facts that disagree with the map (wrong dependencies listed, wrong module name, contradictory description). Surfaces as tension so the human or agent can reconcile.
- **Multi-target interface divergence.** A module with multiple paths has different interface shapes across targets. Structural error if targets claim to implement the same contract but diverge; tension if intentional asymmetry is documented.

Only the first two classes are "contradictions" in the strong sense. The third is a tension: the framework is drawing attention, not making a claim about correctness.

### 10.3 map.md

Auto-generated catalogue of every node in the map.

```markdown
---
generated_by: cairn@0.5.0
scanned_at: 2026-04-13T10:23:00Z
---

# Project index

## Synced
- [saas.api.auth](./meta/contracts/api/auth.md), `./apps/api/auth`, @auth, @api
  (3 decisions, 2 research, 5 todos open)
- [saas.api.billing](./meta/contracts/api/billing.md), `./apps/api/billing`, @billing, @api
  (2 decisions, 1 research, 3 todos open)

## Ghost
- [saas.api.notifications], `./apps/api/notifications` (planned, change: add-notifications-module)

## Active changes
- add-notifications-module: 1 ADDED node, 1 MODIFIED node, 2 ADDED contracts

## Findings
- 0 structural errors
- 0 interface contradictions
- 2 rationale tensions (orphan research, one unresolved revisit trigger)
```

Versioned output. Never edited by hand.

### 10.4 .cairn/log.md

Append-only chronological record of reconciliation events. Runtime state, gitignored by default.

```markdown
## [2026-04-13 10:23] scan | 0 errors, 0 contradictions, 2 tensions
## [2026-04-13 10:18] contradiction | saas.api.auth interface hash changed
## [2026-04-13 09:55] hook-block | commit blocked: orphaned file ./scripts/migrate.sh
## [2026-04-13 09:40] archive | add-auth-rate-limiting merged; 1 MODIFIED contract, 1 ADDED decision
## [2026-04-12 17:30] summariser | regenerated contract draft for saas.api.billing
```

Event types: `scan`, `contradiction`, `tension`, `archive`, `summariser`, `hook-block`, `hook-pass`.

## 11. Hooks

Three kinds in v1.

**The structural hook.** Runs at task end or commit time. Blocks on any structural error.

**The interface hook.** Runs at the same boundaries. Computes current interface hashes, compares to recorded. Blocks if any differ without resolution.

**The tension hook.** Runs at scan time and surfaces findings. Never blocks. Output goes to `cairn lint` and is visible in `map.md`.

The optional summariser plugs into the interface hook: when an interface contradiction is detected, the summariser can propose an updated contract draft. The proposal is never auto-applied.

The framework is a fence around the authority chain, not a proof system. Hooks enforce structural integrity; they do not enforce semantic correctness.

## 12. The query interface

Primary form is a CLI. Same underlying queries exposed via MCP (v2) and LSP (v3) without changing the API. All queries accept either names or IDs; internal representation is always ID.

**Core queries**

- `cairn get <node>`: node metadata, tags, path, current state, list of attached artefacts.
- `cairn neighbourhood <node> [--include-types contract,todos,research] [--include-changes]`: the node, its inbound and outbound edges, and requested artefact types for directly connected nodes. **Default returns contracts and accepted decisions.** Todos, research, reviews, deprecated decisions, and active changes are opt-in.
- `cairn contract <node>`: the parsed contract.
- `cairn todos <node> [--status open]`: todos for a node.
- `cairn decisions <node> [--status accepted]`: decisions applying to a node.
- `cairn research <node>`: research artefacts linked to a node.
- `cairn sources <node>`: sources cited by research and decisions attached to a node. Transitive.
- `cairn rationale <node>`: convenience command. Returns accepted decisions attached to this node and its direct neighbours, plus the research and sources informing them. The canonical "why was it built this way" lookup.
- `cairn files <node>`: reality-layer elements claimed by a module. (For the code reconciler, files with extracted symbols.)
- `cairn dependents <node> [--transitive]`: nodes that edge into this one. Impact analysis. With `--transitive`, walks inbound edges recursively.
- `cairn depends <node> [--transitive]`: nodes this one edges into. Inverse of `dependents`. What does this node rely on?
- `cairn order [--from <node>] [--scope <id-prefix>]`: returns nodes in dependency-tier order. Tier 0 contains nodes with no outbound edges (or no outbound edges within scope); tier N contains nodes whose outbound targets are all in tiers 0..N-1. Cycles make the `order` query fail with a structural error naming the cycle participants, while basic map queries can still read the otherwise valid graph. With `--from`, restricts output to ancestors of the given node. With `--scope`, restricts to nodes whose ID starts with the given prefix. Enables downstream consumers (parallel orchestration, migration planning, rollout sequencing) to compute work order without re-implementing the graph traversal.
- `cairn changes`: list active change directories.
- `cairn show <change>`: show what a change proposes.
- `cairn archive <change>`: merge a change into the main tree.
- `cairn rename <old-id> <new-id>`: create a rename change that propagates to all references. See section 9.6.
- `cairn status`: composed view of "what's in flight": active change directories, open todos across nodes, and recent entries from `.cairn/log.md`. Answers "where is this project right now" without requiring the caller to compose three separate queries. Replaces the need for a dedicated session-state artefact.
- `cairn docstring <node> [--language <lang>]`: emits a docstring template for the module, grounded in map facts (name, description, declared dependencies, tags, contract sections). Language-aware: knows how to format for Rust, Python, TypeScript, Go. The human or agent fills in prose; the structural facts are guaranteed accurate because they came from the graph. (Declared, see section 17.)
- `cairn init --from-code`: generates an initial blueprint and contract set from an existing codebase. The reconciler extracts structural candidates; the summariser proposes names, descriptions, and tags; the human refines. See section 16. (Declared, see section 17.)
- `cairn refine`: re-runs brownfield extraction against the current codebase, proposing a delta against the existing blueprint rather than a fresh draft. (Declared, see section 17.)
- `cairn lint`: runs every integrity rule. Groups findings by class (structural, interface, tension).
- `cairn scan`: rescans, regenerates the map, `map.md`, and `.cairn/state/`.

**Design principle.** Small composable queries. Default responses are tight. Anything heavy is opt-in. The agent pulls what it needs.

Responses are assembled with project-level `context` prepended and artefact-type `rules` composed in. This matches OpenSpec's dynamic-instruction pattern: project conventions reach the agent at query time without being baked into the blueprint.

**Why accepted decisions are in the default neighbourhood response.** Decisions are sparse by design (low payload cost) and are the canonical "why" layer of the authority chain. An agent proposing a change needs to see the rationale that constrains the change before committing. Making this opt-in would mean most agents wouldn't opt in; making it default means safe changes by construction.

## 13. The summariser (optional component)

Pluggable callout invoked when an interface contradiction is detected. Proposes an updated contract. Never auto-applied.

**Configuration.** Project config specifies the inference backend: local (Ollama/llama.cpp), API (hosted provider), or disabled.

**Resolution actions.** When the summariser produces a draft, the human or agent has three first-class actions:

1. **Accept**: the draft replaces the existing contract; the interface hash is re-recorded.
2. **Edit**: the draft is written to an editable draft file under `.cairn/state/summariser/editable/`; a later explicit edited accept command replaces the contract with that edited content and re-records the interface hash.
3. **Discard**: the draft is thrown away; the contradiction remains unresolved until the human or agent takes another action (typically editing the contract directly or reverting the reality-layer change).

The discard path is first-class to prevent the summariser from subtly degrading into an auto-applier. The human remains the ultimate authority over contract content.

**Constraints.** Contracts have a soft size limit (suggested: a few hundred words per module). The summariser's prompt enforces this.

The summariser can optionally draft other artefact types (e.g. research from a conversation transcript), but contracts are the primary use case.

## 14. Phased build order

Each phase produces something usable on its own. Phases determine implementation order, not scope: every capability listed in section 5 is part of v1 regardless of which phase implements it.

**Phase 0: Rust project foundation.** Cargo workspace, CI pipeline (`cargo fmt --check`, `cargo clippy`, `cargo test`), project skeleton, and development infrastructure. Output: a buildable, testable Rust project with lint enforcement from day one.

**Phase 1: kernel.** blueprint grammar with stable IDs. The reconciler *interface* as an abstract contract. The code reconciler as its first implementation (Tree-sitter-based, minimal). Reconciliation logic for the contract artefact type only. CLI exposing `get`, `neighbourhood`, `contract`, `files`, `dependents`, `depends`, `order`, `lint`, and `scan`. Generate `map.md` and `.cairn/log.md` on scan. Output: a working CLI that answers structural queries against a reconciled map with contracts as the only artefact type. Validates the kernel architecture end-to-end.

**Phase 2: full artefact type system.** Add todos, decisions, reviews (with subtypes), research, and sources with integrity rules. Add the corresponding CLI commands (`rationale`, `sources`, `research`, `decisions`, `todos`, `status`). Output: the map carries full project metadata with provenance from source to reality.

**Phase 3: change system.** Add change directories, delta semantics, archive command, rename command, and change-aware queries (`changes`, `show`, `archive`, `rename`). Output: safe change isolation; proposed modifications never corrupt current truth; restructures propagate atomically.

**Phase 4: hooks.** Add the structural, interface, and tension hooks. Make them runnable as pre-commit or agent-task-end gates. Detect conflicts between concurrent active changes at authoring time, not only at archive time.

**Phase 2.5: graph explorer.** Interactive terminal UI for navigating the map graph. Visualises nodes, edges, and artefact summaries in a TUI. Output: a `cairn explore` command that lets humans and agents browse the graph interactively.

**Phase 5: edge validation and docstring generation.** The code reconciler grows semantic capability (either via deeper Tree-sitter analysis or by integrating LSP where available) to verify declared edges against observed imports and to check docstrings against map facts. Add `cairn docstring <node>` command. Both edge divergence and docstring drift surface as rationale tensions.

**Phase 6: multi-target and additional language support.** Modules with multiple paths are reconciled across all targets. Per-target interface hashes. The code reconciler grows beyond its initial language (likely Rust or TypeScript) to cover the primary languages of the project's real use cases.

**Phase 7: MCP wrapper.** Wrap CLI queries as an MCP server. Compose project `context` and `rules` into query responses. Makes the map directly queryable by agents in Claude Code, Cursor, and similar tools.

**Phase 8: summariser.** Add the optional summariser with pluggable backends and the three-action resolution (accept / edit / discard). Summariser also drives brownfield extraction (phase 9) and docstring generation (phase 5, where it proposes prose to fill templates).

**Phase 9: brownfield extraction.** `cairn init --from-code` and `cairn refine`. The reconciler extracts structural candidates from an existing codebase; the summariser names and describes; the human refines. This phase lands last because it benefits from every earlier capability: the blueprint is battle-tested, the reconciler is mature, docstring generation exists, and the summariser is working. Building brownfield earlier would require Cairn to reverse-engineer blueprint from code before knowing what good blueprint looks like.

**Phase 10 onward (distribution).** LSP server for editor UX (autocomplete on IDs, hover for node metadata, jump-to-definition on edges). Claude Code plugin packaging. Additional reconcilers for non-code domains (org structure, product BOMs, research programmes).

Phases 1 to 4 are the kernel. Phases 5 to 9 complete the v1 capability set. Phase 10 onward is distribution, packaging, and domain extension.

## 15. Brownfield extraction (Declared)

This section is at Declared maturity: the approach is decided, the command surface is named, but the detailed schema and prompt strategy are specified when closer to implementation.

**Approach.** LLM generates; human refines. This is the cavekit pattern and it matches how most Cairn users will already be working with AI. The alternative (human authors by hand, Cairn reconciles) is slower and higher-friction on codebases that already have significant structure.

**Mechanics.**

1. The reconciler walks the codebase and extracts structural candidates: top-level directories, major subdirectories that look like modules, file clusters with strong internal coupling.
2. The summariser receives the structural candidates plus a sample of the code in each, and proposes node names, descriptions, tags, and obvious edges.
3. The output is a draft `cairn.blueprint` in a special change directory `./meta/changes/brownfield-init/`, along with stub contracts for each proposed node.
4. The human reviews, renames, regroups, adds missing edges, deletes spurious nodes, and runs `cairn archive brownfield-init`.
5. On subsequent runs, `cairn refine` detects what's changed in the code since the last blueprint update and proposes a delta rather than a full redraft.

**What makes this robust.** The summariser is not asked to understand the codebase's architecture from scratch. It names and describes structure that the reconciler has already extracted. This avoids the typical "LLM hallucinates a plausible-sounding architecture that doesn't match the real code" failure mode.

**What's specified later.** The exact prompt template the summariser uses. The heuristics the reconciler uses to detect "module-like" structural candidates. The merge semantics for `cairn refine`. The frontmatter schema for the brownfield-init change directory.

## 16. Open questions

Many v0.5.1 open questions were resolved in v0.6 through the scope correction. What remains:

1. **The "shared utilities" pattern.** Where the line falls between "this is a module" and "this is two helpers." Convention, not grammar. Projects decide based on their coupling preferences; the framework does not formalise.

2. **Todo coverage strictness.** Default is loose (warnings for orphans, no coverage enforcement). Projects wanting strict enforcement set it in config. When to promote strict to default, if ever, is open.

3. **meta/ directory layout.** Current convention: organised by artefact type first, then by subsystem (`meta/decisions/kernel/`). Alternative: organised by node first, artefact type second (`meta/kernel/parser/decisions/`). Both have appeal. Spec does not mandate either. Real usage may surface a clear winner.

4. **Detailed schemas for agent review subtypes.** The `agent_introspective` and `agent_cross_model` review subtypes are declared in section 8.4 with a basic schema. Refinements needed once real usage exists: when exactly is introspective review generated (during apply, at verify, as a self-review step), how does it promote to proposed decisions, what severity taxonomy does cross-model review use, how do multiple reviews on the same node aggregate.

5. **Name.** *Cairn* is a working placeholder. Decision needed before any code ships.

**Resolved in v0.6** (all via the scope correction that separated capability from phasing):

- *Tree-sitter vs LSP.* Both. Tree-sitter for phase 1's basic reconciler; LSP added in phase 10 (distribution) when editor integration requires semantic depth.
- *Docstring binding format.* Rejected as originally framed. Replaced with `cairn docstring` command and scanner-level drift detection (section 12, phase 5).
- *Ignore-list default.* Specified in section 6.1: built-in defaults + `.gitignore` respected + `.cairnignore` overrides + `cairn.config.yaml` ad-hoc rules + hardcoded allowlist for Cairn's own files.
- *Edge validation.* Capability confirmed in v1 scope. Implementation in phase 5. Surfaces as rationale tension, not structural error.
- *Multi-target modules.* Path accepts list (section 7). Reconciler dispatches per-path. Phase 6 implementation.
- *Delta granularity.* Node-and-edge level. Finer grain unnecessary.
- *Session state.* `cairn status` command composes the existing queries. No new artefact.
- *Change directory composition.* Conflicts detected at authoring time via phase 4 hooks, not only at archive.
- *ID stability.* Resolved via `cairn rename` operation (section 9.6). IDs stay readable; restructures propagate atomically.

**Resolved in v0.5.1.** Source deduplication. Stable IDs prevent accidental duplication by construction.

## 17. What this spec deliberately does not commit to

- ~~A specific implementation language for the toolchain.~~ **Resolved:** Rust has been selected as the implementation language.
- A specific Tree-sitter setup or LSP integration strategy.
- A specific MCP framework or transport.
- A specific summariser model or provider.
- A visual rendering format.
- A specific test framework binding.
- A specific source file format beyond markdown sidecars for metadata.

All downstream. Pick when the kernel is proven.

---

## Changelog

**v0.6**

Scope revision. The driving realisation: "deferred to v2" and "non-goal for v1" had been used interchangeably, quietly discounting real capabilities from the product. v0.6 separates capability scope (what the framework IS) from implementation phasing (what ships first). Every item reframed below is part of v1; phases determine build order only.

- Added section 0.1: spec maturity levels (Declared, Designed, Implemented). Sections can exist at Declared level without blocking adjacent work.
- Moved brownfield extraction from non-goals to a declared v1 capability. Introduced `cairn init --from-code` and `cairn refine`. Brownfield is the final implementation phase because it benefits from every earlier capability being mature. Section 15 specifies the approach at Declared level.
- Made multi-target modules a v1 capability. Path accepts a single path or a list; the reconciler dispatches per-target. Enables polyglot modules and future non-code reconcilers within a single project.
- Added docstring generation and drift detection as a v1 capability. `cairn docstring <node>` emits a template grounded in map facts. Scanner checks authored docstrings for drift against the map. Resolves the original "optional `@cairn-id` tag" question by rejecting the tag and replacing it with a graph-grounded generation approach.
- Added edge validation as a v1 capability. The code reconciler compares declared edges against observed dependencies and surfaces divergence as rationale tension. Phase 5 implementation.
- Added `cairn rename <old-id> <new-id>` in section 9.6. Resolves ID stability by making restructures atomic and propagating through all references via the change system.
- Extended the Review artefact type with subtypes: `human` (default), `agent_introspective`, `agent_cross_model`. The last two are specified at Declared level; initial schema ships but refinements are expected from real use. Resolves the agent dissent open question without pre-committing to perfect design.
- Added `cairn status` as a composed query returning active changes, open todos, and recent log entries. Resolves the session-state question without introducing a new artefact.
- Specified ignore-list semantics in section 6.1: layered defaults, `.gitignore` respected, `.cairnignore` overrides, protected paths for Cairn's own state, init-time assistance.
- Revised phased build order to reflect the scope corrections: kernel → artefacts → change system → hooks → edge validation & docstrings → multi-target → MCP → summariser → brownfield → distribution.
- Cleaned up open questions: most v0.5.1 questions resolved through scope correction. Five remain.
- The kernel architecture from v0.5 stands unchanged. No changes to the two chains, stable IDs, contradiction classes, change directories, or artefact type system structure.

**v0.5.1**

- Made `Container` optional in the blueprint grammar. A `System` may contain `Module` nodes directly. Previously all Modules had to nest under a Container, which added ceremony for small projects.
- Added ID-depth advisory to section 7: three levels scan cleanly, four or more are usually a smell.
- Added three optional frontmatter fields to the Decision artefact type: `supersedes`, `refines`, `related`, all arrays of ADR IDs. Formalises the ADR-to-ADR linking that the bootstrap needed but v0.5 did not define. `supersedes` has integrity semantics (target ADR must have `status: superseded`); the other two are informational.
- Expanded the Source artefact type with a `verification` field taking one of three values: `verified` (local file with matching sha256), `external` (URL-referenced, no checksum possible), `unverified` (transitional state, surfaces as a rationale tension).
- Added `cairn order` query for dependency-tier ordering. Returns nodes grouped by topological tier, detecting cycles as `order`-specific structural errors. Supports `--from` and `--scope` for restricted queries. Enables downstream consumers (parallel orchestration, migration planning, rollout sequencing) without re-implementing graph traversal.
- Added `cairn depends <node>` as the inverse of `dependents`. Returns what a node relies on. Both now support `--transitive` explicitly.
- Resolved open question 10 (source deduplication). Stable IDs already handle this; no framework logic needed.
- Added open question about agent dissent and cross-model peer review as potential first-class artefact types (may collapse to one type with two subtypes).
- Added open question about meta/ directory layout (by type vs by node).
- Added open question about ID stability across blueprint restructuring. Surfaced when bootstrapping Cairn on itself.
- No structural changes to the kernel. v0.5 architecture stands.

**v0.5**

- Split the six-layer authority hierarchy into two linked chains: provenance (source → research → decision) and authority (decision → blueprint → contract → code). Decision is the hinge.
- Made the blueprint the current-state architectural truth. Decisions are rationale and commitments, not a higher authority than the blueprint.
- Introduced stable IDs as first-class node and artefact identifiers. All references in artefact frontmatter use IDs, not names or paths.
- Defined file ownership precisely: only leaf nodes own files by default; most-specific-path wins; ties are illegal; internal nodes can opt into ownership.
- Added explicit ignore-list handling in project config.
- Separated authored artefacts from machine state. All machine-maintained files move to `.cairn/state/`. `log.md` moves into `.cairn/` and is gitignored by default. `map.md` remains at project root and is versioned, with a `generated_by` frontmatter stamp.
- Removed `last_interface_hash` from the contract file.
- Replaced the overloaded "contradiction" with three classes: structural errors (block), interface contradictions (block), rationale tensions (advisory).
- Added SHA-256 checksums to source sidecars; source immutability is now mechanical, not conventional.
- Unified artefact cross-references around typed ID objects (`{type: research, id: "..."}`).
- Introduced the change directory pattern borrowed from OpenSpec. Proposed modifications live in `./meta/changes/<change-name>/` and do not affect the main tree until merged.
- Added delta operations (ADDED / MODIFIED / REMOVED / RENAMED) with a strict merge order (RENAMED → REMOVED → MODIFIED → ADDED), adopted from OpenSpec.
- Added the `cairn changes`, `cairn show <change>`, and `cairn archive <change>` commands.
- Added project-level `context` and per-artefact `rules` in project config, composed into query responses (dynamic-instruction pattern from OpenSpec).
- Made the summariser's discard action first-class alongside accept and edit.
- Made `Actor` optional in v1.
- Added OpenSpec to related work; clarified complementary relationship.
- Added open questions 8–12; renumbered accordingly.

**v0.4**

- Added Vocabulary section.
- Added six-layer authority hierarchy.
- Added research and source artefact types.
- Extended ADR frontmatter with `revisit_triggers` and `informed_by`.
- Added `cairn rationale`, `cairn research`, `cairn sources` commands.

**v0.3**

- Added map-first framing and related work sections.
- Added `map.md` and `log.md` scanner outputs.
- Renamed interface-change hook to contradiction hook.
- Added `cairn lint` and `cairn scan` commands.

**v0.2**

- Generalized contract pointer into typed artefact pointers.
- Added todos, decisions, and reviews as artefact types.
- Locked CLI-first build order.

**v0.1**

- Initial draft.
