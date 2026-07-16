---
id: res.a2ui-architecture-lessons
nodes:
  - cairn.root
date: 2026-07-16
method: primary
---

# What cairn can learn from A2UI (Google's Agent-to-User Interface project)

**Method:** Live inspection of a fresh clone of `a2ui-project/a2ui` at commit
`6cb4599ee501fa0b0b856f8f471b5e96a87125e4`, via a multi-agent workflow: six
parallel deep-readers over a2ui subsystems (their blueprints system, spec
versioning, protocol design, conformance/eval, multi-renderer architecture,
agent-experience layer) plus one cairn-baseline reader, a synthesis pass
producing 11 candidate lessons, and one adversarial verification agent per
lesson reading the actual cairn codebase with a default-skeptical stance.
Verdicts: 9 adapt, 2 reject, 0 adopt-as-proposed. A2UI file citations are
repo-relative to that clone; cairn citations are repo-relative here.

## Why A2UI is a relevant comparator

A2UI is an open protocol for agents to "speak UI": the agent streams
declarative JSON describing UI intent, and the client renders it with its own
trusted component catalog. Structurally it shares cairn's core bet: a
declarative format consumed and produced by LLMs, where a deterministic
validator (not the model) owns enforcement. Its slogan, "safe like data,
expressive like code", is a close cousin of cairn's fence-and-navigator
framing. It also independently evolved a `blueprints/` Spec-Driven Development
layer (language-agnostic module blueprints, per-codebase compliance files
pinned by git commit hash, a CI validator, four lifecycle skills) that is the
nearest external analogue to cairn found so far.

Key structural differences that limit transfer: a2ui is a multi-party wire
protocol with N independent implementations (Python, Kotlin, Swift, several
web renderers), so much of its machinery exists to keep strangers honest.
Cairn is a single Rust implementation whose four surfaces already share one
query registry. Lessons were rejected or reshaped wherever they cargo-culted
that multi-implementation burden.

## Verified lessons (9 adaptations, ordered by leverage per effort)

### 1. Close the hook-block self-correction loop (S)

A2UI standardizes a machine-readable validation error format explicitly so
the LLM can self-correct, and its inference SDK institutionalizes error
repair and retries (`specification/v1_0/docs/a2ui_protocol.md`).
Verification found cairn already has the remediation engine
(`src/query_api/handlers/remediate.rs`, exposed as `cairn remediate` /
`next` / `brief` and the `cairn_remediate` MCP tool), so a new per-code hint
table would create duplicate fix knowledge. The one dead end is the hook
block surface: `src/hooks/render.rs::render_human_verbose` lists findings
with no pointer to remediation, so a blocked agent must already know the
command exists. First step: on `ExitDecision::Block`, append a copy-registry
footer pointing at `cairn remediate --json` (or inline the top actions for
the blocking codes), reusing the existing engine.

### 2. Nudge decision compaction with an accumulation finding (S)

A2UI's feature blueprints are deltas that get merged into the module
blueprint and archived, with merge-as-compaction stated policy
(`specification/proposals/spec_driven_development.md`). Cairn has the full
supersession machinery (DecisionStatus::Superseded, `supersedes` links,
accepted-only default payloads in bundle and neighbourhood) but nothing ever
prompts anyone to use it: 67 of 68 dogfood decisions are still accepted, and
cairn.root carries 19 accepted decisions that every default bundle drags
along. First step: a deterministic count check in `src/scanner/checks.rs`
emitting an Info finding (allocate CA006) above a threshold (default 10),
with remediation prescribing the consolidating-decision workflow. Keep
compaction itself judgment work for humans or agents, never automatic.
Caveat surfaced during verification: one live decision file uses a
`superseded_by` field the parser does not read; the canonical direction is
the superseding decision's `supersedes` list, and guidance should say so.

### 3. Contract-vs-blueprint staleness via the existing fingerprint machinery (S)

