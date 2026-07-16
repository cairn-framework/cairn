---
id: src.a2ui
file: https://github.com/a2ui-project/a2ui
verification: external
type: tool
date: 2026-07-16
---

# A2UI: Google's agent-to-user-interface protocol monorepo

Open protocol and reference implementations for LLM agents that "speak UI":
the agent streams declarative JSON describing UI intent, and the client
renders it with its own trusted native component catalog. Repo cloned and
read directly 2026-07-16 at commit `6cb4599ee501fa0b0b856f8f471b5e96a87125e4`
via six parallel deep-read agents; all claims below are tree-verified facts
with repo-relative citations.

Load-bearing facts for cairn:

- **Independent blueprint-driven development layer.** `blueprints/` holds a
  "Spec-Driven Development" system: language-agnostic module blueprints
  (`blueprints/modules/*.blueprint.md`, markdown with YAML frontmatter,
  270-490 lines, normative MUST/SHOULD rules with interface signatures in
  TypeScript-as-pseudocode), feature blueprints as deltas that get merged
  into module blueprints and archived on promotion
  (`specification/proposals/spec_driven_development.md`: the module
  blueprint "will find concise ways to describe the overall system which
  are smaller than a simple concatenation of the feature blueprints"), and
  per-codebase compliance files (`blueprints/codebases/*/codebase.blueprint.md`)
  that pin `module_blueprint_commit`, the git hash of the module blueprint
  commit the codebase complies with, so owed work is exactly
  `git diff <hash>..HEAD` on one file. A zero-dependency CI validator
  (`blueprints/validate_blueprints.py`) enforces frontmatter typing and
  referential integrity across the three tiers.
- **Blueprint layer is opt-in for agents.** `blueprints/README.md` opens
  with "IGNORE THIS ENTIRE blueprints/ FOLDER... UNLESS the user explicitly
  specifies"; four lifecycle skills (navigator, create, implement,
  maintenance; 30-60 line recipe cards each ending with the validator
  command) live in `blueprints/skills/` outside the agent-discovered
  `.agents/skills/` and are symlinked in on demand by `link_skills.sh`.
- **Versioned frozen spec siblings.** `specification/v0_8`, `v0_9`,
  `v0_9_1`, `v1_0` are complete immutable snapshots, each with JSON schemas
  (`json/`), component catalogs, prose docs, declarative test suites
  (`test/cases/*.json`, positive and negative cases run by
  `test/run_tests.py`), and an eval harness. Version READMEs declare
  lifecycle state and redirect contributions; each version ships a
  `docs/evolution_guide.md` with migration checklists split by audience
  ("For agents and servers" vs "For renderers and clients").
- **Flat adjacency-list data model.** The UI is a flat list of components
  referencing children by ID with exactly one `root`, chosen for LLM
  generation ergonomics and streaming: components arrive in any order,
  mutations reduce to ID-keyed upserts (`specification/v1_0/docs/`).
  Structure (updateComponents) and state (updateDataModel, JSON Pointer
  addressed) travel in separate channels.
- **Catalog trust model.** Never ships code: everything on the wire is data
  validated against a catalog of pre-approved component and function
  schemas (`specification/v1_0/json/catalog_definition.json`). The envelope
  is catalog-agnostic via a placeholder `$ref` any client can substitute.
  One catalog file triples as validation schema, capability negotiation
  currency, and LLM prompt material (v1.0 embeds a Markdown `instructions`
  field). Function calls carry `callableFrom` execution boundaries enforced
  at runtime.
- **Machine-readable error format for self-correction.** A standardized
  VALIDATION_FAILED shape (code, surfaceId, JSON-pointer path, message)
  exists "to ensure the LLM can understand and correct the error"
  (`specification/v1_0/docs/a2ui_protocol.md`); the inference SDK
  institutionalizes error repair and retries.
- **Shared test vectors, not shared test code.** `agent_sdks/conformance/`
  holds ~215 YAML cases across 7 suites with expected errors typed down to
  category, message regex, and structured detail; each SDK runs them
  through its own thin harness. The vector format is itself
  schema-validated (`conformance_schema.json`). CI workflows path-trigger
  on both SDK sources and `specification/**/json/**`. Caveat: only the
  Python and Kotlin SDKs actually execute the shared vectors at the pinned
  commit; Swift and Flutter tests do not reference them.
- **Eval scored by the production validator.** `eval/` (Inspect AI) scores
  LLM-generated payloads with the same SDK validator used in production
  (`eval/a2ui_eval/scorers.py`), then an LLM-judge second stage with a
  leniency rubric. CI runs date-seeded 100-sample slices daily at a 90%
  threshold (`eval/bin/run_ci_evals.py`, `.github/workflows/run_evals.yml`)
  and on failure on main auto-files a deduplicated GitHub issue embedding
  the markdown scorecard (`scripts/create_issue.sh`). Datasets are
  transcrypt-encrypted against training contamination. The v0.9
  "prompt-first" schema rewrite was driven by this measurement
  (`specification/v0_9/docs/evolution_guide.md`).
- **Two-layer renderer architecture.** `renderers/web_core` holds all
  framework-agnostic logic (message processing, state, Zod validation,
  catalog registry); `renderers/{lit,react,angular}` are thin adapters
  mapping component names to native widgets. `web_core` structurally diffs
  its Zod runtime schemas against the canonical spec JSON schemas in CI
  (`renderers/web_core/src/v0_9/schema/verify-schema.test.ts`).
- **Docs and agent onboarding.** `AGENTS.md` is the vendor-neutral agent
  source of truth with an explicit authority order (schemas first, catalogs
  second, protocol guides third); `.gemini/GEMINI.md` is a one-line pointer
  to it. A numbered example corpus (01 through 36) is ported forward
  through every spec version and CI-validated against each
  (`specification/scripts/validate.py`). `samples/README.md` is a
  maintenance registry table with dated demo videos; unmaintained community
  samples are quarantined with their own lockfiles and CI.