A2UI pins each codebase blueprint to a `module_blueprint_commit` git hash so
the owed work is exactly `git diff <hash>..HEAD` on one file. The diagnosed
cairn gap is real but narrow: a contract authored against an old node shape
stays silently current when the blueprint changes (interface hash covers
code-vs-contract; CAIRN_BLUEPRINT_CHANGE_NO_DECISION covers
blueprint-vs-decision; nothing covers contract-prose-vs-blueprint-shape).
Git-hash pinning misfits cairn (one whole-graph blueprint file, scanner
deliberately git-free, and spec forbids machine state inside contract
files). First step: record the node's NodeFingerprint as a baseline in
`.cairn/state/contract-baselines.json` when a contract's interface hash is
recorded, and add a Warning check comparing current vs baseline fingerprint
(CAIRN_CONTRACT_STALE_VS_BLUEPRINT), naming the changed fields.

### 4. Finding-code coverage meta-test (S)

A2UI's conformance suites pin what schemas must reject, not just accept, and
the vector files are themselves schema-validated. Cairn already has strong
negative testing (40 tests asserting specific CAIRN_* codes in
`src/artefacts/registry/validate/tests.rs`, plus the spec-rule coverage
check CK004), so a declarative vector corpus would be redundant for a
single-implementation project. The genuine gap: about 20 of 103 emitted
finding codes have no triggering test and drifted in silently. First step:
`tests/finding_code_coverage.rs`, a meta-test that diffs emitted CAIRN_*
literals in src/ against asserted codes in tests, with a documented
allowlist seeded with the currently uncovered IO/plumbing codes, burned down
over time.

### 5. Anti-rot assertions on the existing example corpus (S)

A2UI ports one numbered example corpus through every spec version; it is
simultaneously docs, CI fixture, and prompt material. About 70 percent of
this already exists in cairn (examples/demo, reconcile_baseline fixtures,
insta wire-format snapshots, the dogfood CI gate), so a new six-fixture
corpus would be a third parallel surface with the same rot risk
CLAUDE.md already admits for `test/fixtures/cairn-bootstrap` ("may lag
behind root"). First step: a test that runs `cairn scan --json` against
examples/demo and cairn-bootstrap and asserts each against a committed
expected-findings JSON (demo expected clean), forcing the lagging fixture to
be fixed or deleted. Optionally add one deliberately finding-bearing
brownfield example later.

### 6. Advisory tag registry with typo detection (S core)

A2UI's catalog is a closed declared vocabulary that triples as validation
schema, capability negotiation, and LLM prompt material. Cairn tags are
freeform strings the parser merely collects, yet at least three code paths
key behavior off exact tag strings (`no-contract` in contract_coverage,
`no-test-coverage` in test_coverage, MatchRule::HasTag in brownfield
templates): a typo'd exemption tag silently fails to exempt, and the
cairn-dev skill wrongly documents tags as purely informational. Adaptation
keeps the vocabulary open, unlike a2ui: an opt-in `tags:` section in
cairn.config.yaml declaring known tags with one-line descriptions; when
present, the scanner emits an Info finding (allocate a CK-series code) for
undeclared tags. Info tier is load-bearing: `scan --strict` fails on
Warning, and this must warn, never block. Seed with the root repo's ten
tags, flagging the two behavior-affecting ones.

### 7. Surface real verification gates in the agent packet (M)

A2UI's codebase blueprints carry per-codebase test/lint/format commands and
its skills mandate running exactly those; AGENTS.md bans guessed build
sequences. Verification found the sharper cairn gap upstream of the
proposal: `cairn bundle` and `brief` do not surface even the global
configured gates. They emit static copy from `docs/design-system/copy.toml`
whose [brief] gates text still claims gates do not live in cairn,
contradicting the shipped `gates:` config feature (PR #340). First step:
thread the actual gate recipe (config gates or battery selection) into the
bundle/brief render paths and fix the stale copy. Per-node gate overrides on
`targets:` entries are a legitimate second step once a polyglot consumer
materialises, with supplement-not-replace precedence decided explicitly.

### 8. JSON Schemas for externally consumed wire formats (M)

A2UI treats machine-readable schemas as the normative core and re-proves
conformance empirically in CI (web_core structurally diffs its Zod types
against the spec schemas). Cairn's equivalent gap is real and worse than the
proposal claimed: `docs/integration-contract.md` promises envelope stability
with no mechanism; SCHEMA_VERSION is stamped but shapeless; the registry's
response_schema fields are bare string labels and only 2 of ~30 named
response types exist as structs (the rest are ad-hoc json! literals a field
rename ships through silently). Scope tightly to what external consumers
parse: map.json first (already proper serde structs, so schemars works
immediately), then the Finding wire shape and envelope, with a test
validating dogfood output against each schema file and a registry test
requiring every response_schema label to resolve. jsonschema as
dev-dependency only.

### 9. LLM authorability eval scored by the production scanner (M)

A2UI scores model-generated payloads with the exact production validator,
and that measurement drove its v0.9 prompt-first schema rewrite. Cairn's
blueprint syntax, blueprint.delta, and artefact frontmatter are increasingly
agent-authored (init --from-code, draft family, gap, change authoring) with
zero measurement of whether models produce them validly. Two reshapes:
measure convergence cost (iterations and tokens to a clean scan under the
deterministic repair loop) as the primary metric, first-shot validity
secondary; and drop the a2ui-scale apparatus (daily CI slices, auto-filed
issues, encrypted datasets) in favour of an on-demand harness of 5-10 task
prompts run against a temp copy of the bootstrap fixture, scored by
`cairn scan --strict` and `lint --json`, reusing the summariser's
LocalCommandBackend pattern and the METRIC-line convention. Results feed
format decisions (is nested block syntax or the delta section-marker format
an LLM failure hotspot?).

## Rejected lessons (verified redundant or misfit)

**Frozen side-by-side spec versions with evolution guides.** The substance
already exists: docs/spec.md carries a per-version changelog, state files
mandate version fields plus migration chains (conventions.md section 3), and
integration-contract.md declares stability tiers for the surfaces external
consumers actually parse. Frozen spec siblings would plant live-looking
pre-phase-2.6 terminology in docs/, exactly the stale-context hazard the
repo guards against. Revisit only at the first breaking change to an
externally consumed format.

**Layered agent instruction payloads (tight core plus on-demand packs).**
Already implemented: agent_guide.md is a 79-line core, and `cairn init`
emits per-workflow recipe-card skills (explore/propose/apply/archive plus
cairn-dev references) that activate on demand. The one residue is that
brownfield onboarding has no recipe card; that is a coverage gap, not a
layering lesson.

## Patterns considered and deliberately not imported

- Flat adjacency-list blueprint syntax: blueprint.delta already gives
  ID-keyed order-independent mutation; lexical nesting keeps ownership
  visible to humans.
- Commit-hash pinning as the primary drift mechanism: strictly weaker than
  cairn's content-addressable interface fingerprint (spec section 3.5).
- Ignore-by-default isolation of the blueprint layer: contradicts cairn's
  map-as-default-context value proposition.
- Multi-language SDK codegen, YAML conformance harnesses, pixel-parity
  renderer testing, wire-level version routing: multi-implementation
  machinery with no second implementation to keep honest.
- LLM-as-judge second-stage scoring: premature before any programmatic
  authorability eval exists.
- Proposals directory with runnable prototype DSLs: the change system plus
  the Declared/Designed/Implemented maturity ladder already provide the
  incubation path.

## Suggested sequencing

The five S-sized items (hook remediation footer, decision-accumulation
finding, contract baseline check, finding-code coverage meta-test, example
corpus assertions) are independent, each small enough for one change, and
each closes a loop on machinery cairn already owns. The tag registry
follows. The two M items (gate surfacing plus copy fix, wire-format schemas)
touch external contracts and deserve their own proposals. The authorability
eval is the most novel and should wait until the corpus assertions land so
it has a trustworthy fixture substrate.
